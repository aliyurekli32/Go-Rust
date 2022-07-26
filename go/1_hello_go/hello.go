package main

import "fmt"

const englishHelloPrefix string = "Hello, "

func Hello(name string) string {
	return englishHelloPrefix + name
}

func main() {
	fmt.Println()
}
