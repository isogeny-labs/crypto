#!/usr/bin/env python3
"""Lay out recorded summary fields; this script performs no scientific analysis."""
import json
import sys
from pathlib import Path

def render(summary):
    status = summary.get("comparison_status", "unqualified historical summary")
    lines = ["# Recorded study inventory", "", f"Comparative status: **{status}**.", "",
             "All numerical observations remain in summary.json. Row validity is separate from control acceptance and complete accounting.", ""]
    lines += ["- " + reason for reason in summary.get("comparison_blockers", ["No explicit comparison eligibility metadata was recorded."])]
    lines += ["", "| p | dense IC rows | sparse IC rows | R3 rows | comparison status |",
              "|--:|--:|--:|--:|:--|"]
    for row in summary.get("sizes", []):
        rho = row.get("rho", {}).get("R3", {})
        def count(key, fallback):
            return row.get(key, row.get(fallback, {}).get("n", "unknown"))
        lines.append(f"| {row['p']} | {count('ic_dense_valid_rows', 'ic_S')} | {count('ic_sparse_valid_rows', 'ic_S')} | {rho.get('S_harness', {}).get('n', 'unknown')} | {row.get('comparison_status', 'unqualified')} |")
    lines += ["", f"K1: {summary.get('K1_status', {}).get('status', 'unqualified')}. "
              f"K2: {summary.get('K2_status', {}).get('status', 'unqualified')}. "
              f"Exponent qualification: {summary.get('exponent_status', 'unqualified')}.", "",
              "Historical S+ and native-price-band observations predate the hash-counting correction; consult the review qualification before using them."]
    return "\n".join(lines) + "\n"

if __name__ == "__main__":
    directory = Path(sys.argv[1])
    if directory.resolve() == Path(__file__).resolve().parent:
        raise SystemExit("Original TABLES.md is archival; use review-20261020 or a separate output directory.")
    (directory / "TABLES.md").write_text(render(json.loads((directory / "summary.json").read_text())))
    print("wrote", directory / "TABLES.md")
