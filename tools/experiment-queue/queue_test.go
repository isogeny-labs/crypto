package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"testing"

	"gopkg.in/yaml.v3"
)

func put(t *testing.T, file string, b []byte) {
	t.Helper()
	if e := os.MkdirAll(filepath.Dir(file), 0755); e != nil {
		t.Fatal(e)
	}
	if e := os.WriteFile(file, b, 0644); e != nil {
		t.Fatal(e)
	}
}
func j(v any) []byte {
	b, e := json.Marshal(v)
	if e != nil {
		panic(e)
	}
	return b
}

func TestSelectionProgressAndUnclaimedPlanRefusal(t *testing.T) {
	claims := map[string]gitRef{"a": {}, "b": {}, "c": {}}
	items := []Candidate{{Key: "a"}, {Key: "new"}, {Key: "b"}, {Key: "c"}}
	for hour := int64(0); hour < 3; hour++ {
		ordered := selectionOrder(items, claims, hour)
		if ordered[0].Key != "new" || ordered[1].Key != []string{"a", "b", "c"}[hour] {
			t.Fatalf("unresolved claim starves work at hour %d: %+v", hour, ordered)
		}
	}
	file := filepath.Join(t.TempDir(), "unclaimed.json")
	put(t, file, j(Plan{Schema: "crypto.isolab-plan/v1", Repository: "o/r"}))
	if err := mainErr([]string{"run", "--repo", "o/r", "--file", file}); err == nil || err.Error() != "invalid plan identity" {
		t.Fatalf("unclaimed dry-run plan must fail before AWS: %v", err)
	}
}
func gitTest(t *testing.T, dir string, args ...string) string {
	t.Helper()
	a := append([]string{"-C", dir}, args...)
	b, e := exec.Command("git", a...).CombinedOutput()
	if e != nil {
		t.Fatalf("git: %s %v", b, e)
	}
	return strings.TrimSpace(string(b))
}
func schemas(t *testing.T) string {
	t.Helper()
	s := os.Getenv("EXPERIMENT_SCHEMAS")
	if s == "" {
		t.Fatal("EXPERIMENT_SCHEMAS must point at the pinned canonical schema directory")
	}
	return s
}
func protocolFixture(t *testing.T) map[string]any {
	t.Helper()
	file := os.Getenv("EXPERIMENT_PROTOCOL_FIXTURE")
	if file == "" {
		t.Fatal("EXPERIMENT_PROTOCOL_FIXTURE must name the pinned worked-example protocol")
	}
	p, e := readObject(file)
	if e != nil {
		t.Fatal(e)
	}
	return p
}
func baseEntry() Entry {
	return Entry{Schema: registrySchema, ID: "EXP-TEST-001", Status: "ready", Record: "experiments/specs/EXP-TEST-001.yaml", Trial: "first", SourceCommit: strings.Repeat("1", 40), Protocol: FileHash{Path: "protocol/protocol.json", SHA256: strings.Repeat("a", 64)}, Kind: "command", Image: Image{URI: "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:" + strings.Repeat("b", 64), SourceCommit: strings.Repeat("1", 40), RuntimeFiles: []FileHash{{Path: "/opt/run", SHA256: strings.Repeat("c", 64)}, {Path: "/opt/verify", SHA256: strings.Repeat("d", 64)}}}, Command: []string{"/opt/run", "{out}"}, Verify: []string{"/opt/verify", "{out}"}, Outputs: []string{"export.json"}, CPU: 2, Memory: "4Gi", Seconds: 900}
}
func candidate(e Entry) Candidate {
	key := e.key()
	return Candidate{Entry: e, Key: key, JobName: "q-exp-test-001-" + key[:20], RegistryPath: "experiments/queue/" + e.ID + ".json", ControllerCommit: strings.Repeat("2", 40), Protocol: json.RawMessage(`{}`)}
}

