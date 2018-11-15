module fortmodule
  use iso_c_binding
  implicit none

  integer(C_INT), bind(C), dimension(5) :: numbers

  contains

subroutine fortransub() bind(C)
      print *, "!! Hello from Fortran !!"
      numbers(1) = 1
      numbers(2) = 2
      numbers(3) = 3
      numbers(4) = 4
      numbers(5) = 5

end subroutine

end module

subroutine fortrantest(a, b, c) bind(C)
      use iso_c_binding
      implicit none
      real (C_FLOAT), intent(in) :: a, b
      real (C_FLOAT), intent(out) :: c

      print *, "Hello from Fortran!"
      c = sqrt(a*a + b*b)
end subroutine fortrantest

subroutine fortrantest2(a, b, c)
      implicit none
      double precision, intent(in) :: a, b
      double precision, intent(out) :: c

      print *, "Hello from Fortran2!"
      c = sqrt(a*a + b*b)
end subroutine fortrantest2
