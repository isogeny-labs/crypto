// Command expspec validates, tracks and compiles experiment specs
// (crypto.experiment/v1) into mechanical task cards.
//
// An experiment spec (experiments/specs/<id>.yaml) is the decided, frozen
// plan: question, cells, steps, gates, hosts, budget and outcome rules. This
// tool is the only thing that reads it during a run. It never interprets a
// result and never runs an experiment command itself: each step instance
// becomes one tools/task_card.py frozen-command card, which
// executor-mechanical (Haiku) runs unchanged. Progress goes to
// experiments/specs/<id>.state.yaml.
//
//	expspec validate <spec>...
//	expspec plan <spec>               expanded instances; works on drafts
//	expspec next <spec>               one JSON action: run_card, ask, wait, escalate, done, stopped, refused
//	expspec answer <spec> --gate <id> (--yes|--no) --by <name>
//	expspec ls [<dir>]                every spec with its run status and spend
//	expspec facts <spec>              metric values and check results, never a verdict
//
// Build from the repository root:
//
//	go -C tools/expspec build -o "$PWD/target/expspec" .
//
// Exit status: 0 ok, 1 findings or refused, 2 unreadable input.
package main

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"text/tabwriter"

	"gopkg.in/yaml.v3"
)

func main() { os.Exit(run(os.Args[1:])) }

func usage() int {
	fmt.Fprintln(os.Stderr, "usage: expspec validate|plan|next|answer|ls|facts ... (see `go doc` for details)")
	return 2
}

func run(args []string) int {
	if len(args) == 0 {
		return usage()
	}
	out, err := exec.Command("git", "rev-parse", "--show-toplevel").Output()
	if err != nil {
		fmt.Fprintln(os.Stderr, "error: run expspec inside the repository checkout")
		return 2
	}
	repo = strings.TrimSpace(string(out))
	var code int
	switch args[0] {
	case "validate":
		code, err = cmdValidate(args[1:])
	case "plan":
		code, err = cmdPlan(args[1:])
	case "next":
		code, err = cmdNext(args[1:])
	case "answer":
		code, err = cmdAnswer(args[1:])
	case "ls":
		code, err = cmdLs(args[1:])
	case "facts":
		code, err = cmdFacts(args[1:])
	default:
		return usage()
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		return 2
	}
	return code
}

func absSpec(p string) string {
	a, err := filepath.Abs(p)
	if err != nil {
		return p
	}
	return a
}

func printJSON(v any) {
	b, _ := json.MarshalIndent(v, "", " ")
	fmt.Println(string(b))
}

func cmdValidate(args []string) (int, error) {
	if len(args) == 0 {
		return usage(), nil
	}
	bad := 0
	for _, p := range args {
		spec, raw, errs, warns, err := validate(absSpec(p))
		if err != nil {
			fmt.Printf("%s: error: %v\n", p, err)
			bad = 1
			continue
		}
		for _, w := range warns {
			fmt.Printf("%s: warning: %s\n", p, w)
		}
		for _, e := range errs {
			fmt.Printf("%s: error: %s\n", p, e)
		}
		if len(errs) > 0 {
			bad = 1
		} else {
			fmt.Printf("%s: OK (%s v%d, %s, sha256 %s)\n", p, spec.ID, spec.Version, spec.Status, sha256Hex(raw))
		}
	}
	return bad, nil
}

func cmdPlan(args []string) (int, error) {
	if len(args) != 1 {
		return usage(), nil
	}
	spec, _, errs, _, err := validate(absSpec(args[0]))
	if err != nil {
		return 0, err
	}
	if len(errs) > 0 {
		fmt.Println(strings.Join(errs, "\n"))
		return 1, nil
	}
	paths, _ := resolvePaths(spec)
	cells := expandCells(spec)
	host := &spec.Hosts.Candidates[0]
	fmt.Printf("# %s v%d (%s): %d cells, %d steps\n", spec.ID, spec.Version, spec.Status, len(cells), len(spec.Steps))
	for _, c := range cells {
		pj, _ := json.Marshal(c.Params)
		fmt.Printf("cell %-14s role=%-11s prio=%-4d optional=%-5v params=%s\n", c.ID, c.Role, c.Priority, c.Optional, pj)
	}
	tw := tabwriter.NewWriter(os.Stdout, 0, 2, 2, ' ', 0)
	for i := range spec.Steps {
		s := &spec.Steps[i]
		targets := []*Cell{nil}
		if s.Scope == "per_cell" {
			targets = nil
			for _, c := range cellsForStep(s, cells) {
				c := c
				targets = append(targets, &c)
			}
		}
		for _, c := range targets {
			hidx, cid := -1, ""
			if s.Scope == "per_host" {
				hidx = 0
			}
			if c != nil {
				cid = c.ID
			}
			ctx := context(spec, paths, c, host, 0, nil, true)
			what := "(remote; STATUS " + s.StatusDone + ")"
			if len(s.Argv) > 0 {
				argv, _ := renderAll(s.Argv, ctx)
				q := make([]string, len(argv))
				for j, a := range argv {
					q[j] = shellQuote(a)
				}
				what = strings.Join(q, " ")
			}
			gates := ""
			if s.Gates != nil && len(s.Gates.Before) > 0 {
				gates = "gates=" + strings.Join(s.Gates.Before, ",")
			}
			fmt.Fprintf(tw, "%s\t%s\t%s\t%s\n", instanceKey(s.ID, cid, hidx), s.RunsOn, gates, what)
		}
	}
	return 0, tw.Flush()
}

