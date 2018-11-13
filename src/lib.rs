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
    // pub fn fortrantest(a: f32, b: f32, c: &mut f32);
}

#[test]
fn test_fortran() {
    let side_a = 3.0;
    let side_b = 4.0;
    let mut hyp = 0.0;

    unsafe {
        fortransub();
        // fortrantest(side_a, side_b, &mut hyp);
    }

    // println!("The hypotenuse of triangle with sides");
    // println!(" {} and {} is {}", side_a, side_b, hyp);
}
// lib.rs:1 ends here
