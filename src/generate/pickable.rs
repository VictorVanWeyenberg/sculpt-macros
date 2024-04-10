use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::quote;
use syn::{DataEnum, Fields, Type, Variant};

use crate::generate::{Field, field_has_sculpt_attribute, format_builder_field_name, format_builder_type, format_option_field, format_options_type, format_picker_name, format_type};

pub struct Pickable {
    name: String,
    options: Vec<PickableOption>,
}

impl Pickable {
    fn new(name: String) -> Pickable {
        Pickable { name, options: vec![] }
    }

    fn add_option(&mut self, pickable_option: PickableOption) {
        self.options.push(pickable_option)
    }

    pub fn generate(self) -> TokenStream {
        let picker_trait = self.generate_picker_trait();
        let pickable_builder = self.generate_pickable_builder();
        let builder_impl = self.impl_pickable_builder();
        let variant_builders: Vec<proc_macro2::TokenStream> = self.options.into_iter()
            .map(|po| po.generate_variant_builder_and_impl(format_type(&self.name)))
            .collect();
        let gen = quote! {
            #picker_trait
            #pickable_builder
            #builder_impl
            #(#variant_builders)*
        };
        gen.into()
    }

    fn generate_picker_trait(&self) -> proc_macro2::TokenStream {
        let trait_name = format_picker_name(&self.name);
        let options_type_name = format_options_type(&self.name);
        quote! {
            pub trait #trait_name {
                fn options(&self) -> Vec<#options_type_name> {
                    #options_type_name::VARIANTS.to_vec()
                }
                fn fulfill(&mut self, requirement: &#options_type_name);
            }
        }
    }

    fn generate_pickable_builder(&self) -> proc_macro2::TokenStream {
        let builder_type = format_builder_type(&self.name);
        let option_field = format_option_field(&self.name);
        let options_type = format_options_type(&self.name);
        let variant_builders: Vec<proc_macro2::TokenStream> = self.options.iter()
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
        let builder_type = format_builder_type(&self.name);
        let simple_type = format_type(&self.name);
        let option_field = format_option_field(&self.name);
        let options_type = format_options_type(&self.name);
        let pickable_builder_build_calls: Vec<proc_macro2::TokenStream> = self.options.iter()
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
    Struct {
        name: String,
        fields: Vec<Field>,
    },
    Tuple {
        name: String,
        fields: Vec<Field>,
    },
    Raw {
        name: String,
    },
}

impl PickableOption {
    fn new_struct(name: String) -> PickableOption {
        PickableOption::Struct { name, fields: vec![] }
    }

    fn new_tuple(name: String) -> PickableOption {
        PickableOption::Tuple { name, fields: vec![] }
    }

    fn new_raw(name: String) -> PickableOption {
        PickableOption::Raw { name, }
    }

    fn name(&self) -> &String {
        match self {
            PickableOption::Struct { name, .. } => name,
            PickableOption::Tuple { name, .. } => name,
            PickableOption::Raw { name, .. } => name,
        }
    }

    fn fields(&self) -> Option<&Vec<Field>> {
        match self {
            PickableOption::Struct { fields, .. } => Some(fields),
            PickableOption::Tuple { fields, .. } => Some(fields),
            PickableOption::Raw { .. } => None
        }
    }

    fn is_sculptable(&self) -> bool {
        match self {
            PickableOption::Struct { .. } => true,
            PickableOption::Tuple { .. } => true,
            PickableOption::Raw { .. } => false
        }
    }

    fn add_field(&mut self, field: Field) {
        match self {
            PickableOption::Struct { fields, .. } => {
                fields.push(field)
            }
            PickableOption::Tuple { fields, .. } => {
                fields.push(field)
            }
            PickableOption::Raw { .. } => {}
        }
    }

    fn to_pickable_builder_field(&self) -> Option<proc_macro2::TokenStream> {
        if !self.is_sculptable() {
            return None;
        }
        let builder_field = format_builder_field_name(self.name());
        let builder_type = format_builder_type(self.name());
        Some(quote! {
            #builder_field: #builder_type
        })
    }

    fn to_builder_calls(&self, option_field: &Ident, options_type: &Ident) -> proc_macro2::TokenStream {
        let simple_variant = format_type(self.name());
        if self.is_sculptable() {
            let builder_field = format_builder_field_name(self.name());
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

    fn generate_variant_builder_and_impl(self, enum_type_ident: Ident) -> proc_macro2::TokenStream {
        if !self.is_sculptable() {
            return quote!()
        }
        let builder = self.generate_builder();
        let builder_impl = self.generate_builder_impl(enum_type_ident);
        quote! {
            #builder
            #builder_impl
        }
    }

    fn generate_builder(&self) -> proc_macro2::TokenStream {
        let builder_type = format_builder_type(self.name());
        let builder_fields: Vec<proc_macro2::TokenStream> = self.fields()
            .expect("Builder generated while fields are empty.")
            .iter()
            .map(|f| f.to_builder_field())
            .collect();
        quote! {
            #[derive(Default)]
            struct #builder_type {
                #(#builder_fields,)*
            }
        }
    }

    fn generate_builder_impl(&self, enum_type_ident: Ident) -> proc_macro2::TokenStream {
        let builder_type = format_builder_type(self.name());
        let builder_calls: Vec<proc_macro2::TokenStream> = self.fields()
            .expect("Generating builder implementation but there are not fields.")
            .iter()
            .map(|f| f.to_builder_call(&builder_type))
            .collect();
        let constructor = self.generate_constructor_call(&enum_type_ident);
        quote! {
            impl #builder_type {
                pub fn build(self) -> #enum_type_ident {
                    #(#builder_calls;)*
                    #constructor
                }
            }
        }
    }

    fn generate_constructor_call(&self, enum_type_ident: &Ident) -> proc_macro2::TokenStream {
        let fields: Vec<Ident> = match self {
            PickableOption::Struct { fields, .. } => {
                fields
            }
            PickableOption::Tuple { fields, .. } => {
                fields
            }
            PickableOption::Raw { .. } => panic!("Generaring constructor call for RAW enum type.")
        }.iter().map(|f| f.format_field_name()).collect();
        let enum_type = format_type(self.name());
        match self {
            PickableOption::Struct { .. } => {
                quote! {
                    #enum_type_ident::#enum_type { #(#fields,)* }
                }
            }
            PickableOption::Tuple { .. } => {
                quote! {
                    #enum_type_ident::#enum_type ( #(#fields,)* )
                }
            }
            PickableOption::Raw { .. } => {
                quote! {
                    panic!("Generating constructor arguments for RAW enum type.")
                }
            }
        }
    }
}

pub fn build_pickable(pickable_ident: Ident, pickable_enum: DataEnum) -> Pickable {
    let mut pickable = Pickable::new(pickable_ident.to_string());
    pickable_enum.variants.into_iter()
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
        Fields::Unit => return PickableOption::new_raw(name)
    };
    variant_fields.iter()
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