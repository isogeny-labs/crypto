#!/usr/bin/env python3
"""Replay both exact XCNF censuses and the mathematical exclusion screens."""

import argparse
import hashlib
import json
from pathlib import Path

import analyze
import domain_screen
import pair_screen
import support_screen
from run import HERE, MANIFEST, PARENT, require


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compare_rows(first: dict, second: dict) -> None:
    require(first["schema"] == second["schema"] == "n83.full-identity-mask-census/v1"
            and first["formula_count"] == second["formula_count"] == 192
            and first["protocol_sha256"] == second["protocol_sha256"]
            and first["worker_sha256"] == second["worker_sha256"]
            and first["input_manifest_sha256"] == second["input_manifest_sha256"]
            and first["solver_processes"] == second["solver_processes"] == 0
            and first["selected_best_total_runtime"] is None
            and second["selected_best_total_runtime"] is None,
            "same complete formula-only runs")
    for index, (left, right) in enumerate(zip(first["rows"], second["rows"], strict=True)):
        require(left.keys() == right.keys(), f"row {index}: same fields")
        require({key: value for key, value in left.items()
                 if key != "export_wall_seconds_L0"}
                == {key: value for key, value in right.items()
                    if key != "export_wall_seconds_L0"},
                f"row {index}: exact deterministic export replay")
        require(left["export_wall_seconds_L0"] > 0
                and right["export_wall_seconds_L0"] > 0,
                f"row {index}: positive export charge")
    for run in (first, second):
        require(abs(run["summed_export_wall_seconds_L0"]
                    - sum(row["export_wall_seconds_L0"] for row in run["rows"])) < 1e-6
                and run["elapsed_panel_wall_seconds_L0"]
                >= run["summed_export_wall_seconds_L0"],
                "complete local construction wall accounting")


def audit(inputs: Path) -> dict:
    first_path = HERE / "census.json"
    second_path = HERE / "census-repeat.json"
    first, second = (json.loads(path.read_bytes()) for path in (first_path, second_path))
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    parent = json.loads(PARENT.read_bytes())
    manifest = json.loads(MANIFEST.read_bytes())
    compare_rows(first, second)
    first_sha = sha(first_path)
    second_sha = sha(second_path)
    structural = analyze.analyze(first, protocol, parent, first_sha)
    require(structural == json.loads((HERE / "analysis.json").read_bytes()),
            "recomputed source-identity screen")
    domain = domain_screen.screen(first, protocol, parent, manifest, inputs, first_sha)
    require(domain == json.loads((HERE / "domain-screen.json").read_bytes()),
            "recomputed exact target-x pool exclusion")
    pair = pair_screen.screen(first, protocol, parent, manifest, inputs, first_sha)
    require(pair == json.loads((HERE / "pair-screen.json").read_bytes())
            and all(row["base_plus_residual_ordered_pair_hits"] == 0
                    and row["residual_plus_residual_ordered_pair_hits"] == 0
                    for row in pair["results"]),
            "recomputed exact two-summand exclusion")
    support = support_screen.screen(pair, sha(HERE / "pair-screen.json"))
    require(support == json.loads((HERE / "support-screen.json").read_bytes())
            and len(support["rows"]) == 36,
            "recomputed complete unexcluded-mask support ceilings")
    return {"schema": "n83.full-identity-mask-census-replay/v1",
            "status": "PASS_TWO_COMPLETE_EXPORTS_AND_GROUP_SCREENS",
            "first_census_sha256": first_sha,
            "repeat_census_sha256": second_sha,
            "identical_deterministic_formula_rows": 192,
            "first_export_wall_seconds_L0": first["summed_export_wall_seconds_L0"],
            "repeat_export_wall_seconds_L0": second["summed_export_wall_seconds_L0"],
            "structural_counts": structural["counts"],
            "domain_counts": domain["counts"],
            "zero_pair_gate_trials_and_pools": len(pair["results"]),
            "support_ceiling_rows": len(support["rows"]),
            "unexcluded_masks_per_trial_pool_split": [0, 1, 2],
            "relation_search_executed": False,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--inputs", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.inputs), indent=2, sort_keys=True))