func newRunner(specPath string) (*runner, Action, error) {
	spec, raw, errs, _, err := validate(specPath)
	if err != nil {
		return nil, nil, err
	}
	if len(errs) > 0 {
		if len(errs) > 20 {
			errs = errs[:20]
		}
		return nil, act("refused", "reason", "spec does not validate", "errors", errs), nil
	}
	if spec.Status != "approved" {
		return nil, act("refused", "reason", "spec status is "+spec.Status+"; only approved specs run"), nil
	}
	st, err := loadState(specPath, spec, raw)
	if err != nil {
		return nil, act("escalate", "reason", err.Error()), nil
	}
	paths, _ := resolvePaths(spec)
	cells := expandCells(spec)
	byID := map[string]*Cell{}
	for i := range cells {
		byID[cells[i].ID] = &cells[i]
	}
	return &runner{spec: spec, specPath: specPath, st: st, paths: paths, cells: cells, cellsByID: byID}, nil, nil
}

func cmdNext(args []string) (int, error) {
	if len(args) != 1 {
		return usage(), nil
	}
	p := absSpec(args[0])
	r, early, err := newRunner(p)
	if err != nil {
		return 0, err
	}
	if early != nil {
		printJSON(early)
		return 1, nil
	}
	a, err := r.next()
	if err != nil {
		// Keep what happened before the error (ingested receipts, gate results).
		r.st.event("error", err.Error())
		_ = saveState(p, r.st)
		printJSON(act("escalate", "reason", err.Error()))
		return 1, nil
	}
	a["state"] = r.st.Status
	a["spend_usd"] = spend(r.spec, r.st)
	brief := map[string]any{}
	for _, k := range []string{"action", "key", "gate", "reason"} {
		if v, ok := a[k]; ok {
			brief[k] = v
		}
	}
	bj, _ := json.Marshal(brief)
	r.st.event("next", string(bj))
	if err := saveState(p, r.st); err != nil {
		return 0, err
	}
	printJSON(a)
	return 0, nil
}

func cmdAnswer(args []string) (int, error) {
	fs := flag.NewFlagSet("answer", flag.ContinueOnError)
	gate := fs.String("gate", "", "gate id waiting for an answer")
	yes := fs.Bool("yes", false, "answer yes")
	no := fs.Bool("no", false, "answer no")
	by := fs.String("by", "", "who answered")
	if len(args) < 1 {
		return usage(), nil
	}
	if err := fs.Parse(args[1:]); err != nil {
		return 2, nil
	}
	if *gate == "" || *by == "" || *yes == *no {
		return 0, errors.New("answer needs --gate, --by and exactly one of --yes/--no")
	}
	p := absSpec(args[0])
	spec, _, raw, err := loadSpec(p)
	if err != nil {
		return 0, err
	}
	st, err := loadState(p, spec, raw)
	if err != nil {
		return 0, err
	}
	g := st.Gates[*gate]
	if g == nil || g.Result != "asked" {
		fmt.Printf("gate %s is not waiting for an answer\n", *gate)
		return 1, nil
	}
	word := map[bool]string{true: "yes", false: "no"}[*yes]
	g.Result, g.At, g.By = "answered_"+word, now(), *by
	if st.BlockedOn != nil && *st.BlockedOn == *gate && st.Status != "stopped" {
		st.Status, st.BlockedOn = "running", nil
	}
	st.event("answer", fmt.Sprintf("%s: %s by %s", *gate, word, *by))
	if err := saveState(p, st); err != nil {
		return 0, err
	}
	fmt.Printf("recorded %s: %s\n", *gate, word)
	return 0, nil
}

