package main

// End-to-end tests. Each one builds a throwaway git repository holding only
// tools/task_card.py, the fixture files and a committed approval decision,
// then plays the main session with the built binary: call `next`, lint and
// run every card with the real tools/task_card.py, answer every ask,
// fast-forward every wait, until a terminal action. Nothing touches the
// real checkout. Needs git, go and python3 with PyYAML (task_card.py).
//
//	go -C tools/expspec test ./...

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
	"time"

	"gopkg.in/yaml.v3"
)

var (
	binPath  string
	realRepo string
)

func TestMain(m *testing.M) {
	wd, _ := os.Getwd()
	realRepo = filepath.Clean(filepath.Join(wd, "..", ".."))
	dir, err := os.MkdirTemp("", "expspec-bin-")
	if err != nil {
		panic(err)
	}
	binPath = filepath.Join(dir, "expspec")
	if out, err := exec.Command("go", "build", "-o", binPath, ".").CombinedOutput(); err != nil {
		panic(string(out))
	}
	code := m.Run()
	os.RemoveAll(dir)
	os.Exit(code)
}

type sandbox struct {
	t    *testing.T
	dir  string
	code string // commit holding the code and inputs
}

func sh(t *testing.T, dir string, name string, args ...string) string {
	t.Helper()
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("%s %v: %v\n%s", name, args, err, out)
	}
	return strings.TrimSpace(string(out))
}

func copyFile(t *testing.T, src, dst string) {
	t.Helper()
	b, err := os.ReadFile(src)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(dst, b, 0o644); err != nil {
		t.Fatal(err)
	}
}

