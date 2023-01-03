extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c99.h");

    let pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    env::set_var("CFLAGS", format!("-m{pointer_width}"));

    let file = PathBuf::from("src").join("c99.h");

    let bindings = bindgen::Builder::default()
        .header(file.to_string_lossy())
        .clang_arg(format!("-m{pointer_width}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
