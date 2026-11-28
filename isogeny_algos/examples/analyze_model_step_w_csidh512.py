#!/usr/bin/env python3
"""Recompute the archived native-w CSIDH-512 action comparison."""

import argparse
import csv
import random
import statistics
from collections import defaultdict
from pathlib import Path


DEFAULT_CSV = Path(__file__).resolve().parents[1] / "docs/model_step_w_csidh512_runs_20261122.csv"
CASES = ("small_mixed", "large_mixed", "all_74_mixed")
ARMS = (
    "montgomery_action_fast",
    "native_edwards_w",
    "native_edwards_w_batched",
    "native_edwards_w_tree",
)
DEGREES = (3, 5, 7, 31, 587)


def read_rows(path):
    by_key = {}
    by_process = defaultdict(int)
    with path.open(newline="") as handle:
        reader = csv.DictReader(handle)
        expected = ["process", "field", "task", "degree", "arm", "median_ns", "exact_output_checked"]
        if reader.fieldnames != expected:
            raise ValueError(f"unexpected CSV columns: {reader.fieldnames}")
        for row in reader:
            process = int(row["process"])
            if row["field"] != "CSIDH-512" or row["exact_output_checked"] != "true":
                raise ValueError(f"bad field or unchecked output: {row}")
            ns = int(row["median_ns"])
            if ns <= 0:
                raise ValueError(f"nonpositive timing: {row}")
            key = (process, row["task"], row["degree"], row["arm"])
            if key in by_key:
                raise ValueError(f"duplicate row: {key}")
            by_key[key] = ns
            by_process[process] += 1
    if sorted(by_process) != list(range(1, 16)) or any(count != 32 for count in by_process.values()):
        raise ValueError(f"expected 15 processes and 32 rows each: {dict(by_process)}")
    return by_key


def paired_bootstrap_interval(ratios):
    rng = random.Random(20261122)
    replicates = sorted(statistics.median(rng.choices(ratios, k=len(ratios))) for _ in range(20_000))
    return replicates[500], replicates[19_499]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path, nargs="?", default=DEFAULT_CSV)
    rows = read_rows(parser.parse_args().csv)
    expected = set()
    for process in range(1, 16):
        for case in CASES:
            for arm in ARMS:
                expected.add((process, "complete_action", case, arm))
        for sign in ("positive", "negative"):
            for ell in DEGREES:
                for arm in ("montgomery_step", "native_edwards_w"):
                    expected.add((process, f"seed_sweep_step_{sign}", str(ell), arm))
    if set(rows) != expected:
        raise ValueError(f"missing keys: {len(expected - set(rows))}; extra keys: {len(set(rows) - expected)}")

    print("Complete action: across-process median milliseconds")
    for case in CASES:
        arms = {arm: [rows[(i, "complete_action", case, arm)] for i in range(1, 16)] for arm in ARMS}
        medians = [statistics.median(arms[arm]) / 1e6 for arm in ARMS]
        mont = arms["montgomery_action_fast"]
        tree = arms["native_edwards_w_tree"]
        ratios = [a / b for a, b in zip(mont, tree)]
        wins = sum(a > b for a, b in zip(mont, tree))
        print(f"{case}: " + ", ".join(f"{arm}={median:.3f}" for arm, median in zip(ARMS, medians))
              + f", tree_wins={wins}/15, paired_ratio={statistics.median(ratios):.3f}")
        if case == "all_74_mixed":
            low, high = paired_bootstrap_interval(ratios)
            print(f"all_74_mixed: paired_ratio_bootstrap_95pct=[{low:.3f}, {high:.3f}]")

    print("Step sweep: across-process median milliseconds (Montgomery / native w)")
    for sign in ("positive", "negative"):
        for ell in DEGREES:
            task = f"seed_sweep_step_{sign}"
            mont = statistics.median(rows[(i, task, str(ell), "montgomery_step")] for i in range(1, 16))
            native = statistics.median(rows[(i, task, str(ell), "native_edwards_w")] for i in range(1, 16))
            print(f"{sign} {ell}: {mont / 1e6:.3f} / {native / 1e6:.3f}")


if __name__ == "__main__":
    main()
