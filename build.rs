// build.rs
// :PROPERTIES:
// :header-args: :tangle build.rs
// :END:
// [[https://doc.rust-lang.org/cargo/reference/build-scripts.html][Build Scripts - The Cargo Book]]


// [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*build.rs][build.rs:1]]
use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // compile c code
    // cc::Build::new()
    //     .file("lib/test.c")
    //     .include("lib")
    //     .compile("test");

    // Tell cargo to tell rustc to link static library under lib directory
    // println!("cargo:rustc-flags=-l static=sub -L lib");
    println!("cargo:rustc-flags=-l static=41_geometry -l static=14_hidewrite -l static=12_hide_mpi -l static=16_hideleave -l static=32_util -l static=28_numeric_noabirule -l static=18_timing -l static=10_defs -l static=10_dumpinfo -l static=17_libtetra_ext -l static=14_hidewrite -l static=27_toolbox_oop -L abi");
    // Tell cargo to tell rustc to link the system gfortran shared library.
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=blas");

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
