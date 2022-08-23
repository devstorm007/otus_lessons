//use frunk::prelude::*;

type I32F32Bool = Coprod!(i32, f32, bool);

#[test]
fn test1() {
    let mut co1 = I32F32Bool::inject(3);
    let mut co2 = I32F32Bool::inject(true);

    let get_from_1a: Option<&i32> = co1.get();
    let get_from_1b: Option<&bool> = co1.get();

    assert_eq!(get_from_1a, Some(&3));
    assert_eq!(get_from_1b, None);

    let v = vec![co1, co2];
}
