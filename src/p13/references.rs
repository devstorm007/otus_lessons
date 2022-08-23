// Smart pointers owns th data and manage their lifecycle
// Box - unique possessing reference
// Rc - shared ref to immutable data

use std::cell::Ref;
use std::ops::Deref;

#[derive(PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn test_box1() {
    let mut a = Box::new(Point { x: 1.0, y: 2.0 });
    let b = Point { x: 1.0, y: 2.0 };
    assert_eq!(*a, b);
    // оператор * для разыменования указателя
    a.x = 5.0;
    // оператор . разыменует автоматически
    assert_ne!(*a, b);
}

#[test]
fn test_deref1() {
    let x = Box::new(0);
    *x; // *Deref::deref(&x);

    //x.foo(); // foo(&*x), если fn foo(&self)

    let mut y = Box::new(0);
    *y = 10; // *DerefMut::deref_mut(&mut y) = 10;
}
// https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082

// String is the same as vector of u8-bytes
// Box own data and dealloc data
// if T into Box<T> implement clone then Box also can be cloned
// Box::leak(self) -> &'static T allow continue lifecycle for T intentionally

// &*x helps into pattern matching

// Box::into_raw
// Box::from_raw

// Rc is like light garbage collector
// Rc refers to data on the heap and counts owners
// Rc clone() inc counter on 1
// drop(rc) dec counter on 1
// deallocate T when counter is 0

#[test]
fn test_rc1() {
    use std::rc::Rc;

    struct State {
        x: u32,
    }

    let a = Rc::new(State { x: 1 });
    let b = Rc::clone(&a);

    assert_eq!(a.x, 1); // также реализован Deref

    assert_eq!(b.x, 1);
    drop(b);

    assert_eq!(a.x, 1);
    drop(a);
}

/*
struct MyRc<T> {
    ptr: NonNull<RcBox<T>>,
}
struct RcBox<T> {
    value: T,
    counter: usize,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Box::leak(Box::new(RcBox { value, counter: 1 })).into()
        } // ...
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        self.ptr.counter += 1; // ???
        Self::from_ptr(self.ptr)
    }
}
*/

// Interior mutability - get mut access using shared ref

// Cell allow change data that implement Copy
#[test]
fn test_cell1() {
    use std::cell::Cell;

    let a = Cell::new(10);
    let b = Cell::new(20);
    let c = Cell::new(Some(1));

    assert_eq!(a.get(), 10); // get(&self) -> T: Copy
    a.set(5); // set(&self, value: T)
    a.swap(&b); // swap(&self, other: &Cell<T>)

    let x = a.replace(0); // replace(&self, value: T) -> T
    let y = c.take(); // take(&self) -> T: Default
}

#[test]
fn test_refcell1() {
    // For types that don't implement copy
    // It's like borrow checker but check in runtime instead of compile time
    // It's like single thread mutex
    use std::cell::RefCell;

    #[derive(Debug)]
    struct NonCloneable {
        value: i32,
    }

    let x = RefCell::new(NonCloneable { value: 10 });
    let value = x.borrow().value; // borrow(&self) -> Ref<'_, T>
    x.borrow_mut().value = 20; // borrow_mut(&self) -> RefMut<'_, T>
    assert_eq!(x.borrow().value, 20);

    let x = RefCell::new(NonCloneable { value: 10 });
    let x_ref = x.borrow();
    let x_ref2 = x.borrow(); // все ок
                             // drop(x_ref);
    let x_ref_mut = x.borrow_mut(); // паника в рантайме!
    println!("{:?}", x_ref);

    /*
    fn borrow(&'a self) -> Ref<'a, T> { ... }

    struct Ref<'a, T> {
        inner: &'a T,
    }

    impl<T> Deref for Ref<'_, T> {
        type Target = T;
        // ...
    }

    impl<T> Drop for Ref<'_, T> { ... }
    */
}

/*
Multithreading
*/
#[test]
fn test_mutex1() {
    // very heavy type
    use std::sync::Mutex;
    use std::sync::{LockResult, MutexGuard};

    let x = Mutex::new(10);

    // lock(&self) -> Result<MutexGuard<'_, T>, PoisonError>
    //let l: LockResult<MutexGuard<i32>> = x.lock();
    let mut mut_x = x.lock().unwrap();

    *mut_x = 5;
}

// popular pattern in single-thread code
#[test]
fn test_rc_with_refcell1() {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct State {
        x: u32,
    }

    let state: Rc<RefCell<State>> = Rc::new(RefCell::new(State { x: 1 }));
    let state_clone = Rc::clone(&state);
    state.borrow_mut().x = 10;
    // изменения отражаются во всех копиях
    assert_eq!(state.borrow().x, 10);
    assert_eq!(state_clone.borrow().x, 10);
}

// This cycle will give memory leaks, every variable block each other to drop
#[test]
fn test_rc_cycle1() {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    struct Node {
        next: Option<Rc<RefCell<Node>>>,
    }

    let x = Rc::new(RefCell::new(Node::default()));
    let y = Rc::new(RefCell::new(Node::default()));
    x.borrow_mut().next = Some(Rc::clone(&y));
    y.borrow_mut().next = Some(Rc::clone(&x));
}

// To avoid cycle and memory leaks use weak
// Into Rc we have two reference counters: strong and weak
#[test]
fn test_rc_weak1() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::rc::Weak;

    #[derive(Default)]
    struct Node {
        next: Option<Weak<RefCell<Node>>>,
    }

    let x = Rc::new(RefCell::new(Node::default()));
    let y = Rc::new(RefCell::new(Node::default()));

    x.borrow_mut().next = Some(Rc::downgrade(&y));
    y.borrow_mut().next = Some(Rc::downgrade(&x));

    let x1: Ref<Node> = x.borrow();
    let option: Option<&Weak<RefCell<Node>>> = x1.next.as_ref(); // won't compile I don't now why
    let x_next: Option<Rc<RefCell<Node>>> = option.map(|n| n.upgrade()).flatten();
}
