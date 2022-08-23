use std::cmp::Ordering;
use std::ops;
use std::ops::{Add, AddAssign};

struct Cat {
    name: String,
    age: u32,
}

impl PartialEq<Cat> for Cat {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

impl Eq for Cat {}

#[test]
fn test1() {
    // f32::INFINITY is reflective
    let r = f32::INFINITY == f32::INFINITY;
    /*
    wont compile,
    because f32::NAN is non reflective and doesn't implement Eq byt implements PartialEq
    */
    //let r = f32::NAN == f32::NAN;
}

// PartialOrd some values can't be compared
impl PartialOrd<Cat> for Cat {
    fn partial_cmp(&self, other: &Cat) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

#[test]
fn test2() {
    let c = 1.partial_cmp(&2);
    let result = f64::NAN.partial_cmp(&1.0); //can't be compared
    assert_eq!(result, None);
}

impl Ord for Cat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[test]
fn test3() {
    let c = 1.clamp(1, 10);
}

impl Add for Cat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cat {
            name: format!("{} {}", self.name, rhs.name),
            age: self.age + rhs.age,
        }
    }
}

impl Add<u32> for Cat {
    type Output = Self;

    fn add(self, age: u32) -> Self::Output {
        Self {
            name: self.name,
            age: self.age + age,
        }
    }
}

//"+=" AddAssign
impl AddAssign<u32> for Cat {
    fn add_assign(&mut self, rhs: u32) {
        self.age += rhs
    }
}

// "*"
/*impl ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}*/
//String it's a smart pointer to &str located in the heap
// *String has all methods of &str

/*impl ops::DerefMut for String {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked(&mut *self.vec) }
    }
}*/

#[test]
fn test3s() {
    let s: String = "memory".to_string();
    let m: &str = s.trim(); //trim this is a method of &str
}
