package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

type CommandRunner interface {
	Run(context.Context, []string) ([]byte, error)
}
type AWSCLI struct{}

func (AWSCLI) Run(ctx context.Context, args []string) ([]byte, error) {
	ctx, cancel := context.WithTimeout(ctx, 60*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "aws", args...)
	cmd.Env = append(os.Environ(), "AWS_MAX_ATTEMPTS=1", "AWS_PAGER=")
	b, err := cmd.CombinedOutput()
	if err != nil {
		if len(b) > 2048 {
			b = b[:2048]
		}
		return nil, fmt.Errorf("AWS %s failed: %s", args[0], b)
	}
	return b, nil
}

type Dispatch struct {
	Schema         string    `json:"schema"`
	Candidate      Candidate `json:"candidate"`
	WorkerBase64   string    `json:"worker_base64"`
	WorkerSHA256   string    `json:"worker_sha256"`
	ExporterURI    string    `json:"exporter_uri,omitempty"`
	ExporterSHA256 string    `json:"exporter_sha256,omitempty"`
}
type Execution struct {
	AWS                               CommandRunner
	Bucket, Project, Schemas, ToolDir string
	Poll                              time.Duration
	Wait                              time.Duration
}

func s3Parts(uri string) (string, string, error) {
	if !strings.HasPrefix(uri, "s3://") {
		return "", "", fmt.Errorf("not an S3 URI")
	}
	s := strings.SplitN(strings.TrimPrefix(uri, "s3://"), "/", 2)
	if len(s) != 2 || s[0] == "" || !relpath(s[1]) {
		return "", "", fmt.Errorf("invalid S3 URI")
	}
	return s[0], s[1], nil
}
func (x Execution) get(ctx context.Context, uri string) ([]byte, error) {
	bucket, key, err := s3Parts(uri)
	if err != nil {
		return nil, err
	}
	b, err := x.AWS.Run(ctx, []string{"s3api", "head-object", "--bucket", bucket, "--key", key})
	if err != nil {
		return nil, err
	}
	var info struct{ ContentLength int64 }
	if json.Unmarshal(b, &info) != nil || info.ContentLength < 0 || info.ContentLength > 16<<20 {
		return nil, fmt.Errorf("remote metadata/output exceeds 16 MiB")
	}
	dir, err := os.MkdirTemp("", "isolab-read-")
	if err != nil {
		return nil, err
	}
	defer os.RemoveAll(dir)
	file := filepath.Join(dir, "object")
	if _, err = x.AWS.Run(ctx, []string{"s3api", "get-object", "--bucket", bucket, "--key", key, file}); err != nil {
		return nil, err
	}
	b, err = os.ReadFile(file)
	if err != nil {
		return nil, err
	}
	if int64(len(b)) != info.ContentLength {
		return nil, fmt.Errorf("S3 size changed while reading")
	}
	return b, nil
}
func (x Execution) request(c Candidate) ([]byte, error) {
	c.CollectOnly = false
	w, err := os.ReadFile(filepath.Join(x.ToolDir, "worker.sh"))
	if err != nil {
		return nil, err
	}
	d := Dispatch{Schema: "crypto.isolab-dispatch/v1", Candidate: c, WorkerBase64: base64.StdEncoding.EncodeToString(w), WorkerSHA256: digest(w)}
	if c.Entry.Kind == "benchmark" {
		exe, err := os.Executable()
		if err != nil {
			return nil, err
		}
		b, err := os.ReadFile(exe)
		if err != nil {
			return nil, err
		}
		d.ExporterSHA256 = digest(b)
		d.ExporterURI = "s3://" + x.Bucket + "/inputs/requests/" + d.ExporterSHA256 + ".queue-bin"
	}
	return json.Marshal(d)
}
func (x Execution) launch(ctx context.Context, c Candidate, request []byte) error {
	script, err := os.ReadFile(filepath.Join(x.ToolDir, "remote.sh"))
	if err != nil {
		return err
	}
	// Buildspec is trusted controller code from main, never experiment output.
	command := "cat > /tmp/isolab-queue-controller.sh <<'ISOLAB_QUEUE_CONTROLLER'\n" + string(script) + "\nISOLAB_QUEUE_CONTROLLER\nbash /tmp/isolab-queue-controller.sh"
	buildspec, _ := json.Marshal(map[string]any{"version": 0.2, "env": map[string]string{"shell": "bash"}, "phases": map[string]any{"build": map[string]any{"commands": []string{command}}}})
	dir, err := os.MkdirTemp("", "isolab-submit-")
	if err != nil {
		return err
	}
	defer os.RemoveAll(dir)
	file := filepath.Join(dir, "request.json")
	if err = os.WriteFile(file, request, 0600); err != nil {
		return err
	}
	key := "inputs/requests/" + digest(request) + ".json"
	if c.Entry.Kind == "benchmark" {
		var d Dispatch
		if err = json.Unmarshal(request, &d); err != nil {
			return err
		}
		exe, err := os.Executable()
		if err != nil {
			return err
		}
		if _, err = x.AWS.Run(ctx, []string{"s3", "cp", exe, d.ExporterURI, "--only-show-errors"}); err != nil {
			return err
		}
	}
	// The complete frozen request is durable before the first submission.
	if _, err = x.AWS.Run(ctx, []string{"s3", "cp", file, "s3://" + x.Bucket + "/" + key, "--only-show-errors"}); err != nil {
		return err
	}
	payload := map[string]any{"projectName": x.Project, "idempotencyToken": c.Key, "timeoutInMinutesOverride": 80, "queuedTimeoutInMinutesOverride": 15, "buildspecOverride": string(buildspec), "environmentVariablesOverride": []map[string]string{{"name": "ISOLAB_REQUEST_KEY", "value": key, "type": "PLAINTEXT"}}}
	b, _ := json.Marshal(payload)
	start := filepath.Join(dir, "start.json")
	if err = os.WriteFile(start, b, 0600); err != nil {
		return err
	}
	// Exactly one StartBuild. An ambiguous error leaves the permanent claim.
	_, err = x.AWS.Run(ctx, []string{"codebuild", "start-build", "--cli-input-json", "file://" + start})
	return err
}
func checksumManifest(b []byte) (map[string]string, error) {
	files := map[string]string{}
	for _, line := range strings.Split(strings.TrimSpace(string(b)), "\n") {
		if len(line) < 67 {
			return nil, fmt.Errorf("invalid SHA256SUMS")
		}
		hash := line[:64]
		name := strings.TrimPrefix(line[66:], "./")
		if !hex64.MatchString(hash) || !relpath(name) || files[name] != "" {
			return nil, fmt.Errorf("unsafe/duplicate manifest entry")
		}
		files[name] = hash
	}
	return files, nil
}
func (x Execution) fetchManifest(ctx context.Context, uri, dir string) error {
	b, err := x.get(ctx, uri+"SHA256SUMS")
	if err != nil {
		return err
	}
	files, err := checksumManifest(b)
	if err != nil {
		return err
	}
	if len(files) > 1024 {
		return fmt.Errorf("too many result payloads")
	}
	for name, hash := range files {
		data, err := x.get(ctx, uri+name)
		if err != nil {
			return err
		}
		if digest(data) != hash {
			return fmt.Errorf("result hash mismatch: %s", name)
		}
		file := filepath.Join(dir, name)
		if err = os.MkdirAll(filepath.Dir(file), 0700); err != nil {
			return err
		}
		f, err := os.OpenFile(file, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0600)
		if err != nil {
			return err
		}
		_, err = f.Write(data)
		if e := f.Close(); err == nil {
			err = e
		}
		if err != nil {
			return err
		}
	}
	return os.WriteFile(filepath.Join(dir, "SHA256SUMS"), b, 0600)
}
func (x Execution) collect(ctx context.Context, c Candidate, request []byte) (Result, error) {
	r := Result{Candidate: c, State: "pending"}
	uri := "s3://" + x.Bucket + "/runs/" + c.JobName + "/launcher/queue/result.json"
	b, err := x.get(ctx, uri)
	if err != nil {
		if strings.Contains(err.Error(), "404") || strings.Contains(err.Error(), "Not Found") || strings.Contains(err.Error(), "NoSuchKey") {
			return r, nil
		}
		return r, err
	}
	var receipt struct {
		Schema, Key, State string
		JobName            string `json:"job_name"`
		JobUID             string `json:"job_uid"`
		PodUID             string `json:"pod_uid"`
		ResultsURI         string `json:"results_uri"`
		RequestSHA256      string `json:"request_sha256"`
		ExitCode           int    `json:"exit_code"`
	}
	if err = json.Unmarshal(b, &receipt); err != nil {
		return r, err
	}
	if request == nil {
		request, err = x.get(ctx, strings.TrimSuffix(uri, "result.json")+"request.json")
		if err != nil {
			return r, err
		}
		var original Dispatch
		if json.Unmarshal(request, &original) != nil || original.Candidate.Key != c.Key || original.Candidate.Entry.key() != c.Key || original.Candidate.ControllerCommit != c.ControllerCommit {
			return r, fmt.Errorf("stored request does not match permanent claim")
		}
	}
	if receipt.Schema != "crypto.isolab-queue-result/v1" || receipt.Key != c.Key || receipt.JobName != c.JobName || receipt.RequestSHA256 != digest(request) {
		return r, fmt.Errorf("receipt/request identity mismatch")
	}
	r.Receipt = b
	r.State = receipt.State
	succeeded := receipt.State == "succeeded" && receipt.ExitCode == 0
	if !succeeded {
		r.Error = "Isolab execution did not pass; retain the private launcher and pod receipts."
		if receipt.ResultsURI == "" {
			return r, nil
		}
	}
	if receipt.JobUID == "" || receipt.PodUID == "" || receipt.ResultsURI != "s3://"+x.Bucket+"/runs/"+c.JobName+"/"+receipt.PodUID+"/" {
		return r, fmt.Errorf("receipt has invalid Job/Pod/output identity")
	}
	proxy := strings.TrimSuffix(uri, "result.json")
	manifest, err := x.get(ctx, proxy+"SHA256SUMS")
	if err != nil {
		return r, err
	}
	hashes, err := checksumManifest(manifest)
	if err != nil {
		return r, err
	}
	for _, name := range c.Entry.Outputs {
		if !relpath(name) {
			return r, fmt.Errorf("unsafe output name")
		}
		expectedHash := hashes[name]
		if c.Entry.Kind == "benchmark" && name == "export.json" {
			sum, err := x.get(ctx, proxy+"export.sha256")
			if err == nil && len(strings.Fields(string(sum))) > 0 {
				expectedHash = strings.Fields(string(sum))[0]
			}
		}
		if expectedHash == "" {
			if !succeeded {
				continue
			}
			return r, fmt.Errorf("required output absent from hashed manifest: %s", name)
		}
		data, err := x.get(ctx, proxy+"artifacts/"+name)
		if err != nil {
			return r, err
		}
		if digest(data) != expectedHash {
			return r, fmt.Errorf("output hash mismatch: %s", name)
		}
		r.RemoteOutputs = append(r.RemoteOutputs, RemoteOutput{Path: name, URI: proxy + "artifacts/" + name, SHA256: expectedHash, Bytes: len(data)})
		if name == "export.json" {
			if err = schemaCheck(filepath.Join(x.Schemas, "experiment-export.v1.json"), data); err != nil {
				return r, fmt.Errorf("unified export schema: %w", err)
			}
			v, err := object(data)
			if err != nil {
				return r, err
			}
			protocol, _ := v["protocol"].(map[string]any)
			source, _ := v["source"].(map[string]any)
			if protocol["sha256"] != c.Entry.Protocol.SHA256 || source["commit"] != c.Entry.SourceCommit {
				return r, fmt.Errorf("export source/protocol mismatch")
			}
			seen := map[string]bool{}
			for _, item := range array(v["runs"]) {
				row := asMap(item)
				id, _ := row["run_id"].(string)
				if id == "" || seen[id] {
					return r, fmt.Errorf("duplicate/missing export run id")
				}
				seen[id] = true
				m := asMap(row["measurements"])
				resumed := row["resumed"]
				if (c.Entry.Kind == "command" || row["status"] == "interrupted" || (resumed != nil && resumed != json.Number("0"))) && m["wall_valid"] == true {
					return r, fmt.Errorf("export claims an inadmissible clock")
				}
			}
			if succeeded {
				if err = checkCoverage(c.Protocol, v); err != nil {
					return r, err
				}
			}
		}
		limit := 64 << 10
		if strings.HasSuffix(name, ".pdf") {
			limit = 10 << 20
		}
		// Larger payloads remain at their hashed private S3 URI, never in Git.
		if len(data) <= limit {
			r.Files = append(r.Files, ReturnedFile{Path: name, SHA256: expectedHash, Data: data})
		}
	}
	return r, nil
}
func (x Execution) execute(ctx context.Context, c Candidate) Result {
	var request []byte
	var err error
	if !c.CollectOnly {
		request, err = x.request(c)
		if err != nil {
			return Result{Candidate: c, State: "controller_failed", Error: err.Error()}
		}
	}
	if !c.CollectOnly {
		if err = x.launch(ctx, c, request); err != nil {
			return Result{Candidate: c, State: "submission_unknown", Error: err.Error() + "; permanent claim retained; reconcile without resubmitting"}
		}
	}
	end := time.Now().Add(x.Wait)
	for {
		r, err := x.collect(ctx, c, request)
		if err != nil {
			r.State = "verification_failed"
			r.Error = err.Error()
			return r
		}
		if r.State != "pending" || c.CollectOnly || x.Wait == 0 || time.Now().After(end) {
			return r
		}
		select {
		case <-ctx.Done():
			return Result{Candidate: c, State: "pending", Error: "controller interrupted; collect the permanent claim on the next invocation"}
		case <-time.After(x.Poll):
		}
	}
}
func writeJSON(file string, v any) error {
	b, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		return err
	}
	if err = os.MkdirAll(filepath.Dir(file), 0755); err != nil {
		return err
	}
	return os.WriteFile(file, append(b, '\n'), 0600)
}
func envInt(name string, fallback int) int {
	if s := os.Getenv(name); s != "" {
		if n, e := strconv.Atoi(s); e == nil {
			return n
		}
	}
	return fallback
}
