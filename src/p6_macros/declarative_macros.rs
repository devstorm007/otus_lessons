//declarative macros are bed, they are full of design mistakes

#[derive(Debug)]
struct Foo;

/* declarative macros
macro_rules! $name {
    $rule0; //($matcher) => { $expansion }
    $rule1;
    ...
}
*/

macro_rules! four {
    () => {
        1 + 3
    };
}

#[test]
fn test1() {
    println!("{}", four!());
    println!("{}", four![]);
    println!("{}", four! {});
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

#[test]
fn test2() {
    println!("{}", add!(5, 1 + 3));
}

macro_rules! vec1 {
    ($($elem:expr), *) => {
        {
            let mut vec = Vec::new();
            $( vec.push($elem); )*
            vec
        }
    };
}

macro_rules! repeat2 {
    ($($i:ident)*, $($j:ident)*) => {
        $( let $i = (); )*
        $( let $j = (); )*
        //$( let $i: (); let $j: (); )* it won't work if ident length would be different repeat2!(a, u v);
    };
}

macro_rules! make_local {
    () => {
        let local = 42;
    };
}
macro_rules! set_local {
    () => {
        local1 += 2;
    };
}
/*
not compile
    make_local!();
    assert_eq!(local, 42);

    let local1 = 42;
    set_local!();
*/
macro_rules! set_a {
    ($a:ident, $e: expr) => {{
        let $a = 42;
        $e
    }};
}
#[test]
fn test3() {
    let _ = set_a!(a, a / 10);
}

macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            use std::collections::HashMap;
            let mut hm = HashMap::new();
            $(hm.insert($k, $v);)*
            hm
        }
    }
}

macro_rules! hashmap1 {
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            //:: in the beginning serve to avoid name conflict resolution
            ::std::collections::HashMap::from([$(($k, $v),)*])
        }
    }
}

#[test]
fn test4() {
    let my_hashmap = hashmap1! { 1 => 2, 3 => 4, };
    println!("This is my hashmap: {my_hashmap:?}");
}
