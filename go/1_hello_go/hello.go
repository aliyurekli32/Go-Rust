package main

import "fmt"

const spanish = "Spanish"
const englishHelloPrefix string = "Hello, "
const spanishHelloPrefix string = "Hola, "

func Hello(name string, language string) string {
	if name == "" {
		name = "World"
	}

	if language == spanish {
		return spanishHelloPrefix + name
	}
	return englishHelloPrefix + name
}

func main() {
	fmt.Println()
}
