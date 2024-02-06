package mr

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"hash/fnv"
	"io"
	"log"
	"net/rpc"
	"os"
	"sort"
	"time"

	"golang.org/x/text/runes"
)

//
// Map functions return a slice of KeyValue.
//
type KeyValue struct {
	Key   string
	Value string
}

// for sorting by key.
type ByKey []KeyValue

// for sorting by key.
func (a ByKey) Len() int           { return len(a) }
func (a ByKey) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByKey) Less(i, j int) bool { return a[i].Key < a[j].Key }

//
// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by Map.
//
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}


//
// main/mrworker.go calls this function.
//
func Worker(mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) {

	// Your worker implementation here.

	// uncomment to send the Example RPC to the coordinator.
	for {
		// CallExample()
		CallGet(mapf, reducef)
		time.Sleep(time.Millisecond * 500)
	}
	// CallExample()
	// Use RPC to acquire task, as filename
	// Map result
	// Write to file

}

//
// example function to show how to make an RPC call to the coordinator.
//
// the RPC argument and reply types are defined in rpc.go.
//
func CallExample() {

	// declare an argument structure.
	args := ExampleArgs{}

	// fill in the argument(s).
	args.X = 99

	// declare a reply structure.
	reply := ExampleReply{}

	// send the RPC request, wait for the reply.
	call("Coordinator.Example", &args, &reply)

	// reply.Y should be 100.
	fmt.Printf("reply.Y %v\n", reply.Y)
}

func CallGet(mapf func(string, string) []KeyValue,
reducef func(string, []string) string) {
	args := GetArgs{}
	args.X = 1
	reply := GetReply{}

	call("Coordinator.GetTask", &args, &reply)
	// if !reply.successful { return }
	switch reply.Tasktype {
	case 0: // Handle Map
		shouldReturn := handleMap(reply, mapf)
		if shouldReturn {
			return
		}
	case 1: // Handle reduce

	default:
		log.Printf("Non-existent type??")
		return
	}
	

	for i := 0; i < 5; i++ {
		if sendSuccess(reply.Tasktype, reply.Tasknum) {
			break
		}
	}
	

}

func handleMap(reply GetReply, mapf func( string,  string) []KeyValue) bool {
	fmt.Printf("Trying to read %s\n", reply.Filename)
	file, err := os.Open(reply.Filename)

	if err != nil {
		log.Printf("Failed to open file: %s\n", reply.Filename)
		return true
	}
	content, err := io.ReadAll(file)
	if err != nil {
		log.Printf("Could not read file: %s\n", reply.Filename)
	}
	file.Close()

	kva := mapf(reply.Filename, string(content))

	sort.Sort(ByKey(kva))

	fmt.Printf("Mapped %s", reply.Filename)
	ofiles := make([]*os.File, reply.N_reduce)
	encs := make([]*json.Encoder, reply.N_reduce)
	for y := 0; y < reply.N_reduce; y++ {
		ofile, _ := os.CreateTemp(".", fmt.Sprintf("mr-%d-%d", reply.Tasknum, y))
		ofiles[y] = ofile
		encs[y] = json.NewEncoder(ofile)
	}

	for _, kv := range kva {
		encs[ihash(kv.Key)%reply.N_reduce].Encode(&kv)
	}

	for i := 0; i < len(ofiles); i++ {
		name := fmt.Sprintf("mr-%d-%d", reply.Tasknum, i)
		_, exists := os.Open(name)
		if exists != nil {
			os.Rename(ofiles[i].Name(), name)
		} else {
			os.Remove(ofiles[i].Name())
		}
	}
	return false
}

func handleReduce(reply GetReply, reducef func(string, []string) string) bool {
	// Open all task files where mr-X-Y, Y==reply.Tasknum
	// Read KV of all files into n_reduce arrays, then do a k-Read to generate reduces
	// Grab all keys and reduce it all.
	// Write to semi-final-output
	arrs := make([][]string, reply.N_reduce)
	for i := 0;;i++{
		file, err := os.Open(fmt.Sprint("mr-%v-%v", i, reply.Tasknum))
		if err != nil {
			return true
		}
		contents, err := io.ReadAll(file)
		if err != nil {
			return true
		}
		for _, raw_json := range bytes.Split(contents, bytes['\n']) {
			x := KeyValue{}
			json.Unmarshal()
		}


	}

	return false
}

func sendSuccess(tasktype int, tasknum int) bool {
	args := FinArgs{}
	reply := FinReply{}

	args.Tasknum = tasknum
	args.Tasktype = tasktype
	reply.Acknowledged = false
	call("Coordinator.FinTask", &args, &reply)
	return reply.Acknowledged
}


//
// send an RPC request to the coordinator, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func call(rpcname string, args interface{}, reply interface{}) bool {
	// c, err := rpc.DialHTTP("tcp", "127.0.0.1"+":1234")
	sockname := coordinatorSock()
	c, err := rpc.DialHTTP("unix", sockname)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		return true
	}

	fmt.Println(err)
	return false
}
