use proc_macro2::Ident;
use quote::format_ident;

pub use field::*;
pub use pickable::*;
pub use sculptable_struct::*;

mod sculptable_struct;
mod pickable;
mod field;

pub const OPTIONS: &str = "Discriminants";

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

fn field_has_sculpt_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}