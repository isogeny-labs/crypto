package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

const cardSchema = "crypto.autoresearch.mechanical-task-card.v1"

var stopWhen = []string{"python3 tools/task_card.py run exits with status 2."}

// Card is the tools/task_card.py frozen-command card, fields in its order.
type Card struct {
	Schema           string        `yaml:"schema"`
	CardID           string        `yaml:"card_id"`
	ExperimentID     string        `yaml:"experiment_id"`
	AuthorRole       string        `yaml:"author_role"`
	ExecutorRole     string        `yaml:"executor_role"`
	Kind             string        `yaml:"kind"`
	Commit           string        `yaml:"commit"`
	ApprovalDecision string        `yaml:"approval_decision"`
	Receipt          string        `yaml:"receipt"`
	Binds            []Frozen      `yaml:"binds"`
	MustNotExist     []string      `yaml:"must_not_exist"`
	Commands         []CardCommand `yaml:"commands"`
	BusAddress       string        `yaml:"bus_address"`
	Background       bool          `yaml:"background"`
	WriteScope       []string      `yaml:"write_scope"`
	StopWhen         []string      `yaml:"stop_when"`
	Notes            string        `yaml:"notes"`
}

type Retry struct {
	ExitCodes   []int `yaml:"exit_codes"`
	MaxAttempts int   `yaml:"max_attempts"`
}

type CardCommand struct {
	ID               string            `yaml:"id"`
	Argv             []string          `yaml:"argv"`
	Cwd              string            `yaml:"cwd,omitempty"`
	Env              map[string]string `yaml:"env,omitempty"`
	TimeoutSeconds   int               `yaml:"timeout_seconds"`
	SuccessExitCodes []int             `yaml:"success_exit_codes"`
	Retry            *Retry            `yaml:"retry,omitempty"`
	Outputs          []string          `yaml:"outputs"`
	Stdout           string            `yaml:"stdout"`
	Stderr           string            `yaml:"stderr"`
}

// pending command plus the card-level fields it implies.
type cmdSpec struct {
	CardCommand
	mustNotExist []string
	background   bool
	extraScope   []string
}

var nonSlug = regexp.MustCompile(`[^a-z0-9-]`)

func busAddress(id string) string {
	s := nonSlug.ReplaceAllString(strings.ToLower(id), "-")
	if len(s) > 40 {
		s = s[:40]
	}
	return "executor-mechanical-" + s
}

func checkHead(spec *Spec, paths map[string]string) (string, error) {
	_, head := git("rev-parse", "HEAD")
	src := spec.Source.Commit
	if head == src {
		return head, nil
	}
	if code, _ := git("merge-base", "--is-ancestor", src, head); code != 0 {
		return "", fmt.Errorf("spec_drift: HEAD %s does not descend from source.commit %s", head, src)
	}
	if spec.Source.AllowDrift {
		return head, nil
	}
	allowed := []string{"experiments/specs/", path.Dir(spec.Approval.Decision) + "/",
		paths["cards"] + "/", paths["collected"] + "/", paths["out"] + "/"}
	_, changed := git("diff", "--name-only", src, head)
	var stray []string
	for _, c := range strings.Split(changed, "\n") {
		if c == "" {
			continue
		}
		ok := false
		for _, a := range allowed {
			ok = ok || strings.HasPrefix(c, a)
		}
		if !ok {
			stray = append(stray, c)
		}
	}
	if len(stray) > 0 {
		if len(stray) > 5 {
			stray = stray[:5]
		}
		return "", fmt.Errorf("spec_drift: commits after source.commit change %v; only specs, the decision "+
			"and run products may change unless allow_drift", stray)
	}
	return head, nil
}

func attemptsTotal(st *State) int {
	n := 0
	for _, i := range st.Instances {
		n += len(i.Attempts)
	}
	return n
}

