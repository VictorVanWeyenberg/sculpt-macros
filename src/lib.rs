use proc_macro::TokenStream;

use syn::{Data, DeriveInput};

use generate::*;

mod generate;

#[proc_macro_derive(Sculptor, attributes(sculptable))]
pub fn derive_root_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (name, data_struct) = match ast.data {
        Data::Struct(s) => (ast.ident, s),
        Data::Enum(_) | Data::Union(_) => panic!("Deriving Sculptor is only supported for structs."),
    };
    let sculptable = build_sculptable(name, data_struct, true);
    sculptable.generate().into()
}

#[proc_macro_derive(Picker, attributes(sculptable))]
pub fn derive_picker(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (name, data_enum) = match ast.data {
        Data::Enum(s) => (ast.ident, s),
        Data::Struct(_) | Data::Union(_) => panic!("Deriving Sculptor is only supported for structs."),
    };
    let pickable = build_pickable(name, data_enum);
    pickable.generate()
}
