#!/usr/bin/env python3
"""Fail-closed offline audit of CT226's six GitHub-hosted native timing records.

Not an independent mathematical replay or a PCC signature. Never modifies raw rows.
"""
import argparse
import copy
import json
import math
import statistics
from pathlib import Path

HEADS = {
    "44467973031": ("95d374fe1e31214b5eff586866404a1a3dc53216", 135370252212),
    "44468520835": ("9b203b4bb12922dbfa56b9d5eb14b543a990c5e3", 135372027935),
}
PHASES = ("ic_setup_ns", "ic_relation_ns", "ic_la_ns", "ic_recovery_ns", "ic_verify_ns")
CONSTANTS = {
    "ic_candidates": 1421, "ic_smooth": 62, "ic_rank": 54,
    "offsets": 14, "cantor_adds": 1435, "target_adds": 4,
    "direct_bsgs_adds": 14, "field_add": 1273478,
    "field_mul": 956007, "field_sqr": 17786, "field_inv": 24532,
    "source": "known_answer_F101", "memory": "unmeasured", "pcc": "unverified",
}

def require(test, message):
    if not test:
        raise ValueError(message)

def audit(document):
    require(document.get("schema") == "ct227.native-ci-pairs.v1", "schema mismatch")
    observations = document.get("observations")
    require(isinstance(observations, list) and len(observations) == 6, "six observations required")
    visited = set()
    ratios, costs, baselines, floors, fractions = [], [], [], [], []
    for obs in observations:
        require(isinstance(obs, dict), "observation must be an object")
        run = obs.get("run_id")
        require(run in HEADS, "unexpected workflow")
        head, job = HEADS[run]
        require(obs.get("head_sha") == head and obs.get("job_id") == job, "provenance mismatch")
        m = obs.get("record")
        require(isinstance(m, dict), "record must be an object")
        round_num = m.get("round")
        require(type(round_num) is int and round_num in (0, 1, 2), "invalid round")
        require((run, round_num) not in visited, "duplicate run/round")
        visited.add((run, round_num))
        require(m.get("arm_order") == ("BSGS_IC" if round_num == 1 else "IC_BSGS"), "bad arm order")
        for key, value in CONSTANTS.items():
            require(m.get(key) == value, "constant mismatch: " + key)
        for key in (*PHASES, "candidate_ns", "direct_bsgs_ns"):
            require(type(m.get(key)) is int and m[key] > 0, "missing/nonpositive integer: " + key)
        require(sum(m[name] for name in PHASES) == m["candidate_ns"], "phase sum mismatch")
        ratio = m["direct_bsgs_ns"] / m["candidate_ns"]
        claimed = m.get("ratio_direct_over_candidate")
        require(type(claimed) is float and math.isfinite(claimed), "missing/nonfinite ratio")
        require(abs(ratio - claimed) <= 0.000000000051, "rounded ratio mismatch")
        require(ratio < 1, "candidate outperformed baseline: review before claiming negative")
        ratios.append(ratio)
        costs.append(m["candidate_ns"])
        baselines.append(m["direct_bsgs_ns"])
        floors.append(m["candidate_ns"] - m["ic_relation_ns"])
        fractions.append(m["ic_relation_ns"] / m["candidate_ns"])
    require(visited == {(run, i) for run in HEADS for i in range(3)}, "incomplete paired run coverage")
    slowdown = [1 / x for x in ratios]
    floor_slowdown = [f / b for f, b in zip(floors, baselines)]
    return {
        "observations": len(observations),
        "median_candidate_ns": statistics.median(costs),
        "median_baseline_ns": statistics.median(baselines),
        "median_paired_baseline_over_candidate": statistics.median(ratios),
        "median_paired_slowdown": statistics.median(slowdown),
        "min_paired_slowdown": min(slowdown),
        "max_paired_slowdown": max(slowdown),
        "mean_relation_fraction": statistics.mean(fractions),
        "median_nonrelation_ns": statistics.median(floors),
        "median_nonrelation_over_bsgs": statistics.median(floor_slowdown),
        "memory_measured": False,
        "independent_pcc": False,
    }

def self_test(doc):
    audit(doc)
    for label, mutate in (
        ("phase sum", lambda x: x["observations"][0]["record"].__setitem__("ic_setup_ns", 1)),
        ("ratio", lambda x: x["observations"][0]["record"].__setitem__("ratio_direct_over_candidate", 1.0)),
        ("provenance", lambda x: x["observations"][0].__setitem__("head_sha", "0" * 40)),
        ("pcc", lambda x: x["observations"][0]["record"].__setitem__("pcc", "verified")),
        ("duplicate", lambda x: x["observations"][1]["record"].__setitem__("round", 0)),
        ("missing", lambda x: x["observations"].pop()),
    ):
        changed = copy.deepcopy(doc)
        mutate(changed)
        try:
            audit(changed)
        except ValueError:
            continue
        raise AssertionError("tampering not caught: " + label)
    return 6

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--data", type=Path, default=Path(__file__).with_name("ct227_native_ci_pairs.json"))
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    doc = json.loads(args.data.read_text())
    if args.self_test:
        self_test(doc)
    summary = audit(doc)
    print(json.dumps(summary, sort_keys=True, separators=(",", ":")))
    if args.self_test:
        print("CT227_SELFTEST_PASS 6 6")

if __name__ == "__main__":
    main()
