package main

import (
	"fmt"
	"log"

	"gonum.org/v1/hdf5"
)

func main() {
	// Open the HDF5 file
	file, err := hdf5.OpenFile("test.h5", hdf5.F_ACC_RDONLY)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Open the dataset
	dset, err := file.OpenDataset("/cosine_dataset")
	if err != nil {
		log.Fatal(err)
	}
	defer dset.Close()

	// Get dataspace and dimensions
	space := dset.Space()
	defer space.Close()

	dims, _, err := space.SimpleExtentDims()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Dataset dimensions: %v\n", dims)

	// Read the data into a slice
	data := make([]float64, dims[0]) // assuming 1D dataset
	if err := dset.Read(&data); err != nil {
		log.Fatal(err)
	}

	// Print the data
	for i, v := range data {
		fmt.Printf("[%d] %f\n", i, v)
	}
}
