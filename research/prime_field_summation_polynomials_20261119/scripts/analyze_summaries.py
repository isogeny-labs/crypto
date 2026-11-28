#!/usr/bin/env python3
"""Aggregate retained per-curve campaign summaries without reading the large corpora."""

from __future__ import annotations

import argparse
import csv
from collections import defaultdict
from pathlib import Path


SUM_FIELDS = (
    "samples",
    "generated_coefficients",
    "finite_roots",
    "infinity_roots",
    "distinct_roots",
    "repeated_samples",
    "degree_drop_samples",
    "zero_coefficients",
    "total_coefficients",
    "even_only_samples",
    "palindromic_samples",
)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("results", type=Path, nargs="?", default=Path("results"))
    args = parser.parse_args()
    files = sorted((args.results / "corpus").glob("*.summary.csv"))
    if not files:
        raise SystemExit("no corpus summaries found")

    grouped: dict[tuple[int, int], dict[str, object]] = defaultdict(
        lambda: {"curves": set(), **{name: 0 for name in SUM_FIELDS}}
    )
    all_curves: set[tuple[int, int, int]] = set()
    for path in files:
        with path.open(newline="") as handle:
            for row in csv.DictReader(handle):
                key = (int(row["p"]), int(row["arity"]))
                all_curves.add((int(row["p"]), int(row["a"]), int(row["b"])))
                item = grouped[key]
                item["curves"].add((int(row["a"]), int(row["b"])))
                item["nominal_degree"] = int(row["nominal_degree"])
                for name in SUM_FIELDS:
                    item[name] += int(row[name])

    output = args.results / "aggregate_summary.csv"
    columns = [
        "p", "arity", "curves", "samples", "nominal_degree", "nominal_roots",
        "finite_roots", "infinity_roots", "distinct_roots", "duplicate_finite_roots",
        "repeated_samples", "repeated_sample_rate", "degree_drop_samples",
        "degree_drop_rate", "zero_coefficients", "nonleading_coefficients",
        "zero_nonleading_rate", "even_only_samples", "palindromic_samples",
    ]
    with output.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=columns)
        writer.writeheader()
        for (p, arity), item in sorted(grouped.items()):
            samples = int(item["samples"])
            nominal_degree = int(item["nominal_degree"])
            finite = int(item["finite_roots"])
            distinct = int(item["distinct_roots"])
            nonleading = int(item["total_coefficients"]) - samples
            writer.writerow({
                "p": p,
                "arity": arity,
                "curves": len(item["curves"]),
                "samples": samples,
                "nominal_degree": nominal_degree,
                "nominal_roots": samples * nominal_degree,
                "finite_roots": finite,
                "infinity_roots": item["infinity_roots"],
                "distinct_roots": distinct,
                "duplicate_finite_roots": finite - distinct,
                "repeated_samples": item["repeated_samples"],
                "repeated_sample_rate": f"{int(item['repeated_samples']) / samples:.9f}",
                "degree_drop_samples": item["degree_drop_samples"],
                "degree_drop_rate": f"{int(item['degree_drop_samples']) / samples:.9f}",
                "zero_coefficients": item["zero_coefficients"],
                "nonleading_coefficients": nonleading,
                "zero_nonleading_rate": f"{int(item['zero_coefficients']) / nonleading:.9f}",
                "even_only_samples": item["even_only_samples"],
                "palindromic_samples": item["palindromic_samples"],
            })
    curves = sorted(curve for curve in all_curves if curve[0] <= 1_000_003)
    point_output = args.results / "point_counts.csv"
    with point_output.open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(("p", "a", "b", "curve_points"))
        for p, a, b in curves:
            count = 1
            for x in range(p):
                value = (x * x % p * x + a * x + b) % p
                count += 1 if value == 0 else 2 if pow(value, (p - 1) // 2, p) == 1 else 0
            writer.writerow((p, a, b, count))
    print(output)
    print(point_output)


if __name__ == "__main__":
    main()
