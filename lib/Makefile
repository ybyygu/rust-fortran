build:
	gcc -c main.c
	gfortran main.o sub.f90

main:
	gcc main.c -lgfortran -lsub -L .

clean:
	rm *.o
	rm *.out

libsub.a:
	ar -r libsub.a sub.o
