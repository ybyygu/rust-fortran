// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*lib.rs][lib.rs:1]]
#![feature(link_args)]

#[link(name="test")]
#[link_args="-lgfortran"]
extern "C" {
    pub fn fortransub();
}

#[test]
fn test_fortran() {
    unsafe {
        fortransub();
    }
}
// lib.rs:1 ends here
