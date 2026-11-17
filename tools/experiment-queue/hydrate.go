package main

// Materialize only small, reviewed metadata needed by the native spec validator.
// Large inputs are fetched by the pod from their checksum-pinned S3 locations.
import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"
)

func (g *GitHub) materialize(root, path, revision string) error {
	if !relpath(path) {
		return fmt.Errorf("unsafe metadata path")
	}
	if _, err := readFile(root, path); err == nil {
		return nil
	}
	parts := strings.Split(path, "/")
	parent := root
	for _, part := range parts[:len(parts)-1] {
		parent = filepath.Join(parent, part)
		if s, e := os.Lstat(parent); e == nil && (!s.IsDir() || s.Mode()&os.ModeSymlink != 0) {
			return fmt.Errorf("metadata parent is not a plain directory")
		}
		if e := os.MkdirAll(parent, 0755); e != nil {
			return e
		}
	}
	b, err := g.content(path, revision)
	if err != nil {
		return err
	}
	if len(b) > 8<<20 {
		return fmt.Errorf("metadata exceeds 8 MiB")
	}
	f, err := os.OpenFile(filepath.Join(root, path), os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0644)
	if err != nil {
		return err
	}
	_, err = f.Write(b)
	if e := f.Close(); err == nil {
		err = e
	}
	return err
}
func (g *GitHub) hydrate(root, revision string) []Blocked {
	blocked := []Blocked{}
	files, _ := filepath.Glob(filepath.Join(root, "experiments/queue/*.json"))
	for _, file := range files {
		b, err := os.ReadFile(file)
		var e Entry
		if err != nil || json.Unmarshal(b, &e) != nil || e.Status != "ready" {
			continue
		}
		ensure := func(path string) bool {
			if err := g.materialize(root, path, revision); err != nil {
				blocked = append(blocked, Blocked{e.Record, "cannot materialize reviewed metadata " + path + ": " + err.Error()})
				return false
			}
			return true
		}
		if !ensure(e.Record) || !ensure(e.Protocol.Path) {
			continue
		}
		record, err := readFile(root, e.Record)
		if err != nil {
			continue
		}
		var spec map[string]any
		if yaml.Unmarshal(record, &spec) == nil {
			if path, ok := asMap(spec["approval"])["decision"].(string); ok {
				ensure(path)
			}
			for _, f := range array(asMap(spec["source"])["frozen"]) {
				if path, ok := asMap(f)["path"].(string); ok {
					ensure(path)
				}
			}
		}
		p, err := readObject(filepath.Join(root, e.Protocol.Path))
		if err != nil {
			continue
		}
		if path, ok := asMap(asMap(p["analysis_plan"])["script"])["path"].(string); ok {
			ensure(filepath.ToSlash(filepath.Clean(filepath.Join(filepath.Dir(e.Protocol.Path), path))))
		}
	}
	return blocked
}