func TestIdentityStableForNotesButNotWorkload(t *testing.T) {
	e := baseEntry()
	k := e.key()
	e.Note = "new explanation"
	e.Status = "blocked"
	if e.key() != k {
		t.Fatal("cosmetic metadata created another run")
	}
	e.Command = append(e.Command, "new-work")
	if e.key() == k {
		t.Fatal("workload change did not change identity")
	}
}
func TestManifestRejectsTraversalDuplicateAndMalformed(t *testing.T) {
	hash := strings.Repeat("a", 64)
	for _, s := range []string{hash + "  ../secret", hash + "  ./out\n" + hash + "  out", "not-a-hash"} {
		if _, e := checksumManifest([]byte(s)); e == nil {
			t.Fatalf("accepted unsafe manifest %q", s)
		}
	}
	if v, e := checksumManifest([]byte(hash + "  ./export.json\n")); e != nil || v["export.json"] != hash {
		t.Fatal(v, e)
	}
}
func TestReadRejectsSymlinkOutsideCheckout(t *testing.T) {
	root := t.TempDir()
	outside := filepath.Join(t.TempDir(), "private")
	put(t, outside, []byte("private"))
	if e := os.Symlink(outside, filepath.Join(root, "link")); e != nil {
		t.Fatal(e)
	}
	if _, e := readFile(root, "link"); e == nil {
		t.Fatal("read outside checkout")
	}
}

func fixture(t *testing.T) (string, Entry, string) {
	t.Helper()
	root := t.TempDir()
	gitTest(t, root, "init", "-q")
	put(t, filepath.Join(root, "workload.txt"), []byte("frozen input"))
	gitTest(t, root, "add", ".")
	gitTest(t, root, "-c", "user.name=test", "-c", "user.email=test@example.invalid", "commit", "-qm", "code")
	source := gitTest(t, root, "rev-parse", "HEAD")
	e := baseEntry()
	e.SourceCommit = source
	e.Image.SourceCommit = source
	p := protocolFixture(t)
	execution := asMap(p["execution"])
	execution["backend"] = "isolab-k8s"
	execution["measurement_class"] = "search"
	execution["timeout_seconds"] = 900
	execution["placement"] = map[string]any{"cpus": 2, "memory_mb": 4096, "disk_gb": 8, "spot": true}
	execution["checkpoint"] = map[string]any{"policy": "none", "interval_seconds": nil, "grace_seconds": nil, "restore_validation": "none", "after_restore": map[string]any{"wall_time": "invalid", "counted_units": "valid"}}
	p["protocol_id"] = e.ID
	delete(p, "freeze")
	e.Protocol.SHA256 = digest(canonical(p))
	p["freeze"] = map[string]any{"status": "frozen", "protocol_sha256": e.Protocol.SHA256, "commit": source, "frozen_at": "2026-10-27T00:00:00Z", "filed_before_affected_data": true}
	put(t, filepath.Join(root, e.Protocol.Path), j(p))
	decision := "decisions/approve.yaml"
	put(t, filepath.Join(root, decision), []byte("coordinator_decision:\n  decision: approve\n  target_ids: [EXP-TEST-001]\n"))
	spec := map[string]any{"schema": "crypto.experiment/v1", "id": e.ID, "version": 1, "title": "queue integration test", "status": "approved", "owner": "test", "created": "2026-10-27", "approval": map[string]any{"decision": decision, "approved_by": "test", "approved_at": "2026-10-27T00:00:00Z"}, "question": map[string]any{"summary": "test metadata", "questions": []any{map[string]any{"id": "Q1", "text": "test"}}}, "source": map[string]any{"repo": "crypto", "ref": "main", "commit": source, "frozen": []any{map[string]any{"path": "workload.txt", "sha256": digest([]byte("frozen input"))}}}, "paths": map[string]any{"root": "research/test", "out": "research/test/out"}, "cells": []any{map[string]any{"id": "cell"}}, "steps": []any{map[string]any{"id": "run", "scope": "once", "runs_on": "host", "argv": []string{"true"}, "timeout_s": 1}}, "hosts": map[string]any{"selection": "ordered", "candidates": []any{map[string]any{"id": "test", "backend": "local", "provides": map[string]any{"vcpus": 2, "memory_gb": 4}, "timing_valid": false, "preemptible": false, "paid": false}}}, "budget": map[string]any{"max_cost_usd": 0, "max_wall_hours": 1}, "outcome": map[string]any{"decision_rules": []any{map[string]any{"id": "R1", "about": []string{"Q1"}, "when": "fixture", "verdict": "test"}}}, "reporting": map[string]any{"results_file": "research/test/RESULTS.md", "visuals": false}, "agent": map[string]any{"executor_model": "claude-haiku-5-5", "autonomy": map[string]any{"launch_paid_host": "never"}}}
	asMap(spec["source"])["repo"] = "isogeny-labs/crypto"
	asMap(spec["outcome"])["censoring"] = []string{"all failures retained"}
	asMap(spec["reporting"])["required_artifacts"] = []string{}
	asMap(asMap(spec["agent"])["autonomy"])["launch_paid_host"] = "allowed"
	asMap(asMap(spec["agent"])["autonomy"])["spend_without_asking_usd"] = 5
	asMap(spec["budget"])["max_cost_usd"] = 5
	b, _ := yaml.Marshal(spec)
	put(t, filepath.Join(root, e.Record), b)
	put(t, filepath.Join(root, "experiments/queue/"+e.ID+".json"), j(e))
	gitTest(t, root, "add", ".")
	gitTest(t, root, "-c", "user.name=test", "-c", "user.email=test@example.invalid", "commit", "-qm", "approve metadata")
	return root, e, gitTest(t, root, "rev-parse", "HEAD")
}
func TestDiscoveryHonorsApprovalFreezeAndBounds(t *testing.T) {
	root, e, revision := fixture(t)
	p := discover(root, schemas(t), "o/r", revision, 1)
	if len(p.Candidates) != 1 {
		t.Fatalf("valid fixture blocked: %+v", p.Blocked)
	}
	for _, tc := range []struct {
		name   string
		change func(*Entry)
	}{{"image", func(e *Entry) { e.Image.URI = "image:latest" }}, {"time", func(e *Entry) { e.Seconds = 3601 }}, {"CPU", func(e *Entry) { e.CPU = 4 }}, {"output escape", func(e *Entry) { e.Outputs = []string{"../export.json"} }}, {"unhashed verifier", func(e *Entry) { e.Verify = []string{"/other"} }}, {"draft", func(e *Entry) { e.Status = "draft" }}} {
		t.Run(tc.name, func(t *testing.T) {
			v := e
			tc.change(&v)
			if _, _, err := validateEntry(root, schemas(t), revision, v); err == nil {
				t.Fatal("accepted incompatible registration")
			}
		})
	}
	put(t, filepath.Join(root, e.Protocol.Path), []byte(`{}`))
	if _, _, err := validateEntry(root, schemas(t), revision, e); err == nil {
		t.Fatal("accepted corrupted protocol")
	}
}
func TestUnapprovedRecordDoesNotRun(t *testing.T) {
	root, e, revision := fixture(t)
	put(t, filepath.Join(root, e.Record), []byte("status: draft\n"))
	p := discover(root, schemas(t), "o/r", revision, 1)
	if len(p.Candidates) != 0 || len(p.Blocked) == 0 {
		t.Fatal("draft was runnable")
	}
}

