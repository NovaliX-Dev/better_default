use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, DataEnum, DataStruct, DeriveInput, Expr,
    Fields, Ident, Token,
};

use crate::{attrs, field_assign::FieldAssign, traits::JoinTokens, Span2, TokenStream2};

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

fn find_default_attributes_and_handle_duplicates<'l>(
    attrs: &'l [Attribute],
    error_tokens: &mut Vec<TokenStream2>,
) -> Option<&'l syn::Attribute> {
    let (attr, duplicates) = match attrs::find_attribute_unique(attrs, crate::DEFAULT_IDENT) {
        Some(tuple) => tuple,
        None => return None,
    };

    for duplicate in duplicates {
        error!(
            error_tokens,
            duplicate.meta.span(),
            "this attribute is already declared."
        );
    }

    Some(attr)
}

struct DefaultValue {
    ident: Option<Ident>,
    value: TokenStream2,
}

impl ToTokens for DefaultValue {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if let Some(ident) = &self.ident {
            ident.to_tokens(tokens);
            Token![:](Span2::call_site()).to_tokens(tokens);
        }

        self.value.to_tokens(tokens);
    }
}

fn get_field_default_values(
    top_default_values: Option<&HashMap<String, Expr>>,
    fields: &Fields,
    error_tokens: &mut Vec<TokenStream2>,
) -> Vec<DefaultValue> {
    let mut default_values_vec = Vec::with_capacity(fields.len());
    for (i, field) in fields.iter().enumerate() {
        let ident = field.ident.to_owned();
        let ident_str = ident
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or(i.to_string());

        let ty = &field.ty;

        let default_tokens =
            find_default_attributes_and_handle_duplicates(&field.attrs, error_tokens)
                .and_then(|attr| handle_error!(attr.meta.require_list(), error_tokens));
        
        let top_default_tokens = top_default_values.and_then(|h| h.get(&ident_str))
            .map(|expr| expr.to_token_stream());
        
        if let Some(meta_list) = default_tokens {
            if top_default_tokens.is_some() {
                error!(error_tokens, meta_list.span(), "a default value for this field already exists in the top default attribute.");
            }
        }

        let default_tokens = default_tokens
            .map(|meta| meta.tokens.to_token_stream())
            .or(top_default_tokens)
            .unwrap_or(quote! { <#ty as Default>::default() });

        let default_value = DefaultValue {
            ident,
            value: default_tokens,
        };
        default_values_vec.push(default_value);
    }

    default_values_vec
}

fn derive_fields(
    top_default_values: Option<&HashMap<String, Expr>>,
    fields: &Fields,
    error_tokens: &mut Vec<TokenStream2>,
) -> TokenStream2 {
    if let Fields::Unit = fields {
        return TokenStream2::new();
    }

    let default_value_vec = get_field_default_values(top_default_values, fields, error_tokens);

    let delimiter = match fields {
        Fields::Named(_) => proc_macro2::Delimiter::Brace,
        Fields::Unnamed(_) => proc_macro2::Delimiter::Parenthesis,
        Fields::Unit => unreachable!(),
    };

    let flattened_tokens = default_value_vec.join_tokens(&Token![,](Span2::call_site()));
    proc_macro2::Group::new(delimiter, flattened_tokens).into_token_stream()
}

fn parse_top_attribute(
    attr: &Attribute,
    field_names: &[String],
    error_tokens: &mut Vec<TokenStream2>,
) -> Option<HashMap<String, Expr>> {
    let list = match &attr.meta {
        syn::Meta::Path(_) => return None,
        syn::Meta::List(list) => list,
        syn::Meta::NameValue(nv) => {
            let ident = attr.path().get_ident().unwrap();
            error!(
                error_tokens,
                nv.span(),
                "expected attribute arguments in parentheses (`{ident}(...)`) or single `{ident}`"
            );

            return None;
        }
    };

    let punctuated: Punctuated<FieldAssign, Token![,]> = handle_error!(
        list.parse_args_with(Punctuated::parse_terminated),
        error_tokens
    )?;
    
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

        if hash_map.contains_key(&ident_str) {
            error!(
                error_tokens,
                field.ident.span(),
                "this field is already declared."
            );
            continue;
        } else {
            hash_map.insert(ident_str, field.value);
        }
    }

    hash_map.shrink_to_fit();
    Some(hash_map)
}

fn get_fields_name(fields: &Fields) -> Vec<String> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|i| i.ident.as_ref().unwrap().to_string())
            .collect(),
        Fields::Unnamed(_) => (0..fields.len()).map(|i| i.to_string()).collect(),
        Fields::Unit => Vec::new(),
    }
}

fn derive_struct(top_attribute: Option<&Attribute>, data: &DataStruct, error_tokens: &mut Vec<TokenStream2>) -> TokenStream2 {
    let field_names = get_fields_name(&data.fields);
    let top_attribute = top_attribute.and_then(|attr| parse_top_attribute(attr, &field_names, error_tokens));
    let body_tokens = derive_fields(top_attribute.as_ref(), &data.fields, error_tokens);

    quote! { Self #body_tokens }
}

fn default_enum(top_attribute: Option<&Attribute>, data: &DataEnum, error_tokens: &mut Vec<TokenStream2>) -> TokenStream2 {
    if let Some(attr) = top_attribute {
        error!(
            error_tokens,
            attr.meta.span(),
            "top default attributes are not allowed on enums."
        );
    }

    let mut default_variant = None;
    for variant in &data.variants {
        let attr = match find_default_attributes_and_handle_duplicates(&variant.attrs, error_tokens)
        {
            Some(value) => value,
            None => continue,
        };

        if let Some((ident, _)) = default_variant.as_ref() {
            error!(
                error_tokens,
                attr.meta.span(),
                "the default value is already assigned to `{}`",
                ident
            );

            continue;
        }

        let field_names = get_fields_name(&variant.fields);
        let top_attribute =
            find_default_attributes_and_handle_duplicates(&variant.attrs, error_tokens)
                .and_then(|attr| parse_top_attribute(attr, &field_names, error_tokens));

        let headless_default_tokens =
            derive_fields(top_attribute.as_ref(), &variant.fields, error_tokens);
        let ident = &variant.ident;
        let default_tokens = quote! { Self::#ident #headless_default_tokens };

        let ident = variant.ident.to_owned();
        default_variant = Some((ident, default_tokens));
    }

    match default_variant {
        Some((_, tokens)) => tokens,
        None => {
            error!(
                error_tokens,
                Span2::call_site(),
                "the default variant has not been set."
            );

            quote! { panic!() }
        }
    }
}

pub fn derive(input: DeriveInput) -> TokenStream2 {
    let mut error_tokens = Vec::new();

    let top_attribute =
        find_default_attributes_and_handle_duplicates(&input.attrs, &mut error_tokens);

    let tokens = match &input.data {
        syn::Data::Struct(data) => derive_struct(top_attribute, data, &mut error_tokens),
        syn::Data::Enum(data) => default_enum(top_attribute, data, &mut error_tokens),
        syn::Data::Union(data) => {
            return error!(
                data.union_token.span(),
                "this derive is not implemented for unions."
            )
            .into_compile_error();
        }
    };

    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let error_tokens: TokenStream2 = error_tokens.into_iter().collect();

    quote! {
        impl #impl_generics Default for #ident #type_generics #where_clause {
            fn default() -> Self {
                #tokens
            }
        }

        #error_tokens
    }
}