// writeCard compiles one command into a card, lints it with task_card.py and
// records the instance as carded. Returns the card's repo path.
func writeCard(spec *Spec, st *State, paths map[string]string, key string, c cmdSpec) (string, error) {
	head, err := checkHead(spec, paths)
	if err != nil {
		return "", err
	}
	var binds []Frozen
	bound := map[string]bool{}
	for _, f := range spec.Source.Frozen {
		got, err := sha256File(repoPath(f.Path))
		if err != nil || got != f.Sha256 {
			return "", fmt.Errorf("spec_drift: frozen file %s is missing or changed", f.Path)
		}
		binds = append(binds, f)
		bound[f.Path] = true
	}
	for _, tok := range c.Argv {
		if bound[tok] {
			continue
		}
		if fi, err := os.Stat(repoPath(tok)); err == nil && fi.Mode().IsRegular() && !filepath.IsAbs(tok) {
			sum, _ := sha256File(repoPath(tok))
			binds = append(binds, Frozen{Path: tok, Sha256: sum})
			bound[tok] = true
		}
	}
	for _, d := range paths {
		if err := os.MkdirAll(repoPath(d), 0o755); err != nil {
			return "", err
		}
	}
	cardID := fmt.Sprintf("CARD-%s-%03d", spec.ID, attemptsTotal(st)+1)
	cards := paths["cards"]
	cmd := c.CardCommand
	id := strings.Trim(nonSlug.ReplaceAllString(strings.ToLower(key), "-"), "-")
	if len(id) > 60 {
		id = id[:60]
	}
	if id == "" {
		id = "step"
	}
	cmd.ID = id
	cmd.Stdout = cards + "/" + cardID + ".stdout"
	cmd.Stderr = cards + "/" + cardID + ".stderr"
	if cmd.Outputs == nil {
		cmd.Outputs = []string{}
	}
	scopeSet := map[string]bool{}
	for _, p := range paths {
		scopeSet[p] = true
	}
	for _, p := range c.extraScope {
		scopeSet[p] = true
	}
	scope := sortedKeys(scopeSet)
	mne := c.mustNotExist
	if mne == nil {
		mne = []string{}
	}
	if binds == nil {
		binds = []Frozen{}
	}
	card := Card{
		Schema: cardSchema, CardID: cardID, ExperimentID: spec.ID,
		AuthorRole: "experiment-director", ExecutorRole: "executor-mechanical",
		Kind: "frozen_command", Commit: head, ApprovalDecision: spec.Approval.Decision,
		Receipt: cards + "/" + cardID + ".receipt.json", Binds: binds, MustNotExist: mne,
		Commands: []CardCommand{cmd}, BusAddress: busAddress(spec.ID), Background: c.background,
		WriteScope: scope, StopWhen: stopWhen,
		Notes: fmt.Sprintf("Compiled by tools/expspec from the experiment spec, version %d, instance %s.", spec.Version, key),
	}
	rel := cards + "/" + cardID + ".yaml"
	if _, err := os.Stat(repoPath(rel)); err == nil {
		return "", fmt.Errorf("%s exists; cards are write-once", rel)
	}
	b, err := yamlMarshal(card)
	if err != nil {
		return "", err
	}
	if err := os.WriteFile(repoPath(rel), b, 0o644); err != nil {
		return "", err
	}
	lint := exec.Command("python3", "tools/task_card.py", "lint", rel)
	lint.Dir = repo
	if out, err := lint.CombinedOutput(); err != nil {
		_ = os.Remove(repoPath(rel))
		return "", fmt.Errorf("compiled card does not lint (fix the spec): %s", strings.TrimSpace(string(out)))
	}
	inst := st.Instances[key]
	if inst == nil {
		step := strings.SplitN(strings.SplitN(key, "@", 2)[0], "#", 2)[0]
		inst = &Instance{Step: step, Status: "pending", Attempts: []Attempt{}}
		st.Instances[key] = inst
	}
	inst.Status = "carded"
	inst.Attempts = append(inst.Attempts, Attempt{Card: rel, Receipt: sp(card.Receipt), CardedAt: now(),
		Outputs: map[string]*string{}})
	st.event("card", key+": "+rel)
	return rel, nil
}

