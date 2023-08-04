use std::env;
use std::path::PathBuf;

fn main() {
    /// directory where openmm was built
    const OPENMM_BUILD: &str = "/home/brent/omsf/clone/openmm/build";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={OPENMM_BUILD}");

    // Tell cargo to tell rustc to link the libOpenMM.so from that directory
    println!("cargo:rustc-link-lib=OpenMM");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point to bindgen, and lets you
    // build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{OPENMM_BUILD}/wrappers"))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
