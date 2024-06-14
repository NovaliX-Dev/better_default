use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use syn::DeriveInput;

// the macros a here so that the other files can have access to them
// (i know that's kinda weird)
macro_rules! error {
    ($span: expr, $message: literal $(,$format_args: expr)*) => {
        syn::Error::new($span, format!($message $(,$format_args)*))
    };

    ($error_vec: ident, $span: expr, $message: literal $(,$format_args: expr)*) => {{
        let tokens = syn::Error::new($span, format!($message $(,$format_args)*))
            .into_compile_error();

        $error_vec.push(tokens);
    }};
}

macro_rules! handle_error {
    ($expr: expr, $error_vec: ident) => {
        match $expr {
            Ok(val) => Some(val),
            Err(err) => {
                $error_vec.push(err.into_compile_error());

                None
            }
        }
    };
}

mod attrs;
mod default;
mod derive;
mod top_attribute;
mod traits;

const DEFAULT_IDENT: &str = "default";

#[proc_macro_derive(BetterDefault, attributes(default))]
pub fn better_default(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    derive::derive(input).into()
}