func stepCommand(spec *Spec, s *Step, ctx ctxMap, host *Host) (cmdSpec, error) {
	argv, err := renderAll(s.Argv, ctx)
	if err != nil {
		return cmdSpec{}, err
	}
	envIn := map[string]string{}
	for k, v := range spec.Env {
		envIn[k] = v
	}
	for k, v := range s.Env {
		envIn[k] = v
	}
	env, err := renderMap(envIn, ctx)
	if err != nil {
		return cmdSpec{}, err
	}
	remote := s.RunsOn == "host" && host != nil && host.Backend == "ssh"
	if remote {
		workdir := scalarString(host.Config["workdir"])
		var envs []string
		for _, k := range sortedKeys(env) {
			envs = append(envs, k+"="+shellQuote(env[k]))
		}
		prefix := ""
		if len(envs) > 0 {
			prefix = strings.Join(envs, " ") + " "
		}
		quoted := make([]string, len(argv))
		for i, a := range argv {
			quoted[i] = shellQuote(a)
		}
		argv = []string{"ssh", scalarString(host.Config["host"]),
			"cd " + shellQuote(workdir) + " && " + prefix + strings.Join(quoted, " ")}
		env = nil
	}
	codes := s.SuccessExitCodes
	if len(codes) == 0 {
		codes = []int{0}
	}
	var outputs []string
	if !remote {
		if outputs, err = renderAll(s.Outputs, ctx); err != nil {
			return cmdSpec{}, err
		}
	}
	mne, err := renderAll(s.MustNotExist, ctx)
	if err != nil {
		return cmdSpec{}, err
	}
	scope, err := renderAll(s.WriteScope, ctx)
	if err != nil {
		return cmdSpec{}, err
	}
	bg := s.TimeoutS > 600
	if s.Background != nil {
		bg = *s.Background
	}
	c := cmdSpec{CardCommand: CardCommand{Argv: argv, TimeoutSeconds: s.TimeoutS, SuccessExitCodes: codes,
		Outputs: outputs}, mustNotExist: mne, background: bg, extraScope: scope}
	if len(env) > 0 {
		c.Env = env
	}
	if s.Cwd != "" {
		if c.Cwd, err = render(s.Cwd, ctx); err != nil {
			return cmdSpec{}, err
		}
	}
	if len(s.RetryExitCodes) > 0 {
		n := spec.Budget.MaxAttemptsPerStep
		if n < 2 {
			n = 2
		}
		if n > 3 {
			n = 3
		}
		c.Retry = &Retry{ExitCodes: s.RetryExitCodes, MaxAttempts: n}
	}
	return c, nil
}

func lifecycleCommand(h *Host, phase string, ctx ctxMap) (cmdSpec, error) {
	lc := h.Lifecycle.phase(phase)
	argv, err := renderAll(lc.Argv, ctx)
	if err != nil {
		return cmdSpec{}, err
	}
	c := cmdSpec{CardCommand: CardCommand{Argv: argv, TimeoutSeconds: lc.TimeoutS, SuccessExitCodes: []int{0}},
		background: lc.TimeoutS > 600}
	if len(lc.Env) > 0 {
		if c.Env, err = renderMap(lc.Env, ctx); err != nil {
			return cmdSpec{}, err
		}
	}
	if len(lc.RetryExitCodes) > 0 {
		c.Retry = &Retry{ExitCodes: lc.RetryExitCodes, MaxAttempts: 3}
	}
	return c, nil
}

func shellQuote(s string) string {
	if s != "" && regexp.MustCompile(`^[A-Za-z0-9_@%+=:,./-]+$`).MatchString(s) {
		return s
	}
	return "'" + strings.ReplaceAll(s, "'", `'"'"'`) + "'"
}

// ---------------------------------------------------------------------------
// Ingest
// ---------------------------------------------------------------------------

