use crate::macros::format::SculptFormatter;
use proc_macro2::Ident;
use quote::{format_ident, quote};

pub struct Field {
    name: Option<String>,
    type_name: String,
    sculpt: bool,
}

impl Field {
    pub fn pick(name: Option<String>, type_name: String) -> Self {
        Self {
            name,
            type_name,
            sculpt: false,
        }
    }

    pub fn sculpt(name: Option<String>, type_name: String) -> Self {
        Self {
            name,
            type_name,
            sculpt: true,
        }
    }

    pub fn to_builder_field(&self) -> proc_macro2::TokenStream {
        let type_formatter: SculptFormatter = self.type_name.clone().into();
        if self.sculpt {
            let builder_field = type_formatter.format_builder_field_name();
            let builder_type = type_formatter.format_builder_type();
            quote! {
                #builder_field: #builder_type
            }
        } else {
            let option_field = self.format_field_name();
            let option_type = type_formatter.format_options_type();
            quote! {
                #option_field: Option<#option_type>
            }
        }
    }

    pub fn tokenize_field_initializer(&self) -> proc_macro2::TokenStream {
        if self.sculpt {
            let builder_name = format_ident!("{}_builder", self.type_name.to_lowercase());
            let builder_type = format_ident!("{}Builder", self.type_name);
            quote! { #builder_name: #builder_type::default() }
        } else {
            let option_name = format_ident!("{}", self.type_name.to_lowercase());
            quote! { #option_name: None }
        }
    }

    pub fn to_builder_call(&self, builder_type: &Ident) -> proc_macro2::TokenStream {
        let variable = self.format_field_name();
        let type_formatter: SculptFormatter = self.type_name.clone().into();
        if self.sculpt {
            let builder_field = type_formatter.format_builder_field_name();
            quote! {
                let #variable = self.#builder_field.build()
            }
        } else {
            let message = format!("Field {} not set in {}.", variable, builder_type);
            quote! {
                let #variable = self.#variable.expect(#message).into()
            }
        }
    }

    pub fn format_field_name(&self) -> Ident {
        format_ident!(
            "{v}",
            v = self.name.as_ref().unwrap_or(&self.type_name).to_lowercase()
        )
    }

    pub fn format_type(&self) -> Ident {
        format_ident!("{}", self.type_name)
    }
}
