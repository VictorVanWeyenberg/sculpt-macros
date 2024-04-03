use quote::{format_ident, quote};

pub use builder::*;
pub use root_builder::*;

use crate::Field;

mod root_builder;
mod builder;

const OPTIONS: &str = "Discriminants";

fn tokenize_fields(fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let tokenized_fields: Vec<proc_macro2::TokenStream> = fields.iter().map(tokenize_field).collect();
    quote! { #(#tokenized_fields)* }
}

fn tokenize_field(field: &Field) -> proc_macro2::TokenStream {
    if field.sculpt {
        let builder_name = format_ident!("{}_builder", field.type_name.to_lowercase());
        let builder_type = format_ident!("{}Builder", field.type_name);
        quote! { #builder_name: #builder_type, }
    } else {
        let option_name = format_ident!("{}", field.name.to_lowercase());
        let options_name = format_ident!("{}{}", field.type_name, OPTIONS);
        quote! { #option_name: Option<#options_name>, }
    }
}