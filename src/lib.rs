use proc_macro::TokenStream;

use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type};

use generate::*;

mod generate;

struct SculptableStruct {
    root: bool,
    name: String,
    fields: Vec<Field>,
}

impl SculptableStruct {
    fn new(name: String, root: bool) -> Self {
        Self { root, name, fields: vec![] }
    }

    fn add_field(&mut self, field: Field) {
        self.fields.push(field)
    }
}

struct Field {
    name: String,
    type_name: String,
    sculpt: bool,
}

impl Field {
    fn pick(name: String, type_name: String) -> Self {
        Self { name, type_name, sculpt: false }
    }

    fn sculpt(name: String, type_name: String) -> Self {
        Self { name, type_name, sculpt: true }
    }
}

#[proc_macro_derive(Sculptor, attributes(sculptable))]
pub fn derive_root_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (name, data_struct) = match ast.data {
        Data::Struct(s) => (ast.ident, s),
        Data::Enum(_) | Data::Union(_) => panic!("Deriving Sculptor is only supported for structs."),
    };
    let sculptable = build_sculptable(name, data_struct, true);
    derive_builder_from_sculptable(sculptable)
}

fn build_sculptable(name: Ident, data_struct: DataStruct, is_root: bool) -> SculptableStruct {
    let mut sculptable_struct = SculptableStruct::new(name.to_string(), is_root);
    match data_struct.fields {
        Fields::Named(named_fields) => named_fields.named.iter()
            .for_each(|field| sculptable_struct.add_field(map_syn_field(field))),
        Fields::Unnamed(_) | Fields::Unit => panic!("Unsupported fields for struct case."),
    }
    sculptable_struct
}

fn map_syn_field(field: &syn::Field) -> Field {
    let sculpt = has_sculpt_attribute(&field);
    let field_name = field.ident.as_ref().unwrap().to_string();
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

fn has_sculpt_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}

fn derive_builder_from_sculptable(sculptable: SculptableStruct) -> TokenStream {
    let gen = if sculptable.root {
        generate_root_builder(sculptable)
    } else {
        generate_builder(sculptable)
    };
    gen.into()
}