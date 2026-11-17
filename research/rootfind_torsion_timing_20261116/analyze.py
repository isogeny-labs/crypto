#!/usr/bin/env python3
"""Pre-data local diagnostic analyzer; never promotes an L2 performance claim."""

import argparse
import json
import random
import statistics
from collections import defaultdict
from pathlib import Path

ARMS = ("f6-ic", "harvest-m3", "torsion-m3", "torsion-m3-aa")
ORACLES = {
    "f6-ic": "pdp3-binary:m=3,engine=f6-ic,degree=3,node_budget=8192",
    "harvest-m3": "pdp3-binary:m=3,engine=rootfind-negfold-batch-harvest",
    "torsion-m3": "pdp3-binary:m=3,engine=rootfind-negfold-batch-harvest-torsion",
    "torsion-m3-aa": "pdp3-binary:m=3,engine=rootfind-negfold-batch-harvest-torsion",
}
EXPECTED_CURVE_UID = "urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60"
EXPECTED_BASE_SHA = "cc3c0ee034174a528849ca7efd95d70459404f6cc1e9361f0b4da7f6c3da2ae3"
PHASES = {"factor_base", "oracle_setup", "relations", "linear_algebra", "verify"}
TARGETS = range(4)
ROUNDS = range(1, 9)
BOOTSTRAP_SEED = 202611160917
RESAMPLES = 5000


def percentile(values, fraction):
    ordered = sorted(values)
    x = (len(ordered) - 1) * fraction
    lo = int(x)
    hi = min(lo + 1, len(ordered) - 1)
    return ordered[lo] + (ordered[hi] - ordered[lo]) * (x - lo)


