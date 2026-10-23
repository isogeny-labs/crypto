package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"time"

	"gopkg.in/yaml.v3"
)

// State is crypto.experiment-state/v1, written only by this tool.
type State struct {
	Schema       string                `yaml:"schema"`
	ExperimentID string                `yaml:"experiment_id"`
	SpecVersion  int                   `yaml:"spec_version"`
	SpecSha256   string                `yaml:"spec_sha256"`
	Status       string                `yaml:"status"`
	BlockedOn    *string               `yaml:"blocked_on"`
	CreatedAt    string                `yaml:"created_at"`
	UpdatedAt    string                `yaml:"updated_at"`
	Vars         map[string]string     `yaml:"vars"`
	Hosts        []*HostState          `yaml:"hosts"`
	Instances    map[string]*Instance  `yaml:"instances"`
	Gates        map[string]*GateState `yaml:"gates"`
	Events       []Event               `yaml:"events"`
}

type HostState struct {
	Candidate  string  `yaml:"candidate"`
	State      string  `yaml:"state"`
	Handle     *string `yaml:"handle"`
	HourlyUSD  float64 `yaml:"hourly_usd"`
	StartedAt  *string `yaml:"started_at"`
	EndedAt    *string `yaml:"ended_at"`
	LastPollAt *string `yaml:"last_poll_at"`
}

type Instance struct {
	Step     string    `yaml:"step"`
	Cell     *string   `yaml:"cell"`
	Host     *int      `yaml:"host"`
	Status   string    `yaml:"status"`
	Reason   *string   `yaml:"reason"`
	Attempts []Attempt `yaml:"attempts"`
}

type Attempt struct {
	Card          string             `yaml:"card"`
	Receipt       *string            `yaml:"receipt"`
	ReceiptStatus *string            `yaml:"receipt_status"`
	ExitCode      *int               `yaml:"exit_code"`
	Seconds       *float64           `yaml:"seconds"`
	CardedAt      string             `yaml:"carded_at"`
	IngestedAt    *string            `yaml:"ingested_at"`
	Outputs       map[string]*string `yaml:"outputs"`
}

type GateState struct {
	Result  string `yaml:"result"`
	FirstAt string `yaml:"first_at,omitempty"`
	At      string `yaml:"at,omitempty"`
	Detail  string `yaml:"detail,omitempty"`
	By      string `yaml:"by,omitempty"`
}

type Event struct {
	At   string `yaml:"at"`
	Kind string `yaml:"kind"`
	Text string `yaml:"text"`
}

var (
	terminal = map[string]bool{"done": true, "skipped_done": true, "failed": true, "censored": true, "not_attempted": true, "void": true}
	okDone   = map[string]bool{"done": true, "skipped_done": true}
)

// clock is replaceable in tests.
var clock = func() time.Time { return time.Now().UTC() }

func now() string { return clock().Format("2006-01-02T15:04:05Z") }

func parseTS(s string) time.Time {
	t, err := time.Parse(time.RFC3339, s)
	if err != nil {
		return clock()
	}
	return t
}

func after(seconds int, base *string) string {
	start := clock()
	if base != nil {
		start = parseTS(*base)
	}
	return start.Add(time.Duration(seconds) * time.Second).Format("2006-01-02T15:04:05Z")
}

func sp(s string) *string { return &s }
func ip(i int) *int       { return &i }

func strOr(p *string, d string) string {
	if p == nil {
		return d
	}
	return *p
}

func (st *State) event(kind, text string) {
	st.Events = append(st.Events, Event{At: now(), Kind: kind, Text: text})
}

func statePath(specPath string) string {
	return strings.TrimSuffix(specPath, ".yaml") + ".state.yaml"
}

func loadState(specPath string, spec *Spec, raw []byte) (*State, error) {
	digest := sha256Hex(raw)
	p := statePath(specPath)
	b, err := os.ReadFile(p)
	if os.IsNotExist(err) {
		st := &State{Schema: "crypto.experiment-state/v1", ExperimentID: spec.ID, SpecVersion: spec.Version,
			SpecSha256: digest, Status: "not_started", CreatedAt: now(), UpdatedAt: now(),
			Vars: map[string]string{}, Hosts: []*HostState{}, Instances: map[string]*Instance{},
			Gates: map[string]*GateState{}, Events: []Event{}}
		st.event("init", fmt.Sprintf("state created for %s v%d", spec.ID, spec.Version))
		return st, nil
	}
	if err != nil {
		return nil, err
	}
	var st State
	if err := yaml.Unmarshal(b, &st); err != nil {
		return nil, fmt.Errorf("%s: %v", p, err)
	}
	if st.SpecSha256 != digest {
		return nil, fmt.Errorf("spec_drift: %s changed since the run started (state pins %s, file is %s). "+
			"An approved spec is frozen; amend with a new version and a new state", filepath.Base(specPath), st.SpecSha256, digest)
	}
	if st.Vars == nil {
		st.Vars = map[string]string{}
	}
	if st.Instances == nil {
		st.Instances = map[string]*Instance{}
	}
	if st.Gates == nil {
		st.Gates = map[string]*GateState{}
	}
	return &st, nil
}

