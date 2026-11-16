#!/usr/bin/env python3
"""Recompute the bounded evidence summary from committed JSONL records."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def records(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def data_rows(path: Path) -> list[dict[str, Any]]:
    return [row for row in records(path) if "summary" not in row]


def phase_native(row: dict[str, Any], name: str) -> dict[str, Any]:
    return next(phase["native"] for phase in row["phases"] if phase["name"] == name)


def summarize_pkm() -> dict[str, Any]:
    paths = {
        8: ROOT / "research/pkm_tower_round4_20260926/runs/T4-kummer-m4-p1-N8.jsonl",
        12: ROOT / "research/pkm_tower_round4_20260926/runs/T4-kummer-m4-p1-N12.jsonl",
        16: ROOT / "research/pkm_tower_round3_20260925/runs/M4b-kummer-m4-p1-N16.jsonl",
    }
    cells = []
    for n, path in paths.items():
        rows = data_rows(path)
        cells.append(
            {
                "N": n,
                "rows": len(rows),
                "completed": sum(not row.get("timed_out", False) and not row.get("oversize", False) for row in rows),
                "degree_min": min(row["solving_degree_max"] for row in rows),
                "degree_max": max(row["solving_degree_max"] for row in rows),
                "max_cols": max(row["max_cols"] for row in rows),
                "max_nnz": max(row["max_nnz"] for row in rows),
                "max_muladds": max(row["muladds"] for row in rows),
                "zero_dim_true": sum(row.get("zero_dim") is True for row in rows),
                "inconsistent_true": sum(row.get("inconsistent") is True for row in rows),
            }
        )

    planted_rows = data_rows(ROOT / "research/pkm_tower_pilot_20260924/runs/G2b-m4-tower-planted-stop64.jsonl")
    unique: dict[tuple[int, int], dict[str, Any]] = {}
    for row in planted_rows:
        key = (row["N"], row["target_index"])
        unique.setdefault(key, row)
    planted = []
    for n in (8, 12):
        rows = [row for (cell_n, _), row in unique.items() if cell_n == n]
        planted.append(
            {
                "N": n,
                "targets": len(rows),
                "planted_ok": sum(row.get("planted_ok") is True for row in rows),
                "zero_dim_true": sum(row.get("zero_dim") is True for row in rows),
                "inconsistent_true": sum(row.get("inconsistent") is True for row in rows),
                "basis_len": max(row["basis_len"] for row in rows),
            }
        )
    return {
        "cells": cells,
        "planted_geometry": planted,
        "interpretation": "random rows certify unit-ideal refutations; planted rows are the bounded positive geometry controls",
    }


def summarize_yield_rank() -> dict[str, Any]:
    path = ROOT / "research/ecbench_yield_sweep_20261004/sessions/prime/records.jsonl"
    selected = [
        row
        for row in records(path)
        if row["method"]["family"] == "ic" and row["arm"].endswith("-subtract")
    ]
    groups: dict[tuple[int, int], list[dict[str, Any]]] = defaultdict(list)
    for row in selected:
        groups[(row["workload"]["curve"]["field_bits"], row["factor_base"]["abscissae"])].append(row)
    cells = []
    for (bits, base), rows in sorted(groups.items()):
        trials = sum(row["counters"]["targets_tried"] for row in rows)
        hits = sum(row["counters"]["relations_found"] for row in rows)
        matrix_rows = sum(phase_native(row, "linear_algebra")["rows"] for row in rows)
        rank = sum(row["counters"]["matrix_rank"] for row in rows)
        dependent = sum(row["counters"]["matrix_dependent"] for row in rows)
        group_order = int(rows[0]["workload"]["curve"]["r"])
        observed = hits / trials
        mean = 2 * base * base / group_order
        cells.append(
            {
                "bits": bits,
                "r": group_order,
                "B": base,
                "runs": len(rows),
                "trials": trials,
                "hits": hits,
                "rows": matrix_rows,
                "rank": rank,
                "dependent": dependent,
                "observed_rate": observed,
                "mean_model_rate": mean,
                "ratio_to_mean": observed / mean,
            }
        )
    totals = {
        key: sum(cell[key] for cell in cells)
        for key in ("runs", "trials", "hits", "rows", "rank", "dependent")
    }
    return {
        "selection": "method.family == ic and arm suffix -subtract; MITM duplicates the same relation population and is excluded",
        "cells": cells,
        "totals": totals,
        "limitations": [
            "2B^2/r is a leading hit-rate model here, not an exact identity between mean decomposition count and probability of at least one decomposition",
            "adaptive recovery stopping prevents an unconditional target-count variance estimate",
            "early independent rows do not establish full factor-base rank",
            "this is a generic m=2 control, not evidence for an algebraic m=3 or m=4 oracle",
        ],
    }


def build() -> dict[str, Any]:
    return {
        "schema": "ic-weak-assumptions-evidence/v1",
        "created": "2026-11-02",
        "pkm_m4_random_f4": summarize_pkm(),
        "prime_m2_yield_rank_control": summarize_yield_rank(),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", type=Path)
    args = parser.parse_args()
    actual = build()
    if args.check:
        expected = json.loads(args.check.read_text())
        if actual != expected:
            print("evidence summary mismatch")
            return 1
        print("evidence summary matches committed records")
    else:
        print(json.dumps(actual, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