type receipt struct {
	Status   string `json:"status"`
	Commands []struct {
		ExitCode *int               `json:"exit_code"`
		TimedOut bool               `json:"timed_out"`
		Seconds  *float64           `json:"seconds"`
		Outputs  map[string]*string `json:"outputs"`
	} `json:"commands"`
}

func capture(spec map[string]Capture, stdout string, st *State) error {
	for _, name := range sortedKeys(spec) {
		how := spec[name]
		val := ""
		found := false
		if how.JSONKey != "" {
			for _, line := range strings.Split(stdout, "\n") {
				var doc map[string]any
				if json.Unmarshal([]byte(line), &doc) != nil {
					continue
				}
				if v, ok := doc[how.JSONKey]; ok {
					val, found = scalarString(v), true
					break
				}
			}
		} else if m := regexp.MustCompile(how.Regex).FindStringSubmatch(stdout); len(m) > 1 {
			val, found = m[1], true
		}
		if !found {
			return fmt.Errorf("capture %s: not found in stdout", name)
		}
		st.Vars[name] = val
		st.event("capture", name+" = "+val)
	}
	return nil
}

func sortedInstanceKeys(st *State) []string { return sortedKeys(st.Instances) }

// ingestPending reads every receipt that appeared since the last call and
// returns the escalation reasons it produced.
func ingestPending(spec *Spec, st *State, paths map[string]string, cellsByID map[string]*Cell) []string {
	var problems []string
	for _, key := range sortedInstanceKeys(st) {
		inst := st.Instances[key]
		if inst.Status != "carded" {
			continue
		}
		att := &inst.Attempts[len(inst.Attempts)-1]
		raw, err := os.ReadFile(repoPath(strOr(att.Receipt, "")))
		if err != nil {
			continue
		}
		var rc receipt
		if err := json.Unmarshal(raw, &rc); err != nil {
			problems = append(problems, fmt.Sprintf("%s: receipt does not parse: %v", key, err))
			continue
		}
		var code *int
		timedOut := false
		if n := len(rc.Commands); n > 0 {
			last := rc.Commands[n-1]
			code, timedOut = last.ExitCode, last.TimedOut
			att.Seconds = last.Seconds
			if last.Outputs != nil {
				att.Outputs = last.Outputs
			}
		}
		att.ReceiptStatus, att.ExitCode, att.IngestedAt = sp(rc.Status), code, sp(now())
		ok := rc.Status == "completed"
		stdoutB, _ := os.ReadFile(repoPath(strings.TrimSuffix(att.Card, ".yaml") + ".stdout"))
		stdout := string(stdoutB)
		exit := "none"
		if code != nil {
			exit = fmt.Sprint(*code)
		}
		if strings.HasPrefix(inst.Step, "_") && inst.Host != nil {
			phase := inst.Step[1:]
			h := st.Hosts[*inst.Host]
			cand := spec.host(h.Candidate)
			switch phase {
			case "provision":
				if ok {
					if err := capture(cand.Lifecycle.Provision.Capture, stdout, st); err != nil {
						ok = false
						problems = append(problems, fmt.Sprintf("provision of %s: %v", h.Candidate, err))
					} else {
						h.State, h.Handle = "up", sp(st.Vars["handle"])
					}
				}
				if !ok {
					h.State, h.EndedAt = "unavailable", sp(now())
					problems = append(problems, fmt.Sprintf("host_unavailable: provision of %s failed (%s, exit %s)",
						h.Candidate, rc.Status, exit))
				}
			case "poll":
				h.LastPollAt = sp(now())
				if !ok {
					problems = append(problems, fmt.Sprintf("poll of %s failed (exit %s); the host may be gone", h.Candidate, exit))
				}
			case "teardown":
				if ok {
					h.State, h.EndedAt = "torn_down", sp(now())
				} else {
					problems = append(problems, fmt.Sprintf("teardown of %s failed: terminate it by hand (handle %s)",
						h.Candidate, strOr(h.Handle, "none")))
				}
			}
			inst.Status = map[bool]string{true: "done", false: "failed"}[ok]
			st.event("ingest", key+": "+rc.Status)
			continue
		}
		s := spec.step(inst.Step)
		if s == nil {
			// a wait_until refresh card
			inst.Status = map[bool]string{true: "done", false: "failed"}[ok]
			st.event("ingest", key+": "+rc.Status)
			continue
		}
		if ok {
			if err := capture(s.Capture, stdout, st); err != nil {
				ok = false
				problems = append(problems, key+": "+err.Error())
			}
		}
		switch {
		case ok:
			inst.Status = "done"
		case timedOut || (code != nil && containsInt(s.CensoredExitCodes, *code)):
			inst.Status, inst.Reason = "censored", sp(fmt.Sprintf("exit %s, timed_out %v", exit, timedOut))
		default:
			inst.Status, inst.Reason = "failed", sp(fmt.Sprintf("%s, exit %s", rc.Status, exit))
		}
		st.event("ingest", fmt.Sprintf("%s: %s (%s)", key, inst.Status, rc.Status))
		switch inst.Status {
		case "done":
			var cell *Cell
			if inst.Cell != nil {
				cell = cellsByID[*inst.Cell]
			}
			hidx := st.hostIndex()
			var host *Host
			if hidx >= 0 {
				host = spec.host(st.Hosts[hidx].Candidate)
			}
			ctx := context(spec, paths, cell, host, hidx, st.Vars, false)
			if s.Gates != nil {
				for _, gid := range s.Gates.After {
					g := spec.gate(gid)
					passed, detail := allChecks(g, ctx, paths)
					st.Gates[gid+":"+key] = &GateState{Result: map[bool]string{true: "pass", false: "fail"}[passed],
						At: now(), Detail: detail}
					if !passed {
						if p := applyFail(st, g, inst, fmt.Sprintf("gate %s after %s: %s", gid, key, detail)); p != "" {
							problems = append(problems, p)
						}
					}
				}
			}
		case "failed":
			if p := applyOnFailure(st, s, inst, key); p != "" {
				problems = append(problems, p)
			}
		case "censored":
			if spec.Agent.escalates("step_censored") {
				problems = append(problems, fmt.Sprintf("step_censored: %s (%s)", key, strOr(inst.Reason, "")))
			}
		}
	}
	return problems
}

