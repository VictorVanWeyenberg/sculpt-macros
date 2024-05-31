use proc_macro2::Ident;
use quote::format_ident;

const OPTIONS: &str = "Options";

pub struct SculptFormatter(String);

impl From<String> for SculptFormatter {
    fn from(value: String) -> Self {
        SculptFormatter(value)
    }
}

impl SculptFormatter {

    pub fn format_builder_type(&self) -> Ident {
        format_ident!("{}Builder", self.0)
    }

    pub fn format_options_type(&self) -> Ident {
        format_ident!("{}{}", self.0, OPTIONS)
    }

    pub fn format_builder_field_name(&self) -> Ident {
        format_ident!("{}_builder", self.0.to_lowercase())
    }

    pub fn format_option_field(&self) -> Ident {
        format_ident!("{}", self.0.to_lowercase())
    }

    pub fn format_type(&self) -> Ident {
        format_ident!("{}", self.0)
    }
}
