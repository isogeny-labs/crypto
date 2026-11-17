package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"os"
	"path"
	"regexp"
	"sort"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

// Spec is crypto.experiment/v1. Only the fields expspec acts on are typed;
// the schema validates the rest (question, links, tags, ...).
type Spec struct {
	Schema     string            `yaml:"schema"`
	ID         string            `yaml:"id"`
	Version    int               `yaml:"version"`
	Title      string            `yaml:"title"`
	Status     string            `yaml:"status"`
	Approval   *Approval         `yaml:"approval"`
	Source     Source            `yaml:"source"`
	Paths      map[string]string `yaml:"paths"`
	Env        map[string]string `yaml:"env"`
	Cells      []Cell            `yaml:"cells"`
	Matrix     *Matrix           `yaml:"matrix"`
	Checkpoint *Checkpoint       `yaml:"checkpoint"`
	Steps      []Step            `yaml:"steps"`
	Gates      []Gate            `yaml:"gates"`
	Hosts      Hosts             `yaml:"hosts"`
	Budget     Budget            `yaml:"budget"`
	Metrics    []Metric          `yaml:"metrics"`
	Outcome    Outcome           `yaml:"outcome"`
	Reporting  Reporting         `yaml:"reporting"`
	Agent      Agent             `yaml:"agent"`
}

type Approval struct {
	Decision   string `yaml:"decision"`
	ApprovedBy string `yaml:"approved_by"`
	ApprovedAt string `yaml:"approved_at"`
}

type Source struct {
	Repo       string   `yaml:"repo"`
	Ref        string   `yaml:"ref"`
	Commit     string   `yaml:"commit"`
	AllowDrift bool     `yaml:"allow_drift"`
	Frozen     []Frozen `yaml:"frozen"`
}

type Frozen struct {
	Path   string `yaml:"path"`
	Sha256 string `yaml:"sha256"`
}

type Resources struct {
	Vcpus    *float64 `yaml:"vcpus"`
	MemoryGB *float64 `yaml:"memory_gb"`
	DiskGB   *float64 `yaml:"disk_gb"`
	Gpus     *float64 `yaml:"gpus"`
	Arch     string   `yaml:"arch"`
	CPUFlags []string `yaml:"cpu_flags"`
	GpuModel string   `yaml:"gpu_model"`
	EstHours float64  `yaml:"est_hours"`
}

type Cell struct {
	ID       string         `yaml:"id"`
	Params   map[string]any `yaml:"params"`
	Role     string         `yaml:"role"`
	Priority int            `yaml:"priority"`
	Optional bool           `yaml:"optional"`
	Requires *Resources     `yaml:"requires"`
	Note     string         `yaml:"note"`
}

type Matrix struct {
	Axes       map[string][]any `yaml:"axes"`
	IDTemplate string           `yaml:"id_template"`
	Exclude    []map[string]any `yaml:"exclude"`
	Defaults   Cell             `yaml:"defaults"`
}

type Checkpoint struct {
	Resume    string `yaml:"resume"`
	Transport string `yaml:"transport"`
}

type Capture struct {
	JSONKey string `yaml:"json_key"`
	Regex   string `yaml:"regex"`
}

type StepGates struct {
	Before []string `yaml:"before"`
	After  []string `yaml:"after"`
}

type Step struct {
	ID                string             `yaml:"id"`
	Title             string             `yaml:"title"`
	Scope             string             `yaml:"scope"`
	RunsOn            string             `yaml:"runs_on"`
	Cells             []string           `yaml:"cells"`
	CellRoles         []string           `yaml:"cell_roles"`
	Needs             []string           `yaml:"needs"`
	Argv              []string           `yaml:"argv"`
	Cwd               string             `yaml:"cwd"`
	Env               map[string]string  `yaml:"env"`
	TimeoutS          int                `yaml:"timeout_s"`
	Background        *bool              `yaml:"background"`
	RetryExitCodes    []int              `yaml:"retry_exit_codes"`
	SuccessExitCodes  []int              `yaml:"success_exit_codes"`
	CensoredExitCodes []int              `yaml:"censored_exit_codes"`
	Outputs           []string           `yaml:"outputs"`
	MustNotExist      []string           `yaml:"must_not_exist"`
	Capture           map[string]Capture `yaml:"capture"`
	StatusDone        string             `yaml:"status_done"`
	StatusFailed      []string           `yaml:"status_failed"`
	DoneWhen          []Check            `yaml:"done_when"`
	Gates             *StepGates         `yaml:"gates"`
	Evidence          bool               `yaml:"evidence"`
	OnFailure         string             `yaml:"on_failure"`
	WriteScope        []string           `yaml:"write_scope"`
}

