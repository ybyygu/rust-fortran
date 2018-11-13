// build.rs
// :PROPERTIES:
// :header-args: :tangle build.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*build.rs][build.rs:1]]
use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .object("lib/sub.o")
        .file("lib/test.c")
        .include("lib")
        .compile("test");

    // Tell cargo to tell rustc to link the system gfortran shared library.
    println!("cargo:rustc-link-lib=gfortran");

    // let bindings = bindgen::Builder::default()
    //     .header("lib/test.c")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
// build.rs:1 ends here
