pub use field::*;
pub use pickable::*;
pub use sculptable_struct::*;

mod field;
mod format;
mod pickable;
mod sculptable_struct;

fn field_has_sculpt_attribute(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}
