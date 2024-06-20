use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sculpt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<syn::Item>(item.clone()) {
        Ok(parsed_item) => match parsed_item {
            syn::Item::Struct(item_struct) => {
                if item_struct.fields.is_empty() {
                    panic!("The root struct does not have any fields.")
                }
                item
            },
            _ => panic!("The sculpt attribute is meant for structs only.")
        }
        Err(err) => panic!("{}", err)
    }
}

#[proc_macro_attribute]
pub fn sculpt_alias(_attr: TokenStream, field_or_variant: TokenStream) -> TokenStream {
    let valid = if let Ok(item) = syn::parse::<syn::Item>(field_or_variant.clone()) {
        if let syn::Item::Struct(item_struct) = item {
            if item_struct.fields.is_empty() {
                panic!("The {} struct does not have any fields while it has alias(es) defined.", item_struct.ident)
            }
            true
        } else {
            false
        }
    } else if let Ok(variant) = syn::parse::<syn::Variant>(field_or_variant.clone()) {
        if variant.fields.is_empty() {
            panic!("The {} variant does not have any fields while it has alias(es) defined.", variant.ident)
        }
        true
    } else {
        false
    };
    if !valid {
        panic!("The sculpt_alias attribute is meant for structs or variants only.")
    }
    field_or_variant
}
