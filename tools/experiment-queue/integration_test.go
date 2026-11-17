package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestNativeBenchmarkExportMatchesCanonicalSchema(t *testing.T) {
	dir := t.TempDir()
	e := baseEntry()
	e.Kind = "benchmark"
	e.Image.EcbenchSHA256 = strings.Repeat("c", 64)
	c := candidate(e)
	p := protocolFixture(t)
	c.Protocol = j(p)
	request := filepath.Join(dir, "request.json")
	put(t, request, j(Dispatch{Schema: "crypto.isolab-dispatch/v1", Candidate: c}))
	session := filepath.Join(dir, "session")
	put(t, filepath.Join(session, "session.json"), j(map[string]any{"schema": "ecbench.session/v1", "git_commit": e.SourceCommit, "git_dirty": false, "env_class_id": "test"}))
	put(t, filepath.Join(dir, "build-info.json"), j(map[string]any{"source_revision": e.SourceCommit, "ecbench_sha256": e.Image.EcbenchSHA256}))
	rows := []string{}
	for i, a := range array(p["arms"]) {
		arm := asMap(a)["id"]
		r := map[string]any{"record_id": []string{"one", "two", "three"}[i], "arm": arm, "binary_sha256": e.Image.EcbenchSHA256, "workload": map[string]any{"workload_id": "fixture"}, "algorithm_seed": 1, "round": 0, "warmup": false, "outcome": map[string]any{"status": "verified", "matches_target": true}, "isolation": map[string]any{"level": "L2", "blockers": []string{}}, "cost": map[string]any{"total_gae": 10, "s": 1.25}, "time": map[string]any{"solve_wall_ns": 1000, "max_rss_kib": 1024}}
		rows = append(rows, string(j(r)))
	}
	put(t, filepath.Join(session, "records.jsonl"), []byte(strings.Join(rows, "\n")+"\n"))
	out := filepath.Join(dir, "export.json")
	if err := exportEcbench(request, session, out, true); err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(out)
	if err := schemaCheck(filepath.Join(schemas(t), "experiment-export.v1.json"), b); err != nil {
		t.Fatal(err)
	}
	v, _ := object(b)
	if asMap(v["source"])["commit"] != e.SourceCommit || len(array(v["runs"])) != 3 {
		t.Fatal(v)
	}
}

func TestPublisherReturnsOneScopedCommitAndPR(t *testing.T) {
	c := candidate(baseEntry())
	data := []byte(`{"observation":"fixture"}`)
	r := Result{Candidate: c, State: "succeeded", Files: []ReturnedFile{{Path: "export.json", SHA256: digest(data), Data: data}}, Receipt: json.RawMessage(`{"receipt_uri":"s3://private/receipts/"}`)}
	branch := false
	created := false
	commits := 0
	prefix := "research/experiment_queue/results/" + c.Entry.ID + "/" + c.Key + "/"
	s := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, req *http.Request) {
		path := strings.TrimPrefix(req.URL.Path, "/repos/o/r/")
		switch {
		case req.Method == "GET" && strings.HasPrefix(path, "git/ref/heads/automation/"):
			if !branch {
				http.Error(w, "missing", 404)
			} else {
				json.NewEncoder(w).Encode(map[string]any{"object": map[string]string{"sha": "commit", "type": "commit"}})
			}
		case path == "git/ref/heads/main":
			json.NewEncoder(w).Encode(map[string]any{"object": map[string]string{"sha": "base", "type": "commit"}})
		case path == "git/commits/base":
			json.NewEncoder(w).Encode(map[string]any{"tree": map[string]string{"sha": "base-tree"}})
		case req.Method == "POST" && path == "git/blobs":
			json.NewEncoder(w).Encode(map[string]string{"sha": "blob"})
		case req.Method == "POST" && path == "git/trees":
			var body struct{ Tree []map[string]string }
			json.NewDecoder(req.Body).Decode(&body)
			for _, f := range body.Tree {
				if !strings.HasPrefix(f["path"], prefix) {
					t.Errorf("write left result namespace: %s", f["path"])
				}
			}
			json.NewEncoder(w).Encode(map[string]string{"sha": "tree"})
		case req.Method == "POST" && path == "git/commits":
			commits++
			json.NewEncoder(w).Encode(map[string]string{"sha": "commit"})
		case req.Method == "POST" && path == "git/refs":
			branch = true
			json.NewEncoder(w).Encode(map[string]string{"ref": "result"})
		case req.Method == "GET" && path == "pulls":
			if created {
				json.NewEncoder(w).Encode([]Publication{{Number: 42, URL: "https://example/pr/42"}})
			} else {
				json.NewEncoder(w).Encode([]Publication{})
			}
		case req.Method == "POST" && path == "pulls":
			created = true
			json.NewEncoder(w).Encode(Publication{Number: 42, URL: "https://example/pr/42"})
		default:
			t.Errorf("unexpected API %s %s", req.Method, path)
			http.Error(w, "bad", 500)
		}
	}))
	defer s.Close()
	g := GitHub{Base: s.URL, Repo: "o/r"}
	for i := 0; i < 2; i++ {
		p, e := g.publish(r)
		if e != nil || p.Number != 42 {
			t.Fatal(p, e)
		}
	}
	if commits != 1 {
		t.Fatalf("created %d commits for same execution", commits)
	}
}

