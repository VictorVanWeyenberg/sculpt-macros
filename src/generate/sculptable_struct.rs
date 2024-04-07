use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataStruct, Fields, Type};
use crate::generate::{format_builder_field_name, format_builder_type, format_options_type, generate_builder, generate_root_builder, generate_root_struct_build_impl};

pub struct SculptableStruct {
    pub root: bool,
    pub name: String,
    pub fields: Vec<Field>,
}

impl SculptableStruct {
    pub fn new(name: String, root: bool) -> Self {
        Self { root, name, fields: vec![] }
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }
}

pub struct Field {
    name: Option<String>,
    pub type_name: String,
    pub sculpt: bool,
}

impl Field {
    pub fn pick(name: Option<String>, type_name: String) -> Self {
        Self { name, type_name, sculpt: false }
    }

    pub fn sculpt(name: Option<String>, type_name: String) -> Self {
        Self { name, type_name, sculpt: true }
    }

    pub fn to_builder_field(&self) -> proc_macro2::TokenStream {
        if self.sculpt {
            let builder_field = format_builder_field_name(&self.type_name);
            let builder_type = format_builder_type(&self.type_name);
            quote! {
                #builder_field: #builder_type
            }
        } else {
            let option_field = self.format_field_name();
            let option_type = format_options_type(&self.type_name);
            quote! {
                #option_field: Option<#option_type>
            }
        }
    }

    pub fn to_builder_call(&self, builder_type: &Ident) -> proc_macro2::TokenStream {
        let variable = self.format_field_name();
        if self.sculpt {
            let builder_field = format_builder_field_name(&self.type_name);
            quote! {
                let #variable = self.#builder_field.build()
            }
        } else {
            let message = format!("Field {} not set in {}.", variable, builder_type);;
            quote! {
                let #variable = self.#variable.expect(#message).into()
            }
        }
    }

    pub fn format_field_name(&self) -> Ident {
        format_ident!("{}", self.name.as_ref().unwrap_or(&self.type_name).to_lowercase())
    }

    pub fn is_sculptable(&self) -> bool {
        self.sculpt
    }
}



pub fn build_sculptable(name: Ident, data_struct: DataStruct, is_root: bool) -> SculptableStruct {
    let mut sculptable_struct = SculptableStruct::new(name.to_string(), is_root);
    match data_struct.fields {
        Fields::Named(named_fields) => named_fields.named.into_iter()
            .for_each(|field| sculptable_struct.add_field(map_syn_field(field))),
        Fields::Unnamed(_) | Fields::Unit => panic!("Unsupported fields for struct case."),
    }
    sculptable_struct
}

fn map_syn_field(field: syn::Field) -> Field {
    let sculpt = field_has_sculpt_attribute(&field);
    let field_name = field.ident.as_ref().map(|f| f.to_string());
    let field_type = match &field.ty {
        Type::Path(path) => path.path.get_ident().unwrap().to_string(),
        _ => panic!("Unsupported field type."),
    };
    if sculpt {
        Field::sculpt(field_name, field_type)
    } else {
        Field::pick(field_name, field_type)
    }
}

pub fn field_has_sculpt_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}

pub fn derive_builder_from_sculptable(sculptable: SculptableStruct) -> TokenStream {
    let gen = if sculptable.root {
        let root_builder = generate_root_builder(&sculptable);
        let root_struct_build_impl = generate_root_struct_build_impl(&sculptable);
        quote! {
            #root_builder
            #root_struct_build_impl
        }
    } else {
        generate_builder(&sculptable)
    };
    gen.into()
}