type JSONRef struct {
	File string `yaml:"file"`
	Path string `yaml:"path"`
	Line string `yaml:"line"`
}

type Check struct {
	Kind     string   `yaml:"kind"`
	Path     string   `yaml:"path"`
	Sha256   string   `yaml:"sha256"`
	JSON     *JSONRef `yaml:"json"`
	Op       string   `yaml:"op"`
	Value    any      `yaml:"value"`
	Token    string   `yaml:"token"`
	Argv     []string `yaml:"argv"`
	TimeoutS int      `yaml:"timeout_s"`
}

type Gate struct {
	ID          string   `yaml:"id"`
	Description string   `yaml:"description"`
	Kind        string   `yaml:"kind"`
	Checks      []Check  `yaml:"checks"`
	Refresh     []string `yaml:"refresh"`
	EveryS      int      `yaml:"every_s"`
	MaxWaitS    int      `yaml:"max_wait_s"`
	Ask         string   `yaml:"ask"`
	OnFail      string   `yaml:"on_fail"`
}

type LifecycleCmd struct {
	Argv           []string           `yaml:"argv"`
	Env            map[string]string  `yaml:"env"`
	TimeoutS       int                `yaml:"timeout_s"`
	RetryExitCodes []int              `yaml:"retry_exit_codes"`
	Capture        map[string]Capture `yaml:"capture"`
}

type Lifecycle struct {
	Provision  LifecycleCmd `yaml:"provision"`
	Poll       LifecycleCmd `yaml:"poll"`
	Collect    LifecycleCmd `yaml:"collect"`
	Teardown   LifecycleCmd `yaml:"teardown"`
	PollEveryS int          `yaml:"poll_every_s"`
	LifetimeS  int          `yaml:"lifetime_s"`
}

func (l *Lifecycle) phase(name string) LifecycleCmd {
	switch name {
	case "provision":
		return l.Provision
	case "poll":
		return l.Poll
	case "collect":
		return l.Collect
	}
	return l.Teardown
}

type Host struct {
	ID           string         `yaml:"id"`
	Backend      string         `yaml:"backend"`
	Launcher     string         `yaml:"launcher"`
	Provides     Resources      `yaml:"provides"`
	TimingValid  bool           `yaml:"timing_valid"`
	Preemptible  bool           `yaml:"preemptible"`
	Paid         *bool          `yaml:"paid"`
	EstHourlyUSD float64        `yaml:"est_hourly_usd"`
	Cells        []string       `yaml:"cells"`
	Config       map[string]any `yaml:"config"`
	Lifecycle    *Lifecycle     `yaml:"lifecycle"`
	Note         string         `yaml:"note"`
}

func (h *Host) paid() bool { return h.Paid == nil || *h.Paid }

type Hosts struct {
	Selection  string `yaml:"selection"`
	Candidates []Host `yaml:"candidates"`
}

type Budget struct {
	MaxCostUSD         float64 `yaml:"max_cost_usd"`
	MaxWallHours       float64 `yaml:"max_wall_hours"`
	MaxAttemptsPerStep int     `yaml:"max_attempts_per_step"`
	EscalateAtFraction float64 `yaml:"escalate_at_fraction"`
}

type Metric struct {
	ID     string   `yaml:"id"`
	Kind   string   `yaml:"kind"`
	Unit   string   `yaml:"unit"`
	Source *JSONRef `yaml:"source"`
}

type Rule struct {
	ID     string   `yaml:"id"`
	Cells  []string `yaml:"cells"`
	Checks []Check  `yaml:"checks"`
}

type Outcome struct {
	DecisionRules []Rule `yaml:"decision_rules"`
}

type Reporting struct {
	ResultsFile       string   `yaml:"results_file"`
	RequiredArtifacts []string `yaml:"required_artifacts"`
}

type Agent struct {
	Autonomy   map[string]any `yaml:"autonomy"`
	EscalateOn []string       `yaml:"escalate_on"`
}

