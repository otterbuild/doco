use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro]
pub fn init(_input: TokenStream) -> TokenStream {
    // Generate code that initializes the asynchronous runtime, the inventory for tests, and then
    // sets up the given function as the entry point for the program
    let initialization_and_function = quote! {
        struct Test {
            function: fn(),
        }

        inventory::collect!(Test);

        static ASYNC_RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> = std::sync::LazyLock::new(|| {
            tokio::runtime::Builder::new_current_thread().enable_all().build().expect("failed to create Tokio runtime")
        });

        fn main() {
            for test in inventory::iter::<Test> {
                (test.function)()
            }
        }
    };

    initialization_and_function.into()
}

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function that has been annotated with the `#[doco_derive::test]` attribute
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the function name, arguments, and body for the final test function
    let test_name = input_fn.sig.ident;
    let test_args = input_fn.sig.inputs;
    let test_body = input_fn.block;

    // Generate a test function that executes the test block inside doco's asynchronous runtime
    let test_function = quote! {
        fn #test_name(#test_args) {
            ASYNC_RUNTIME.block_on(async {
                #test_body
            });
        }

        inventory::submit!(Test {
            function: #test_name,
        });
    };

    test_function.into()
}
