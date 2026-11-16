package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"github.com/santhosh-tekuri/jsonschema/v6"
	"gopkg.in/yaml.v3"
)

const registrySchema = "crypto.isolab-queue/v1"

var hex40 = regexp.MustCompile(`^[a-f0-9]{40}$`)
var hex64 = regexp.MustCompile(`^[a-f0-9]{64}$`)
var identifier = regexp.MustCompile(`^[A-Za-z0-9][A-Za-z0-9_-]{0,95}$`)

type FileHash struct {
	Path   string `json:"path"`
	SHA256 string `json:"sha256"`
}
type Input struct {
	URI    string `json:"uri"`
	Path   string `json:"path"`
	SHA256 string `json:"sha256"`
}
type Image struct {
	URI           string     `json:"uri"`
	SourceCommit  string     `json:"source_commit"`
	RuntimeFiles  []FileHash `json:"runtime_files,omitempty"`
	EcbenchSHA256 string     `json:"ecbench_sha256,omitempty"`
}
type Entry struct {
	Schema       string   `json:"schema"`
	ID           string   `json:"id"`
	Status       string   `json:"status"`
	Note         string   `json:"note,omitempty"`
	Record       string   `json:"record"`
	Protocol     FileHash `json:"protocol"`
	SourceCommit string   `json:"source_commit"`
	Trial        string   `json:"trial"`
	Kind         string   `json:"kind"`
	Image        Image    `json:"image"`
	Command      []string `json:"command,omitempty"`
	Verify       []string `json:"verify,omitempty"`
	Inputs       []Input  `json:"inputs,omitempty"`
	Outputs      []string `json:"outputs"`
	WorkloadSpec string   `json:"workload_spec,omitempty"`
	CPU          int      `json:"cpu"`
	Memory       string   `json:"memory"`
	Seconds      int      `json:"seconds"`
}
type Candidate struct {
	Entry            Entry           `json:"entry"`
	Key              string          `json:"key"`
	JobName          string          `json:"job_name"`
	RegistryPath     string          `json:"registry_path"`
	ControllerCommit string          `json:"controller_commit"`
	Protocol         json.RawMessage `json:"protocol"`
	Spec             json.RawMessage `json:"spec,omitempty"`
	CollectOnly      bool            `json:"collect_only"`
}
type Blocked struct {
	Record string `json:"record"`
	Reason string `json:"reason"`
}
type Plan struct {
	Schema       string      `json:"schema"`
	Claimed      bool        `json:"claimed"`
	Repository   string      `json:"repository"`
	SourceCommit string      `json:"source_commit"`
	Candidates   []Candidate `json:"candidates"`
	Blocked      []Blocked   `json:"blocked"`
}

func digest(b []byte) string { s := sha256.Sum256(b); return hex.EncodeToString(s[:]) }
func canonical(v any) []byte {
	var b bytes.Buffer
	e := json.NewEncoder(&b)
	e.SetEscapeHTML(false)
	_ = e.Encode(v)
	return bytes.TrimSuffix(b.Bytes(), []byte{'\n'})
}
func decode(b []byte, v any) error {
	d := json.NewDecoder(bytes.NewReader(b))
	d.DisallowUnknownFields()
	if err := d.Decode(v); err != nil {
		return err
	}
	if err := d.Decode(new(any)); err != io.EOF {
		return fmt.Errorf("expected one JSON document")
	}
	return nil
}
func object(b []byte) (map[string]any, error) {
	var v map[string]any
	d := json.NewDecoder(bytes.NewReader(b))
	d.UseNumber()
	err := d.Decode(&v)
	return v, err
}
func relpath(p string) bool {
	if p == "" || strings.ContainsAny(p, "\\\x00\n\r") || filepath.IsAbs(p) {
		return false
	}
	for _, s := range strings.Split(p, "/") {
		if s == "" || s == "." || s == ".." {
			return false
		}
	}
	return true
}
func readFile(root, p string) ([]byte, error) {
	if !relpath(p) {
		return nil, fmt.Errorf("unsafe repository path %q", p)
	}
	r, err := filepath.EvalSymlinks(filepath.Join(root, p))
	if err != nil {
		return nil, err
	}
	base, err := filepath.Abs(root)
	if err != nil {
		return nil, err
	}
	rel, err := filepath.Rel(base, r)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) {
		return nil, fmt.Errorf("path leaves checkout")
	}
	s, err := os.Stat(r)
	if err != nil || !s.Mode().IsRegular() {
		return nil, fmt.Errorf("not a regular file: %s", p)
	}
	if s.Size() > 8<<20 {
		return nil, fmt.Errorf("metadata exceeds 8 MiB: %s", p)
	}
	return os.ReadFile(r)
}
func schemaCheck(schemaPath string, b []byte) error {
	r, err := os.ReadFile(schemaPath)
	if err != nil {
		return err
	}
	doc, err := jsonschema.UnmarshalJSON(bytes.NewReader(r))
	if err != nil {
		return err
	}
	c := jsonschema.NewCompiler()
	if err = c.AddResource("mem:///schema.json", doc); err != nil {
		return err
	}
	s, err := c.Compile("mem:///schema.json")
	if err != nil {
		return err
	}
	v, err := jsonschema.UnmarshalJSON(bytes.NewReader(b))
	if err != nil {
		return err
	}
	return s.Validate(v)
}
func argvOK(a []string) bool {
	if len(a) == 0 || len(a) > 256 || a[0] == "" {
		return false
	}
	for _, s := range a {
		if len(s) > 8192 || strings.ContainsRune(s, 0) {
			return false
		}
	}
	return true
}

