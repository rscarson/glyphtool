use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=speak");

    let bindings = bindgen::Builder::default()
        .header("speak_lib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //
    // Provide an ENV var with the path to the generated file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");
    println!(concat!("cargo:rustc-env=OUT_PATH={}"), out_path.display());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
