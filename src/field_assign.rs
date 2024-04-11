use syn::{parse::Parse, Expr, Ident, LitInt, Token};

use crate::Span2;

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

impl ToString for FieldName {
    fn to_string(&self) -> String {
        match self {
            FieldName::Ident(ident) => ident.to_string(),
            FieldName::IntLiteral(int_literal) => int_literal.to_string(),
        }
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
