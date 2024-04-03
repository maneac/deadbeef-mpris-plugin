extern crate bindgen;

use std::{env, path::PathBuf};

const HEADER_PATH: &str = "deadbeef/include/deadbeef/deadbeef.h";

fn main() {
    println!("cargo:rerun-if-changed={HEADER_PATH}");

    let bindings = bindgen::Builder::default()
        .header(HEADER_PATH)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate DeaDBeeF bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("deadbeef.rs"))
        .expect("Failed to write bindings");
}
