extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir).join("include").join("gbln.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_documentation(true)
        .with_include_guard("GBLN_H")
        .with_pragma_once(true)
        .with_tab_width(4)
        .generate()
        .expect("Unable to generate C header file")
        .write_to_file(output_file);
}
