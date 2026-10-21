#!/usr/bin/env python3
"""Post hoc equal-column comparison, including the orbit-distinct flat control.

Flat B=12 and folded B=24 each have 12 unknown logarithms.  This audit
recomputes its table from raw native records and checks target pairing.  The
orbit-distinct control selects one representative from each of the same 12
orbits used by folded B=24; paired-flat B=12 is retained as a sensitivity.
"""

import csv
import json
from collections import defaultdict
from pathlib import Path
from statistics import median


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "gls_symmetry_results_v2.ndjson"
CONTROL = ROOT / "gls_distinct_control_v1.ndjson"
OUT = ROOT / "gls_equal_columns_v2.csv"


def extra_ops(row):
    return sum(
        row[key]
        for key in (
            "f2_inv",
            "probes",
            "mod_inv",
            "psi_eval",
            "row_ops",
            "mod_square_tests",
        )
    )


def fmt_ratio(numerator, denominator):
    return f"{numerator / denominator:.6f}"


def main():
    census = {}
    solves = defaultdict(dict)
    for line in RAW.read_text().splitlines():
        row = json.loads(line)
        if row["kind"] == "census":
            key = row["p"], row["b"]
            assert key not in census
            census[key] = row
        elif row["kind"] == "solve":
            key = row["p"], row["b"], row["arm"]
            assert row["seed"] not in solves[key]
            solves[key][row["seed"]] = row
    for line in CONTROL.read_text().splitlines():
        row = json.loads(line)
        if row["kind"] == "distinct_census":
            key = row["p"], "distinct"
            assert key not in census
            census[key] = row
        elif row["kind"] == "distinct_solve":
            key = row["p"], "distinct"
            assert row["seed"] not in solves[key]
            solves[key][row["seed"]] = row
        else:
            raise AssertionError(f"unexpected control record: {row['kind']}")

    fields = (
        "p", "flat_policy", "nonzero_targets", "unknown_columns", "flat_physical_b", "fold_physical_b",
        "flat_covered", "fold_covered", "coverage_ratio",
        "flat_raw_hits", "fold_raw_hits", "raw_hit_ratio",
        "flat_queries", "fold_queries", "query_ratio",
        "flat_median_f2_mul", "fold_median_f2_mul", "f2_mul_ratio",
        "flat_median_group_add", "fold_median_group_add", "group_add_ratio",
        "cost_ratio_w0p1", "cost_ratio_w1",
        "flat_median_ns", "fold_median_ns", "wall_ratio_l0",
    )
    output = []
    for p in (101, 211, 307):
        fold_c = census[p, 24]
        fold = solves[p, 24, "psi-folded"]
        for policy in ("distinct", "paired"):
            flat_c = census[p, "distinct"] if policy == "distinct" else census[p, 12]
            flat = solves[p, "distinct"] if policy == "distinct" else solves[p, 12, "signed-flat"]
            assert (flat_c["columns"] if policy == "distinct" else flat_c["flat_columns"]) == 12
            assert (flat_c["rank"] if policy == "distinct" else flat_c["flat_rank"]) == 12
            assert fold_c["folded_columns"] == fold_c["folded_rank"] == 12
            assert flat_c["targets"] == fold_c["targets"]
            assert set(flat) == set(fold) == {0, 1, 2}
            for seed in flat:
                assert flat[seed]["target"] == fold[seed]["target"]
                assert flat[seed]["recovered"] == fold[seed]["recovered"]
                assert flat[seed]["correct"] is fold[seed]["correct"] is True
                assert flat[seed]["columns"] == fold[seed]["columns"] == 12
                assert flat[seed]["rank"] == fold[seed]["rank"] == 12

            f = list(flat.values())
            g = list(fold.values())
            metric = lambda rows, key: median(row[key] for row in rows)
            fm, gm = metric(f, "f2_mul"), metric(g, "f2_mul")
            fa, ga = metric(f, "group_add"), metric(g, "group_add")
            fq, gq = metric(f, "trials"), metric(g, "trials")
            ft, gt = metric(f, "total_ns"), metric(g, "total_ns")
            weighted = lambda rows, weight: median(
                row["f2_mul"] + weight * extra_ops(row) for row in rows
            )
            output.append(dict(zip(fields, (
                p, policy, flat_c["targets"], 12, 12, 24,
                flat_c["covered"], fold_c["covered"],
                fmt_ratio(fold_c["covered"], flat_c["covered"]),
                flat_c["raw_hits"], fold_c["raw_hits"],
                fmt_ratio(fold_c["raw_hits"], flat_c["raw_hits"]),
                fq, gq, fmt_ratio(fq, gq),
                fm, gm, fmt_ratio(fm, gm),
                fa, ga, fmt_ratio(fa, ga),
                fmt_ratio(weighted(f, 0.1), weighted(g, 0.1)),
                fmt_ratio(weighted(f, 1), weighted(g, 1)),
                ft, gt, fmt_ratio(ft, gt),
            ))))

    with OUT.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(output)
    print(f"audited {len(output)} equal-column cells: {OUT.name}")


if __name__ == "__main__":
    main()