func saveState(specPath string, st *State) error {
	st.UpdatedAt = now()
	b, err := yamlMarshal(st)
	if err != nil {
		return err
	}
	var node yaml.Node
	if err := yaml.Unmarshal(b, &node); err != nil {
		return err
	}
	generic, err := nodeToAny(&node)
	if err != nil {
		return err
	}
	if errs := schemaFindings("experiment-state.v1.schema.json", stateSchemaJSON, generic); len(errs) > 0 {
		return fmt.Errorf("internal: state would not validate: %s", strings.Join(errs, "; "))
	}
	tmp := statePath(specPath) + ".tmp"
	if err := os.WriteFile(tmp, b, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, statePath(specPath))
}

func (st *State) hostIndex() int {
	for i, h := range st.Hosts {
		if h.State == "provisioning" || h.State == "up" {
			return i
		}
	}
	return -1
}

func spend(spec *Spec, st *State) float64 {
	total := 0.0
	for _, h := range st.Hosts {
		if h.StartedAt == nil {
			continue
		}
		start := parseTS(*h.StartedAt)
		end := clock()
		if h.EndedAt != nil {
			end = parseTS(*h.EndedAt)
		}
		if c := spec.host(h.Candidate); c != nil && c.Lifecycle != nil && c.Lifecycle.LifetimeS > 0 {
			if capT := start.Add(time.Duration(c.Lifecycle.LifetimeS) * time.Second); end.After(capT) {
				end = capT
			}
		}
		if d := end.Sub(start).Hours(); d > 0 {
			total += d * h.HourlyUSD
		}
	}
	return math.Round(total*100) / 100
}

// ---------------------------------------------------------------------------
// Checks
// ---------------------------------------------------------------------------

var pathPartRE = regexp.MustCompile(`[^.\[\]]+|\[\d+\]`)

func jsonValue(ref *JSONRef, ctx ctxMap) (any, error) {
	file, err := render(ref.File, ctx)
	if err != nil {
		return nil, err
	}
	raw, err := os.ReadFile(repoPath(file))
	if err != nil {
		return nil, err
	}
	var docs []any
	if strings.HasSuffix(file, ".jsonl") {
		var rows []any
		sc := bufio.NewScanner(strings.NewReader(string(raw)))
		sc.Buffer(make([]byte, 1<<20), 1<<28)
		for sc.Scan() {
			if strings.TrimSpace(sc.Text()) == "" {
				continue
			}
			var v any
			if err := json.Unmarshal(sc.Bytes(), &v); err != nil {
				return nil, err
			}
			rows = append(rows, v)
		}
		if len(rows) == 0 {
			return nil, fmt.Errorf("%s: empty", file)
		}
		switch ref.Line {
		case "all":
			docs = rows
		case "first":
			docs = rows[:1]
		default:
			docs = rows[len(rows)-1:]
		}
	} else {
		var v any
		if err := json.Unmarshal(raw, &v); err != nil {
			return nil, err
		}
		docs = []any{v}
	}
	var out []any
	for _, d := range docs {
		cur := d
		for _, part := range pathPartRE.FindAllString(ref.Path, -1) {
			if strings.HasPrefix(part, "[") {
				i, _ := strconv.Atoi(part[1 : len(part)-1])
				arr, ok := cur.([]any)
				if !ok || i >= len(arr) {
					return nil, fmt.Errorf("no index %s", part)
				}
				cur = arr[i]
			} else {
				m, ok := cur.(map[string]any)
				if !ok {
					return nil, fmt.Errorf("no key %s", part)
				}
				v, ok := m[part]
				if !ok {
					return nil, fmt.Errorf("no key %s", part)
				}
				cur = v
			}
		}
		out = append(out, cur)
	}
	if ref.Line == "all" {
		return out, nil
	}
	return out[0], nil
}

func truthy(v any) bool {
	switch x := v.(type) {
	case nil:
		return false
	case bool:
		return x
	case float64:
		return x != 0
	case string:
		return x != ""
	case []any:
		return len(x) > 0
	case map[string]any:
		return len(x) > 0
	}
	return true
}

