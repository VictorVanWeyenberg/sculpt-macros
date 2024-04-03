use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type};

const OPTIONS: &str = "Discriminants";

struct SculptableStruct {
    root: bool,
    name: String,
    fields: Vec<Field>
}

impl SculptableStruct {
    fn new(name: String, root: bool) -> SculptableStruct {
        SculptableStruct {
            root,
            name,
            fields: vec![]
        }
    }

    fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }
}

struct Field {
    name: String,
    type_name: String,
    pick: bool,
    sculpt: bool
}

impl Field {
    fn pick(name: String, type_name: String) -> Field {
        Field {
            name, type_name, pick: true, sculpt: false
        }
    }

    fn sculpt(name: String, type_name: String) -> Field {
        Field {
            name, type_name, pick: true, sculpt: true
        }
    }
}

#[proc_macro_derive(Sculptor, attributes(sculptable))]
pub fn derive_root_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (name, data_struct) = match ast.data {
        Data::Struct(s) => (ast.ident, s),
        Data::Enum(_) => panic!("Deriving Sculptor is only supported for structs."),
        Data::Union(_) => panic!("Deriving Sculptor is only supported for structs.")
    };
    let sculptable = build_sculptable(name, data_struct, true);
    derive_root_builder_from_sculptable(sculptable)
}

fn build_sculptable(name: Ident, data_struct: DataStruct, is_root: bool) -> SculptableStruct {
    let mut sculptable_struct = SculptableStruct::new(name.to_string(), is_root);
    match data_struct.fields {
        Fields::Named(named_fields) => {
            named_fields.named.iter()
                .map(|field| map_syn_field(field))
                .for_each(|f| sculptable_struct.add_field(f))
        }
        Fields::Unnamed(_) => panic!("Unsupported unnamed fields for struct case."),
        Fields::Unit => panic!("Unsupported unit fields for struct case.")
    }
    sculptable_struct
}

fn map_syn_field(field: &syn::Field) -> Field {
    let sculpt = has_sculpt_attribute(&field);
    let field_name = field.ident.clone().expect("Field appears not to have an ident.").to_string();
    let field_type = match &field.ty {
        Type::Path(path) => {
            path.path.get_ident().unwrap().to_string()
        },
        _ => panic!("Unsupported not type path field in struct.")
    };
    if sculpt {
        Field::sculpt(field_name, field_type)
    } else {
        Field::pick(field_name, field_type)
    }
}

fn has_sculpt_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}

fn derive_root_builder_from_sculptable(sculptable: SculptableStruct) -> TokenStream {
    let SculptableStruct {
        root: is_root, name: sculptable_name, fields: sculptable_fields
    } = sculptable;
    let builder_name = format_ident!("{}Builder", sculptable_name.clone());
    let callbacks_name = format_ident!("{}Callbacks", builder_name.clone());
    let tokenized_fields = tokenize_fields(sculptable_fields);
    let gen = quote! {
        pub struct #builder_name<'a, T: #callbacks_name> {
            #tokenized_fields

            callbacks: &'a T
        }
    };
    gen.into()
}

fn tokenize_fields(fields: Vec<Field>) -> proc_macro2::TokenStream {
    let tokenized_fields: Vec<proc_macro2::TokenStream> = fields.iter()
        .map(tokenize_field).collect();
    quote! {
        #(#tokenized_fields)*
    }
}

fn tokenize_field(field: &Field) -> proc_macro2::TokenStream {
    if field.sculpt {
        let builder_name = format_ident!("{}_builder", field.type_name.to_lowercase());
        let builder_type = format_ident!("{}Builder", field.type_name);
        quote! {
            #builder_name: #builder_type,
        }
    } else {
        let option_name = format_ident!("{}", field.type_name.to_lowercase());
        let options_name = format_ident!("{}{}", field.type_name.clone(), OPTIONS);
        quote! {
            #option_name: Option<#options_name>,
        }
    }
}