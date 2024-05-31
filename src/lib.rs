use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sculpt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<syn::Item>(item.clone()) {
        Ok(parsed_item) => match parsed_item {
            syn::Item::Struct(_) => item,
            _ => panic!("The sculpt attribute is meant for structs only.")
        }
        Err(err) => panic!("{}", err)
    }
}
