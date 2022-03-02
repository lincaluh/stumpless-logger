extern crate bindgen;

fn main() {
    let stumpless_out = cmake::Config::new("/mnt/c/users/reall/code/stumpless")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", stumpless_out.display());
    println!("cargo:rustc-link-lib=static=stumpless");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/stumpless.h", stumpless_out.display()))
        .clang_arg(format!("-I{}/include/", stumpless_out.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/stumpless_bindings.rs")
        .expect("Couldn't write bindings!");
}
