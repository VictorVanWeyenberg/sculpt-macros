use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Type, Variant};

use crate::macros::{Field, field_has_sculpt_attribute};
use crate::macros::format::SculptFormatter;

pub struct Pickable {
    name: String,
    options: Vec<PickableOption>,
}

impl Pickable {
    fn new(name: String) -> Pickable {
        Pickable {
            name,
            options: vec![],
        }
    }

    fn add_option(&mut self, pickable_option: PickableOption) {
        self.options.push(pickable_option)
    }

    pub fn generate(self) -> TokenStream {
        let pickable_builder = self.generate_pickable_builder();
        let builder_impl = self.impl_pickable_builder();
        let gen = quote! {
            #pickable_builder
            #builder_impl
        };
        gen.into()
    }

    fn generate_pickable_builder(&self) -> proc_macro2::TokenStream {
        let pickable_name_formatter: SculptFormatter = self.name.clone().into();
        let builder_type = pickable_name_formatter.format_builder_type();
        let option_field = pickable_name_formatter.format_option_field();
        let options_type = pickable_name_formatter.format_options_type();
        let variant_builders: Vec<proc_macro2::TokenStream> = self
            .options
            .iter()
            .map(|po| po.to_pickable_builder_field())
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
            .collect();
        quote! {
            #[derive(Default)]
            struct #builder_type {
                #option_field: Option<#options_type>,
                #(#variant_builders,)*
            }
        }
    }

    fn impl_pickable_builder(&self) -> proc_macro2::TokenStream {
        let pickable_name_formatter: SculptFormatter = self.name.clone().into();
        let builder_type = pickable_name_formatter.format_builder_type();
        let simple_type = pickable_name_formatter.format_type();
        let option_field = pickable_name_formatter.format_option_field();
        let options_type = pickable_name_formatter.format_options_type();
        let pickable_builder_build_calls: Vec<proc_macro2::TokenStream> = self
            .options
            .iter()
            .map(|po| po.to_builder_calls(&option_field, &options_type))
            .collect();
        quote! {
            impl #builder_type {
                fn build(self) -> #simple_type {
                    match self.#option_field.unwrap() {
                        #(#pickable_builder_build_calls,)*
                    }
                }
            }
        }
    }
}

pub enum PickableOption {
    Struct { name: String, fields: Vec<Field> },
    Tuple { name: String, fields: Vec<Field> },
    Raw { name: String },
}

impl PickableOption {
    fn new_struct(name: String) -> PickableOption {
        PickableOption::Struct {
            name,
            fields: vec![],
        }
    }

    fn new_tuple(name: String) -> PickableOption {
        PickableOption::Tuple {
            name,
            fields: vec![],
        }
    }

    fn new_raw(name: String) -> PickableOption {
        PickableOption::Raw { name }
    }

    fn name(&self) -> &String {
        match self {
            PickableOption::Struct { name, .. } => name,
            PickableOption::Tuple { name, .. } => name,
            PickableOption::Raw { name, .. } => name,
        }
    }

    fn is_sculptable(&self) -> bool {
        match self {
            PickableOption::Struct { .. } => true,
            PickableOption::Tuple { .. } => true,
            PickableOption::Raw { .. } => false,
        }
    }

    fn add_field(&mut self, field: Field) {
        match self {
            PickableOption::Struct { fields, .. } => fields.push(field),
            PickableOption::Tuple { fields, .. } => fields.push(field),
            PickableOption::Raw { .. } => {}
        }
    }

    fn to_pickable_builder_field(&self) -> Option<proc_macro2::TokenStream> {
        if !self.is_sculptable() {
            return None;
        }
        let pickable_option_name_formatter: SculptFormatter = self.name().clone().into();
        let builder_field = pickable_option_name_formatter.format_builder_field_name();
        let builder_type = pickable_option_name_formatter.format_builder_type();
        Some(quote! {
            #builder_field: #builder_type
        })
    }

    fn to_builder_calls(
        &self,
        option_field: &Ident,
        options_type: &Ident,
    ) -> proc_macro2::TokenStream {
        let pickable_option_name_formatter: SculptFormatter = self.name().clone().into();
        let simple_variant = pickable_option_name_formatter.format_type();
        if self.is_sculptable() {
            let builder_field = pickable_option_name_formatter.format_builder_field_name();
            quote! {
                #options_type::#simple_variant => self.#builder_field.build()
            }
        } else {
            let message = format!("Field {} not set in {}Builder", option_field, options_type);
            quote! {
                #options_type::#simple_variant => self.#option_field.expect(#message).into()
            }
        }
    }
}

pub fn build_pickable(pickable_ident: Ident, pickable_enum: DataEnum) -> Pickable {
    let mut pickable = Pickable::new(pickable_ident.to_string());
    pickable_enum
        .variants
        .into_iter()
        .map(variant_to_pickable_option)
        .for_each(|po| pickable.add_option(po));
    pickable
}

fn variant_to_pickable_option(variant: Variant) -> PickableOption {
    let name = variant.ident.to_string();
    let (mut option, variant_fields) = match variant.fields {
        Fields::Named(struct_fields) => {
            let option = PickableOption::new_struct(name);
            (option, struct_fields.named)
        }
        Fields::Unnamed(tuple_fields) => {
            let option = PickableOption::new_tuple(name);
            (option, tuple_fields.unnamed)
        }
        Fields::Unit => return PickableOption::new_raw(name),
    };
    variant_fields
        .iter()
        .map(variant_field_to_field)
        .for_each(|f| option.add_field(f));
    option
}

fn variant_field_to_field(field: &syn::Field) -> Field {
    let sculptable = field_has_sculpt_attribute(field);
    let field_name = field.ident.clone().map(|f| f.to_string());
    let field_type = match &field.ty {
        Type::Path(path) => path.path.get_ident().unwrap().to_string(),
        _ => panic!("Unsupported field type."),
    };
    if sculptable {
        Field::sculpt(field_name, field_type)
    } else {
        Field::pick(field_name, field_type)
    }
}
