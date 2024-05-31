pub use field::*;
pub use sculptable_struct::*;

mod field;
mod format;
mod sculptable_struct;

fn field_has_sculpt_attribute(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().get_ident().unwrap().to_string() == "sculptable")
}
