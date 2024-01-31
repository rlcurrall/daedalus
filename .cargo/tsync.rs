use std::path::PathBuf;

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    let inputs = vec![PathBuf::from_iter([dir, "src"])];
    let output = PathBuf::from_iter([dir, "app/src/types/rust.d.ts"]);

    tsync::generate_typescript_defs(inputs, output, false);
}
