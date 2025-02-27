use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    build_espeak();
    bind_espeak();
}

fn build_espeak() {
    let status = Command::new("make")
        .arg("all")
        .current_dir("c_src")
        .status()
        .expect("Failed to build espeak");

    if !status.success() {
        panic!("Makefile build for espeak failed");
    }
}

fn bind_espeak() {
    println!("cargo:rustc-link-search=c_src/lib");
    println!("cargo:rustc-link-search=espeak/c_src/lib");
    println!("cargo:rustc-link-lib=espeak_mini");

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
