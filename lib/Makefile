# makefile

# [[file:~/Workspace/Programming/rust-libs/rust-fortran/rust-fortran.note::*makefile][makefile:1]]
build:
	gcc -c main.c
	gfortran main.o sub.f90

gcc:
	gcc -c main.c
	gcc main.o sub.o -lgfortran

clean:
	rm *.o
	rm *.out
# makefile:1 ends here
