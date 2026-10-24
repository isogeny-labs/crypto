package main

import (
	"bytes"
	_ "embed"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"github.com/santhosh-tekuri/jsonschema/v6"
	"gopkg.in/yaml.v3"
)

//go:embed experiment.v1.schema.json
var specSchemaJSON []byte

//go:embed experiment-state.v1.schema.json
var stateSchemaJSON []byte

// repo is the root of the checkout the tool runs in (git toplevel of cwd).
var repo string

func repoPath(rel string) string { return filepath.Join(repo, filepath.FromSlash(rel)) }

func git(args ...string) (int, string) {
	cmd := exec.Command("git", args...)
	cmd.Dir = repo
	var out bytes.Buffer
	cmd.Stdout = &out
	err := cmd.Run()
	code := 0
	if err != nil {
		code = 1
		var ee *exec.ExitError
		if errors.As(err, &ee) {
			code = ee.ExitCode()
		}
	}
	return code, strings.TrimSpace(out.String())
}

func committed(rel string) bool {
	code, blob := git("rev-parse", "HEAD:"+rel)
	if code != 0 {
		return false
	}
	c2, h := git("hash-object", rel)
	return c2 == 0 && h == blob
}

func compileSchema(name string, raw []byte) (*jsonschema.Schema, error) {
	doc, err := jsonschema.UnmarshalJSON(bytes.NewReader(raw))
	if err != nil {
		return nil, err
	}
	c := jsonschema.NewCompiler()
	url := "mem:///" + name
	if err := c.AddResource(url, doc); err != nil {
		return nil, err
	}
	return c.Compile(url)
}

// schemaFindings returns one line per leaf error, "<instance path>: <message>".
func schemaFindings(name string, raw []byte, doc any) []string {
	sch, err := compileSchema(name, raw)
	if err != nil {
		return []string{"internal: schema does not compile: " + err.Error()}
	}
	err = sch.Validate(doc)
	if err == nil {
		return nil
	}
	var ve *jsonschema.ValidationError
	if !errors.As(err, &ve) {
		return []string{err.Error()}
	}
	seen := map[string]bool{}
	var out []string
	var walk func(u jsonschema.OutputUnit)
	walk = func(u jsonschema.OutputUnit) {
		if u.Error != nil {
			msg := u.Error.String()
			loc := strings.TrimPrefix(u.InstanceLocation, "/")
			if loc == "" {
				loc = "(root)"
			}
			line := loc + ": " + msg
			if !seen[line] && !strings.HasPrefix(msg, "validation failed") && !strings.Contains(msg, "doesn't validate with") {
				seen[line] = true
				out = append(out, line)
			}
		}
		for _, c := range u.Errors {
			walk(c)
		}
	}
	walk(*ve.BasicOutput())
	sort.Strings(out)
	return out
}

var slugRE = regexp.MustCompile(`^[a-z][a-z0-9-]{0,47}$`)

