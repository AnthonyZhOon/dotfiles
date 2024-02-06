package mr

import (
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"

	"fmt"
)


type Coordinator struct {
	// Your definitions here.
	input_files []string
	mapped []bool
	reduced []bool


}

// Your code here -- RPC handlers for the worker to call.

//
// an example RPC handler.
//
// the RPC argument and reply types are defined in rpc.go.
//
func (c *Coordinator) Example(args *ExampleArgs, reply *ExampleReply) error {
	reply.Y = args.X + 1
	return nil
}

func (c *Coordinator) GetTask(args *GetArgs, reply *GetReply) error {
	cap := len(c.mapped)
	i := 0
	for ; i < cap; i++  {
		if !c.mapped[i] {
			_, alread_mapped := os.Open(fmt.Sprintf("mr-%d-0", i))
			if alread_mapped != nil { // Not 
				break
			} else {
				c.mapped[i] = true
			}
		}
	}

	reply.Filename = (c.input_files)[i]
	reply.Tasknum = i
	// reply.successful = true
	// reply.operation = 0
	return nil
}


//
// start a thread that listens for RPCs from worker.go
//
func (c *Coordinator) server() {
	rpc.Register(c)
	rpc.HandleHTTP()
	// l, e := net.Listen("tcp", ":1234")
	sockname := coordinatorSock()
	os.Remove(sockname)
	l, e := net.Listen("unix", sockname)
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(l, nil)
}

//
// main/mrcoordinator.go calls Done() periodically to find out
// if the entire job has finished.
//
func (c *Coordinator) Done() bool {
	ret := false

	// Your code here.


	return ret
}

//
// create a Coordinator.
// main/mrcoordinator.go calls this function.
// nReduce is the number of reduce tasks to use.
//
func MakeCoordinator(files []string, nReduce int) *Coordinator {
	c := Coordinator{}
	n_files := len(files)

	c.input_files = files
	// Your code here.
	c.mapped = make([]bool, n_files)
	c.reduced = make([]bool, n_files)
	for i := 0; i < n_files; i++  {
		fmt.Printf("Filename: %s; Mapped: %v; Reduced %v\n", (c.input_files)[i], c.mapped[i], c.reduced[i])
	}

	c.server()
	return &c
}
