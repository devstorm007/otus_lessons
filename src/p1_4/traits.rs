struct MyStruct {}

pub trait HasId {
    const ID: usize = 1; // associated constant
}
impl HasId for MyStruct {
    const ID: usize = 10;
}
//--------------------------------------------------------------------------------------------------
pub trait FromStr1: Sized {
    type Err; // associated type
    fn from_str(s: &str) -> Result<Self, Self::Err>;

    fn def_impl(s: &str) -> String {
        s.to_string()
    }
}

impl FromStr1 for String {
    type Err = std::convert::Infallible; //empty enum can't be instantiated

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.to_string())
    }

    fn def_impl(s: &str) -> String {
        s.to_ascii_lowercase().to_string()
    }
}

//--------------------------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
enum MyEnum {
    Single,
    Second,
}

trait Default {
    fn default() -> Self;
}

impl Default for MyEnum {
    fn default() -> Self {
        MyEnum::Single
    }
}

#[test]
fn run() {
    let s: MyEnum = Default::default();
    let s1 = MyEnum::default();
    assert_eq!(s, MyEnum::Single);
}

//--------------------------------------------------------------------------------------------------
//orphan rules
//https://github.com/Ixrec/rust-orphan-rules
//own traits for any types
//external traits for ONLY own types

trait Foo {}
trait Baz {}
trait Bar: Foo + Baz {}
//shortcut for trait Bar where Self: Foo + Baz {}
//Bar can be marker trait to link several traits
