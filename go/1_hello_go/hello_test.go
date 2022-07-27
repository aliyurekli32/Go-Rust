package main

import "testing"

func TestHello(t *testing.T) {
	t.Run("say hello to people", func(t *testing.T) {
		got := Hello("Chris")
		want := "Hello, Chris"
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

	t.Run("empty string defaults to 'world'", func(t *testing.T) {
		got := Hello("")
		want := "Hello, World"
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	})

}