func newSandbox(t *testing.T, files map[string]string, ids []string) *sandbox {
	t.Helper()
	dir := t.TempDir()
	copyFile(t, filepath.Join(realRepo, "tools", "task_card.py"), filepath.Join(dir, "tools", "task_card.py"))
	for rel, src := range files {
		copyFile(t, src, filepath.Join(dir, rel))
	}
	dec, _ := yaml.Marshal(map[string]any{"coordinator_decision": map[string]any{
		"decision": "approve", "target_ids": ids, "by": "owner"}})
	copyFile(t, writeTemp(t, dec), filepath.Join(dir, "decisions", "DEC-TEST.yaml"))
	if err := os.WriteFile(filepath.Join(dir, ".gitignore"), []byte(".fakehost/\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	s := &sandbox{t: t, dir: dir}
	sh(t, dir, "git", "init", "-q")
	s.code = s.commit("code")
	return s
}

func writeTemp(t *testing.T, b []byte) string {
	p := filepath.Join(t.TempDir(), "f")
	if err := os.WriteFile(p, b, 0o644); err != nil {
		t.Fatal(err)
	}
	return p
}

func (s *sandbox) commit(msg string) string {
	sh(s.t, s.dir, "git", "add", "-A")
	sh(s.t, s.dir, "git", "-c", "user.email=t@t", "-c", "user.name=t", "commit", "-qm", msg)
	return sh(s.t, s.dir, "git", "rev-parse", "HEAD")
}

var shaRE = regexp.MustCompile(`sha256: [0-9a-f]{64}`)

// approve pins the spec to the code commit and the frozen file's hash, marks
// it approved and commits it.
func (s *sandbox) approve(rel, frozen string) {
	p := filepath.Join(s.dir, rel)
	b, _ := os.ReadFile(p)
	digest, _ := sha256File(filepath.Join(s.dir, frozen))
	t := strings.ReplaceAll(string(b), "COMMIT", s.code)
	t = strings.ReplaceAll(t, "FSHA", digest)
	t = shaRE.ReplaceAllString(t, "sha256: "+digest)
	if strings.Contains(t, "status: draft") {
		t = regexp.MustCompile(`status: draft.*`).ReplaceAllString(t, "status: approved")
		t = strings.Replace(t, "  ref: main\n", "  ref: main\n  commit: "+s.code+"\n", 1)
		t = regexp.MustCompile(`\ntags: (.*)\n`).ReplaceAllString(t, "\ntags: $1\napproval: {decision: decisions/DEC-TEST.yaml, approved_by: owner, approved_at: '2026-10-20T20:00:00Z'}\n")
	}
	if err := os.WriteFile(p, []byte(t), 0o644); err != nil {
		s.t.Fatal(err)
	}
	s.commit("approve")
}

func (s *sandbox) expspec(args ...string) (string, int) {
	cmd := exec.Command(binPath, args...)
	cmd.Dir = s.dir
	out, err := cmd.Output()
	code := 0
	if ee, ok := err.(*exec.ExitError); ok {
		code = ee.ExitCode()
		out = append(out, ee.Stderr...)
	}
	return string(out), code
}

func (s *sandbox) state(rel string) *State {
	b, err := os.ReadFile(filepath.Join(s.dir, strings.TrimSuffix(rel, ".yaml")+".state.yaml"))
	if err != nil {
		s.t.Fatal(err)
	}
	var st State
	if err := yaml.Unmarshal(b, &st); err != nil {
		s.t.Fatal(err)
	}
	return &st
}

func always(ans string) func(string) string { return func(string) string { return ans } }

// drive plays the main session until a terminal action and returns every
// action. answer picks --yes or --no for each gate id.
func (s *sandbox) drive(rel string, answer func(gate string) string) []map[string]any {
	var actions []map[string]any
	statePath := filepath.Join(s.dir, strings.TrimSuffix(rel, ".yaml")+".state.yaml")
	for n := 0; n < 60; n++ {
		out, code := s.expspec("next", rel)
		var a map[string]any
		if err := json.Unmarshal([]byte(out), &a); err != nil {
			s.t.Fatalf("next printed non-JSON (exit %d): %s", code, out)
		}
		actions = append(actions, a)
		switch a["action"] {
		case "run_card":
			card := a["card"].(string)
			lint := exec.Command("python3", "tools/task_card.py", "lint", card)
			lint.Dir = s.dir
			if out, err := lint.CombinedOutput(); err != nil {
				s.t.Fatalf("card %s does not lint: %s", card, out)
			}
			run := exec.Command("python3", "tools/task_card.py", "run", card)
			run.Dir = s.dir
			_ = run.Run()
		case "ask", "escalate":
			st := s.state(rel)
			if st.BlockedOn == nil {
				return actions
			}
			if out, code := s.expspec("answer", rel, "--gate", *st.BlockedOn, answer(*st.BlockedOn), "--by", "test-owner"); code != 0 {
				s.t.Fatalf("answer: %s", out)
			}
		case "wait":
			st := s.state(rel)
			for _, h := range st.Hosts {
				if h.State != "up" && h.State != "provisioning" {
					continue
				}
				for _, p := range []**string{&h.LastPollAt, &h.StartedAt} {
					if *p != nil {
						*p = sp(parseTS(**p).Add(-400 * time.Second).Format("2006-01-02T15:04:05Z"))
					}
				}
			}
			b, _ := yamlMarshal(st)
			if err := os.WriteFile(statePath, b, 0o644); err != nil {
				s.t.Fatal(err)
			}
		default:
			return actions
		}
	}
	s.t.Fatalf("no terminal action after 60 steps: %v", actions[len(actions)-3:])
	return nil
}

func keysOf(actions []map[string]any, kind, field string) []string {
	var out []string
	for _, a := range actions {
		if a["action"] == kind {
			out = append(out, fmt.Sprint(a[field]))
		}
	}
	return out
}

const smokeRel = "experiments/specs/EXP-EXPSPEC-SMOKE-001.yaml"
const smokeCode = "research/expspec_smoke_20261020/smoke.go"

func smokeBox(t *testing.T) *sandbox {
	s := newSandbox(t, map[string]string{
		smokeRel:  filepath.Join(realRepo, smokeRel),
		smokeCode: filepath.Join(realRepo, smokeCode),
	}, []string{"EXP-EXPSPEC-SMOKE-001"})
	s.approve(smokeRel, smokeCode)
	return s
}

func TestCommittedSpecsValidate(t *testing.T) {
	files, _ := filepath.Glob(filepath.Join(realRepo, "experiments", "specs", "EXP-*[0-9].yaml"))
	if len(files) == 0 {
		t.Fatal("no specs found")
	}
	for _, f := range files {
		cmd := exec.Command(binPath, "validate", f)
		cmd.Dir = realRepo
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Errorf("%s: %s", f, out)
		}
	}
}

func TestDraftRefusesToRun(t *testing.T) {
	cmd := exec.Command(binPath, "next", filepath.Join(realRepo, smokeRel))
	cmd.Dir = realRepo
	out, _ := cmd.Output()
	var a map[string]any
	_ = json.Unmarshal(out, &a)
	if a["action"] != "refused" {
		t.Fatalf("want refused, got %s", out)
	}
}

func TestLocalRunsToDone(t *testing.T) {
	s := smokeBox(t)
	actions := s.drive(smokeRel, always("--yes"))
	if last := actions[len(actions)-1]; last["action"] != "done" {
		t.Fatalf("last action %v", last)
	}
	got := strings.Join(keysOf(actions, "run_card", "key"), " ")
	if want := "host#0 measure@n3 measure@n10 measure@n100 summarize"; got != want {
		t.Fatalf("cards %q, want %q", got, want)
	}
	st := s.state(smokeRel)
	if st.Status != "completed" || st.Vars["last_cell"] != "100" {
		t.Fatalf("state %s vars %v", st.Status, st.Vars)
	}
	out, _ := s.expspec("facts", smokeRel)
	var facts struct {
		Metrics map[string]struct {
			Values map[string]float64 `json:"values"`
		} `json:"metrics"`
		Missing []string `json:"missing_artifacts"`
	}
	if err := json.Unmarshal([]byte(out), &facts); err != nil {
		t.Fatal(out)
	}
	v := facts.Metrics["value"].Values
	if v["n3"] != 6 || v["n10"] != 55 || v["n100"] != 5050 || len(facts.Missing) != 0 {
		t.Fatalf("facts %s", out)
	}
}

func TestEditingARunningSpecIsDrift(t *testing.T) {
	s := smokeBox(t)
	s.expspec("next", smokeRel)
	p := filepath.Join(s.dir, smokeRel)
	b, _ := os.ReadFile(p)
	_ = os.WriteFile(p, []byte(strings.Replace(string(b), "max_wall_hours: 1", "max_wall_hours: 2", 1)), 0o644)
	out, _ := s.expspec("next", smokeRel)
	if !strings.Contains(out, `"escalate"`) || !strings.Contains(out, "spec_drift") {
		t.Fatalf("want spec_drift escalation, got %s", out)
	}
}

func TestResumeSkipsDoneCells(t *testing.T) {
	s := smokeBox(t)
	out := filepath.Join(s.dir, "research/expspec_smoke_20261020/out")
	_ = os.MkdirAll(out, 0o755)
	_ = os.WriteFile(filepath.Join(out, "cell-n10.json"), []byte(`{"n": 10, "value": 55, "ok": true}`+"\n"), 0o644)
	actions := s.drive(smokeRel, always("--yes"))
	for _, k := range keysOf(actions, "run_card", "key") {
		if k == "measure@n10" {
			t.Fatal("measure@n10 ran although its output existed")
		}
	}
	if got := s.state(smokeRel).Instances["measure@n10"].Status; got != "skipped_done" {
		t.Fatalf("measure@n10 is %s", got)
	}
}

const batchRel = "experiments/specs/EXP-EXPSPEC-BATCH-001.yaml"

func batchBox(t *testing.T) *sandbox {
	s := newSandbox(t, map[string]string{
		batchRel: "testdata/EXP-EXPSPEC-BATCH-001.yaml",
		"research/expspec_batch_test/fakespot.go": "testdata/fakespot.go",
		"research/expspec_batch_test/score.go":    "testdata/score.go",
	}, []string{"EXP-EXPSPEC-BATCH-001"})
	s.approve(batchRel, "research/expspec_batch_test/fakespot.go")
	return s
}

func TestBatchTwoHostsToDone(t *testing.T) {
	s := batchBox(t)
	actions := s.drive(batchRel, always("--yes"))
	if last := actions[len(actions)-1]; last["action"] != "done" {
		t.Fatalf("last action %v", actions[len(actions)-3:])
	}
	if esc := keysOf(actions, "escalate", "reason"); len(esc) > 0 {
		t.Fatalf("unexpected escalations %v", esc)
	}
	asks := strings.Join(keysOf(actions, "ask", "gate"), " ")
	if want := "launch-small-0 go:score@a1 launch-big-1 go:score@a2"; asks != want {
		t.Fatalf("asks %q, want %q", asks, want)
	}
	var phases []string
	for _, k := range keysOf(actions, "run_card", "key") {
		phases = append(phases, strings.SplitN(k, "#", 2)[0])
	}
	count := func(p string) int {
		n := 0
		for _, x := range phases {
			if x == p {
				n++
			}
		}
		return n
	}
	if phases[0] != "_provision" || phases[1] != "_poll" || count("_provision") != 2 || count("_teardown") != 2 {
		t.Fatalf("lifecycle %v", phases)
	}
	st := s.state(batchRel)
	if len(st.Hosts) != 2 || st.Hosts[0].Candidate != "small" || st.Hosts[1].Candidate != "big" {
		t.Fatalf("hosts %+v", st.Hosts)
	}
	for _, h := range st.Hosts {
		if h.State != "torn_down" {
			t.Fatalf("host %s is %s", h.Candidate, h.State)
		}
		if _, err := os.Stat(filepath.Join(s.dir, ".fakehost", *h.Handle, "TERMINATED")); err != nil {
			t.Fatalf("host %s was not terminated", *h.Handle)
		}
	}
	for _, c := range []string{"a1", "a2"} {
		for _, step := range []string{"measure", "score"} {
			if got := st.Instances[step+"@"+c].Status; got != "done" {
				t.Fatalf("%s@%s is %s", step, c, got)
			}
		}
	}
}

func TestBatchNoToLaunchStops(t *testing.T) {
	s := batchBox(t)
	actions := s.drive(batchRel, always("--no"))
	if last := actions[len(actions)-1]; last["action"] != "stopped" {
		t.Fatalf("last action %v", last)
	}
	if st := s.state(batchRel); len(st.Hosts) != 0 {
		t.Fatalf("hosts launched after a no: %+v", st.Hosts)
	}
}

func TestSchemaRejectsGateOnBatchHostStep(t *testing.T) {
	b, _ := os.ReadFile("testdata/EXP-EXPSPEC-BATCH-001.yaml")
	filled := strings.NewReplacer("COMMIT", strings.Repeat("a", 40), "FSHA", strings.Repeat("a", 64)).Replace(string(b))
	bad := strings.Replace(filled, "    status_done: \"MEASURED {{cell.id}}\"\n",
		"    status_done: \"MEASURED {{cell.id}}\"\n    gates: {before: [go]}\n", 1)
	dir := t.TempDir()
	p := filepath.Join(dir, "EXP-EXPSPEC-BATCH-001.yaml")
	_ = os.WriteFile(p, []byte(bad), 0o644)
	cmd := exec.Command(binPath, "validate", p)
	cmd.Dir = realRepo
	out, _ := cmd.CombinedOutput()
	if !strings.Contains(string(out), "a batch host runs its steps by itself") {
		t.Fatalf("gate on a batch host step was accepted: %s", out)
	}
}

// editSpec rewrites a sandbox spec through a generic map and commits it.
func (s *sandbox) editSpec(rel, msg string, edit func(spec map[string]any)) {
	p := filepath.Join(s.dir, rel)
	b, _ := os.ReadFile(p)
	var spec map[string]any
	if err := yaml.Unmarshal(b, &spec); err != nil {
		s.t.Fatal(err)
	}
	edit(spec)
	out, _ := yaml.Marshal(spec)
	if err := os.WriteFile(p, out, 0o644); err != nil {
		s.t.Fatal(err)
	}
	s.commit(msg)
}

func stepAt(spec map[string]any, i int) map[string]any {
	return spec["steps"].([]any)[i].(map[string]any)
}

func addGate(spec map[string]any, g map[string]any) {
	spec["gates"] = append(spec["gates"].([]any), g)
}

func TestHumanAfterGateRequiresAnswerBeforeMoreWork(t *testing.T) {
	s := smokeBox(t)
	s.editSpec(smokeRel, "add after gate", func(spec map[string]any) {
		addGate(spec, map[string]any{"id": "owner-after", "kind": "human", "description": "review output",
			"ask": "Continue after this output.", "on_fail": "stop"})
		g := stepAt(spec, 1)["gates"].(map[string]any)
		g["after"] = append(g["after"].([]any), "owner-after")
	})
	actions := s.drive(smokeRel, always("--no"))
	if last := actions[len(actions)-1]; last["action"] != "stopped" {
		t.Fatalf("last %v", last)
	}
	if asks := strings.Join(keysOf(actions, "ask", "gate"), " "); asks != "owner-after:measure@n3" {
		t.Fatalf("asks %q", asks)
	}
	for _, k := range keysOf(actions, "run_card", "key") {
		if k == "measure@n10" {
			t.Fatal("work scheduled past an unanswered after-gate")
		}
	}
	if st := s.state(smokeRel); st.Status != "stopped" {
		t.Fatalf("status %s", st.Status)
	}
}

func TestRefreshReceiptResumesWaitGate(t *testing.T) {
	s := smokeBox(t)
	s.editSpec(smokeRel, "add refresh gate", func(spec map[string]any) {
		addGate(spec, map[string]any{"id": "refresh-host", "kind": "wait_until", "description": "host metadata",
			"checks":  []any{map[string]any{"kind": "file_exists", "path": "{{paths.out}}/host.json"}},
			"refresh": stepAt(spec, 0)["argv"], "every_s": 60, "max_wait_s": 60, "on_fail": "stop"})
		stepAt(spec, 0)["gates"] = map[string]any{"before": []any{"refresh-host"}}
	})
	actions := s.drive(smokeRel, always("--yes"))
	if last := actions[len(actions)-1]; last["action"] != "done" {
		t.Fatalf("last %v", last)
	}
	if got := s.state(smokeRel).Instances["_refresh-refresh-host"].Status; got != "done" {
		t.Fatalf("refresh instance is %s", got)
	}
}

func TestWaitAfterGateRefreshesBeforeDependentWork(t *testing.T) {
	s := smokeBox(t)
	s.editSpec(smokeRel, "add wait after gate", func(spec map[string]any) {
		refresh := append([]any{}, stepAt(spec, 0)["argv"].([]any)...)
		refresh[len(refresh)-1] = "{{paths.out}}/after"
		addGate(spec, map[string]any{"id": "after-ready", "kind": "wait_until", "description": "after metadata",
			"checks":  []any{map[string]any{"kind": "file_exists", "path": "{{paths.out}}/after/host.json"}},
			"refresh": refresh, "every_s": 60, "max_wait_s": 120, "on_fail": "stop"})
		stepAt(spec, 0)["gates"] = map[string]any{"after": []any{"after-ready"}}
	})
	actions := s.drive(smokeRel, always("--yes"))
	if last := actions[len(actions)-1]; last["action"] != "done" {
		t.Fatalf("last %v", last)
	}
	keys := keysOf(actions, "run_card", "key")
	idx := func(k string) int {
		for i, x := range keys {
			if x == k {
				return i
			}
		}
		return -1
	}
	if r, m := idx("_refresh-after-ready"), idx("measure@n3"); r < 0 || m < 0 || r > m {
		t.Fatalf("refresh at %d, measure@n3 at %d: %v", r, m, keys)
	}
}

func TestResumeStillValidatesCachedOutput(t *testing.T) {
	s := smokeBox(t)
	out := filepath.Join(s.dir, "research/expspec_smoke_20261020/out")
	_ = os.MkdirAll(out, 0o755)
	_ = os.WriteFile(filepath.Join(out, "cell-n10.json"), []byte(`{"n": 10, "value": 55, "ok": false}`+"\n"), 0o644)
	s.drive(smokeRel, always("--yes"))
	if got := s.state(smokeRel).Instances["measure@n10"].Status; got != "void" {
		t.Fatalf("measure@n10 is %s, want void", got)
	}
}

func TestBatchNoToRunningHostGateTearsDownAndStaysStopped(t *testing.T) {
	s := batchBox(t)
	actions := s.drive(batchRel, func(g string) string {
		if strings.HasPrefix(g, "launch-") {
			return "--yes"
		}
		return "--no"
	})
	if last := actions[len(actions)-1]; last["action"] != "stopped" {
		t.Fatalf("last %v", last)
	}
	st := s.state(batchRel)
	if st.Status != "stopped" || len(st.Hosts) != 1 || st.Hosts[0].State != "torn_down" {
		t.Fatalf("status %s hosts %+v", st.Status, st.Hosts)
	}
	if _, err := os.Stat(filepath.Join(s.dir, ".fakehost", *st.Hosts[0].Handle, "TERMINATED")); err != nil {
		t.Fatal("host not terminated")
	}
	out, _ := s.expspec("next", batchRel)
	if !strings.Contains(out, `"stopped"`) {
		t.Fatalf("a stopped run resumed: %s", out)
	}
}

func TestBatchFailedStepStopSurvivesTeardown(t *testing.T) {
	s := batchBox(t)
	s.editSpec(batchRel, "stop on failed score", func(spec map[string]any) {
		score := stepAt(spec, 2)
		score["argv"] = []any{"false"}
		score["outputs"] = []any{}
		score["on_failure"] = "stop"
	})
	actions := s.drive(batchRel, always("--yes"))
	if last := actions[len(actions)-1]; last["action"] != "stopped" {
		t.Fatalf("last %v", last)
	}
	st := s.state(batchRel)
	if st.Status != "stopped" || len(st.Hosts) != 1 || st.Hosts[0].State != "torn_down" {
		t.Fatalf("status %s hosts %+v", st.Status, st.Hosts)
	}
}

func TestBatchTeardownPermissionAskPreservesStop(t *testing.T) {
	s := batchBox(t)
	s.editSpec(batchRel, "ask before teardown", func(spec map[string]any) {
		spec["agent"].(map[string]any)["autonomy"].(map[string]any)["terminate_own_hosts"] = "ask"
	})
	actions := s.drive(batchRel, func(g string) string {
		if strings.HasPrefix(g, "go:") {
			return "--no"
		}
		return "--yes"
	})
	if last := actions[len(actions)-1]; last["action"] != "stopped" {
		t.Fatalf("last %v", last)
	}
	if !strings.Contains(strings.Join(keysOf(actions, "ask", "gate"), " "), "teardown-0") {
		t.Fatalf("no teardown ask: %v", keysOf(actions, "ask", "gate"))
	}
	st := s.state(batchRel)
	if st.Status != "stopped" || len(st.Hosts) != 1 || st.Hosts[0].State != "torn_down" {
		t.Fatalf("status %s hosts %+v", st.Status, st.Hosts)
	}
}
