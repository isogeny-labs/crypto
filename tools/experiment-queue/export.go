package main

// Metadata adapter only: copies authenticated native ecbench observations into
// the canonical export shape. It runs no cryptographic computation or analysis.
import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

func asMap(v any) map[string]any {
	m, _ := v.(map[string]any)
	if m == nil {
		return map[string]any{}
	}
	return m
}
func array(v any) []any {
	a, _ := v.([]any)
	if a == nil {
		return []any{}
	}
	return a
}
func scaled(v any, divisor float64) any {
	n, ok := v.(json.Number)
	if !ok {
		return nil
	}
	f, e := n.Float64()
	if e != nil {
		return nil
	}
	return f / divisor
}
func readObject(file string) (map[string]any, error) {
	b, e := os.ReadFile(file)
	if e != nil {
		return nil, e
	}
	return object(b)
}
func exportEcbench(request, session, out string, admitted bool) error {
	b, err := os.ReadFile(request)
	if err != nil {
		return err
	}
	var d Dispatch
	if err = json.Unmarshal(b, &d); err != nil {
		return err
	}
	c := d.Candidate
	build, err := readObject(filepath.Join(session, "..", "build-info.json"))
	if err != nil {
		return err
	}
	if build["source_revision"] != c.Entry.SourceCommit || build["ecbench_sha256"] != c.Entry.Image.EcbenchSHA256 {
		return fmt.Errorf("observed build identity differs from the request")
	}
	p, err := object(c.Protocol)
	if err != nil {
		return err
	}
	s, err := readObject(filepath.Join(session, "session.json"))
	if err != nil {
		return err
	}
	if s["schema"] != "ecbench.session/v1" {
		return fmt.Errorf("not an ecbench session")
	}
	if observed, ok := s["git_commit"].(string); ok && observed != "" && observed != c.Entry.SourceCommit {
		return fmt.Errorf("session source mismatch")
	}
	if s["git_dirty"] == true {
		return fmt.Errorf("dirty session source")
	}
	arms := []any{}
	known := map[string]bool{}
	for _, v := range array(p["arms"]) {
		a := asMap(v)
		id, _ := a["id"].(string)
		known[id] = true
		arms = append(arms, map[string]any{"id": id, "role": a["role"], "binary_sha256": c.Entry.Image.EcbenchSHA256, "commit": c.Entry.SourceCommit})
	}
	f, err := os.Open(filepath.Join(session, "records.jsonl"))
	if err != nil {
		return err
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	scanner.Buffer(make([]byte, 4096), 8<<20)
	runs := []any{}
	seen := map[string]bool{}
	completed := 0
	excluded := []any{}
	excludedByRule := map[string][]string{}
	best := any(nil)
	for scanner.Scan() {
		if len(scanner.Bytes()) == 0 {
			continue
		}
		r, err := object(scanner.Bytes())
		if err != nil {
			return err
		}
		id, _ := r["record_id"].(string)
		arm, _ := r["arm"].(string)
		if id == "" || seen[id] || !known[arm] {
			return fmt.Errorf("unknown arm or repeated native record")
		}
		seen[id] = true
		if r["binary_sha256"] != c.Entry.Image.EcbenchSHA256 {
			return fmt.Errorf("native record binary identity mismatch")
		}
		outcome := asMap(r["outcome"])
		status := "failed"
		if outcome["status"] == "verified" {
			status = "succeeded"
		} else if outcome["status"] == "timeout" {
			status = "timeout"
		}
		completed++
		iso := asMap(r["isolation"])
		tier := iso["level"]
		if tier == "L3" {
			best = "L3"
		} else if tier == "L2" && best != "L3" {
			best = "L2"
		}
		cost := asMap(r["cost"])
		clock := asMap(r["time"])
		counted := map[string]any{}
		for _, k := range []string{"total_gae", "s"} {
			if cost[k] != nil {
				counted[k] = cost[k]
			}
		}
		counters := asMap(r["counters"])
		for k, v := range asMap(r["field_ops"]) {
			counters["field_ops."+k] = v
		}
		warmup := r["warmup"] == true
		valid := admitted && status == "succeeded" && !warmup && (tier == "L2" || tier == "L3") && clock["solve_wall_ns"] != nil
		verify := "no_claim"
		if outcome["matches_target"] == true {
			verify = "verified"
		} else if outcome["recovered"] != nil {
			verify = "refuted"
		}
		bad := status != "succeeded" && !warmup && outcome["recovered"] != nil
		var rule any
		if bad {
			rule = "invalid_measurement: wrong answer"
			excludedByRule[rule.(string)] = append(excludedByRule[rule.(string)], id)
		}
		round := r["round"]
		if round == nil {
			round = 0
		}
		blockers := array(iso["blockers"])
		runs = append(runs, map[string]any{"run_id": id, "arm": arm, "instance_id": asMap(r["workload"])["workload_id"], "seed": r["algorithm_seed"], "replicate": round, "warmup": warmup, "status": status, "outcome_class": "completed", "attempts": 1, "resumed": 0, "checkpoint": nil,
			"measurements": map[string]any{"counted": counted, "wall_seconds": scaled(clock["solve_wall_ns"], 1e9), "wall_valid": valid, "cpu_user_seconds": scaled(clock["user_ns"], 1e9), "cpu_sys_seconds": scaled(clock["sys_ns"], 1e9), "max_rss_bytes": scaled(clock["max_rss_kib"], 1.0/1024), "counters": counters, "perf": asMap(clock["hw_solve"])},
			"isolation":    map[string]any{"tier": tier, "contended": tier != "L2" && tier != "L3", "blockers": blockers}, "verification": map[string]any{"status": verify, "verifier": "native ecbench audit and replay"}, "exclusion": map[string]any{"excluded": bad, "rule": rule}, "artifacts": []any{}, "started_at": nil, "finished_at": nil, "worker": nil, "backend_ref": r["record_id"]})
	}
	if err = scanner.Err(); err != nil {
		return err
	}
	if len(runs) == 0 {
		return fmt.Errorf("no native records")
	}
	hostSummary := map[string]any{"cpu_model": nil, "logical_cpus": nil, "memory_bytes": nil, "kernel": nil, "hypervisor": nil, "isolation_tier_max": best}
	hostPath := any(nil)
	hostHash := any(nil)
	if hb, e := os.ReadFile(filepath.Join(session, "host.json")); e == nil {
		h, _ := object(hb)
		stable := asMap(h["stable"])
		hostPath = "session/host.json"
		hostHash = digest(hb)
		hostSummary["cpu_model"] = stable["cpu_model"]
		hostSummary["logical_cpus"] = stable["logical_cpus"]
		hostSummary["memory_bytes"] = scaled(stable["mem_total_kib"], 1.0/1024)
		hostSummary["kernel"] = stable["kernel_release"]
	}
	files := []any{}
	for _, name := range []string{"session.json", "records.jsonl", "host.json", "spec.json", "plan.json"} {
		if b, e := os.ReadFile(filepath.Join(session, name)); e == nil {
			kind := "raw"
			if name == "host.json" {
				kind = "host"
			}
			files = append(files, map[string]any{"path": "session/" + name, "sha256": digest(b), "bytes": len(b), "kind": kind})
		}
	}
	for rule, ids := range excludedByRule {
		excluded = append(excluded, map[string]any{"rule": rule, "run_ids": ids})
	}
	export := map[string]any{"schema": "experiment.export/v1", "export_id": "isolab-" + c.Key[:20], "created": time.Now().UTC().Format(time.RFC3339), "protocol": map[string]any{"path": c.Entry.Protocol.Path, "sha256": c.Entry.Protocol.SHA256, "protocol_id": p["protocol_id"], "version": p["version"]}, "producer": map[string]any{"runtime": "github_actions", "model": nil, "session": c.JobName, "role": "executor"}, "source": map[string]any{"repo": p["repository"], "commit": c.Entry.SourceCommit, "dirty": false}, "backend": "isolab-k8s", "host": map[string]any{"path": hostPath, "sha256": hostHash, "class_id": s["env_class_id"], "summary": hostSummary}, "arms": arms, "runs": runs, "flow": map[string]any{"planned": len(runs), "attempted": len(runs), "completed": completed, "not_completed": 0, "interrupted": 0, "resumed": 0, "excluded": excluded}, "files": files, "analysis": map[string]any{"script": nil, "blinded": false, "blinding_key_sha256": nil, "outputs": []any{}, "unmasked_at": nil}, "notes": []any{map[string]any{"kind": "observation", "text": "Copied native records and authenticated source/image identity; no scientific verdict was generated.", "by": "native Isolab queue adapter"}}, "reviews_dir": "reviews"}
	return writeJSON(out, export)
}