// The identity excludes descriptive notes and readiness status. Cosmetic changes
// cannot create a new paid execution. A reviewed trial or workload change can.
func (e Entry) key() string {
	e.Status = ""
	e.Note = ""
	return digest(canonical(e))
}
func validateEntry(root, schemas, revision string, e Entry) (json.RawMessage, json.RawMessage, error) {
	fail := func(s string) (json.RawMessage, json.RawMessage, error) { return nil, nil, fmt.Errorf("%s", s) }
	if e.Schema != registrySchema || !identifier.MatchString(e.ID) {
		return fail("invalid queue identity/schema")
	}
	if e.Status != "ready" {
		return fail("registration " + e.Status + ": " + e.Note)
	}
	if !identifier.MatchString(e.Trial) || !hex40.MatchString(e.SourceCommit) || e.Image.SourceCommit != e.SourceCommit {
		return fail("missing trial or matching immutable source/image revision")
	}
	if e.CPU < 2 || e.CPU > 28 || e.CPU%2 != 0 || e.Seconds < 1 || e.Seconds > 3600 {
		return fail("CPU/deadline outside Isolab limits")
	}
	if !regexp.MustCompile(`^([1-9][0-9]*)Gi$`).MatchString(e.Memory) {
		return fail("memory must be integer Gi")
	}
	var gib int
	_, _ = fmt.Sscanf(e.Memory, "%dGi", &gib)
	if gib > 128 {
		return fail("memory exceeds pool limit")
	}
	if !regexp.MustCompile(`^[0-9]{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/isolab/tasks@sha256:[a-f0-9]{64}$`).MatchString(e.Image.URI) {
		return fail("image must be digest-pinned in the Isolab task repository")
	}
	if err := exec.Command("git", "-C", root, "merge-base", "--is-ancestor", e.SourceCommit, revision).Run(); err != nil {
		return fail("source commit is not merged into the selected main revision")
	}
	record, err := readFile(root, e.Record)
	if err != nil {
		return fail("record: " + err.Error())
	}
	if strings.HasSuffix(e.Record, ".yaml") || strings.HasSuffix(e.Record, ".yml") {
		var s struct {
			ID     string `yaml:"id"`
			Status string `yaml:"status"`
			Source struct {
				Commit string `yaml:"commit"`
			} `yaml:"source"`
		}
		if yaml.Unmarshal(record, &s) != nil || s.Status != "approved" {
			return fail("experiment spec is not approved")
		}
		if s.ID != e.ID || s.Source.Commit != e.SourceCommit {
			return fail("approved spec/source identity differs from registration")
		}
		var generic map[string]any
		_ = yaml.Unmarshal(record, &generic)
		for _, v := range array(generic["gates"]) {
			if asMap(v)["kind"] == "human" {
				return fail("experiment contains a human gate; automation cannot answer it")
			}
		}
		if asMap(asMap(generic["agent"])["autonomy"])["launch_paid_host"] != "allowed" {
			return fail("approved spec does not authorize automated paid execution")
		}
		amount := func(v any) float64 {
			switch n := v.(type) {
			case int:
				return float64(n)
			case float64:
				return n
			}
			return 0
		}
		budget := amount(asMap(generic["budget"])["max_cost_usd"])
		allowance := amount(asMap(asMap(generic["agent"])["autonomy"])["spend_without_asking_usd"])
		if budget <= 0 || allowance < budget {
			return fail("approved spec needs a positive budget within its unattended spending allowance")
		}
		validator := os.Getenv("EXPSPEC_BIN")
		if validator == "" {
			return fail("native expspec validator is not configured")
		}
		cmd := exec.Command(validator, "validate", e.Record)
		cmd.Dir = root
		if result, err := cmd.CombinedOutput(); err != nil {
			if len(result) > 1024 {
				result = result[:1024]
			}
			return fail("expspec validation: " + string(result))
		}
	} else {
		return fail("attach the queue registration to an approved expspec; legacy backlog cards remain inventory, not implicit approval")
	}
	pb, err := readFile(root, e.Protocol.Path)
	if err != nil {
		return fail("protocol: " + err.Error())
	}
	if err = schemaCheck(filepath.Join(schemas, "experiment-protocol.v1.json"), pb); err != nil {
		return fail("protocol schema: " + err.Error())
	}
	p, err := object(pb)
	if err != nil {
		return fail(err.Error())
	}
	freeze, _ := p["freeze"].(map[string]any)
	if freeze["status"] != "frozen" || freeze["filed_before_affected_data"] != true {
		return fail("protocol is not prospectively frozen")
	}
	delete(p, "freeze")
	if !hex64.MatchString(e.Protocol.SHA256) || digest(canonical(p)) != e.Protocol.SHA256 || freeze["protocol_sha256"] != e.Protocol.SHA256 {
		return fail("frozen protocol hash mismatch")
	}
	execution, _ := p["execution"].(map[string]any)
	backend, _ := execution["backend"].(string)
	if backend != "isolab-k8s" && backend != "isolab" {
		return fail("frozen protocol does not select Isolab EKS")
	}
	ck, _ := execution["checkpoint"].(map[string]any)
	if ck["policy"] != "none" {
		return fail("resumable protocol needs the separate CRIU/EBS contract; ordinary jobs cannot substitute")
	}
	placement, _ := execution["placement"].(map[string]any)
	integer := func(v any) int64 {
		n, ok := v.(json.Number)
		if !ok {
			return -1
		}
		x, err := n.Int64()
		if err != nil {
			return -1
		}
		return x
	}
	if integer(placement["cpus"]) != int64(e.CPU) {
		return fail("CPU request differs from the frozen placement")
	}
	if spot, ok := placement["spot"].(bool); ok && !spot {
		return fail("frozen placement forbids the EKS Spot backend")
	}
	if limit := integer(execution["timeout_seconds"]); limit < 1 || int64(e.Seconds) > limit {
		return fail("deadline exceeds the frozen execution budget")
	}
	if limit := integer(placement["memory_mb"]); limit > 0 && int64(gib)*1024 > limit {
		return fail("memory exceeds the frozen placement")
	}
	if disk := integer(placement["disk_gb"]); disk > 8 {
		return fail("requested working disk exceeds the current 8 GiB Isolab pod volume")
	}
	if len(e.Outputs) > 16 {
		return fail("too many published outputs")
	}
	if len(canonical(e)) > 8192 {
		return fail("registration exceeds the native Isolab argv envelope")
	}
	hasExport := false
	for _, name := range e.Outputs {
		if !relpath(name) {
			return fail("unsafe output path")
		}
		if name == "export.json" {
			hasExport = true
		}
		ext := filepath.Ext(name)
		if ext != ".json" && ext != ".md" && ext != ".pdf" && ext != ".svg" {
			return fail("only declared JSON/Markdown/PDF/SVG outputs can return to Git")
		}
	}
	if !hasExport {
		return fail("export.json is required")
	}
	for _, in := range e.Inputs {
		if !relpath(in.Path) || !hex64.MatchString(in.SHA256) || !strings.HasPrefix(in.URI, "s3://") || strings.ContainsAny(in.URI, "\n\r\x00") {
			return fail("invalid pinned input")
		}
	}
	var spec json.RawMessage
	if e.Kind == "command" {
		if execution["measurement_class"] == "measurement" {
			return fail("clock/measurement experiments must use the benchmark adapter")
		}
		if !argvOK(e.Command) || !argvOK(e.Verify) || len(e.Image.RuntimeFiles) == 0 {
			return fail("command, independent verifier and runtime hashes are required")
		}
		for _, f := range e.Image.RuntimeFiles {
			if !filepath.IsAbs(f.Path) || strings.ContainsAny(f.Path, "\n\r\x00") || !hex64.MatchString(f.SHA256) {
				return fail("invalid runtime hash")
			}
		}
		bound := map[string]bool{}
		for _, f := range e.Image.RuntimeFiles {
			bound[f.Path] = true
		}
		if !bound[e.Command[0]] || !bound[e.Verify[0]] {
			return fail("command and verifier executables must have runtime hashes")
		}
	} else if e.Kind == "benchmark" {
		if e.CPU != 4 || !hex64.MatchString(e.Image.EcbenchSHA256) {
			return fail("native L2 benchmark requires 4 CPUs and the ecbench hash")
		}
		if !relpath(e.WorkloadSpec) {
			return fail("unsafe benchmark spec path")
		}
		sb, err := exec.Command("git", "-C", root, "show", e.SourceCommit+":"+e.WorkloadSpec).Output()
		if err != nil {
			return fail("benchmark spec is absent at the tested commit")
		}
		var v map[string]any
		if json.Unmarshal(sb, &v) != nil || v["schema"] != "ecbench.spec/v1" {
			return fail("invalid ecbench spec")
		}
		spec = sb
	} else {
		return fail("unsupported adapter kind")
	}
	return pb, spec, nil
}
func discover(root, schemas, repo, revision string, max int) Plan {
	p := Plan{Schema: "crypto.isolab-plan/v1", Repository: repo, SourceCommit: revision, Candidates: []Candidate{}, Blocked: []Blocked{}}
	entries, _ := filepath.Glob(filepath.Join(root, "experiments/queue/*.json"))
	sort.Strings(entries)
	registered := map[string]bool{}
	for _, path := range entries {
		rel, _ := filepath.Rel(root, path)
		rel = filepath.ToSlash(rel)
		b, err := readFile(root, rel)
		var e Entry
		if err == nil {
			err = decode(b, &e)
		}
		if err != nil {
			p.Blocked = append(p.Blocked, Blocked{rel, err.Error()})
			continue
		}
		registered[e.Record] = true
		pb, spec, err := validateEntry(root, schemas, revision, e)
		if err != nil {
			p.Blocked = append(p.Blocked, Blocked{rel, err.Error()})
			continue
		}
		if filepath.Base(rel) != e.ID+".json" {
			p.Blocked = append(p.Blocked, Blocked{rel, "registration filename must match id"})
			continue
		}
		key := e.key()
		id := strings.ReplaceAll(strings.ToLower(e.ID), "_", "-")
		if len(id) > 26 {
			id = id[:26]
		}
		p.Candidates = append(p.Candidates, Candidate{Entry: e, Key: key, JobName: "q-" + id + "-" + key[:20], RegistryPath: rel, ControllerCommit: revision, Protocol: pb, Spec: spec})
	}
	records, _ := filepath.Glob(filepath.Join(root, "experiments/specs/*.yaml"))
	cards, _ := filepath.Glob(filepath.Join(root, "backlog/*/card.json"))
	records = append(records, cards...)
	sort.Strings(records)
	for _, file := range records {
		rel, _ := filepath.Rel(root, file)
		rel = filepath.ToSlash(rel)
		if registered[rel] || strings.HasSuffix(rel, ".state.yaml") {
			continue
		}
		p.Blocked = append(p.Blocked, Blocked{rel, "no reviewed Isolab queue registration; approval, image, protocol and inputs must be bound before dispatch"})
	}
	// Apply the limit after permanent claims/results have been reconciled.
	_ = max
	return p
}