func semanticFindings(spec *Spec, specPath string) (errs, warns []string) {
	add := func(f string, a ...any) { errs = append(errs, fmt.Sprintf(f, a...)) }
	warn := func(f string, a ...any) { warns = append(warns, fmt.Sprintf(f, a...)) }

	if strings.TrimSuffix(filepath.Base(specPath), ".yaml") != spec.ID {
		add("file name %s must be %s.yaml", filepath.Base(specPath), spec.ID)
	}
	paths, err := resolvePaths(spec)
	if err != nil {
		return []string{err.Error()}, nil
	}
	cells := expandCells(spec)
	ids := map[string]int{}
	for _, c := range cells {
		ids[c.ID]++
		if !slugRE.MatchString(c.ID) {
			add("cells: generated id %q is not a slug", c.ID)
		}
	}
	for _, id := range sortedKeys(ids) {
		if ids[id] > 1 {
			add("cells: duplicate id %s", id)
		}
	}
	gateIDs := map[string]bool{}
	for _, g := range spec.Gates {
		gateIDs[g.ID] = true
	}
	backends := map[string]bool{}
	for _, h := range spec.Hosts.Candidates {
		backends[h.Backend] = true
	}
	nonBatch := backends["local"] || backends["ssh"]
	seen := map[string]bool{}
	used := map[string]bool{}
	for i := range spec.Steps {
		s := &spec.Steps[i]
		at := "steps." + s.ID
		if seen[s.ID] {
			add("steps: duplicate id %s", s.ID)
		}
		for _, n := range s.Needs {
			if spec.step(n) == nil {
				add("%s.needs: unknown step %s", at, n)
			} else if !seen[n] {
				add("%s.needs: %s must come earlier in `steps`", at, n)
			}
		}
		seen[s.ID] = true
		if s.Gates != nil {
			for _, g := range append(append([]string{}, s.Gates.Before...), s.Gates.After...) {
				used[g] = true
				if !gateIDs[g] {
					add("%s.gates: unknown gate %s", at, g)
				}
			}
		}
		for _, c := range s.Cells {
			if ids[c] == 0 {
				add("%s.cells: unknown cell %s", at, c)
			}
		}
		if s.Scope == "per_cell" && len(cellsForStep(s, cells)) == 0 {
			add("%s: per_cell step selects no cells", at)
		}
		if s.RunsOn == "host" && backends["batch"] {
			if s.StatusDone == "" {
				add("%s: a host step needs status_done when any candidate is a batch host", at)
			}
			if s.Gates != nil && (len(s.Gates.Before) > 0 || len(s.Gates.After) > 0) {
				add("%s.gates: a batch host runs its steps by itself, so expspec cannot gate this one; "+
					"gate a controller step that reads the collected outputs, or rely on the launch ask", at)
			}
		}
		if s.RunsOn == "host" && nonBatch && len(s.Argv) == 0 {
			add("%s: needs argv to run on a local or ssh host", at)
		}
		if s.Evidence && len(s.DoneWhen) > 0 {
			warn("%s: evidence step with done_when; make sure done_when reads the committed output, not a partial file", at)
		}
		var sample *Cell
		if s.Scope == "per_cell" {
			if cs := cellsForStep(s, cells); len(cs) > 0 {
				sample = &cs[0]
			}
		}
		ctx := context(spec, paths, sample, &spec.Hosts.Candidates[0], 0, nil, true)
		probe := func(field string, list []string) {
			if _, err := renderAll(list, ctx); err != nil {
				add("%s.%s: %v", at, field, err)
			}
		}
		probe("argv", s.Argv)
		probe("cwd", []string{s.Cwd})
		probe("outputs", s.Outputs)
		probe("must_not_exist", s.MustNotExist)
		probe("status_done", []string{s.StatusDone})
		probe("write_scope", s.WriteScope)
		for _, v := range s.Env {
			probe("env", []string{v})
		}
		for _, c := range s.DoneWhen {
			probe("done_when", checkStrings(c))
		}
	}
	for _, g := range sortedKeys(gateIDs) {
		if !used[g] {
			warn("gates.%s: no step uses it", g)
		}
	}
	for _, h := range spec.Hosts.Candidates {
		for _, c := range h.Cells {
			if ids[c] == 0 {
				add("hosts.%s.cells: unknown cell %s", h.ID, c)
			}
		}
		if h.Backend == "batch" && h.Lifecycle != nil {
			if _, ok := h.Lifecycle.Provision.Capture["handle"]; !ok {
				add("hosts.%s.lifecycle.provision: must capture `handle`", h.ID)
			}
			if cp := spec.Checkpoint; h.Preemptible && cp != nil && cp.Resume == "from_ckpt" && (cp.Transport == "" || cp.Transport == "none") {
				add("hosts.%s: preemptible with resume from_ckpt needs checkpoint.transport", h.ID)
			}
		}
	}
	for i := range cells {
		c := &cells[i]
		ok := false
		for j := range spec.Hosts.Candidates {
			if cellFitsHost(c, &spec.Hosts.Candidates[j]) {
				ok = true
			}
		}
		if !ok {
			msg := fmt.Sprintf("cells.%s: no host candidate fits; it will be recorded not_attempted", c.ID)
			if c.Optional {
				warns = append(warns, msg)
			} else {
				errs = append(errs, msg)
			}
		}
	}
	timing, timingHost := false, false
	for _, m := range spec.Metrics {
		timing = timing || m.Kind == "timing"
	}
	for _, h := range spec.Hosts.Candidates {
		timingHost = timingHost || h.TimingValid
	}
	if timing && !timingHost {
		warn("metrics: a timing metric with no timing_valid host; wall times will be descriptive only")
	}
	if spec.Status == "approved" {
		for _, f := range spec.Source.Frozen {
			got, err := sha256File(repoPath(f.Path))
			if err != nil {
				add("source.frozen: %s does not exist", f.Path)
			} else if got != f.Sha256 {
				add("source.frozen: %s does not match its sha256", f.Path)
			}
		}
		if code, _ := git("cat-file", "-e", spec.Source.Commit+"^{commit}"); code != 0 {
			add("source.commit: %s is not a commit here", spec.Source.Commit)
		}
		if spec.Approval != nil {
			dec := spec.Approval.Decision
			raw, err := os.ReadFile(repoPath(dec))
			if err != nil {
				add("approval.decision: %s does not exist", dec)
			} else {
				var d struct {
					CD struct {
						Decision  string   `yaml:"decision"`
						TargetIDs []string `yaml:"target_ids"`
					} `yaml:"coordinator_decision"`
				}
				_ = yaml.Unmarshal(raw, &d)
				if d.CD.Decision != "approve" || !contains(d.CD.TargetIDs, spec.ID) {
					add("approval.decision: %s does not approve %s", dec, spec.ID)
				} else if !committed(dec) {
					add("approval.decision: %s is not committed as it is on disk", dec)
				}
			}
		}
	}
	est, dearest := 0.0, 0.0
	for _, c := range cells {
		if c.Requires != nil {
			est += c.Requires.EstHours
		}
	}
	for _, h := range spec.Hosts.Candidates {
		if h.EstHourlyUSD > dearest {
			dearest = h.EstHourlyUSD
		}
	}
	if est > 0 && dearest > 0 && est*dearest > spec.Budget.MaxCostUSD {
		warn("budget: cells' est_hours (%g h) x the dearest host (%g $/h) exceeds max_cost_usd %g; optional cells may be dropped",
			est, dearest, spec.Budget.MaxCostUSD)
	}
	return errs, warns
}

func checkStrings(c Check) []string {
	out := []string{c.Path, c.Token}
	out = append(out, c.Argv...)
	if c.JSON != nil {
		out = append(out, c.JSON.File)
	}
	return out
}

// validate loads and checks a spec. errs non-empty means it may not run.
func validate(specPath string) (*Spec, []byte, []string, []string, error) {
	spec, generic, raw, err := loadSpec(specPath)
	if err != nil {
		return nil, nil, nil, nil, err
	}
	if errs := schemaFindings("experiment.v1.schema.json", specSchemaJSON, generic); len(errs) > 0 {
		return spec, raw, errs, nil, nil
	}
	errs, warns := semanticFindings(spec, specPath)
	return spec, raw, errs, warns, nil
}
