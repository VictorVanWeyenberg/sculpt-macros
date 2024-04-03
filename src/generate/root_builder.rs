use proc_macro2::Ident;
use quote::{format_ident, quote};

use crate::{Field, SculptableStruct};
use crate::generate::tokenize_fields;

pub fn generate_root_builder(sculptable: SculptableStruct) -> proc_macro2::TokenStream {
    let SculptableStruct { name: sculptable_name, fields: sculptable_fields, .. } = sculptable;
    let sculptable_ident = format_ident!("{}", sculptable_name);
    let builder_name = format_ident!("{}Builder", sculptable_name);
    let callbacks_name = format_ident!("{}Callbacks", builder_name);
    let tokenized_fields = tokenize_fields(&sculptable_fields);
    let tokenized_field_initializers = tokenize_field_initializers(&sculptable_fields);
    let first_field_type_pick_method_name = tokenize_first_field_type(&sculptable_fields);
    let tokenized_field_builders = tokenize_field_builders(&sculptable_name, &sculptable_fields);
    let field_names = tokenize_field_names(&sculptable_fields);
    quote! {
        pub struct #builder_name<'a, T: #callbacks_name> { #tokenized_fields callbacks: &'a T }

        impl<'a, T: #callbacks_name> #builder_name<'a, T> {
            pub fn new(t: &'a mut T) -> Self {
                Self { #tokenized_field_initializers callbacks: t }
            }

            pub fn build(mut self) -> #sculptable_ident {
                self.callbacks.#first_field_type_pick_method_name(&mut self);
                #tokenized_field_builders #sculptable_ident { #field_names }
            }
        }
    }
}

fn tokenize_first_field_type(fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let type_name = fields.get(0).expect("No fields for root sculptor.").type_name.to_lowercase();
    let type_name = format_ident!("pick_{}", type_name);
    quote! { #type_name }
}

fn tokenize_field_initializers(fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let tokenized_fields: Vec<proc_macro2::TokenStream> = fields.iter().map(tokenize_field_initializer).collect();
    quote! { #(#tokenized_fields)* }
}

fn tokenize_field_initializer(field: &Field) -> proc_macro2::TokenStream {
    if field.sculpt {
        let builder_name = format_ident!("{}_builder", field.type_name.to_lowercase());
        let builder_type = format_ident!("{}Builder", field.type_name);
        quote! { #builder_name: #builder_type::default(), }
    } else {
        let option_name = format_ident!("{}", field.type_name.to_lowercase());
        quote! { #option_name: None, }
    }
}

fn tokenize_field_names(fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let names: Vec<Ident> = fields.iter().map(|f| format_ident!("{}", f.name)).collect();
    quote! { #(#names, )* }
}

fn tokenize_field_builders(sculptable_name: &String, fields: &Vec<Field>) -> proc_macro2::TokenStream {
    let tokenized: Vec<proc_macro2::TokenStream> = fields.iter().map(|f| tokenize_field_builder(sculptable_name, f)).collect();
    quote! { #(#tokenized)* }
}

fn tokenize_field_builder(sculptable_name: &String, field: &Field) -> proc_macro2::TokenStream {
    let field_name = format_ident!("{}", field.name);
    if field.sculpt {
        let builder_name = format_ident!("{}_builder", field.type_name.to_lowercase());
        quote! { let #field_name = self.#builder_name.build(); }
    } else {
        let option_name = format_ident!("{}", field.type_name.to_lowercase());
        let panic_message = format!("No {} set for {}.", field.type_name.to_lowercase(), sculptable_name);
        quote! { let #field_name = self.#option_name.expect(#panic_message).into(); }
    }
}
