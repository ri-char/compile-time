use std::time::UNIX_EPOCH;

use proc_macro::TokenStream;
use quote::quote;

extern crate proc_macro;

#[proc_macro]
pub fn compile_time(_input: TokenStream) -> TokenStream {
    let time = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    TokenStream::from(quote! {#time})
}
