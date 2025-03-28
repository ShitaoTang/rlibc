use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("rlibc.h");

    std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(output_file);
}
