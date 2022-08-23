pub struct Trie {}
impl Trie {
    fn new() -> Trie {
        Trie {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let trie = Trie::new();
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test() {
        let (x, y) = setup();
        assert_eq!(y, 0); // или assert!(y == 0);
        assert_ne!(x, 0); // или assert!(x != 0);
        x / y;
    }

    fn setup() -> (i32, i32) {
        (1, 3)
    }
}

//tests/integration.rs
mod my_library_integ_tests {}
// tests/integration.rs
use my_library_integ_tests::*;

#[test]
fn integration_test1() {
    // ...
}

#[test]
fn integration_test2() {
    // ...
}

//doc tests ******************************************************************
/// This is my awesome Trie data structure.
/// See how easily you can use it:
/// ```
/// let mut trie = Trie::new();
/// trie.insert(33, "a");
/// # assert!(!trie.is_empty());
/// ```
pub struct Trie1 {
    // ...
}

/// ```no_run
/// loop {
///
//println!("Hello");
/// }
pub struct Trie2 {}

/// ```
/// ```compile_fail
/// let x = 5;
/// x += 2; // shouldn't compile!
/// ```
pub struct Trie3 {}

//benches *****************************************************************
/*
Как тесты, но отмечены атрибутом #[bench] (nightly-only)
Располагаются в директории benches/
Лучше использовать сторонние библиотеки, например https://github.com/bheisler/criterion.rs
*/

//execution ***************************************************************
/*
cargo test [<substring_of_test_name>]
cargo test -- --show-output (чтобы вывести stdout)
cargo test -- --test-threads=1 (чтобы запустить тесты в один поток)
but better use https://lib.rs/crates/serial_test (чтобы запустить тесты в один поток)
*/

/*
lib structure
    Cargo.toml
    Cargo.lock # не включаем в git
    src/
        lib.rs
        some_module.rs
        another_module/
            mod.rs
    tests/
        integration_test.rs
    benches/
        benchmark.rs
    examples/
        simple.rs

examples
    Отдельные исполняемые программы в директории examples/
    Могут использовать публичный API библиотеки
    Могут использовать зависимости библиотеки
    Запустить можно командой cargo run --example <name>
*/

/*
app structure
    Cargo.toml
    Cargo.lock #  включаем в git
    src/
        main.rs
        lib.rs
        some_module.rs
        another_module/
            mod.rs
    tests/
        integration_test.rs

// main.rs
    use my_library::PublicApi;
    fn main() {
        PublicApi::use();
    }
// lib.rs
    mod some_module;
    mod another_module;
*/
