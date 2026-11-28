#!/usr/bin/env python3
"""Bounded PKM auxiliary-curve and low-degree-chain sweep."""
import argparse
import hashlib
import json
import resource
import subprocess
import time
from pathlib import Path

import sweep as m

HERE = Path(__file__).resolve().parent
SEEDS = range(1, 9)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--out", type=Path, default=HERE / "chain_results.jsonl")
    args = ap.parse_args()
    binary_sha = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    rows = {}
    if args.out.exists():
        for line in args.out.read_text().splitlines():
            row = json.loads(line)
            rows[row["id"]] = row
    targets = m.targets()
    for seed in SEEDS:
        name = f"seed{seed}"
        generation_id = f"generate:{name}"
        artifact = HERE / f"{name}_source.json"
        command = [str(args.bin), "isogeny", "--p", str(m.P), "--bound", "13",
                   "--min-size", "500", "--seed", str(seed), "--max-curves", "1000"]
        if generation_id not in rows:
            if artifact.exists():
                raise RuntimeError(f"orphan artifact without generation row: {artifact}")
            before = resource.getrusage(resource.RUSAGE_CHILDREN)
            wall_start = time.perf_counter()
            p = subprocess.run(command, capture_output=True, text=True)
            wall = time.perf_counter()-wall_start
            after = resource.getrusage(resource.RUSAGE_CHILDREN)
            child_cpu = (after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime)
            record = {"id":generation_id, "kind":"generator", "command":command,
                      "binary_sha256":binary_sha,"exit_code":p.returncode,
                      "generator_wall_s":wall,"generator_cpu_s":child_cpu,
                      "stderr":p.stderr[-4000:]}
            if p.returncode == 0:
                artifact.write_text(p.stdout)
                record["artifact_sha256"] = hashlib.sha256(p.stdout.encode()).hexdigest()
            else:
                record["stdout_tail"] = p.stdout[-1000:]
            m.append(args.out, record)
            rows[generation_id] = record
        record = rows[generation_id]
        if record["exit_code"] != 0:
            continue
        assert record["binary_sha256"] == binary_sha
        assert hashlib.sha256(artifact.read_bytes()).hexdigest() == record["artifact_sha256"]
        if name in rows:
            continue
        t, source_sha = m.source(name)
        start_wall, start_cpu = time.perf_counter(), time.process_time()
        base = sorted(x for x in t["V"] if m.lift(x) is not None)
        base_wall, base_cpu = time.perf_counter()-start_wall, time.process_time()-start_cpu
        control_id = f"interval:size={len(base)}"
        if control_id not in rows:
            start_cpu = time.process_time()
            control = m.measure(control_id, m.interval_base(len(base)), targets,
                                {"kind":"size_matched_control", "base_cpu_s":time.process_time()-start_cpu})
            m.append(args.out, control)
            rows[control_id] = control
        control = rows[control_id]
        row = m.measure(name, base, targets,
            {"kind":"pkm-distinct-chain", "source_sha256":source_sha,
             "binary_sha256":binary_sha,"low_degrees":[s["degree"] for s in t["steps"]],
             "high_degree":t["S"],"aux":t["kind"]["aux"],
             "fiber_size":len(t["V"]),"lifting_count":len(base),
             "fiber_verified":True,"base_wall_s":base_wall,"base_cpu_s":base_cpu,
             "generator_wall_s":record["generator_wall_s"],"generator_cpu_s":record["generator_cpu_s"]})
        pval,wins,losses=m.paired_tail(row["success_bits"][m.SCREEN:],
                                      control["success_bits"][m.SCREEN:])
        row.update({"control_id":control_id,"control_screen_hits":control["screen_hits"],
                    "control_confirm_hits":control["confirm_hits"],
                    "paired_p":pval,"paired_wins":wins,"paired_losses":losses})
        row["screen_advances"]=row["screen_hits"]>control["screen_hits"]
        candidate_cpu=record["generator_cpu_s"]+base_cpu+row["query_cpu_s"]
        control_cpu=control["base_cpu_s"]+control["query_cpu_s"]
        row["confirmed_component_gain"]=(row["screen_advances"] and
            row["confirm_hits"]>control["confirm_hits"] and pval<.01/len(SEEDS) and
            candidate_cpu/max(row["screen_hits"]+row["confirm_hits"],1) <
            control_cpu/max(control["screen_hits"]+control["confirm_hits"],1))
        m.append(args.out,row)
        rows[name]=row
        print(json.dumps({k:row[k] for k in ("id","low_degrees","high_degree","screen_hits","confirm_hits","paired_p","confirmed_component_gain")}),flush=True)
        if row["confirmed_component_gain"]:
            break


if __name__ == "__main__":
    main()
