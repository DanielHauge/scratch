package main

/*
#cgo LDFLAGS: -L. -lsub
#cgo LDFLAGS: -L. -ladd
#include "sub.h"
#include "add.h"
*/
import "C"

func main() {
	a := 10
	b := 20
	println("Subtraction is statically linked, as a .a") // Call the function
	println("Subtraction: ", sub(a, b))

	println("Addition is dynamically linked, as a .so, Must be included the LD_LIBRARY_PATH") // Call the function
	println("Addition: ", add(a, b))                                                          // Call the function
	println("Test:", add(a, b))
	add(2, 5)
}

// Some cool stuff man
func add(a, b int) int {
	res := C.add(C.int(a), C.int(b))
	return int(res)
}

func sub(a, b int) int {
	res := C.sub(C.int(a), C.int(b))
	return int(res)
}
