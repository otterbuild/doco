use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function that has been annotated with the `#[doco_derive::test]` attribute
    let input_fn = parse_macro_input!(input as ItemFn);

    let test_name = input_fn.sig.ident;
    let test_args = input_fn.sig.inputs;
    let test_body = input_fn.block;

    let test_function = quote! {
        #[test]
        fn #test_name(#test_args) {
            #test_body
        }
    };

    test_function.into()
}
