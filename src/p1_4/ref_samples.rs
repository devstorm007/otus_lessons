use crate::p1_4::my_slice;

fn ref_statements() {
    let mut a = 42;

    let b = &mut a;

    //drop(b); drop b
    let _ = *b + 1;
    println!("b = {}", b);

    drop(&a); //do nothing

    a += 1;
    foo(&a);
    a += 2;
    println!("a = {}", a);

    let c = &a;
    let d = &a;
    println!("c = {}", c);
    println!("d = {}", d);

    //*b += 1;
    //println!("b = {}", b);

    let ms1 = my_slice::MyStruct1 {};
    ms1.action();
}

fn foo(p0: &i32) {
    let mut tmp = *p0;
    tmp += 1;
    println!("tmp = {}", tmp);
}
