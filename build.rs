extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let pix_runtime_path = PathBuf::from(env::var("PIX_RUNTIME_PATH").unwrap())
        .to_str()
        .unwrap()
        .to_owned();

    // Build C wrapper over C++ PIX header
    cc::Build::new()
        .cpp(true)
        .include(format!("{}\\include\\", pix_runtime_path))
        .file("wrapper.cpp")
        .compile("cppwrapper");

    // Generate Rust bindings to C wrapper
    println!("cargo:rustc-link-search={}\\bin\\x64", pix_runtime_path);
    println!("cargo:rustc-link-lib=WinPixEventRuntime");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.cpp");
    println!("cargo:rustc-link-lib=static=cppwrapper");

    let bindings = bindgen::Builder::default()
        .layout_tests(false)
        .header("wrapper.h")
        .whitelist_function("pix_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
