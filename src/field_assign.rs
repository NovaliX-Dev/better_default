use std::{
    collections::{hash_map, HashMap},
    fmt::Display,
};

use syn::{parse::Parse, punctuated::Punctuated, Expr, Ident, LitInt, Token};

use crate::{Span2, TokenStream2};

pub(crate) enum FieldName {
    Ident(Ident),
    IntLiteral(LitInt),
}

impl FieldName {
    pub(crate) fn span(&self) -> Span2 {
        match self {
            FieldName::Ident(ident) => ident.span(),
            FieldName::IntLiteral(int_literal) => int_literal.span(),
        }
    }
}

impl Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FieldName::Ident(ident) => ident.to_string(),
            FieldName::IntLiteral(int_literal) => int_literal.to_string(),
        };
        f.write_str(str.as_str())
    }
}

impl Parse for FieldName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else {
            Ok(Self::IntLiteral(input.parse()?))
        }
    }
}

pub(crate) struct FieldAssign {
    pub(crate) ident: FieldName,
    _colon: Token![:],
    pub(crate) value: Expr,
}

impl Parse for FieldAssign {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub(crate) fn parse_punctuated_unique(
    punctuated: Punctuated<FieldAssign, syn::token::Comma>,
    field_names: &[String],
    error_tokens: &mut Vec<TokenStream2>,
) -> HashMap<String, Expr> {
    let mut hash_map = HashMap::with_capacity(punctuated.len());
    for field in punctuated {
        let ident_str = field.ident.to_string();

        if !field_names.contains(&ident_str) {
            error!(
                error_tokens,
                field.ident.span(),
                "unknown field `{}`",
                ident_str
            );
            continue;
        }

        if let hash_map::Entry::Vacant(e) = hash_map.entry(ident_str) {
            e.insert(field.value);
        } else {
            error!(
                error_tokens,
                field.ident.span(),
                "this field is already declared."
            );
            continue;
        }
    }

    hash_map.shrink_to_fit();
    hash_map
}