func (a Agent) permission(name string) string {
	if v, ok := a.Autonomy[name].(string); ok {
		return v
	}
	return "ask"
}

func (a Agent) spendWithoutAsking() float64 {
	return toFloat(a.Autonomy["spend_without_asking_usd"])
}

func (a Agent) escalates(kind string) bool {
	for _, k := range a.EscalateOn {
		if k == kind {
			return true
		}
	}
	return false
}

func (s *Spec) step(id string) *Step {
	for i := range s.Steps {
		if s.Steps[i].ID == id {
			return &s.Steps[i]
		}
	}
	return nil
}

func (s *Spec) gate(id string) *Gate {
	for i := range s.Gates {
		if s.Gates[i].ID == id {
			return &s.Gates[i]
		}
	}
	return nil
}

func (s *Spec) host(id string) *Host {
	for i := range s.Hosts.Candidates {
		if s.Hosts.Candidates[i].ID == id {
			return &s.Hosts.Candidates[i]
		}
	}
	return nil
}

// ---------------------------------------------------------------------------
// YAML loading: typed, and generic (for the schema) with timestamps kept as
// the strings written.
// ---------------------------------------------------------------------------

func readYAML(p string) ([]byte, *yaml.Node, error) {
	raw, err := os.ReadFile(p)
	if err != nil {
		return nil, nil, err
	}
	var doc yaml.Node
	if err := yaml.Unmarshal(raw, &doc); err != nil {
		return nil, nil, fmt.Errorf("%s: YAML does not parse: %v", p, err)
	}
	return raw, &doc, nil
}

func loadSpec(p string) (*Spec, any, []byte, error) {
	raw, node, err := readYAML(p)
	if err != nil {
		return nil, nil, nil, err
	}
	generic, err := nodeToAny(node)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("%s: %v", p, err)
	}
	var spec Spec
	if err := node.Decode(&spec); err != nil {
		return nil, generic, raw, fmt.Errorf("%s: %v", p, err)
	}
	return &spec, generic, raw, nil
}

// nodeToAny converts a YAML node to plain JSON-shaped values. Timestamps and
// any tag other than int/float/bool/null stay strings.
func nodeToAny(n *yaml.Node) (any, error) {
	switch n.Kind {
	case yaml.DocumentNode:
		if len(n.Content) == 0 {
			return nil, nil
		}
		return nodeToAny(n.Content[0])
	case yaml.AliasNode:
		return nodeToAny(n.Alias)
	case yaml.MappingNode:
		m := map[string]any{}
		for i := 0; i+1 < len(n.Content); i += 2 {
			k := n.Content[i].Value
			if n.Content[i].Tag == "!!merge" {
				return nil, fmt.Errorf("line %d: YAML merge keys are not supported", n.Content[i].Line)
			}
			v, err := nodeToAny(n.Content[i+1])
			if err != nil {
				return nil, err
			}
			m[k] = v
		}
		return m, nil
	case yaml.SequenceNode:
		out := make([]any, 0, len(n.Content))
		for _, c := range n.Content {
			v, err := nodeToAny(c)
			if err != nil {
				return nil, err
			}
			out = append(out, v)
		}
		return out, nil
	}
	switch n.ShortTag() {
	case "!!int":
		if i, err := strconv.ParseInt(n.Value, 0, 64); err == nil {
			return float64(i), nil
		}
	case "!!float":
		if f, err := strconv.ParseFloat(n.Value, 64); err == nil {
			return f, nil
		}
	case "!!bool":
		return n.Value == "true" || n.Value == "True" || n.Value == "TRUE", nil
	case "!!null":
		return nil, nil
	}
	return n.Value, nil
}

func sha256File(p string) (string, error) {
	b, err := os.ReadFile(p)
	if err != nil {
		return "", err
	}
	return sha256Hex(b), nil
}

