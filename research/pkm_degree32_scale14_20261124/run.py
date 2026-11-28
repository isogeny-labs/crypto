#!/usr/bin/env python3
"""Resume the frozen three-instance 14-bit degree-32 PKM panel."""
import argparse
import hashlib
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKM = HERE.parent / "pkm_adaptive_sweep_20261123"
sys.path.insert(0, str(PKM))
import f4_height4 as f4  # noqa: E402

EXPECTED = "06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
BITS = 14
REPEATS = 16
SEED_START = 20261200
SEED_END = 20261599
MATCHES = 3
INTERVAL = "prime-abscissa:below=32"
CANDIDATE = "prime-isogeny:t=5,seed=5,aux=ordinary"
INTERVAL_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
CANDIDATE_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
ORDER = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")


def command(binary, seed, fb, oracle, preflight):
    cmd = [str(binary), "bench", "--bits", str(BITS), "--family", "generic",
           "--seed", str(seed), "--repeats", str(REPEATS), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "1000",
           "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", "5"]
    return cmd


def base_size(item):
    rows = (item.get("result") or {}).get("rows") or []
    values = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return values.pop() if len(rows) == REPEATS and len(values) == 1 else None


def read_store(path):
    records = {}
    if path.exists():
        for line in path.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise ValueError(f"duplicate record {item['id']}")
            records[item["id"]] = item
    return records


def sum_metric(rows, path):
    total = 0
    for row in rows:
        value = row
        for key in path:
            value = value.get(key, {}) if isinstance(value, dict) else {}
        total += value if isinstance(value, (int, float)) else 0
    return total