func cmdLs(args []string) (int, error) {
	dir := filepath.Join(repo, "experiments", "specs")
	if len(args) > 0 {
		dir = absSpec(args[0])
	}
	files, _ := filepath.Glob(filepath.Join(dir, "EXP-*.yaml"))
	sort.Strings(files)
	tw := tabwriter.NewWriter(os.Stdout, 0, 2, 2, ' ', 0)
	fmt.Fprintln(tw, "id\tv\tspec\trun\tsteps\tspend\tblocked_on\ttitle")
	for _, f := range files {
		if strings.HasSuffix(f, ".state.yaml") {
			continue
		}
		spec, _, _, err := loadSpec(f)
		if err != nil {
			fmt.Fprintf(tw, "%s\t\t?\t\t\t\t\t%v\n", filepath.Base(f), err)
			continue
		}
		run, steps, cost, blocked := "-", "-", 0.0, ""
		if b, err := os.ReadFile(statePath(f)); err == nil {
			var st State
			if yaml.Unmarshal(b, &st) == nil {
				total, done := 0, 0
				for _, i := range st.Instances {
					if strings.HasPrefix(i.Step, "_") {
						continue
					}
					total++
					if terminal[i.Status] {
						done++
					}
				}
				run, steps, cost, blocked = st.Status, fmt.Sprintf("%d/%d", done, total), spend(spec, &st), strOr(st.BlockedOn, "")
			}
		}
		title := spec.Title
		if len(title) > 60 {
			title = title[:60]
		}
		fmt.Fprintf(tw, "%s\t%d\t%s\t%s\t%s\t$%g/$%g\t%s\t%s\n", spec.ID, spec.Version, spec.Status, run, steps,
			cost, spec.Budget.MaxCostUSD, blocked, title)
	}
	return 0, tw.Flush()
}

func cmdFacts(args []string) (int, error) {
	if len(args) != 1 {
		return usage(), nil
	}
	p := absSpec(args[0])
	spec, _, _, err := loadSpec(p)
	if err != nil {
		return 0, err
	}
	paths, err := resolvePaths(spec)
	if err != nil {
		return 0, err
	}
	cells := expandCells(spec)
	byID := map[string]*Cell{}
	for i := range cells {
		byID[cells[i].ID] = &cells[i]
	}
	st := &State{Instances: map[string]*Instance{}, Vars: map[string]string{}}
	if b, err := os.ReadFile(statePath(p)); err == nil {
		_ = yaml.Unmarshal(b, st)
	}
	r := &runner{spec: spec, st: st, paths: paths, cells: cells, cellsByID: byID}
	timingOK := false
	for _, h := range st.Hosts {
		if c := spec.host(h.Candidate); c != nil && c.TimingValid {
			timingOK = true
		}
	}
	instances := map[string]any{}
	for k, i := range st.Instances {
		if !strings.HasPrefix(i.Step, "_") {
			instances[k] = map[string]any{"status": i.Status, "reason": i.Reason}
		}
	}
	metrics := map[string]any{}
	for _, m := range spec.Metrics {
		if m.Source == nil {
			continue
		}
		per := map[string]any{}
		targets := []*Cell{nil}
		if strings.Contains(strings.ReplaceAll(m.Source.File, " ", ""), "{{cell.") {
			targets = nil
			for i := range cells {
				targets = append(targets, &cells[i])
			}
		}
		for _, c := range targets {
			name := "-"
			if c != nil {
				name = c.ID
			}
			v, err := jsonValue(m.Source, context(spec, paths, c, nil, -1, st.Vars, false))
			if err != nil {
				v = nil
			}
			per[name] = v
		}
		entry := map[string]any{"kind": m.Kind, "unit": m.Unit, "values": per}
		if m.Kind == "timing" && !timingOK {
			entry["label"] = "descriptive (no timing_valid host ran)"
		}
		metrics[m.ID] = entry
	}
	rules := map[string]any{}
	for _, rule := range spec.Outcome.DecisionRules {
		if len(rule.Checks) == 0 {
			continue
		}
		res := map[string]any{}
		targets := rule.Cells
		if len(targets) == 0 {
			targets = []string{""}
		}
		for _, cid := range targets {
			ctx := context(spec, paths, byID[cid], nil, -1, st.Vars, false)
			var out [][2]any
			for _, c := range rule.Checks {
				ok, d := runCheck(c, ctx, paths)
				out = append(out, [2]any{ok, d})
			}
			if cid == "" {
				cid = "-"
			}
			res[cid] = out
		}
		rules[rule.ID] = res
	}
	missing := r.missingArtifacts()
	if missing == nil {
		missing = []string{}
	}
	printJSON(map[string]any{"experiment": spec.ID, "version": spec.Version, "instances": instances,
		"metrics": metrics, "decision_rule_checks": rules, "missing_artifacts": missing})
	return 0, nil
}
