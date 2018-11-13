// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*lib.rs][lib.rs:1]]
use libc::{c_int, c_float};

#[link(name="test")]
extern "C" {
    pub fn fortransub();
    pub fn fortrantest(a: *mut c_float, b: *mut c_float, c: *mut c_float);
}

#[test]
fn test_fortran() {
    let mut side_a = 3.0;
    let mut side_b = 4.0;
    let mut hyp = 0.0;

    unsafe {
        fortransub();
        fortrantest(&mut side_a, &mut side_b, &mut hyp);
    }

    println!("The hypotenuse of triangle with sides");
    println!(" {} and {} is {}", side_a, side_b, hyp);
}
// lib.rs:1 ends here