func TestConcurrentClaimsOnlyOneLauncher(t *testing.T) {
	c := candidate(baseEntry())
	p := Plan{Schema: "crypto.isolab-plan/v1", Repository: "o/r", SourceCommit: c.ControllerCommit, Candidates: []Candidate{c}}
	var mu sync.Mutex
	claimed := false
	posts := 0
	s := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mu.Lock()
		defer mu.Unlock()
		path := strings.TrimPrefix(r.URL.Path, "/repos/o/r/")
		switch {
		case strings.HasPrefix(path, "git/matching-refs/"):
			fmt.Fprint(w, "[]")
		case strings.HasPrefix(path, "git/ref/heads/"):
			http.Error(w, "missing", 404)
		case r.Method == "POST" && path == "git/refs":
			posts++
			if claimed {
				http.Error(w, "exists", 422)
			} else {
				claimed = true
				w.WriteHeader(201)
				fmt.Fprint(w, "{}")
			}
		case strings.HasPrefix(path, "git/ref/tags/"):
			json.NewEncoder(w).Encode(map[string]any{"ref": "refs/tags/isolab-queue/" + c.Entry.ID + "/" + c.Key, "object": map[string]string{"sha": c.ControllerCommit, "type": "commit"}})
		case strings.HasPrefix(path, "contents/"):
			b := []byte(`{}`)
			if strings.Contains(path, "experiments/queue/") {
				b = j(c.Entry)
			}
			json.NewEncoder(w).Encode(map[string]string{"encoding": "base64", "content": base64.StdEncoding.EncodeToString(b)})
		default:
			t.Errorf("unexpected API %s %s", r.Method, path)
			http.Error(w, "bad", 500)
		}
	}))
	defer s.Close()
	g := GitHub{Base: s.URL, Repo: "o/r"}
	out := make(chan Plan, 2)
	errs := make(chan error, 2)
	for i := 0; i < 2; i++ {
		go func() { p, e := g.reserve(p, 1, false); out <- p; errs <- e }()
	}
	launches := 0
	for i := 0; i < 2; i++ {
		v := <-out
		if err := <-errs; err != nil {
			t.Fatal(err)
		}
		if len(v.Candidates) != 1 {
			t.Fatal(v)
		}
		if !v.Candidates[0].CollectOnly {
			launches++
		}
	}
	if launches != 1 || posts < 1 {
		t.Fatalf("launches=%d posts=%d", launches, posts)
	}
}

