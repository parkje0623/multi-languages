package main

import "C"
// import (
// 	"fmt"
// )

//export fib_calc
func fib_calc(n int) int {
	jobs := make(chan int, n)
	results := make(chan int, n)

	go worker(jobs, results)
	go worker(jobs, results)
	go worker(jobs, results)
	go worker(jobs, results)

	//Change i = 0 if fixed
	for i := n-1; i < n; i++ {
		jobs <- i
	}
	close(jobs)

	//I want to return an array of entire fibonacci calculation
	//But making arrays give "makechan: size out of range" error in line 10
	// var a []int
	// for j := 0; j < n; j++ {
	// 	a = append(a, 1)
	// }

	// var m int
	// m = 1

	return <-results
}

func worker(jobs <-chan int, results chan<- int) {
	for n := range jobs {
		results <- fib(n)
	}
}

// //export nonChanResult
// func nonChanResult(n int) int {
// 	return <-fib_calc(n)
// }

func fib(n int) int {
	if n <= 1 {
		return n
	}

	return fib(n-1) + fib(n-2)
}

// Nothing in main function because everything is called from the main.py (Python file)
func main() {}