func containsInt(list []int, x int) bool {
	for _, v := range list {
		if v == x {
			return true
		}
	}
	return false
}

func markCell(st *State, cell, status, reason string) {
	for _, inst := range st.Instances {
		if inst.Cell != nil && *inst.Cell == cell && !terminal[inst.Status] {
			inst.Status, inst.Reason = status, sp(reason)
		}
	}
}

func applyFail(st *State, g *Gate, inst *Instance, text string) string {
	mode := g.OnFail
	if (mode == "skip_cell" || mode == "void_cell") && inst.Cell != nil {
		status := "not_attempted"
		if mode == "void_cell" {
			status = "void"
			inst.Status = "void"
		}
		markCell(st, *inst.Cell, status, text)
		st.event(mode, text)
		return ""
	}
	if mode == "stop" {
		st.Status, st.BlockedOn = "stopped", sp(text)
		return "stopped: " + text
	}
	return "gate_failed: " + text
}

func applyOnFailure(st *State, s *Step, inst *Instance, key string) string {
	text := fmt.Sprintf("step_failed: %s (%s)", key, strOr(inst.Reason, ""))
	switch s.OnFailure {
	case "skip_cell":
		if inst.Cell != nil {
			markCell(st, *inst.Cell, "not_attempted", text)
			return ""
		}
	case "continue":
		return ""
	case "stop":
		st.Status, st.BlockedOn = "stopped", sp(text)
		return "stopped: " + text
	}
	return text
}

func sortStrings(s []string) []string { sort.Strings(s); return s }
