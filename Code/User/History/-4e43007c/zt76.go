package main

import "fmt"

import (
	"os"
)

func main() {
	_, exists := os.Open("rpc.go")
	fact := exists == nil
	fmt.Printf("File exists?? %v , %v", fact, exists)
}