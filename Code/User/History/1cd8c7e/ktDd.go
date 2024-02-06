package mr

//
// RPC definitions.
//
// remember to capitalize all names.
//

import "os"
import "strconv"

//
// example to show how to declare the arguments
// and reply for an RPC.
//

type ExampleArgs struct {
	X int
}

type ExampleReply struct {
	Y int
}

type GetArgs struct {
	X int
}

type GetReply struct {
	Filename string
	Tasknum int
	Tasktype int
	// operation int // 0 for Map, 1 for Reduce
	// successful bool
}

type FinArgs struct {
	Type int
	Tasknum int
}

type FinReply struct {
	Acknowledged bool
}
// Add your RPC definitions here.


// Cook up a unique-ish UNIX-domain socket name
// in /var/tmp, for the coordinator.
// Can't use the current directory since
// Athena AFS doesn't support UNIX-domain sockets.
func coordinatorSock() string {
	s := "/var/tmp/824-mr-"
	s += strconv.Itoa(os.Getuid())
	return s
}