use std::mem::size_of;

struct Result1;
struct NetworkTask;
struct DbTask;

trait Task {
    fn execute(&self) -> Result1;
}

impl Task for NetworkTask {
    fn execute(&self) -> Result1 {
        todo!()
    }
}

impl Task for DbTask {
    fn execute(&self) -> Result1 {
        todo!()
    }
}

fn test() {
    //&dyn Task - trait object
    let tasks: Vec<&dyn Task> = vec![&NetworkTask {}, &DbTask {}];
    for task in tasks {
        task.execute();
    }

    const PTR_SIZE: usize = size_of::<usize>();
    const DOUBLE_PTR_SIZE: usize = PTR_SIZE * 2;
    assert_eq!(size_of::<&NetworkTask>(), PTR_SIZE); // ссылка
    assert_eq!(size_of::<Box<NetworkTask>>(), PTR_SIZE); // умный указатель в кучу
    assert_eq!(size_of::<&[NetworkTask]>(), DOUBLE_PTR_SIZE); // слайс
    assert_eq!(size_of::<&dyn Task>(), DOUBLE_PTR_SIZE); // трейт-объект
    assert_eq!(size_of::<Box<dyn Task>>(), DOUBLE_PTR_SIZE);
    //assert_eq!(size_of::<dyn Task>(), 0); // ошибка компиляции
}

//trait object is reference+reference to vtable
//              |-> ref to Self { a: i32, ... } - some structure implemented MyTrait
//&dyn MyTrait -|
//              |-> ref to vtable for structure
//                         0 -> fn foo(&self)
//                         1 -> fn bar(&self)
//                         2 -> fn to_string() -> String
