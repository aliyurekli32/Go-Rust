package main

import "fmt"

const spanish = "Spanish"
const french = "French"
const englishHelloPrefix string = "Hello, "
const spanishHelloPrefix string = "Hola, "
const frenchHelloPrefix string = "Bonjour, "

func Hello(name string, language string) string {
	if name == "" {
		name = "World"
	}

	if language == spanish {
		return spanishHelloPrefix + name
	}

	if language == french {
		return frenchHelloPrefix + name
	}
	return englishHelloPrefix + name
}

func main() {
	fmt.Println()
}
