// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*lib.rs][lib.rs:1]]
use libc::{c_float, c_int};

extern "C" {
    // pub fn fortransub();
    // pub fn fortrantest(a: *mut c_float, b: *mut c_float, c: *mut c_float);
    // pub fn fortrantest2_(a: *mut c_float, b: *mut c_float, c: *mut c_float);

    // !Arguments ------------------------------------
    // !scalars
    // integer,intent(in) :: natom
    // !arrays
    // real(dp),intent(in) :: rprimd(3,3),xred(3,natom)
    // real(dp),intent(out) :: xcart(3,natom)
    pub fn __m_geometry_MOD_xred2xcart(natom: *mut c_int, rprimd: *mut [c_float; 3], xred: *mut [c_float; 3], xcart: *mut [c_float; 3]);
}

#[test]
fn test_fortran() {
    // let mut side_a = 3.0;
    // let mut side_b = 4.0;
    // let mut hyp = 0.0;

    // unsafe {
    //     fortransub();
    //     fortrantest(&mut side_a, &mut side_b, &mut hyp);
    //     fortrantest2_(&mut side_a, &mut side_b, &mut hyp);
    // }

    // println!("The hypotenuse of triangle with sides");
    // println!(" {} and {} is {}", side_a, side_b, hyp);

    // call abinit subroute
    let mut cell = [
        [1.0_f32, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    let mut xred = vec![[0.2, 0.5, 0.0]];
    let mut xcart = vec![[1.0; 3]];

    let mut natom = 1;

    unsafe {
        __m_geometry_MOD_xred2xcart(&mut natom, cell.as_mut_ptr(), xred.as_mut_ptr(), xcart.as_mut_ptr());
    }

    println!("{:#?}", xred);
    println!("{:#?}", xcart);
}
// lib.rs:1 ends here
