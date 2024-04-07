use proc_macro2::Ident;
use quote::{format_ident, quote};

pub use builder::*;
pub use root_builder::*;
pub use root_struct_build_impl::*;
pub use sculptable_struct::*;
pub use pickable::*;

mod root_builder;
mod builder;
mod root_struct_build_impl;
mod sculptable_struct;
mod pickable;

pub const OPTIONS: &str = "Discriminants";

fn tokenize_fields(fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let tokenized_fields: Vec<proc_macro2::TokenStream> = fields.iter().map(tokenize_field).collect();
    quote! { #(#tokenized_fields)* }
}

fn tokenize_field(field: &Field) -> proc_macro2::TokenStream {
    if field.is_sculptable() {
        let builder_name = format_ident!("{}_builder", field.type_name.to_lowercase());
        let builder_type = format_ident!("{}Builder", field.type_name);
        quote! { #builder_name: #builder_type, }
    } else {
        let option_name = format_ident!("{}", field.format_field_name());
        let options_name = format_ident!("{}{}", field.type_name, OPTIONS);
        quote! { #option_name: Option<#options_name>, }
    }
}

fn format_picker_name(name: &String) -> Ident {
    format_ident!("{}Picker", name)
}

fn format_builder_type(name: &String) -> Ident {
    format_ident!("{}Builder", name)
}

fn format_options_type(name: &String) -> Ident {
    format_ident!("{}{}", name, OPTIONS)
}

fn format_builder_field_name(name: &String) -> Ident {
    format_ident!("{}_builder", name.to_lowercase())
}

fn format_option_field(name: &String) -> Ident {
    format_ident!("{}", name.to_lowercase())
}

fn format_type(name: &String) -> Ident {
    format_ident!("{}", name)
}