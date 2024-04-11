use syn::Attribute;

pub fn find_attribute_unique<'a>(
    attrs: &'a [Attribute],
    ident: &str,
) -> Option<(&'a Attribute, Vec<&'a Attribute>)> {
    let mut iter = attrs
        .iter()
        .filter(|attr| attr.path().get_ident().is_some_and(|i| i == ident));

    let first = match iter.next() {
        Some(first) => first,
        None => return None,
    };

    let vec = iter.collect();

    Some((first, vec))
}

// the imports are here because i'm not sure where to place that function
use syn::spanned::Spanned;
use crate::TokenStream2;

pub fn find_default_attributes_and_handle_duplicates<'l>(
    attrs: &'l [Attribute],
    error_tokens: &mut Vec<TokenStream2>,
) -> Option<&'l syn::Attribute> {

    let (attr, duplicates) = match find_attribute_unique(attrs, crate::DEFAULT_IDENT) {
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
