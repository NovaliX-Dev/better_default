use std::collections::{hash_map, HashMap};

use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, DataEnum, DataStruct, DeriveInput, Expr,
    Fields, Token,
};

use crate::{attrs, field_assign::FieldAssign, fields, Span2, TokenStream2};

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
    let body_tokens = fields::derive_fields(top_attribute.as_ref(), &data.fields, error_tokens);

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
        let attr = match attrs::find_default_attributes_and_handle_duplicates(&variant.attrs, error_tokens)
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
            attrs::find_default_attributes_and_handle_duplicates(&variant.attrs, error_tokens)
                .and_then(|attr| parse_top_attribute(attr, &field_names, error_tokens));

        let headless_default_tokens =
            fields::derive_fields(top_attribute.as_ref(), &variant.fields, error_tokens);
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
        attrs::find_default_attributes_and_handle_duplicates(&input.attrs, &mut error_tokens);

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
