use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use syn::DeriveInput;

mod attrs;
mod derive;
mod field_assign;
mod traits;

const DEFAULT_IDENT: &str = "default";

#[proc_macro_derive(BetterDefault, attributes(default))]
pub fn better_default(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    derive::derive(input).into()
}
