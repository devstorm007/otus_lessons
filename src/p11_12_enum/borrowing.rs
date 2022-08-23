use std::any::Any;
use std::borrow::{Borrow, Cow};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr::null;

#[derive(Default, Clone)]
struct CatB {
    name: String,
    age: u32,
}

//won't compile because Borrow<CatB> auto implemented
/*impl Borrow<CatB> for CatB {
    fn borrow(&self) -> &CatB {
        todo!()
    }
}*/

/*
Обобщение Clone для любого типа позволяющего заимствовать данные.
Clone позволяет преобразовать &T -> T.
ToOwned позволяет преобразовать Borrow<T> -> T. Ex: &str -> String String::from
*/

/*impl ToOwned for str {
    type Owned = String;

    // clone data
    #[inline]
    fn to_owned(&self) -> String {
        unsafe {
            let x: &[u8] = self.as_bytes();
            let vec: Vec<u8> = x.to_owned();
            String::from_utf8_unchecked(vec)
        }
    }

    fn clone_into(&self, target: &mut String) {
        let mut b = mem::take(target).into_bytes();
        self.as_bytes().clone_into(&mut b);
        *target = unsafe { String::from_utf8_unchecked(b) }
    }
}*/

/*Cow - Clone-On-Write*/
/*
pub enum Cow<'a, B: ?Sized + 'a> where B: ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
    match *self {
        Borrowed(borrowed) => {
            *self = Owned(borrowed.to_owned());
            match *self {
                Borrowed(..) => unreachable!(),
                Owned(ref mut owned) => owned,
            }
        }
        Owned(ref mut owned) => owned,
    }
}
*/

#[test]
fn test1a() {
    let s = CatB::default();
    let mut d: Cow<CatB> = Cow::Owned::<CatB>(s);
    let _m: &mut CatB = d.to_mut();
    let _m2: &CatB = d.as_ref();
    let _m1: CatB = d.into_owned();

    let cat = CatB::default();
    let mut c: Cow<CatB> = Cow::Borrowed::<CatB>(&cat);
    let _f: &mut CatB = c.to_mut();
}

// Sized
/*Маркер, определяющий, имеет ли тип известный на стадии компиляции размер.
Выводится компилятором. По умолчанию, все типы-параметры имеют ограничение Sized.
Для снятия ограничения используется конструкция T: ?Sized.
Из трейтов с ограничением Sized нельзя делать трейт-объекты.*/
struct Foo<T>(T);
struct Bar<T: ?Sized>(T);

//struct FooUse(Foo<[i32]>); won't compile [i32] is not Sized
struct BarUse(Bar<[i32]>);

impl Drop for CatB {
    fn drop(&mut self) {
        println!("{} dropped", self.name)
    }
}

#[test]
fn test_any() {
    let mut any: Box<dyn Any> = Box::new(CatB::default());
    any = Box::new(42);
    let number = any.downcast_ref::<u32>();
    let cat = any.downcast_ref::<CatB>();
}

impl Hash for CatB {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
