//go:build ignore

// Stand-in for research/ic_leads_rented_20261018/aws_spot/aws_spot.py with
// the same verbs, used by expspec_test.go:
//
//	fakespot.go run JOB CELLS     prints {"id": ...}
//	fakespot.go fetch ID DEST
//	fakespot.go terminate ID
//
// The "host" writes `<UTC> <TOKEN>` lines to .fakehost/<id>/STATUS, as
// boot.sh does, and finishes every cell at once.
package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

func status(dir, token string) {
	f, err := os.OpenFile(filepath.Join(dir, "STATUS"), os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	fmt.Fprintf(f, "%s %s\n", time.Now().UTC().Format("2006-01-02T15:04:05Z"), token)
}

func copyDir(src, dst string) error {
	return filepath.Walk(src, func(p string, fi os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		rel, _ := filepath.Rel(src, p)
		target := filepath.Join(dst, rel)
		if fi.IsDir() {
			return os.MkdirAll(target, 0o755)
		}
		b, err := os.ReadFile(p)
		if err != nil {
			return err
		}
		return os.WriteFile(target, b, 0o644)
	})
}

func main() {
	base := ".fakehost"
	switch os.Args[1] {
	case "run":
		job, cells := os.Args[2], strings.Split(os.Args[3], ",")
		id := fmt.Sprintf("i-%s-%d", job, time.Now().UnixNano()%1e8)
		dir := filepath.Join(base, id)
		if err := os.MkdirAll(filepath.Join(dir, "out"), 0o755); err != nil {
			panic(err)
		}
		status(dir, "BOOT")
		status(dir, "BUILT")
		for _, c := range cells {
			body := fmt.Sprintf("{\"cell\":%q,\"refuted\":true}\n", c)
			if err := os.WriteFile(filepath.Join(dir, "out", c+".json"), []byte(body), 0o644); err != nil {
				panic(err)
			}
			status(dir, "MEASURED "+c)
		}
		status(dir, "ALL_DONE")
		fmt.Printf("{\"id\": %q, \"type\": %q}\n", id, job)
	case "fetch":
		dir := filepath.Join(base, os.Args[2])
		if _, err := os.Stat(dir); err != nil {
			os.Exit(7)
		}
		if err := copyDir(dir, os.Args[3]); err != nil {
			panic(err)
		}
	case "terminate":
		if err := os.WriteFile(filepath.Join(base, os.Args[2], "TERMINATED"), []byte("1"), 0o644); err != nil {
			panic(err)
		}
		fmt.Println("terminated", os.Args[2])
	default:
		os.Exit(2)
	}
}
