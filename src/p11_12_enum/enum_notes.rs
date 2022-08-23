#[derive(Copy, Clone)]
struct F<'a> {
    s: &'a str,
}

#[test]
fn test2() {
    let m: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    let e = m[1][0];
    println!("{}", e);
}

/*

Option<T> -> Result<T, E>: ok_or(e: E), ok_or_else(|| -> E {...}).
Result<T, E> -> Option<T>: ok().
Result<T, E> -> Option<E>: err().

Option<Option<T>> -> Option<T>: flatten().
Result <Result<T, E>, E> -> Result<T, E>: flatten(). // nightly

&Option<T> -> Option<&T>: as_ref().
&Result<T, E> -> Result<&T, &E>: as_ref().
Option<&T> -> Option<T>: copied()/cloned().
Result<&T, E> -> Result<T, E>: copied()/cloned().

Option<Result<T, E>> -> Result<Option<T>, E>: transpose().
Result<Option<T>, E> -> Option<Result<T, E>>: transpose().

*/

struct CustomErr;

#[test]
fn test_opt_res() {
    let a: Option<i32> = Some(42);
    let b: Result<i32, CustomErr> = a.ok_or(CustomErr);
    let _c: Option<i32> = b.ok();
    let d: Option<Option<i32>> = Some(a);
    let _e: Option<i32> = d.flatten();
    let _f: Option<&i32> = a.as_ref();
    let f: Option<&i32> = (&a).as_ref();
    let _g = f.copied();
    let h: Result<Option<i32>, CustomErr> = Ok(a);
    let i: Option<Result<i32, CustomErr>> = h.transpose();
    let _j: Result<Option<i32>, CustomErr> = i.transpose();
}
