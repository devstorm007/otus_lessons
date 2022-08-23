#[derive(Debug)]
pub struct MyStruct1 {}

impl MyStruct1 {
    pub fn action(&self) {
        let arr = [0, 1, 2, 3, 4, 5];
        MyStruct1::foo(&arr[0..3]);

        let string = String::from("привет world");
        let str = "привет world";
        MyStruct1::bar(&string[0..6])
    }

    fn foo(ints: &[i32]) {
        println!("{:?}", ints)
    }

    fn bar(s: &str) {
        println!("{:?}", s)
    }
}
