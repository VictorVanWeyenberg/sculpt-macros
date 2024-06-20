use std::env;
use std::path::Path;

fn main() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = env::var("OUT_DIR").expect("Cannot find out_dir.");
    let out_dir = Path::new(&out_dir);
    vec![
        "tests/default.rs",
        "tests/enum_to_struct_dependency.rs",
        // "tests/many_to_one.rs"
    ]
        .into_iter()
        .map(Path::new)
        .map(Path::to_path_buf)
        .map(|source| (source, root_dir, out_dir))
        .for_each(|(source, root, out)| sculpt::build(source, root, out))
}