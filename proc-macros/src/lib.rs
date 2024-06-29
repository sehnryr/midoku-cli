use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A simple proc-macro that prints the time taken by a function to execute
#[proc_macro_attribute]
pub fn timeit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;
    let expanded = quote! {
        #(#attrs)*
        fn #name(#inputs) #output {
            let start = std::time::Instant::now();
            let result = {
                #block
            };
            let elapsed = start.elapsed();
            println!("{} took {:?}", stringify!(#name), elapsed);
            result
        }
    };
    TokenStream::from(expanded)
}

/// A simple proc-macro that prints the result of a function
#[proc_macro_attribute]
pub fn printit(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let block = &input.block;
    let expanded = quote! {
        #(#attrs)*
        fn #name(#inputs) #output {
            let result = {
                #block
            };
            println!("{:#?}", result);
            result
        }
    };
    TokenStream::from(expanded)
}
