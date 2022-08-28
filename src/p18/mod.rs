use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{mem, thread};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub fn new(threads_count: usize) {
    let (sender, rx) = mpsc::channel::<Task>();
    let rx: Arc<Mutex<Receiver<Task>>> = rx.clone();
    thread::spawn(move || {
        // It's better to not do things like this:
        // while let Ok(t) = recv.lock().unwrap().recv() {
        // because lock() will leave all time during the 'while' - it's not obvious !!!
        // Good practice is to extract lock to the separate method
        while let Ok(t) = get_task(&rx) {
            t()
        }
    });
}

fn get_task(recv: &Mutex<Receiver<Task>>) -> Result<Task, RecvError> {
    recv.lock().unwrap().recv()
}

//__________________________________________________________________________________________________
pub struct ThreadPool {
    handles: Arc<Vec<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

impl ThreadPool {
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // send boxed task to threads - Why we have to use Box ???
        let task = Box::new(f);

        // to address struct field without move use .as_ref()
        let _ = self.sender.as_ref().unwrap().send(task);
    }
}

// interesting drop
impl Drop for ThreadPool {
    fn drop(&mut self) {
        //clear in Arc
        let arc = mem::replace(&mut self.handles, Arc::new(Default::default()));

        // if we have only one reference to arc it will be Ok to unwrap it
        if let Ok(handles) = Arc::try_unwrap(arc) {
            //manual drop value in option
            // we drop sender to free threads from while cycle because 'while' will get an error if all senders will gone
            mem::drop(self.sender.take());
            for jh in handles {
                jh.join().unwrap();
            }
        }
    }
}