def run_summary(record):
    rows = (record.get("result") or {}).get("rows") or []
    recovered = [r.get("recovered") for r in rows]
    ns_per_op = [r.get("decomposition", {}).get("solver", {}).get("ns_per_op") for r in rows]
    ns_per_op = [x for x in ns_per_op if isinstance(x, (int, float))]
    s_over_rho = [r.get("s_over_rho") for r in rows]
    s_over_rho = [x for x in s_over_rho if isinstance(x, (int, float))]
    systems = sorted({json.dumps(r.get("decomposition", {}).get("system", {}), sort_keys=True) for r in rows})
    return {
        "exit_code": record.get("exit_code"),
        "rows": len(rows),
        "verified": sum(bool(r.get("verified")) for r in rows),
        "distinct_recovered": len({x for x in recovered if x is not None}),
        "wall_s": record.get("wall_s"),
        "child_cpu_s": record.get("child_cpu_s"),
        "total_gae": sum_metric(rows, ("total_gae",)),
        "relations": sum_metric(rows, ("decomposition", "relations_found")),
        "target_trials": sum_metric(rows, ("decomposition", "targets_tried")),
        "f4_multiply_adds": sum_metric(rows, ("decomposition", "solver", "ops")),
        "solver_wall_s": sum_metric(rows, ("decomposition", "solver", "wall_ns")) / 1e9,
        "median_ns_per_op": statistics.median(ns_per_op) if ns_per_op else None,
        "median_s_over_rho": statistics.median(s_over_rho) if s_over_rho else None,
        "systems": [json.loads(x) for x in systems],
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale14.jsonl")
    args = ap.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("binary SHA-256 differs from frozen protocol")
    records = read_store(args.store)
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.degree32-scale14/v1",
                "binary_sha256": digest, "protocol": "PROTOCOL.md", "bits": BITS,
                "repeats": REPEATS, "seed_range": [SEED_START, SEED_END],
                "required_matches": MATCHES, "order": list(ORDER)}
        f4.append(args.store, item); records["header"] = item
    if records["header"]["binary_sha256"] != digest:
        raise ValueError("store binary differs from frozen protocol")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True)); return

    if "selection" in records:
        selected = records["selection"]["instances"]
    else:
        selected = []
        for seed in range(SEED_START, SEED_END + 1):
            for arm, fb in (("baseline", INTERVAL), ("candidate", CANDIDATE)):
                key = f"preflight:{seed}:{arm}"
                if key not in records:
                    item = {"id": key, "seed": seed, "arm": arm,
                            **f4.run(command(args.bin, seed, fb, "", True), 30)}
                    f4.append(args.store, item); records[key] = item
            a, b = records[f"preflight:{seed}:baseline"], records[f"preflight:{seed}:candidate"]
            size_a, size_b = base_size(a), base_size(b)
            if (a["exit_code"] in (0, 1) and b["exit_code"] in (0, 1) and
                    a.get("result", {}).get("instance") == b.get("result", {}).get("instance") and
                    size_a is not None and size_a == size_b):
                selected.append({"seed": seed, "instance": a["result"]["instance"], "base_size": size_a})
                if len(selected) == MATCHES:
                    break
        if len(selected) != MATCHES:
            item = {"id": "verdict", "complete": False, "reason": "fewer-than-three-size-matches",
                    "matches": selected, "seeds_examined": [SEED_START, SEED_END]}
            f4.append(args.store, item); print(json.dumps(item, sort_keys=True)); return
        if len({x["instance"] for x in selected}) != MATCHES:
            raise RuntimeError("selected instances are not distinct")
        item = {"id": "selection", "instances": selected}
        f4.append(args.store, item); records["selection"] = item

    for choice in selected:
        seed = choice["seed"]
        for arm_rep in ORDER:
            key = f"timed:{seed}:{arm_rep}"
            if key in records:
                continue
            arm = arm_rep.split(":")[0]
            fb = INTERVAL if arm == "baseline" else CANDIDATE
            oracle = INTERVAL_ORACLE if arm == "baseline" else CANDIDATE_ORACLE
            item = {"id": key, "seed": seed, "arm": arm,
                    **f4.run(command(args.bin, seed, fb, oracle, False), 600)}
            f4.append(args.store, item); records[key] = item

    instance_verdicts = []
    aggregate = {"baseline_gae": 0, "candidate_gae": 0, "baseline_wall": 0,
                 "candidate_wall": 0, "baseline_cpu": 0, "candidate_cpu": 0,
                 "baseline_relations": 0, "candidate_relations": 0,
                 "baseline_trials": 0, "candidate_trials": 0}
    for choice in selected:
        seed, instance, expected_size = choice["seed"], choice["instance"], choice["base_size"]
        runs = [records[f"timed:{seed}:{x}"] for x in ORDER]
        rowsets = [(x.get("result") or {}).get("rows") or [] for x in runs]
        complete = (all(x.get("exit_code") == 0 and x.get("result", {}).get("instance") == instance and
                        base_size(x) == expected_size for x in runs) and
                    all(len(rs) == REPEATS and len({r.get("recovered") for r in rs}) == REPEATS and
                        all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets) and
                    all(len({rowsets[j][i]["recovered"] for j in range(4)}) == 1 for i in range(REPEATS)))
        summaries = [run_summary(x) for x in runs]
        entry = {"seed": seed, "instance": instance, "base_size": expected_size,
                 "complete": complete, "runs": dict(zip(ORDER, summaries))}
        if complete:
            pairs = []
            for ai, bi in ((0, 1), (3, 2)):
                a, b = summaries[ai], summaries[bi]
                pairs.append({"gae_ratio": b["total_gae"] / a["total_gae"],
                              "wall_ratio": b["wall_s"] / a["wall_s"],
                              "cpu_ratio": b["child_cpu_s"] / a["child_cpu_s"]})
            base_gae = summaries[0]["total_gae"] + summaries[3]["total_gae"]
            cand_gae = summaries[1]["total_gae"] + summaries[2]["total_gae"]
            stability = {}
            for arm, indices in (("baseline", (0, 3)), ("candidate", (1, 2))):
                gae = [summaries[i]["total_gae"] for i in indices]
                wall = [summaries[i]["wall_s"] for i in indices]
                stability[arm] = {"gae_max_over_min": max(gae) / min(gae),
                                  "wall_max_over_min": max(wall) / min(wall)}
            stable = all(x["gae_max_over_min"] <= 1.5 and x["wall_max_over_min"] <= 1.5
                         for x in stability.values())
            passed = (all(p["gae_ratio"] < .9 and p["wall_ratio"] < 1 and p["cpu_ratio"] < 1 for p in pairs)
                      and cand_gae / base_gae < .9 and stable)
            entry.update({"pairs": pairs, "combined_gae_ratio": cand_gae / base_gae,
                          "stability": stability, "stable": stable, "component_gate_pass": passed})
            for i in (0, 3):
                aggregate["baseline_gae"] += summaries[i]["total_gae"]
                aggregate["baseline_wall"] += summaries[i]["wall_s"]
                aggregate["baseline_cpu"] += summaries[i]["child_cpu_s"]
                aggregate["baseline_relations"] += summaries[i]["relations"]
                aggregate["baseline_trials"] += summaries[i]["target_trials"]
            for i in (1, 2):
                aggregate["candidate_gae"] += summaries[i]["total_gae"]
                aggregate["candidate_wall"] += summaries[i]["wall_s"]
                aggregate["candidate_cpu"] += summaries[i]["child_cpu_s"]
                aggregate["candidate_relations"] += summaries[i]["relations"]
                aggregate["candidate_trials"] += summaries[i]["target_trials"]
        else:
            entry.update({"stable": False, "component_gate_pass": False})
        instance_verdicts.append(entry)

    all_complete = all(x["complete"] for x in instance_verdicts)
    overall = {"complete": all_complete}
    if all_complete:
        overall.update({
            "gae_ratio": aggregate["candidate_gae"] / aggregate["baseline_gae"],
            "wall_ratio": aggregate["candidate_wall"] / aggregate["baseline_wall"],
            "cpu_ratio": aggregate["candidate_cpu"] / aggregate["baseline_cpu"],
            "relation_probability_ratio":
                (aggregate["candidate_relations"] / aggregate["candidate_trials"]) /
                (aggregate["baseline_relations"] / aggregate["baseline_trials"]),
            "useful_relations_per_gae_ratio":
                (aggregate["candidate_relations"] / aggregate["candidate_gae"]) /
                (aggregate["baseline_relations"] / aggregate["baseline_gae"]),
            "totals": aggregate,
        })
    overall_pass = (all_complete and all(x["component_gate_pass"] for x in instance_verdicts) and
                    overall["gae_ratio"] < 1 and overall["wall_ratio"] < 1 and overall["cpu_ratio"] < 1)
    item = {"id": "verdict", "complete": all_complete, "instances": instance_verdicts,
            "overall": overall, "scale14_transfer_gate_pass": overall_pass}
    f4.append(args.store, item); print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
