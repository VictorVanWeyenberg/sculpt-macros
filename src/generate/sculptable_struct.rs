use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataStruct, Fields, Type};

use crate::generate::{field_has_sculpt_attribute, Field};

pub struct SculptableStruct {
    pub root: bool,
    pub name: String,
    pub fields: Vec<Field>,
}

impl SculptableStruct {
    fn new(name: String, root: bool) -> Self {
        Self {
            root,
            name,
            fields: vec![],
        }
    }

    fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }

    pub fn generate(self) -> proc_macro2::TokenStream {
        let gen = if self.root {
            let root_builder = self.generate_root_builder();
            let root_struct_build_impl = self.generate_root_struct_build_impl();
            quote! {
                #root_builder
                #root_struct_build_impl
            }
        } else {
            self.generate_builder()
        };
        gen.into()
    }

    fn generate_builder(&self) -> proc_macro2::TokenStream {
        let builder_name = format_ident!("{}Builder", self.name);
        let callbacks_name = format_ident!("{}Callbacks", builder_name);
        let field_names: Vec<Ident> = self
            .fields
            .iter()
            .map(|f| format_ident!("{}", f.format_field_name()))
            .collect();
        quote! {
            struct #builder_name<'a, T: #callbacks_name> { #(#field_names,)*, callbacks: &'a T }
        }
    }

    fn generate_root_struct_build_impl(&self) -> proc_macro2::TokenStream {
        let sculptable_ident = format_ident!("{}", self.name);
        let builder_name = format_ident!("{}Builder", self.name);
        let callbacks_name = format_ident!("{}Callbacks", builder_name);
        quote! {
            impl #sculptable_ident {
                pub fn build<T: #callbacks_name>(t: &mut T) -> #sculptable_ident {
                    #builder_name::<T>::new(t).build()
                }
            }
        }
    }

    fn generate_root_builder(&self) -> proc_macro2::TokenStream {
        let sculptable_ident = format_ident!("{}", self.name);
        let builder_name = format_ident!("{}Builder", self.name);
        let callbacks_name = format_ident!("{}Callbacks", builder_name);
        let tokenized_fields: Vec<proc_macro2::TokenStream> =
            self.fields.iter().map(|f| f.to_builder_field()).collect();
        let field_initializers: Vec<proc_macro2::TokenStream> = self
            .fields
            .iter()
            .map(|f| f.tokenize_field_initializer())
            .collect();
        let first_field_pick_method = self.first_field_pick_method();
        let field_builders: Vec<proc_macro2::TokenStream> = self
            .fields
            .iter()
            .map(|f| f.to_builder_call(&sculptable_ident))
            .collect();
        let field_names: Vec<Ident> = self.fields.iter().map(|f| f.format_field_name()).collect();
        quote! {
            pub struct #builder_name<'a, T: #callbacks_name> {
                #(#tokenized_fields,)*
                callbacks: &'a T
            }

            impl<'a, T: #callbacks_name> #builder_name<'a, T> {
                pub fn new(t: &'a mut T) -> #builder_name<T> {
                    #builder_name {
                        #(#field_initializers,)*
                        callbacks: t
                    }
                }

                pub fn build(mut self) -> #sculptable_ident {
                    self.callbacks.#first_field_pick_method(&mut self);
                    #(#field_builders;)*
                    #sculptable_ident { #(#field_names,)* }
                }
            }
        }
    }

    fn first_field_pick_method(&self) -> proc_macro2::TokenStream {
        let type_name = self
            .fields
            .get(0)
            .expect("No fields for root sculptor.")
            .format_type()
            .to_string()
            .to_lowercase();
        let type_name = format_ident!("pick_{}", type_name);
        quote! { #type_name }
    }
}

pub fn build_sculptable(name: Ident, data_struct: DataStruct, is_root: bool) -> SculptableStruct {
    let mut sculptable_struct = SculptableStruct::new(name.to_string(), is_root);
    match data_struct.fields {
        Fields::Named(named_fields) => named_fields
            .named
            .into_iter()
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
