#!/usr/bin/env python3
"""Apply the frozen pool-128 rank-throughput decision to an ecbench session."""

from __future__ import annotations

import argparse
import json
import math
import random
from pathlib import Path


BASELINE = "tw2cj-lazy-sib8-p64"
CANDIDATE = "tw2cj-lazy-sib8-p128"
CONTROL = "tw2cj-lazy-sib8-p64-aa"
ARMS = (BASELINE, CANDIDATE, CONTROL)
EXPECTED_PER_ARM = 96
BOOTSTRAPS = 20_000
BOOTSTRAP_SEED = 20_261_125


def quantile(values: list[float], q: float) -> float:
    ordered = sorted(values)
    pos = (len(ordered) - 1) * q
    lo = math.floor(pos)
    hi = math.ceil(pos)
    if lo == hi:
        return ordered[lo]
    return ordered[lo] * (hi - pos) + ordered[hi] * (pos - lo)


def rank_and_ns(record: dict) -> tuple[int, int]:
    return int(record["counters"]["matrix_rank"]), int(record["time"]["solve_wall_ns"])


def ratio(left: list[dict], right: list[dict]) -> float:
    left_rank, left_ns = map(sum, zip(*(rank_and_ns(r) for r in left)))
    right_rank, right_ns = map(sum, zip(*(rank_and_ns(r) for r in right)))
    return (left_rank / left_ns) / (right_rank / right_ns)


def paired_stats(left: list[dict], right: list[dict]) -> dict:
    def key(record: dict) -> tuple[int, str]:
        return int(record["round"]), record["workload"]["workload_id"]

    left_by_key = {key(r): r for r in left}
    right_by_key = {key(r): r for r in right}
    if left_by_key.keys() != right_by_key.keys() or len(left_by_key) != len(left):
        raise ValueError("paired keys are missing or duplicated")
    pairs = [(left_by_key[k], right_by_key[k]) for k in sorted(left_by_key)]
    per_pair = [ratio([a], [b]) for a, b in pairs]
    rng = random.Random(BOOTSTRAP_SEED)
    boot = []
    for _ in range(BOOTSTRAPS):
        sample = [pairs[rng.randrange(len(pairs))] for _ in pairs]
        boot.append(ratio([p[0] for p in sample], [p[1] for p in sample]))
    return {
        "pairs": len(pairs),
        "aggregate_ratio": ratio(left, right),
        "geometric_mean_pair_ratio": math.exp(sum(map(math.log, per_pair)) / len(per_pair)),
        "bootstrap_resamples": BOOTSTRAPS,
        "bootstrap_seed": BOOTSTRAP_SEED,
        "ci95": [quantile(boot, 0.025), quantile(boot, 0.975)],
    }


def analyze(records: list[dict]) -> dict:
    measured = [r for r in records if not r["warmup"]]
    failures: list[str] = []
    grouped = {arm: [r for r in measured if r["arm"] == arm] for arm in ARMS}
    unknown = sorted({r["arm"] for r in measured} - set(ARMS))
    if unknown:
        failures.append("unexpected_arms:" + ",".join(unknown))
    for arm, rows in grouped.items():
        if len(rows) != EXPECTED_PER_ARM:
            failures.append(f"{arm}:measured_count={len(rows)}")
        for row in rows:
            prefix = f"{arm}:seq={row.get('seq')}"
            outcome = row.get("outcome", {})
            counters = row.get("counters", {})
            if outcome.get("status") != "verified" or outcome.get("matches_target") is not True:
                failures.append(prefix + ":semantic_failure")
            if (
                counters.get("matrix_rank") != 3
                or counters.get("matrix_rows") != 3
                or counters.get("matrix_dependent") != 0
            ):
                failures.append(prefix + ":rank_or_dependence_failure")
            if not isinstance(row.get("time", {}).get("solve_wall_ns"), int) or row["time"]["solve_wall_ns"] <= 0:
                failures.append(prefix + ":missing_solve_clock")
            if row.get("isolation", {}).get("level") not in {"L2", "L3"}:
                failures.append(prefix + ":below_L2")

    comparisons = None
    if not failures:
        candidate = paired_stats(grouped[CANDIDATE], grouped[BASELINE])
        aa = paired_stats(grouped[CONTROL], grouped[BASELINE])
        comparisons = {"candidate_over_baseline": candidate, "aa_over_baseline": aa}
        if candidate["aggregate_ratio"] < 1.05:
            failures.append("candidate_effect_below_1.05")
        if candidate["ci95"][0] <= 1.0:
            failures.append("candidate_ci_does_not_exclude_1")
        if not (aa["ci95"][0] <= 1.0 <= aa["ci95"][1]):
            failures.append("aa_ci_excludes_1")
        if abs(aa["geometric_mean_pair_ratio"] - 1.0) > 0.03:
            failures.append("aa_geometric_drift_above_0.03")

    return {
        "schema": "independent-relation-p128-decision/v1",
        "decision": "pass" if not failures else "reject",
        "measured_records": len(measured),
        "measured_per_arm": {arm: len(rows) for arm, rows in grouped.items()},
        "metric": "sum(matrix_rank) / sum(solve_wall_ns)",
        "comparisons": comparisons,
        "failures": failures,
    }


def self_test() -> None:
    def record(arm: str, i: int, ns: int) -> dict:
        return {
            "seq": i,
            "round": i // 16,
            "warmup": False,
            "arm": arm,
            "workload": {"workload_id": f"W{i % 16:02d}"},
            "outcome": {"status": "verified", "matches_target": True},
            "counters": {"matrix_rank": 3, "matrix_rows": 3, "matrix_dependent": 0},
            "time": {"solve_wall_ns": ns},
            "isolation": {"level": "L2"},
        }

    passing = []
    for arm, ns in ((BASELINE, 1100), (CANDIDATE, 1000), (CONTROL, 1100)):
        passing.extend(record(arm, i, ns + (i % 7)) for i in range(EXPECTED_PER_ARM))
    assert analyze(passing)["decision"] == "pass"
    failing = json.loads(json.dumps(passing))
    next(r for r in failing if r["arm"] == CANDIDATE)["counters"]["matrix_rank"] = 2
    decision = analyze(failing)
    assert decision["decision"] == "reject"
    assert any("rank_or_dependence_failure" in f for f in decision["failures"])


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("session", nargs="?", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    if args.session is None or args.out is None:
        parser.error("session and --out are required unless --self-test is used")
    with (args.session / "records.jsonl").open() as source:
        decision = analyze([json.loads(line) for line in source if line.strip()])
    args.out.write_text(json.dumps(decision, indent=2, sort_keys=True) + "\n")
    print(json.dumps(decision, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
