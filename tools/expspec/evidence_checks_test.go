package main

import (
	"errors"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type evidenceReadError struct{}

func (evidenceReadError) Read([]byte) (int, error) { return 0, io.ErrUnexpectedEOF }

func TestJSONLinesRejectsPartialRead(t *testing.T) {
	stream := io.MultiReader(strings.NewReader("{\"ok\":true}\n"), evidenceReadError{})
	rows, err := readJSONLines(stream)
	if !errors.Is(err, io.ErrUnexpectedEOF) || rows != nil {
		t.Fatalf("partial evidence accepted: rows=%v error=%v", rows, err)
	}
	rows, err = readJSONLines(strings.NewReader("{\"ok\":true}\n{\"ok\":false}\n"))
	if err != nil || len(rows) != 2 {
		t.Fatalf("complete evidence rejected: rows=%v error=%v", rows, err)
	}
}

func TestChecksumManifestRequiresCompleteEvidence(t *testing.T) {
	oldRepo := repo
	repo = t.TempDir()
	t.Cleanup(func() { repo = oldRepo })
	if err := os.WriteFile(filepath.Join(repo, "evidence file.txt"), []byte("evidence\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	digest, err := sha256File(filepath.Join(repo, "evidence file.txt"))
	if err != nil {
		t.Fatal(err)
	}
	valid := digest + "  evidence file.txt\n"
	for _, tc := range []struct {
		name, body string
		want       bool
	}{
		{"empty", "", false},
		{"blank", "\n \t\n", false},
		{"truncated", digest + "\n", false},
		{"valid_then_truncated", valid + digest + "\n", false},
		{"wrong_digest", strings.Repeat("0", 64) + "  evidence file.txt\n", false},
		{"valid_text", valid, true},
		{"valid_binary", digest + " *evidence file.txt\n", true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			if err := os.WriteFile(filepath.Join(repo, "SHA256SUMS"), []byte(tc.body), 0o644); err != nil {
				t.Fatal(err)
			}
			got, detail := runCheck(Check{Kind: "sha256_manifest", Path: "SHA256SUMS"}, ctxMap{}, nil)
			if got != tc.want {
				t.Fatalf("pass = %v, want %v: %s", got, tc.want, detail)
			}
		})
	}
}

func TestStatusRequiresWholeToken(t *testing.T) {
	for _, tc := range []struct {
		text, token string
		want        bool
	}{
		{"2026-10-21T00:00:00Z A_DONE\n", "A_DONE", true},
		{"A_DONE\r\n", "A_DONE", true},
		{"time\tA_DONE \t\n", "A_DONE", true},
		{"time NOT_A_DONE\n", "A_DONE", false},
		{"time A_DONE_EXTRA\n", "A_DONE", false},
		{"time A_DONE\n", "", false},
	} {
		if got := statusHas(tc.text, tc.token); got != tc.want {
			t.Errorf("statusHas(%q, %q) = %v, want %v", tc.text, tc.token, got, tc.want)
		}
	}
}
