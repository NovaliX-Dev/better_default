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
