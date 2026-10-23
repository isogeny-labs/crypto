// Smoke workload for tools/expspec: deterministic, milliseconds long, no claims.
//
//	go run research/expspec_smoke_20261020/smoke.go host --out DIR
//	go run research/expspec_smoke_20261020/smoke.go measure --n N --out FILE
//	go run research/expspec_smoke_20261020/smoke.go summarize --dir DIR --out FILE
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
)

func write(path string, v any) {
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		panic(err)
	}
	b, _ := json.Marshal(v)
	if err := os.WriteFile(path, append(b, '\n'), 0o644); err != nil {
		panic(err)
	}
}

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "usage: smoke host|measure|summarize ...")
		os.Exit(2)
	}
	fs := flag.NewFlagSet(os.Args[1], flag.ExitOnError)
	out := fs.String("out", "", "output path")
	n := fs.Int("n", 0, "cell parameter")
	dir := fs.String("dir", "", "directory of cell outputs")
	_ = fs.Parse(os.Args[2:])
	switch os.Args[1] {
	case "host":
		write(filepath.Join(*out, "host.json"), map[string]any{"go": runtime.Version(), "arch": runtime.GOARCH, "cpus": runtime.NumCPU()})
	case "measure":
		write(*out, map[string]any{"n": *n, "value": *n * (*n + 1) / 2, "ok": true})
		fmt.Printf("{\"cell_done\": %d}\n", *n)
	case "summarize":
		files, _ := filepath.Glob(filepath.Join(*dir, "cell-*.json"))
		allOK := true
		for _, f := range files {
			var row struct{ OK bool }
			b, err := os.ReadFile(f)
			if err != nil || json.Unmarshal(b, &row) != nil || !row.OK {
				allOK = false
			}
		}
		write(*out, map[string]any{"cells": len(files), "all_ok": allOK})
	default:
		os.Exit(2)
	}
}