func TestCodeBuildCommandBridgeAndUIDFailure(t *testing.T) {
	for _, badUID := range []bool{false, true} {
		name := "complete"
		if badUID {
			name = "changed-job-uid"
		}
		t.Run(name, func(t *testing.T) {
			x, c, _, request := testExecution(t)
			_ = x
			dir := t.TempDir()
			store := filepath.Join(dir, "store")
			requestKey := "inputs/requests/" + digest(request) + ".json"
			put(t, filepath.Join(store, requestKey), request)
			pod := "11111111-1111-1111-1111-111111111111"
			uid := "22222222-2222-2222-2222-222222222222"
			statusUID := uid
			if badUID {
				statusUID = "33333333-3333-3333-3333-333333333333"
			}
			status := map[string]any{"job": map[string]any{"uid": statusUID, "state": "succeeded"}, "pods": []any{map[string]any{"uid": pod, "status": map[string]any{"containerStatuses": []any{map[string]any{"name": "task", "image": c.Entry.Image.URI, "imageID": "docker-pullable://" + c.Entry.Image.URI, "state": map[string]any{"terminated": map[string]any{"exitCode": 0}}}}}}}}
			put(t, filepath.Join(dir, "status.json"), j(status))
			put(t, filepath.Join(dir, "submission.json"), j(map[string]any{"job": map[string]any{"uid": uid}}))
			raw := filepath.Join(store, "runs", c.JobName, pod)
			export := []byte(`{"example":"private declared output"}`)
			job := j(map[string]any{"schema": "crypto.isolab-worker-result/v1", "source_revision": c.Entry.SourceCommit, "pod_uid": pod, "command_exit": 0, "verify_exit": 0, "inputs_and_runtime_verified": true})
			put(t, filepath.Join(raw, "export.json"), export)
			put(t, filepath.Join(raw, "queue-job.json"), job)
			put(t, filepath.Join(raw, "SHA256SUMS"), []byte(digest(export)+"  ./export.json\n"+digest(job)+"  ./queue-job.json\n"))
			bin := filepath.Join(dir, "controller/bin")
			put(t, filepath.Join(dir, "controller/remote/check-ready"), []byte("#!/usr/bin/env bash\nprintf '{}\\n'\n"))
			os.Chmod(filepath.Join(dir, "controller/remote/check-ready"), 0755)
			put(t, filepath.Join(bin, "isolab-eks-mcp"), []byte(`#!/usr/bin/env bash
set -euo pipefail
case "$2" in
  isolab_k8s_overview|isolab_k8s_logs) printf '{}\n' ;;
  isolab_k8s_run) printf '%s' "$3" > "$TEST_DIR/received-args.json"; cat "$TEST_DIR/submission.json" ;;
  isolab_k8s_status) cat "$TEST_DIR/status.json" ;;
  *) exit 2 ;;
esac
`))
			put(t, filepath.Join(bin, "aws"), []byte(`#!/usr/bin/env bash
set -euo pipefail
if [[ $1 == eks ]]; then printf '{}\n'; exit 0; fi
if [[ $1 == s3 && $2 == cp ]]; then
  src=$3 dst=$4
  if [[ $src == s3://bucket/* ]]; then src="$TEST_STORE/${src#s3://bucket/}"; fi
  if [[ $dst == s3://bucket/* ]]; then dst="$TEST_STORE/${dst#s3://bucket/}"; fi
  if [[ -d $src ]]; then mkdir -p "$dst"; cp -a "$src/." "$dst/"; else mkdir -p "$(dirname "$dst")"; cp "$src" "$dst"; fi
  exit 0
fi
if [[ $1 == s3api && $2 == head-object ]]; then
  shift 2
  while [[ $# -gt 0 ]]; do if [[ $1 == --key ]]; then key=$2; shift 2; else shift; fi; done
  stat -c '%s' "$TEST_STORE/$key"
  exit 0
fi
exit 2
`))
			for _, f := range []string{"aws", "isolab-eks-mcp"} {
				os.Chmod(filepath.Join(bin, f), 0755)
			}
			cmd := exec.Command("bash", "remote.sh")
			cmd.Env = append(os.Environ(), "CODEBUILD_SRC_DIR="+filepath.Join(dir, "controller"), "TEST_DIR="+dir, "TEST_STORE="+store, "ISOLAB_CLUSTER=isolab", "ISOLAB_BUCKET=bucket", "AWS_REGION=us-east-2", "ISOLAB_REQUEST_KEY="+requestKey)
			b, err := cmd.CombinedOutput()
			if (err != nil) != badUID {
				t.Fatalf("bridge %s: %v %s", name, err, b)
			}
			proxy := filepath.Join(store, "runs", c.JobName, "launcher/queue")
			receipt, e := readObject(filepath.Join(proxy, "result.json"))
			if e != nil {
				t.Fatal(e)
			}
			if badUID {
				if receipt["state"] != "identity_mismatch" {
					t.Fatal(receipt)
				}
			} else {
				if receipt["state"] != "succeeded" {
					t.Fatal(receipt)
				}
				got, e := os.ReadFile(filepath.Join(proxy, "artifacts/export.json"))
				if e != nil || string(got) != string(export) {
					t.Fatal("declared output did not return through permitted prefix", e)
				}
			}
		})
	}
}
