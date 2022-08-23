use std::fmt::Debug;

trait SomeTrait {}

fn foo1<Type: SomeTrait>(arg: &Type) {
    todo!()
}
//th same is but without T, sometomes it's not good option
fn foo2(arg: &impl SomeTrait) {
    todo!()
}

//generics
trait Operation {
    fn apply(&mut self) -> i32;
}

impl Operation for i32 {
    fn apply(&mut self) -> i32 {
        *self
    }
}

struct UnaryMinus<A>(A);

impl<A: Operation> Operation for UnaryMinus<A> {
    fn apply(&mut self) -> i32 {
        -self.0.apply()
    }
}

struct Plus<A, B> {
    a: A,
    b: B,
}

impl<A: Operation, B: Operation> Operation for Plus<A, B> {
    fn apply(&mut self) -> i32 {
        self.a.apply() + self.b.apply()
    }
}

enum Cache<T> {
    Operation(T),
    Result(i32),
}

impl<T: Operation> Operation for Cache<T> {
    fn apply(&mut self) -> i32 {
        match self {
            Cache::Operation(o) => {
                let result = o.apply();
                *self = Self::Result(result);
                result
            }
            Cache::Result(r) => *r,
        }
    }
}

fn main() {
    let three_plus_two = Plus { a: 3, b: 2 };

    let cached_three_plus_two = Cache::Operation(three_plus_two);

    let minus_one = UnaryMinus(1);

    let mut three_plus_two_minus_one: Plus<Cache<Plus<i32, i32>>, UnaryMinus<i32>> = Plus {
        a: cached_three_plus_two,
        b: minus_one,
    };

    let result = three_plus_two_minus_one.apply();
    print!("3 + 2 - 1 = {result}")
}

//trait objects
struct Values {
    a: i32,
    b: f32,
}
impl Values {
    pub fn get_debugable(&self, is_float: bool) -> &dyn Debug {
        if is_float {
            &self.b
        } else {
            &self.a
        }
    }
}

fn main1() {
    let values = Values { a: 42, b: 1.4 };
    let debugable = values.get_debugable(true);
    dbg!(debugable);

    let debugables: [&dyn Debug; 3] = [&1 as _, &1.5 as _, &"hello" as _];
    for debugable in debugables {
        dbg!(debugable);
    }
}

//Lifetime--------------------------------------------------------------------------------

// 'static - на всё время жизни программы
