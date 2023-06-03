use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn foo(_args: TokenStream, input: TokenStream) -> TokenStream {
    dbg!(&input);
    let parsed_input = parse_macro_input!(input as ItemFn);
    dbg!(parsed_input.to_token_stream().into())
}
