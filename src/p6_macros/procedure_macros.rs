//book exxample
pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

#[test]
fn test1() {
    Pancakes::hello_macro();
}

/*
derive
attribute-like
function-like
*/

// function-like
/*
    use proc_macro::TokenStream;
    #[proc_macro]
    pub fn foo(input: TokenStream) -> TokenStream {
        input
    }

    use our_macro_crate::foo;
    fn main0() {
        foo!(let x = 1);
    }
*/

//attribute-like
/*
    use proc_macro::TokenStream;
    #[proc_macro_attribute]
    pub fn foo(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
        annotated_item
    }

    use our_macro_crate::foo;
    #[foo(some_input)] // или #[foo]
    struct S {}


    #[inner_attribute]
    mod m {
        #[inner_attribute]
        struct S;
    }

    mod m {
        #![outer_attribute] // is the same as #[inner_attribute] mod m {...
        #[inner_attribute]
        struct S;
    }
*/

//derive
/*
    use proc_macro::TokenStream;
    #[proc_macro_derive(Foo)]
    pub fn foo(item: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    use our_macro_crate::Foo;
    #[derive(Foo)]
    struct S {}


    use proc_macro::TokenStream;
    #[proc_macro_derive(Foo, attributes(foo_helper))]
    pub fn foo(item: TokenStream) -> TokenStream {
        TokenStream::new()
    }

    use our_macro_crate::Foo;
    #[derive(Foo)]
    struct S {
        #[foo_helper]
        field: i32,
    }
*/
