//go:build ignore

// score.go IN OUT: copies a fetched cell result into a score file.
package main

import (
	"encoding/json"
	"os"
)

func main() {
	var d struct {
		Cell    string `json:"cell"`
		Refuted bool   `json:"refuted"`
	}
	b, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	if err := json.Unmarshal(b, &d); err != nil {
		panic(err)
	}
	out, _ := json.Marshal(d)
	if err := os.WriteFile(os.Args[2], append(out, '\n'), 0o644); err != nil {
		panic(err)
	}
}
