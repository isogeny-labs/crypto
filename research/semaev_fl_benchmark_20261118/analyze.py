#!/usr/bin/env python3
"""Analyze paired JSONL emitted by examples/semaev_fl_bench.rs."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import random
from collections import defaultdict
from pathlib import Path


METRICS = (
    "solver_wall_ns",
    "outer_ns",
    "word_xors",
    "word_xors_performed",
    "build_ns",
    "eliminate_ns",
    "peak_matrix_bytes",
    "peak_table_bytes",
)


def geometric_mean(values: list[float]) -> float:
    return math.exp(sum(math.log(value) for value in values) / len(values))


def bootstrap_ci(values: list[float], seed: int, samples: int = 20_000) -> list[float]:
    rng = random.Random(seed)
    draws = []
    for _ in range(samples):
        draws.append(geometric_mean([rng.choice(values) for _ in values]))
    draws.sort()
    return [draws[int(0.025 * samples)], draws[int(0.975 * samples)]]


def aggregate(row: dict) -> dict:
    stats = row["stats"]
    result = dict(row)
    for metric in ("word_xors", "word_xors_performed", "build_ns", "eliminate_ns"):
        result[metric] = sum(stat[metric] for stat in stats)
    for metric in ("peak_matrix_bytes", "peak_table_bytes"):
        result[metric] = max(stat[metric] for stat in stats)
    result["full_m4ri_matrices"] = sum(stat["full_m4ri_matrices"] for stat in stats)
    result["full_m4ri_blocks"] = sum(stat["full_m4ri_blocks"] for stat in stats)
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--warmup-repeats", type=int, default=1)
    args = parser.parse_args()

    raw = args.input.read_bytes()
    rows = [aggregate(json.loads(line)) for line in raw.splitlines() if line.strip()]
    args.out.mkdir(parents=True, exist_ok=True)
    grouped: dict[tuple[str, int], dict[str, dict]] = defaultdict(dict)
    for row in rows:
        grouped[(row["case"], row["repeat"])][row["arm"]] = row

    failures = []
    pairs = []
    for (case, repeat), arms in sorted(grouped.items()):
        if set(arms) != {"fl", "full_width"}:
            failures.append(f"{case} repeat {repeat}: missing matched arm")
            continue
        fl, baseline = arms["fl"], arms["full_width"]
        for field in ("input_sha256", "basis_sha256", "systems", "n_vars"):
            if fl[field] != baseline[field]:
                failures.append(f"{case} repeat {repeat}: {field} differs")
        if fl["timed_out"] or baseline["timed_out"] or fl["oversize"] or baseline["oversize"]:
            failures.append(f"{case} repeat {repeat}: timeout or oversize")
        if fl["fl_matrices"] <= 0 or baseline["fl_matrices"] != 0:
            failures.append(f"{case} repeat {repeat}: FL activation gate failed")
        if repeat >= args.warmup_repeats:
            pairs.append((case, repeat, fl, baseline))

    cases = {}
    csv_rows = []
    for case in sorted({pair[0] for pair in pairs}):
        case_pairs = [pair for pair in pairs if pair[0] == case]
        metrics = {}
        for metric_index, metric in enumerate(METRICS):
            ratios = [fl[metric] / baseline[metric] for _, _, fl, baseline in case_pairs]
            metrics[metric] = {
                "candidate_over_full_width_geomean": geometric_mean(ratios),
                "paired_bootstrap_ci95": bootstrap_ci(
                    ratios,
                    seed=202611180000 + metric_index + 100 * len(cases),
                ),
                "ratios": ratios,
                "candidate_values": [fl[metric] for _, _, fl, _ in case_pairs],
                "full_width_values": [baseline[metric] for _, _, _, baseline in case_pairs],
            }
        first = case_pairs[0]
        cases[case] = {
            "measured_pairs": len(case_pairs),
            "input_sha256": first[2]["input_sha256"],
            "basis_sha256": first[2]["basis_sha256"],
            "fl_matrices_per_run": [fl["fl_matrices"] for _, _, fl, _ in case_pairs],
            "metrics": metrics,
        }
        for _, repeat, fl, baseline in case_pairs:
            for metric in METRICS:
                csv_rows.append(
                    {
                        "case": case,
                        "repeat": repeat,
                        "metric": metric,
                        "fl": fl[metric],
                        "full_width": baseline[metric],
                        "ratio": fl[metric] / baseline[metric],
                    }
                )

    summary = {
        "schema": "semaev.fl.analysis/v1",
        "admission": "local diagnostic; not native-L2 EKS",
        "input": str(args.input),
        "input_sha256": hashlib.sha256(raw).hexdigest(),
        "warmup_repeats_excluded": args.warmup_repeats,
        "correctness_gate": "pass" if not failures else "fail",
        "failures": failures,
        "cases": cases,
    }
    (args.out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    with (args.out / "pairs.csv").open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=csv_rows[0].keys())
        writer.writeheader()
        writer.writerows(csv_rows)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
