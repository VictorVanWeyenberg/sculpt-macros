use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, Type, Variant};

use crate::macros::format::SculptFormatter;
use crate::macros::{field_has_sculpt_attribute, Field};

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
        let picker_trait = self.generate_picker_trait();
        let pickable_builder = self.generate_pickable_builder();
        let builder_impl = self.impl_pickable_builder();
        let options_enum = self.generate_options_enum();
        let pickable_name_formatter: SculptFormatter = self.name.into();
        let variant_builders: Vec<proc_macro2::TokenStream> = self
            .options
            .into_iter()
            .map(|po| po.generate_variant_builder_and_impl(pickable_name_formatter.format_type()))
            .collect();
        let gen = quote! {
            #picker_trait
            #pickable_builder
            #builder_impl
            #(#variant_builders)*
            #options_enum
        };
        gen.into()
    }

    fn generate_picker_trait(&self) -> proc_macro2::TokenStream {
        let pickable_name_formatter: SculptFormatter = self.name.clone().into();
        let trait_name = pickable_name_formatter.format_picker_name();
        let options_type_name = pickable_name_formatter.format_options_type();
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

    fn generate_options_enum(&self) -> proc_macro2::TokenStream {
        let pickable_name_formatter: SculptFormatter = self.name.clone().into();
        let self_type = pickable_name_formatter.format_type();
        let options_type = pickable_name_formatter.format_options_type();
        let options: Vec<Ident> = self.options.iter().map(|o| o.to_option_ident()).collect();
        let variants: Vec<proc_macro2::TokenStream> =
            options.iter().map(|o| quote!(#options_type::#o)).collect();
        let conversions: Vec<proc_macro2::TokenStream> = self
            .options
            .iter()
            .map(|o| {
                let pickable_option_name_formatter: SculptFormatter = o.name().clone().into();
                if o.is_sculptable() {
                    let option = pickable_option_name_formatter.format_type();
                    let message = stringify!(
                        Cannot turn #options_type::#option into #self_type without dependencies.);
                    quote!(#options_type::#option => panic!(#message))
                } else {
                    let option = pickable_option_name_formatter.format_type();
                    quote!(#options_type::#option => #self_type::#option)
                }
            })
            .collect();
        quote! {
            #[derive(Clone, Copy)]
            enum #options_type {
                #(#options,)*
            }

            impl #options_type {
                const VARIANTS: &'static [Self] = &[
                    #(#variants,)*
                ];
            }

            impl Into<#self_type> for #options_type {
                fn into(self) -> #self_type {
                    match self {
                        #(#conversions,)*
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

    fn fields(&self) -> Option<&Vec<Field>> {
        match self {
            PickableOption::Struct { fields, .. } => Some(fields),
            PickableOption::Tuple { fields, .. } => Some(fields),
            PickableOption::Raw { .. } => None,
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

    fn generate_variant_builder_and_impl(self, enum_type_ident: Ident) -> proc_macro2::TokenStream {
        if !self.is_sculptable() {
            return quote!();
        }
        let builder = self.generate_builder();
        let builder_impl = self.generate_builder_impl(enum_type_ident);
        quote! {
            #builder
            #builder_impl
        }
    }

    fn generate_builder(&self) -> proc_macro2::TokenStream {
        let pickable_option_name_formatter: SculptFormatter = self.name().clone().into();
        let builder_type = pickable_option_name_formatter.format_builder_type();
        let builder_fields: Vec<proc_macro2::TokenStream> = self
            .fields()
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
        let pickable_option_name_formatter: SculptFormatter = self.name().clone().into();
        let builder_type = pickable_option_name_formatter.format_builder_type();
        let builder_calls: Vec<proc_macro2::TokenStream> = self
            .fields()
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
            PickableOption::Struct { fields, .. } => fields,
            PickableOption::Tuple { fields, .. } => fields,
            PickableOption::Raw { .. } => panic!("Generaring constructor call for RAW enum type."),
        }
        .iter()
        .map(|f| f.format_field_name())
        .collect();
        let pickable_option_name_formatter: SculptFormatter = self.name().clone().into();
        let enum_type = pickable_option_name_formatter.format_type();
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

    fn to_option_ident(&self) -> Ident {
        let name = match self {
            PickableOption::Struct { name, .. } => name,
            PickableOption::Tuple { name, .. } => name,
            PickableOption::Raw { name, .. } => name,
        };
        format_ident!("{}", name)
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
