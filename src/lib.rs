use std::time::UNIX_EPOCH;

use chrono::Local;
use proc_macro::{TokenStream, TokenTree};
use syn::{parse::Parse, parse_macro_input, LitStr};

extern crate proc_macro;

struct ArgsMacroInfo {
    format: Option<String>,
}

impl Parse for ArgsMacroInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(ArgsMacroInfo { format: None });
        }
        let format: LitStr = input.parse()?;
        return Ok(ArgsMacroInfo {
            format: Some(format.value()),
        });
    }
}

#[proc_macro]
pub fn compile_time(input: TokenStream) -> TokenStream {
    let ArgsMacroInfo { format } = parse_macro_input!(input as ArgsMacroInfo);
    match format {
        None => {
            let time = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            TokenTree::Literal(proc_macro::Literal::u128_suffixed(time)).into()
        }
        Some(format) => {
            let time = Local::now().format(&format).to_string();
            TokenTree::Literal(proc_macro::Literal::string(&time)).into()
        }
    }
}