func sha256Hex(b []byte) string {
	h := sha256.Sum256(b)
	return hex.EncodeToString(h[:])
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

var templateRE = regexp.MustCompile(`\{\{\s*([A-Za-z_][A-Za-z0-9_.-]*)\s*\}\}`)

type probeVars struct{}

type ctxMap map[string]any

type unknownVar struct{ name string }

func (u unknownVar) Error() string { return "unknown template variable " + u.name }

func lookup(ctx ctxMap, dotted string) (any, bool) {
	var cur any = map[string]any(ctx)
	for _, part := range strings.Split(dotted, ".") {
		switch c := cur.(type) {
		case probeVars:
			return "VAR", true
		case map[string]any:
			v, ok := c[part]
			if !ok {
				return nil, false
			}
			cur = v
		case map[string]string:
			v, ok := c[part]
			if !ok {
				return nil, false
			}
			cur = v
		default:
			return nil, false
		}
	}
	switch cur.(type) {
	case map[string]any, map[string]string, []any:
		return nil, false
	}
	return cur, true
}

func render(s string, ctx ctxMap) (string, error) {
	var bad error
	out := templateRE.ReplaceAllStringFunc(s, func(m string) string {
		name := templateRE.FindStringSubmatch(m)[1]
		v, ok := lookup(ctx, name)
		if !ok {
			bad = unknownVar{name}
			return m
		}
		return scalarString(v)
	})
	return out, bad
}

func renderAll(in []string, ctx ctxMap) ([]string, error) {
	out := make([]string, len(in))
	for i, s := range in {
		r, err := render(s, ctx)
		if err != nil {
			return nil, err
		}
		out[i] = r
	}
	return out, nil
}

func renderMap(in map[string]string, ctx ctxMap) (map[string]string, error) {
	out := map[string]string{}
	for k, v := range in {
		r, err := render(v, ctx)
		if err != nil {
			return nil, err
		}
		out[k] = r
	}
	return out, nil
}

func scalarString(v any) string {
	switch x := v.(type) {
	case string:
		return x
	case float64:
		return strconv.FormatFloat(x, 'f', -1, 64)
	case int:
		return strconv.Itoa(x)
	case nil:
		return ""
	}
	return fmt.Sprint(v)
}

func toFloat(v any) float64 {
	switch x := v.(type) {
	case float64:
		return x
	case int:
		return float64(x)
	case int64:
		return float64(x)
	}
	return 0
}

// resolvePaths fills defaults and resolves {{paths.*}} references.
func resolvePaths(spec *Spec) (map[string]string, error) {
	paths := map[string]string{}
	for k, v := range spec.Paths {
		paths[k] = v
	}
	if _, ok := paths["cards"]; !ok {
		paths["cards"] = "{{paths.root}}/cards"
	}
	if _, ok := paths["collected"]; !ok {
		paths["collected"] = "{{paths.root}}/collected"
	}
	exp := map[string]any{"id": spec.ID, "version": float64(spec.Version)}
	for i := 0; i < 6; i++ {
		ctx := ctxMap{"paths": paths, "exp": exp}
		changed := false
		next := map[string]string{}
		for k, v := range paths {
			r, _ := render(v, ctx)
			next[k] = r
			changed = changed || r != v
		}
		paths = next
		if !changed {
			break
		}
	}
	for _, k := range sortedKeys(paths) {
		v := paths[k]
		if templateRE.MatchString(v) {
			return nil, fmt.Errorf("paths.%s: unresolved template in %q", k, v)
		}
		if path.IsAbs(v) || hasDotDot(v) {
			return nil, fmt.Errorf("paths.%s: must be repo-relative without '..': %q", k, v)
		}
		paths[k] = strings.TrimSuffix(path.Clean(v), "/")
	}
	return paths, nil
}

func hasDotDot(p string) bool {
	for _, part := range strings.Split(p, "/") {
		if part == ".." {
			return true
		}
	}
	return false
}

func sortedKeys[V any](m map[string]V) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// expandCells returns explicit cells then matrix cells. Matrix axes are
// combined in sorted axis-name order, each axis in its written order.
func expandCells(spec *Spec) []Cell {
	cells := append([]Cell(nil), spec.Cells...)
	if m := spec.Matrix; m != nil {
		keys := sortedKeys(m.Axes)
		var walk func(i int, params map[string]any)
		walk = func(i int, params map[string]any) {
			if i == len(keys) {
				for _, ex := range m.Exclude {
					match := true
					for k, v := range ex {
						if scalarString(params[k]) != scalarString(normalize(v)) {
							match = false
						}
					}
					if match {
						return
					}
				}
				c := m.Defaults
				c.Params = map[string]any{}
				for k, v := range params {
					c.Params[k] = v
				}
				ctx := ctxMap{}
				for k, v := range params {
					ctx[k] = v
				}
				c.ID, _ = render(m.IDTemplate, ctx)
				cells = append(cells, c)
				return
			}
			for _, v := range m.Axes[keys[i]] {
				params[keys[i]] = normalize(v)
				walk(i+1, params)
			}
			delete(params, keys[i])
		}
		walk(0, map[string]any{})
	}
	for i := range cells {
		if cells[i].Role == "" {
			cells[i].Role = "measure"
		}
		norm := map[string]any{}
		for k, v := range cells[i].Params {
			norm[k] = normalize(v)
		}
		cells[i].Params = norm
	}
	return cells
}

// normalize maps YAML-decoded numbers to float64 so values compare like JSON.
func normalize(v any) any {
	switch x := v.(type) {
	case int:
		return float64(x)
	case int64:
		return float64(x)
	case uint64:
		return float64(x)
	case []any:
		out := make([]any, len(x))
		for i := range x {
			out[i] = normalize(x[i])
		}
		return out
	case map[string]any:
		out := map[string]any{}
		for k, val := range x {
			out[k] = normalize(val)
		}
		return out
	}
	return v
}

func fits(provides Resources, need *Resources) bool {
	if need == nil {
		return true
	}
	pairs := [][2]*float64{{provides.Vcpus, need.Vcpus}, {provides.MemoryGB, need.MemoryGB},
		{provides.DiskGB, need.DiskGB}, {provides.Gpus, need.Gpus}}
	for _, p := range pairs {
		if p[1] == nil {
			continue
		}
		have := 0.0
		if p[0] != nil {
			have = *p[0]
		}
		if have < *p[1] {
			return false
		}
	}
	pa, na := provides.Arch, need.Arch
	if na != "" && na != "any" && pa != "" && pa != "any" && pa != na {
		return false
	}
	have := map[string]bool{}
	for _, f := range provides.CPUFlags {
		have[f] = true
	}
	for _, f := range need.CPUFlags {
		if !have[f] {
			return false
		}
	}
	return true
}

func cellFitsHost(c *Cell, h *Host) bool {
	return fits(h.Provides, c.Requires) && (len(h.Cells) == 0 || contains(h.Cells, c.ID))
}

func cellsForStep(st *Step, cells []Cell) []Cell {
	var out []Cell
	for _, c := range cells {
		if len(st.Cells) > 0 && !contains(st.Cells, c.ID) {
			continue
		}
		if len(st.CellRoles) > 0 && !contains(st.CellRoles, c.Role) {
			continue
		}
		out = append(out, c)
	}
	sort.SliceStable(out, func(i, j int) bool { return out[i].Priority > out[j].Priority })
	return out
}

func contains(list []string, s string) bool {
	for _, x := range list {
		if x == s {
			return true
		}
	}
	return false
}

func instanceKey(step string, cell string, host int) string {
	if cell != "" {
		return step + "@" + cell
	}
	if host >= 0 {
		return fmt.Sprintf("%s#%d", step, host)
	}
	return step
}

// context builds the template context for one instance.
func context(spec *Spec, paths map[string]string, cell *Cell, host *Host, hidx int, vars map[string]string, probe bool) ctxMap {
	env := map[string]any{}
	for k, v := range spec.Env {
		env[k] = v
	}
	ctx := ctxMap{
		"exp":   map[string]any{"id": spec.ID, "version": float64(spec.Version)},
		"paths": paths,
		"env":   env,
	}
	v := map[string]any{}
	for k, val := range vars {
		v[k] = val
	}
	ctx["vars"] = v
	if probe {
		ctx["vars"] = probeVars{}
	}
	if cell != nil {
		c := map[string]any{"id": cell.ID, "role": cell.Role}
		for k, val := range cell.Params {
			c[k] = val
		}
		ctx["cell"] = c
	}
	if host != nil {
		h := map[string]any{"id": host.ID, "index": float64(hidx)}
		for k, val := range host.Config {
			h[k] = normalize(val)
		}
		ctx["host"] = h
	}
	return ctx
}

func yamlMarshal(v any) ([]byte, error) {
	var buf bytes.Buffer
	enc := yaml.NewEncoder(&buf)
	enc.SetIndent(2)
	if err := enc.Encode(v); err != nil {
		return nil, err
	}
	return buf.Bytes(), enc.Close()
}
