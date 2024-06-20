use sculpt_macros::{sculpt, sculpt_alias};

// include!(concat!(env!("OUT_DIR"), "/tests/many_to_one.rs"));

/* #[test]
fn primitive_dependencies() {
    let mut callbacks = RootBuilderCallbacksImpl {};
    let _ = Root::build(&mut callbacks);
} */

#[sculpt]
#[sculpt_alias(
    field1 => Enum1,
    field2 => Enum2
)]
pub struct Root {
    field1: Enum,
    field2: Enum
}

pub enum Enum {
    A, B
}

struct RootBuilderCallbacksImpl {}

// impl RootBuilderCallbacks for RootBuilderCallbacksImpl {}