extern crate bindgen;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=deadbeef/deadbeef.h");

    let bindings = bindgen::Builder::default()
        .header("deadbeef/deadbeef.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate DeaDBeeF bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("deadbeef.rs"))
        .expect("Failed to write bindings");
}
