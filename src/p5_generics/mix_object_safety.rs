use std::fmt::Debug;
use std::rc::Rc;

trait MyTrait /*: Debug*/ {}

fn foo<T: MyTrait>(arg: &T) {
    //println!("{:?}", arg);
}

fn foo1(arg: &dyn MyTrait) {
    //println!("{:?}", arg);
}

fn foo2(arg: Box<dyn MyTrait>) {
    //println!("{:?}", arg);
}

#[derive(/*Debug, */ PartialEq)]
struct S;
impl MyTrait for S {}

#[test]
fn test() {
    let s = &S;

    foo(&S);
    //foo(&S {} as &dyn MyTrait); won't compile
    //it's a strange, because it should work
    //foo(Box::new(S {})); this also won't compile

    let trait_object: &dyn MyTrait = &S {} as &dyn MyTrait;
    foo1(trait_object);

    let trait_object1: Box<dyn MyTrait> = Box::new(S);
    foo2(trait_object1);
}

/* Dynamic sized types(DST), not allowed(derived) for embedded type: pub trait Sized{}
- [T]
- str
- dyn Trait
one way we can interact with them it's the reference to type: &T etc.
*/

/* Object Safety 5 rules:
1) Sized
    trait Foo: Sized { // может быть реализован только для Sized типов
    ...
    }
    // компилятор генерирует:
    impl Foo for dyn Foo { ... } // ???
    // мы запретили реализацию для DST, но dyn Trait это DST

2) self by value
    trait Foo {
        fn foo(self) { ... }
    }
    impl Foo for dyn Foo {
        fn foo(self: dyn Foo) {
            // ??? не можем разместить dyn Foo на стеке!
        }
    }

3) Self in args or return value
    trait Foo {
        fn foo(&self, other: &Self) { ... }
    }
    impl Foo for dyn Foo {
        fn foo(&self, other: &dyn Foo) {
            self.vtable.foo(&self.data, ???);
            // не можем проверить, что self и other имеют один тип!
        }
    }

4) static functions
    trait Foo {
        fn foo() -> i32 { ... }
    }
    impl Foo for dyn Foo {
        fn foo() -> i32 {
            // ??? какой код тут генерировать?
            // which exact trait object should we kept?
        }
    }

5) generic methods
    trait Foo {
        fn foo<A>(&self, a: A) { ... }
    }
    impl Foo for dyn Foo {
        fn foo<A>(&self, a: A) {
            self.vtable.???(self.data, a);
            // придется сгенерировать функции для всех возможных A!
        }
    }

fix:
    ➔ Избегать "неправильных" сигнатур
    ➔ Добавить на отдельные методы требование where Self: Sized
*/

/*
Object safety https://doc.rust-lang.org/reference/items/traits.html#object-safety
    Незнание конкретного типа накладывает ограничения на трейт-объекты:
    Отсутствует требование MyTrait: Sized.
    Нет ассоциированных функций.
    Нет generic методов.
    В методах self не принимается без ссылок.
    В методах не используется Self.
    Супертрейты должны быть object safe.
*/

// Examples of non-object safe traits.
trait NotObjectSafe {
    const CONST: i32 = 1; // ERROR: cannot have associated const

    fn foo() {} // ERROR: associated function without Sized
    fn returns(&self) -> Self; // ERROR: Self in return type
    fn typed<T>(&self, x: T) {} // ERROR: has generic type parameters
    fn nested(self: Rc<Box<Self>>) {} // ERROR: nested receiver not yet supported
}

struct S1;
impl NotObjectSafe for S1 {
    fn returns(&self) -> Self {
        S1
    }
}
fn run1() {
    //let obj: Box<dyn NotObjectSafe> = Box::new(S1); // ERROR
}

// Self: Sized traits are not object-safe.
trait TraitWithSize
where
    Self: Sized,
{
}

struct S2;
impl TraitWithSize for S2 {}
fn run2() {
    //let obj: Box<dyn TraitWithSize> = Box::new(S2); // ERROR
}

// Not object safe if `Self` is a type argument.
trait Super<A> {}
trait WithSelf: Super<Self>
where
    Self: Sized,
{
}

struct S3;
impl<A> Super<A> for S3 {}
impl WithSelf for S3 {}
fn run3() {
    //let obj: Box<dyn WithSelf> = Box::new(S3); // ERROR: cannot use `Self` type parameter*/
}

// This trait is object-safe, but these methods cannot be dispatched on a trait object.
trait NonDispatchable {
    // Non-methods cannot be dispatched.
    fn foo()
    where
        Self: Sized,
    {
    }

    // Self type isn't known until runtime.
    fn returns(&self) -> Self
    where
        Self: Sized;

    // `other` may be a different concrete type of the receiver.
    fn param(&self, other: Self)
    where
        Self: Sized,
    {
    }

    // Generics are not compatible with vtables.
    fn typed<T>(&self, x: T)
    where
        Self: Sized,
    {
    }
}

struct S4;
impl NonDispatchable for S4 {
    fn returns(&self) -> Self
    where
        Self: Sized,
    {
        S4
    }
}
fn run4() {
    let obj: Box<dyn NonDispatchable> = Box::new(S4);
    //obj.returns(); // ERROR: cannot call with Self return
    //obj.param(S);  // ERROR: cannot call with Self parameter
    //obj.typed(1);  // ERROR: cannot call with generic type
}