def analyze(records):
    slots = defaultdict(dict)
    failures = []
    warmup_slots = set()
    for rec in records:
        arm = rec.get("arm")
        if arm not in ARMS:
            raise ValueError(f"unexpected arm {arm}")
        workload = rec.get("workload") or {}
        target = workload.get("target_index")
        if target not in TARGETS or workload.get("target_seed") != 202611151301:
            raise ValueError(f"unexpected target identity: {target}")
        if workload.get("curve_spec") != {"kind": "koblitz", "a": 0, "n": 13}:
            raise ValueError(f"unexpected curve spec at seq {rec.get('seq')}")
        curve = workload.get("curve") or {}
        if (curve.get("curve_uid") != EXPECTED_CURVE_UID or curve.get("r") != 2003
                or curve.get("cofactor") != 4 or curve.get("registered") is not True):
            raise ValueError(f"unexpected registered curve at seq {rec.get('seq')}")
        if rec.get("factor_base", {}).get("fb_sha256") != EXPECTED_BASE_SHA:
            raise ValueError(f"unexpected factor base at seq {rec.get('seq')}")
        if rec.get("method", {}).get("params", {}).get("oracle") != ORACLES[arm]:
            raise ValueError(f"unexpected oracle at seq {rec.get('seq')}")
        if not rec.get("binary_sha256"):
            raise ValueError(f"missing binary digest at seq {rec.get('seq')}")
        if rec.get("warmup"):
            key = (target, arm)
            if key in warmup_slots:
                raise ValueError(f"duplicate warmup {key}")
            warmup_slots.add(key)
            outcome = rec.get("outcome") or {}
            if outcome.get("status") != "verified" or outcome.get("matches_target") is not True:
                failures.append({"warmup": key, "reason": "answer not verified", "outcome": outcome})
            continue
        round_id = rec.get("round")
        if round_id not in ROUNDS:
            raise ValueError(f"unexpected measured round {round_id}")
        key = (target, round_id)
        if arm in slots[key]:
            raise ValueError(f"duplicate {key} {arm}")
        slots[key][arm] = rec

    valid_pairs = {}
    for target in TARGETS:
        for round_id in ROUNDS:
            key = (target, round_id)
            group = slots.get(key, {})
            missing = sorted(set(ARMS) - group.keys())
            if missing:
                failures.append({"target_index": target, "round": round_id, "missing": missing})
                continue
            identities = {
                (json.dumps(r["workload"]["target"], sort_keys=True),
                 r["workload"]["workload_sha256"],
                 str(r["algorithm_seed"]), r["binary_sha256"],
                 r["factor_base"]["fb_sha256"])
                for r in group.values()
            }
            if len(identities) != 1:
                failures.append({"target_index": target, "round": round_id,
                                 "reason": "target/workload/algorithm-seed mismatch"})
                continue
            bad = []
            for arm, rec in group.items():
                outcome = rec.get("outcome") or {}
                timing = rec.get("time") or {}
                phases = {p["name"] for p in rec.get("phases") or []}
                if outcome.get("status") != "verified" or outcome.get("matches_target") is not True:
                    bad.append({"arm": arm, "reason": "answer not verified", "outcome": outcome})
                elif rec.get("isolation", {}).get("level") != "L0":
                    bad.append({"arm": arm, "reason": "unexpected isolation level"})
                elif not PHASES <= phases:
                    bad.append({"arm": arm, "reason": "missing phases", "missing": sorted(PHASES - phases)})
                elif not all(isinstance(timing.get(name), int) and timing[name] > 0 for name in
                             ("process_wall_ns", "method_call_wall_ns", "solve_wall_ns")):
                    bad.append({"arm": arm, "reason": "incomplete wall time"})
            if bad:
                failures.append({"target_index": target, "round": round_id, "invalid": bad})
                continue
            valid_pairs[key] = group

    complete = (len(valid_pairs) == len(TARGETS) * len(ROUNDS)
                and len(warmup_slots) == len(TARGETS) * len(ARMS) and not failures)
    output = {"schema": "rootfind-torsion-local-analysis/v1", "scope": "L0 engineering diagnostic",
              "complete": complete, "requested_measured_pairs": 32,
              "valid_measured_pairs": len(valid_pairs), "warmups": len(warmup_slots),
              "failures_or_censoring": failures, "arms": {}, "ratios": {}}
    for arm in ARMS:
        subset = [pair[arm] for pair in valid_pairs.values()]
        output["arms"][arm] = {
            "n": len(subset),
            "process_wall_ns_median": int(statistics.median(r["time"]["process_wall_ns"] for r in subset)) if subset else None,
            "method_call_wall_ns_median": int(statistics.median(r["time"]["method_call_wall_ns"] for r in subset)) if subset else None,
            "outside_method_wall_ns_median": int(statistics.median(
                r["time"]["process_wall_ns"] - r["time"]["method_call_wall_ns"] for r in subset)) if subset else None,
            "total_gae_median": statistics.median(r["cost"]["total_gae"] for r in subset) if subset else None,
        }
    if complete:
        rng = random.Random(BOOTSTRAP_SEED)
        draws = [[rng.choice(tuple(TARGETS)) for _ in TARGETS] for _ in range(RESAMPLES)]
        for candidate, baseline in (("torsion-m3", "harvest-m3"),
                                    ("torsion-m3", "f6-ic"),
                                    ("torsion-m3-aa", "torsion-m3")):
            def ratio(sampled_targets, metric):
                num = sum(valid_pairs[t, rd][candidate]["time"][metric]
                          for t in sampled_targets for rd in ROUNDS)
                den = sum(valid_pairs[t, rd][baseline]["time"][metric]
                          for t in sampled_targets for rd in ROUNDS)
                return num / den
            name = f"{candidate}/{baseline}"
            output["ratios"][name] = {}
            for metric in ("process_wall_ns", "method_call_wall_ns"):
                boot = [ratio(targets, metric) for targets in draws]
                output["ratios"][name][metric] = {
                    "ratio_of_sums": ratio(TARGETS, metric),
                    "target_cluster_95pct_interval": [percentile(boot, 0.025), percentile(boot, 0.975)],
                    "resamples": RESAMPLES, "seed": BOOTSTRAP_SEED,
                }
    return output


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--records", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    records = [json.loads(line) for line in args.records.read_text().splitlines() if line]
    result = analyze(records)
    args.out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"complete": result["complete"], "valid_measured_pairs": result["valid_measured_pairs"],
                      "failures_or_censoring": len(result["failures_or_censoring"])}))


if __name__ == "__main__":
    main()
