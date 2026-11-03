#!/usr/bin/env python3
"""Classify source-impossible identity masks before inspecting XCNF shapes."""

import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
from run import HERE, PARENT, require, specs


def status(mask: int) -> str:
    """The pinned circuit rejects O + affine = O at consecutive identity steps."""
    require(0 <= mask < 16, "six-summand identity mask")
    return ("UNSAT_BY_ADJACENT_IDENTITIES" if mask & (mask >> 1)
            else "NO_UNCONDITIONAL_IDENTITY_CONTRADICTION")


def analyze(census: dict, protocol: dict, parent: dict, census_sha256: str) -> dict:
    expected = list(specs(protocol))
    require(census["schema"] == "n83.full-identity-mask-census/v1"
            and census["protocol_sha256"]
            == hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest()
            and census["worker_sha256"]
            == protocol["worker_binary_sha256_darwin_arm64"]
            and census["input_manifest_sha256"] == protocol["input_manifest_sha256"]
            and census["formula_count"] == len(census["rows"]) == len(expected)
            and census["solver_processes"] == 0
            and census["selected_best_total_runtime"] is None,
            "complete source-pinned formula-only census")
    groups = defaultdict(list)
    targets = defaultdict(set)
    counts = defaultdict(int)
    for recipe, row in zip(expected, census["rows"], strict=True):
        require(all(row[key] == value for key, value in recipe.items())
                and len(row["sha256"]) == 64
                and row["bytes"] > 0
                and row["variables"] > 0
                and row["original_clauses"] > 0
                and row["xor_rows"] >= 0,
                "exact ordered nonempty XCNF row")
        prior = parent["formulas"].get(row["name"])
        if prior:
            require(all(row[key] == prior[key] for key in
                        ("sha256", "bytes", "variables", "original_clauses",
                         "xor_rows", "coef_a", "coef_b")),
                    "sixteen prior formula pins")
        state = status(row["identity_mask"])
        counts[state] += 1
        key = (row["trial"], row["pool"], row["base_summands"],
               row["residual_summands"])
        targets[row["trial"]].add((row["coef_a"], row["coef_b"],
                                   json.dumps(row["group_target"], sort_keys=True)))
        if state == "NO_UNCONDITIONAL_IDENTITY_CONTRADICTION":
            groups[key].append(row)
    require(all(len(values) == 1 for values in targets.values())
            and counts == {"UNSAT_BY_ADJACENT_IDENTITIES": 96,
                           "NO_UNCONDITIONAL_IDENTITY_CONTRADICTION": 96}
            and len(groups) == 12 and all(len(rows) == 8 for rows in groups.values()),
            "same trial targets and exact structural mask partition")
    shape_screen = []
    for key, rows in sorted(groups.items()):
        fewest_variables = min(row["variables"] for row in rows)
        fewest_clauses = min(row["original_clauses"] for row in rows)
        fewest_bytes = min(row["bytes"] for row in rows)
        shape_screen.append({
            "trial": key[0], "pool": key[1], "base_summands": key[2],
            "residual_summands": key[3],
            "masks_without_unconditional_identity_contradiction":
                sorted(row["identity_mask"] for row in rows),
            "minimum_variables": fewest_variables,
            "minimum_variables_masks": sorted(row["identity_mask"] for row in rows
                                              if row["variables"] == fewest_variables),
            "minimum_clauses": fewest_clauses,
            "minimum_clauses_masks": sorted(row["identity_mask"] for row in rows
                                            if row["original_clauses"] == fewest_clauses),
            "minimum_bytes": fewest_bytes,
            "minimum_bytes_masks": sorted(row["identity_mask"] for row in rows
                                          if row["bytes"] == fewest_bytes),
        })
    return {"schema": "n83.full-identity-mask-structural-screen/v1",
            "census_sha256": census_sha256,
            "structural_rule": "adjacent identity bits force O + affine = O in pinned ChainedS3Encoding::build",
            "counts": dict(sorted(counts.items())),
            "shape_screen": shape_screen,
            "relation_search_executed": False,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--census", type=Path, default=HERE / "census.json")
    args = parser.parse_args()
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    parent = json.loads(PARENT.read_bytes())
    census = json.loads(args.census.read_bytes())
    result = analyze(census, protocol, parent,
                     hashlib.sha256(args.census.read_bytes()).hexdigest())
    print(json.dumps(result, indent=2, sort_keys=True))
