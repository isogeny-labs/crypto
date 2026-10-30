#!/usr/bin/env python3
"""Replay the priced pencil cohort against frozen solve and control receipts."""

import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def rows(name):
    return [json.loads(line) for line in (ROOT / name).open()]


def audit():
    frozen = rows("target_labelled_pencil_v1.jsonl")
    targets = {x["target_index"]: x for x in frozen
               if x["schema"] == "pencil-target-summary/v1"}
    priced = rows("priced_pencil_v1.jsonl")
    setup, runs = priced[0], priced[1:]
    scan = rows("priced_pencil_scan_control_v1.jsonl")
    scan_runs = scan[1:]
    assert setup["schema"] == "pencil-priced-setup/v1"
    assert (setup["p"], setup["r"], setup["rational_curve_points"]) == (739, 34147, 739)
    assert len(runs) == len(scan_runs) == 60
    assert scan[0]["schema"] == setup["schema"]
    assert scan[0]["field"] == setup["field"]
    assert scan[0]["arithmetic"] == setup["arithmetic"]
    by_key = {}
    for x in runs:
        key = (x["round"], x["target_index"])
        assert x["schema"] == "pencil-priced-target/v1"
        assert x["candidate_root_mode"] == "witness_bucket"
        assert key not in by_key and 0 <= key[0] < 3 and 0 <= key[1] < 20
        by_key[key] = x
        frozen_target = targets[key[1]]
        assert x["scalar"] == x["recovered"] == frozen_target["recovered"]
        witness = frozen_target["solve_witness"]
        assert x["solution_key"] == [witness["walk_index"], witness["lambda"]]
        assert x["walks"] == witness["walk_index"] + 1
        assert x["verified_relations"] == frozen_target["solver_inserted_relations"]
        assert x["solver_rank"] == frozen_target["solver_rank"]
        assert x["solver_row_updates"] == frozen_target["solver_row_updates"]
        assert x["solver_inversions"] == frozen_target["solver_inversions"]
        assert x["candidate_lambdas"] >= x["verified_relations"] + x["rejected_candidates"]
        phases = x["phases"]
        assert set(phases) == {"walk_generation", "sieve", "candidate_verification",
                               "elimination", "recovery_check"}
        assert sum(p["elapsed_ns"] for p in phases.values()) <= x["online_elapsed_ns"]
        for category in ("field", "arithmetic"):
            for op, total in x[category].items():
                assert total == sum(phase[category][op] for phase in phases.values())
    for index in range(20):
        reference = by_key[0, index]
        for round_id in (1, 2):
            current = by_key[round_id, index]
            for key in ("walks", "solution_key", "candidate_lambdas", "rejected_candidates",
                        "verified_relations", "solver_row_updates", "field", "arithmetic"):
                assert current[key] == reference[key]
    scan_base_muls = witness_base_muls = 0
    for x in scan_runs:
        key = (x["round"], x["target_index"])
        reference = by_key[key]
        assert x["candidate_root_mode"] == "full_scan"
        for field in ("scalar", "recovered", "walks", "solution_key", "candidate_lambdas",
                      "rejected_candidates", "verified_relations", "solver_rank",
                      "solver_row_updates", "solver_inversions"):
            assert x[field] == reference[field]
        for category in ("field", "arithmetic"):
            for op, total in x[category].items():
                assert total == sum(phase[category][op] for phase in x["phases"].values())
        if x["round"] == 0:
            scan_base_muls += x["arithmetic"]["base_muls"]
            witness_base_muls += reference["arithmetic"]["base_muls"]
    assert witness_base_muls < scan_base_muls

    controls = rows("paired_price_controls_v1.jsonl")
    assert len(controls) == 120
    rho = {x["target_index"]: x for x in rows("source_rho_controls_v1.jsonl")}
    two_point = {x["target_index"]: x for x in rows("two_point_controls_v1.jsonl")}
    by_method = {"pencil": [x["online_elapsed_ns"] for x in runs],
                 "rho": [], "two_point": []}
    seen = set()
    for x in controls:
        key = (x["round"], x["target_index"], x["method"])
        assert x["schema"] == "pencil-paired-control/v1" and key not in seen
        seen.add(key)
        reference = (rho if x["method"] == "rho" else two_point)[x["target_index"]]
        assert x["solver_counts"] == reference["counts"]
        assert x["native"] == reference["native"]
        assert x["recovered"] == x["scalar"] == reference["scalar"]
        assert x["verification_counts"]["ec_adds"] > 0
        by_method[x["method"]].append(x["elapsed_ns"])
    assert len(seen) == 120
    assert all(len(times) == 60 for times in by_method.values())
    return {
        "schema": "pencil-priced-audit/v1",
        "target_runs_per_method": 60,
        "targets": 20,
        "rounds": 3,
        "all_scalars_and_operation_counts_replayed": True,
        "pencil_verified_relations_per_round": sum(x["verified_relations"] for x in runs
                                                     if x["round"] == 0),
        "pencil_rejected_candidates_per_round": sum(x["rejected_candidates"] for x in runs
                                                     if x["round"] == 0),
        "scan_base_multiplications_per_round": scan_base_muls,
        "witness_base_multiplications_per_round": witness_base_muls,
        "scan_median_elapsed_ns_diagnostic": statistics.median(
            x["online_elapsed_ns"] for x in scan_runs),
        "median_elapsed_ns_diagnostic": {
            method: statistics.median(times) for method, times in by_method.items()
        },
    }


if __name__ == "__main__":
    print(json.dumps(audit(), sort_keys=True))