type fakeAWS struct {
	objects   map[string][]byte
	starts    int
	ambiguous bool
	calls     []string
}

func (f *fakeAWS) Run(_ context.Context, a []string) ([]byte, error) {
	f.calls = append(f.calls, strings.Join(a, " "))
	if a[0] == "codebuild" {
		f.starts++
		if f.ambiguous {
			return nil, fmt.Errorf("network timeout after StartBuild")
		}
		return []byte(`{"build":{"id":"test"}}`), nil
	}
	if a[0] == "s3" && a[1] == "cp" {
		b, e := os.ReadFile(a[2])
		if e != nil {
			return nil, e
		}
		f.objects[a[3]] = b
		return []byte(`{}`), nil
	}
	if a[0] == "s3api" {
		bucket, key := "", ""
		for i := 0; i < len(a)-1; i++ {
			if a[i] == "--bucket" {
				bucket = a[i+1]
			}
			if a[i] == "--key" {
				key = a[i+1]
			}
		}
		b, ok := f.objects["s3://"+bucket+"/"+key]
		if !ok {
			return nil, fmt.Errorf("404 Not Found")
		}
		if a[1] == "head-object" {
			return j(map[string]int{"ContentLength": len(b)}), nil
		}
		if a[1] == "get-object" {
			return []byte(`{}`), os.WriteFile(a[len(a)-1], b, 0600)
		}
	}
	return nil, fmt.Errorf("unexpected AWS command")
}
func testExecution(t *testing.T) (Execution, Candidate, *fakeAWS, []byte) {
	t.Helper()
	e := baseEntry()
	p := protocolFixture(t)
	e.Protocol.SHA256 = asMap(p["freeze"])["protocol_sha256"].(string)
	c := candidate(e)
	c.Protocol = j(p)
	f := &fakeAWS{objects: map[string][]byte{}}
	x := Execution{AWS: f, Bucket: "bucket", Project: "project", Schemas: schemas(t), ToolDir: ".", Wait: 0}
	request, err := x.request(c)
	if err != nil {
		t.Fatal(err)
	}
	fixture := filepath.Join(filepath.Dir(os.Getenv("EXPERIMENT_PROTOCOL_FIXTURE")), "export.json")
	v, err := readObject(fixture)
	if err != nil {
		t.Fatal(err)
	}
	asMap(v["source"])["commit"] = e.SourceCommit
	export := j(v)
	proxy := "s3://bucket/runs/" + c.JobName + "/launcher/queue/"
	pod := "11111111-1111-1111-1111-111111111111"
	f.objects[proxy+"request.json"] = request
	f.objects[proxy+"artifacts/export.json"] = export
	f.objects[proxy+"SHA256SUMS"] = []byte(digest(export) + "  ./export.json\n")
	f.objects[proxy+"result.json"] = j(map[string]any{"schema": "crypto.isolab-queue-result/v1", "key": c.Key, "job_name": c.JobName, "job_uid": "22222222-2222-2222-2222-222222222222", "pod_uid": pod, "state": "succeeded", "exit_code": 0, "results_uri": "s3://bucket/runs/" + c.JobName + "/" + pod + "/", "receipt_uri": proxy, "request_sha256": digest(request)})
	return x, c, f, request
}
func TestCollectVerifiesIdentityAndPayloads(t *testing.T) {
	x, c, f, request := testExecution(t)
	r, e := x.collect(context.Background(), c, request)
	if e != nil || r.State != "succeeded" || len(r.Files) != 1 {
		t.Fatal(r, e)
	}
	uri := "s3://bucket/runs/" + c.JobName + "/launcher/queue/artifacts/export.json"
	f.objects[uri] = append(f.objects[uri], ' ')
	if _, e = x.collect(context.Background(), c, request); e == nil {
		t.Fatal("accepted changed bytes")
	}
}
func TestCollectExistingClaimNeverStartsAgain(t *testing.T) {
	x, c, f, _ := testExecution(t)
	c.CollectOnly = true
	x.ToolDir = "nonexistent-new-controller-version"
	r := x.execute(context.Background(), c)
	if r.State != "succeeded" || f.starts != 0 {
		t.Fatal(r, f.starts)
	}
}
func TestAmbiguousStartNeverRetries(t *testing.T) {
	x, c, f, _ := testExecution(t)
	f.ambiguous = true
	r := x.execute(context.Background(), c)
	if f.starts != 1 || r.State != "submission_unknown" {
		t.Fatal(r, f.starts)
	}
}
func TestCollectRejectsClaimedClockAndSource(t *testing.T) {
	for _, bad := range []string{"clock", "source"} {
		t.Run(bad, func(t *testing.T) {
			x, c, f, request := testExecution(t)
			prefix := "s3://bucket/runs/" + c.JobName + "/launcher/queue/"
			v, _ := object(f.objects[prefix+"artifacts/export.json"])
			if bad == "clock" {
				asMap(array(v["runs"])[0])["resumed"] = json.Number("1")
				asMap(asMap(array(v["runs"])[0])["measurements"])["wall_valid"] = true
			} else {
				asMap(v["source"])["commit"] = strings.Repeat("9", 40)
			}
			b := j(v)
			f.objects[prefix+"artifacts/export.json"] = b
			f.objects[prefix+"SHA256SUMS"] = []byte(digest(b) + "  export.json\n")
			if _, e := x.collect(context.Background(), c, request); e == nil {
				t.Fatal("accepted inadmissible export")
			}
		})
	}
}
func TestPendingAndAmbiguousDoNotCreateResultBranch(t *testing.T) {
	g := GitHub{Base: "http://invalid", Repo: "o/r"}
	c := candidate(baseEntry())
	for _, s := range []string{"pending", "submission_unknown"} {
		if p, e := g.publish(Result{Candidate: c, State: s}); e != nil || p.Number != 0 {
			t.Fatal(p, e)
		}
	}
}

