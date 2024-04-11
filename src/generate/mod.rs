pub use field::*;
pub use pickable::*;
pub use sculptable_struct::*;

mod sculptable_struct;
mod pickable;
mod field;
mod format;

fn field_has_sculpt_attribute(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}