use quote::{format_ident, quote};

use crate::generate::tokenize_fields;
use crate::SculptableStruct;

pub fn generate_builder(sculptable: SculptableStruct) -> proc_macro2::TokenStream {
    let SculptableStruct { name: sculptable_name, fields: sculptable_fields, .. } = sculptable;
    let builder_name = format_ident!("{}Builder", sculptable_name);
    let callbacks_name = format_ident!("{}Callbacks", builder_name);
    let tokenized_fields = tokenize_fields(&sculptable_fields);
    quote! {
        #[derive(Default)]
        struct #builder_name<'a, T: #callbacks_name> { #tokenized_fields callbacks: &'a T }
    }
}