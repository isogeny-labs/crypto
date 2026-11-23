#!/usr/bin/env python3
"""Resumable, size-matched PKM chain search with an anytime confirmation gate.

Each invocation searches a bounded number of new auxiliary-curve seeds. Reinvoke
with a larger --through-seed to continue; a hit is a candidate for independent
replay and complete-cost benchmarking, not a final speedup claim.
"""
import argparse
import hashlib
import importlib.util
import json
import math
import resource
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "pkm_map_sweep_20261123" / "sweep.py"
spec = importlib.util.spec_from_file_location("pkm_relations", PRIOR)
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def load_rows(path):
    rows = {}
    if path.exists():
        for line in path.read_text().splitlines():
            row = json.loads(line)
            if row["id"] in rows:
                raise ValueError(f"duplicate record: {row['id']}")
            rows[row["id"]] = row
    return rows


def verified_tower(raw):
    outer = json.loads(raw)
    tower = outer.get("tower", outer)
    assert tower["p"] == m.P and tower["kind"]["form"] == "isogeny"
    assert math.prod(step["degree"] for step in tower["steps"]) == tower["S"]
    assert len(set(tower["V"])) == len(tower["V"]) == tower["S"]
    assert all(m.map_value(tower["steps"], x) == tower["c"] for x in tower["V"])
    return tower


def append(path, row):
    m.append(path, row)
    print(json.dumps({k: row.get(k) for k in
          ("id", "kind", "base_size", "screen_hits", "confirm_hits",
           "paired_p", "component_candidate", "exit_code")}), flush=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--through-seed", type=int, required=True)
    ap.add_argument("--out", type=Path, default=HERE / "adaptive.jsonl")
    ap.add_argument("--max-wall-seconds", type=float, default=120.0)
    args = ap.parse_args()
    if args.through_seed < 9 or args.max_wall_seconds <= 0:
        ap.error("through-seed must be >=9 and wall budget positive")
    binary_sha = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    rows = load_rows(args.out)
    if "header" not in rows:
        append(args.out, {"id": "header", "kind": "header", "binary_sha256": binary_sha,
                          "target_seed": m.SEED, "screen": m.SCREEN, "confirm": m.CONFIRM,
                          "curve": {"p": m.P, "a": m.A, "b": m.B},
                          "generator": "ic isogeny --bound 13 --min-size 500 --max-curves 1000",
                          "source_module_sha256": hashlib.sha256(PRIOR.read_bytes()).hexdigest()})
        rows = load_rows(args.out)
    if rows["header"]["binary_sha256"] != binary_sha or rows["header"]["source_module_sha256"] != hashlib.sha256(PRIOR.read_bytes()).hexdigest():
        raise ValueError("binary or relation code changed: start a new store")
    previous_hits = [row["id"] for row in rows.values() if row.get("component_candidate")]
    if previous_hits:
        print(f"confirmation candidate already recorded: {previous_hits[0]}", flush=True)
        return
    targets = m.targets()
    deadline = time.monotonic() + args.max_wall_seconds
    for seed in range(9, args.through_seed + 1):
        # Reserve a full generator window; do not create artificial failures
        # when the invocation's soft wall budget is almost exhausted.
        if deadline - time.monotonic() < 30:
            break
        key = f"seed:{seed}"
        if key in rows:
            continue
        command = [str(args.bin), "isogeny", "--p", str(m.P), "--bound", "13",
                   "--min-size", "500", "--seed", str(seed), "--max-curves", "1000"]
        before = resource.getrusage(resource.RUSAGE_CHILDREN)
        start = time.perf_counter()
        try:
            proc = subprocess.run(command, capture_output=True, text=True, timeout=30)
            exit_code, raw, stderr = proc.returncode, proc.stdout, proc.stderr
        except subprocess.TimeoutExpired as exc:
            exit_code, raw, stderr = 124, "", str(exc)
        generator_wall = time.perf_counter() - start
        after = resource.getrusage(resource.RUSAGE_CHILDREN)
        generator_cpu = (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime)
        common = {"id": key, "kind": "pkm-distinct-chain", "seed": seed,
                  "command": command, "binary_sha256": binary_sha,
                  "generator_wall_s": generator_wall, "generator_cpu_s": generator_cpu,
                  "exit_code": exit_code, "stderr_tail": stderr[-1000:]}
        if exit_code != 0:
            row = {**common, "stdout_tail": raw[-500:]}
            append(args.out, row); rows[key] = row
            continue
        raw_sha = hashlib.sha256(raw.encode()).hexdigest()
        try:
            tower = verified_tower(raw)
        except Exception as exc:
            row = {**common, "exit_code": 2, "source_sha256": raw_sha,
                   "validation_error": repr(exc)}
            append(args.out, row); rows[key] = row
            continue
        start_w, start_c = time.perf_counter(), time.process_time()
        base = sorted(x for x in tower["V"] if m.lift(x) is not None)
        base_wall, base_cpu = time.perf_counter() - start_w, time.process_time() - start_c
        control_key = f"interval:size={len(base)}"
        if control_key not in rows:
            start_w, start_c = time.perf_counter(), time.process_time()
            control_base = m.interval_base(len(base))
            control = m.measure(control_key, control_base, targets,
                {"kind": "size_matched_control",
                 "base_wall_s": time.perf_counter() - start_w,
                 "base_cpu_s": time.process_time() - start_c})
            append(args.out, control); rows[control_key] = control
        control = rows[control_key]
        row = m.measure(key, base, targets, {**common, "source_sha256": raw_sha,
            "low_degrees": [s["degree"] for s in tower["steps"]],
            "high_degree": tower["S"], "aux": tower["kind"]["aux"],
            "fiber_verified": True, "fiber_size": len(tower["V"]),
            "base_wall_s": base_wall, "base_cpu_s": base_cpu,
            "control_id": control_key})
        pval, wins, losses = m.paired_tail(row["success_bits"][m.SCREEN:],
                                           control["success_bits"][m.SCREEN:])
        # Sum_{seed=9}^infinity .01/[n(n+1)] < .01, where n=seed-8.
        n = seed - 8
        alpha = .01 / (n * (n + 1))
        candidate_cpu = generator_cpu + base_cpu + row["query_cpu_s"]
        control_cpu = control["base_cpu_s"] + control["query_cpu_s"]
        row.update({"paired_p": pval, "paired_wins": wins,
                    "paired_losses": losses, "alpha_spent": alpha,
                    "candidate_cpu_per_relation": candidate_cpu / max(1, row["screen_relations"] + row["confirm_relations"]),
                    "control_cpu_per_relation": control_cpu / max(1, control["screen_relations"] + control["confirm_relations"]),
                    "component_candidate": (row["screen_hits"] > control["screen_hits"] and
                        row["confirm_hits"] > control["confirm_hits"] and pval < alpha and
                        candidate_cpu / max(1, row["screen_relations"] + row["confirm_relations"]) <
                        control_cpu / max(1, control["screen_relations"] + control["confirm_relations"]))})
        append(args.out, row); rows[key] = row
        if row["component_candidate"]:
            (HERE / f"candidate_seed{seed}_source.json").write_text(raw)
            break


if __name__ == "__main__":
    main()