func TestWorkerLiteralArgvVerifierFailureAndBadHash(t *testing.T) {
	for _, mode := range []string{"success", "verifier-failed", "wrong-runtime"} {
		t.Run(mode, func(t *testing.T) {
			dir := t.TempDir()
			program := filepath.Join(dir, "program")
			verifier := filepath.Join(dir, "verifier")
			put(t, program, []byte("#!/usr/bin/env bash\nprintf '%s' \"$1\" > \"$ISOLAB_OUTPUT_DIR/literal\"\n"))
			put(t, verifier, []byte("#!/usr/bin/env bash\nexit 0\n"))
			if mode == "verifier-failed" {
				put(t, verifier, []byte("#!/usr/bin/env bash\nexit 7\n"))
			}
			for _, p := range []string{program, verifier} {
				os.Chmod(p, 0755)
			}
			pb, _ := os.ReadFile(program)
			vb, _ := os.ReadFile(verifier)
			e := baseEntry()
			e.Command = []string{program, "$(touch SHOULD_NOT_EXIST)"}
			e.Verify = []string{verifier}
			e.Inputs = []Input{}
			e.Image.RuntimeFiles = []FileHash{{Path: program, SHA256: digest(pb)}, {Path: verifier, SHA256: digest(vb)}}
			if mode == "wrong-runtime" {
				e.Image.RuntimeFiles[0].SHA256 = strings.Repeat("0", 64)
			}
			bin := filepath.Join(dir, "bin")
			put(t, filepath.Join(bin, "aws"), []byte("#!/usr/bin/env bash\nexit 0\n"))
			os.Chmod(filepath.Join(bin, "aws"), 0755)
			out := filepath.Join(dir, "out")
			cmd := exec.Command("bash", "worker.sh", string(j(e)))
			cmd.Env = append(os.Environ(), "PATH="+bin+":"+os.Getenv("PATH"), "ISOLAB_WORK_ROOT="+dir, "ISOLAB_OUTPUT_DIR="+out, "ISOLAB_RESULTS_URI=s3://private/results/", "POD_UID=test-pod")
			b, err := cmd.CombinedOutput()
			if (mode == "success") != (err == nil) {
				t.Fatalf("worker %s: %v %s", mode, err, b)
			}
			v, err := readObject(filepath.Join(out, "queue-job.json"))
			if err != nil {
				t.Fatal(err)
			}
			if mode == "success" {
				literal, _ := os.ReadFile(filepath.Join(out, "literal"))
				if string(literal) != e.Command[1] {
					t.Fatal("argv was interpreted")
				}
				if v["verify_exit"] != json.Number("0") {
					t.Fatal(v)
				}
			}
			if mode == "wrong-runtime" {
				if _, err = os.Stat(filepath.Join(out, "literal")); err == nil {
					t.Fatal("ran with wrong runtime bytes")
				}
			}
		})
	}
}
