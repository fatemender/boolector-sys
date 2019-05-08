use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=boolector");

    bindgen::Builder::default()
        .header("include/bindings.h")
        .whitelist_function("^boolector_(.*)$")
        .generate()
        .expect("failed to generate bindings for Boolector")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("failed to write bindings for Boolector");
}
