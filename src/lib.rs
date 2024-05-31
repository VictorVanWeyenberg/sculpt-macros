use proc_macro::TokenStream;

use syn::{Data, DeriveInput};

use macros::*;

mod macros;

#[proc_macro_derive(Sculptor, attributes(sculptable))]
pub fn derive_root_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (name, data_struct) = match ast.data {
        Data::Struct(s) => (ast.ident, s),
        Data::Enum(_) | Data::Union(_) => {
            panic!("Deriving Sculptor is only supported for structs.")
        }
    };
    let sculptable = build_sculptable(name, data_struct, true);
    sculptable.generate().into()
}

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
