package mr

import (
	"os"
	"fmt"
)

func main() {
	_, exists := os.OpenFile("rpc.go")
	fact := exists != nil
	fmt.Printf("File exists?? %v , %v", fact, exists)
}