func equal(a, b any) bool {
	ja, _ := json.Marshal(normalize(a))
	jb, _ := json.Marshal(normalize(b))
	return string(ja) == string(jb)
}

func compareOp(op string, got, want any) bool {
	want = normalize(want)
	switch op {
	case "eq":
		return equal(got, want)
	case "ne":
		return !equal(got, want)
	case "exists":
		return true
	case "truthy":
		return truthy(got)
	case "in":
		list, _ := want.([]any)
		for _, w := range list {
			if equal(got, w) {
				return true
			}
		}
		return false
	case "all_true":
		list, ok := got.([]any)
		if !ok || len(list) == 0 {
			return false
		}
		for _, x := range list {
			if !truthy(x) {
				return false
			}
		}
		return true
	case "count_ge":
		list, _ := got.([]any)
		n := 0
		for _, x := range list {
			if truthy(x) {
				n++
			}
		}
		return float64(n) >= toFloat(want)
	}
	g, okg := got.(float64)
	w, okw := want.(float64)
	if !okg || !okw {
		return false
	}
	switch op {
	case "lt":
		return g < w
	case "le":
		return g <= w
	case "gt":
		return g > w
	case "ge":
		return g >= w
	}
	return false
}

func runCheck(c Check, ctx ctxMap, paths map[string]string) (bool, string) {
	r := func(s string) string { v, _ := render(s, ctx); return v }
	switch c.Kind {
	case "file_exists":
		p := r(c.Path)
		fi, err := os.Stat(repoPath(p))
		return err == nil && (fi.IsDir() || fi.Size() > 0), p
	case "sha256":
		p := r(c.Path)
		got, _ := sha256File(repoPath(p))
		return got == c.Sha256, fmt.Sprintf("%s sha256 %s", p, got)
	case "sha256_manifest":
		p := r(c.Path)
		raw, err := os.ReadFile(repoPath(p))
		if err != nil || len(raw) == 0 {
			return false, p + " missing"
		}
		for _, line := range strings.Split(string(raw), "\n") {
			f := strings.Fields(line)
			if len(f) < 2 {
				continue
			}
			name := strings.TrimPrefix(strings.Join(f[1:], " "), "*")
			target := filepath.Join(filepath.Dir(repoPath(p)), name)
			got, err := sha256File(target)
			if err != nil || got != f[0] {
				return false, name + " does not verify"
			}
		}
		return true, p + " verifies"
	case "status_token":
		token := r(c.Token)
		raw, _ := os.ReadFile(repoPath(paths["collected"] + "/STATUS"))
		return statusHas(string(raw), token), "STATUS token " + token
	case "command":
		argv, err := renderAll(c.Argv, ctx)
		if err != nil {
			return false, err.Error()
		}
		timeout := c.TimeoutS
		if timeout == 0 {
			timeout = 120
		}
		cmd := exec.Command(argv[0], argv[1:]...)
		cmd.Dir = repo
		done := make(chan error, 1)
		if err := cmd.Start(); err != nil {
			return false, err.Error()
		}
		go func() { done <- cmd.Wait() }()
		select {
		case err := <-done:
			if err != nil {
				return false, err.Error()
			}
			return true, "exit 0"
		case <-time.After(time.Duration(timeout) * time.Second):
			_ = cmd.Process.Kill()
			return false, "timed out"
		}
	case "json":
		got, err := jsonValue(c.JSON, ctx)
		if err != nil {
			if c.Op == "exists" {
				return false, "absent"
			}
			return false, "unreadable: " + err.Error()
		}
		jg, _ := json.Marshal(got)
		jw, _ := json.Marshal(normalize(c.Value))
		s := string(jg)
		if len(s) > 120 {
			s = s[:120]
		}
		return compareOp(c.Op, got, c.Value), fmt.Sprintf("value %s %s %s", s, c.Op, jw)
	}
	return false, "unknown check " + c.Kind
}

func statusHas(text, token string) bool {
	for _, line := range strings.Split(text, "\n") {
		if strings.HasSuffix(strings.TrimRight(line, " \t\r"), token) {
			return true
		}
	}
	return false
}

func allChecks(g *Gate, ctx ctxMap, paths map[string]string) (bool, string) {
	var details []string
	for _, c := range g.Checks {
		ok, d := runCheck(c, ctx, paths)
		if ok {
			details = append(details, "ok "+d)
		} else {
			details = append(details, "FAIL "+d)
			return false, strings.Join(details, "; ")
		}
	}
	return true, strings.Join(details, "; ")
}
