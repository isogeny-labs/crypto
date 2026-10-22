#!/usr/bin/env python3
"""Fresh eight-point single-target IC/rho holdout gate."""

import argparse
import json
import math
import statistics
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import select as screen_selector

SOURCE_SHA256 = screen_selector.SOURCE_SHA256
TARGETS_SHA256 = "e7dcb0fe5779f847d381017b3cb5fb7dd46b6a4a7e834611d200d7cbd6244f68"
T_975_DF7 = 2.3646242510103


def unknown(reason, candidate=None):
    return {"schema": "n83.online-distribution-holdout-selection/v1",
            "admission_status": reason, "status": "BOUNDED_UNKNOWN",
            "screen_selected_candidate_id": candidate,
            "verified_speedup_candidate_id": None}


def finite_positive(value):
    return (isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and value > 0)


def select(screen, data):
    candidate = screen.get("selected_candidate_id")
    if (screen.get("schema") !=
            "n83.online-distribution-screen-selection/v1" or
            screen.get("admission_status") != "PASS" or
            screen.get("status") != "SELECTED" or
            candidate not in screen_selector.IDS):
        return unknown("SCREEN_NOT_SELECTED")
    if (data.get("schema") != "n83.online-distribution-holdout-validated/v1" or
            data.get("source_sha256") != SOURCE_SHA256 or
            data.get("targets_sha256") != TARGETS_SHA256 or
            data.get("candidate_id") != candidate or
            data.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            data.get("single_cloud_instance") is not True or
            data.get("same_candidate_base") is not True):
        return unknown("SOURCE_TARGET_OR_REPLAY_GATE", candidate)
    hosts = data.get("host_strata")
    if (not isinstance(hosts, list) or len(hosts) != 1 or
            hosts[0].get("point_indices") != list(range(8)) or
            hosts[0].get("tier_status") != "PASS_TIER1" or
            hosts[0].get("aa_status") != "PASS_20_PERCENT_PER_PAIR"):
        return unknown("HOST_OR_CALIBRATION_GATE", candidate)
    points = data.get("points")
    if (not isinstance(points, list) or len(points) != 8 or
            {row.get("point_index") for row in points} != set(range(8))):
        return unknown("POINT_COVERAGE_GATE", candidate)
    ratios, differences, cold = [], [], []
    for point in sorted(points, key=lambda row: row["point_index"]):
        ic, rho, cal = (point.get(key) for key in
                        ("ic", "rho", "calibration"))
        if (not isinstance(ic, dict) or not isinstance(rho, dict) or
                not isinstance(cal, dict) or
                ic.get("candidate_id") != candidate or
                ic.get("status") != "PASS_INDEPENDENT_REPLAY" or
                ic.get("resource_status") != "PASS" or
                rho.get("status") != "PASS_INDEPENDENT_REPLAY" or
                rho.get("resource_status") != "PASS" or
                cal.get("status") != "PASS_20_PERCENT" or
                not finite_positive(ic.get("one_target_online_ms")) or
                not finite_positive(ic.get("cold_process_ms")) or
                not finite_positive(rho.get("online_ms"))):
            return unknown("PAIR_REPLAY_RESOURCE_OR_TIMING_GATE", candidate)
        measured, reference = ic["one_target_online_ms"], rho["online_ms"]
        ratios.append(reference / measured)
        differences.append(reference - measured)
        cold.append(ic["cold_process_ms"])
    mean = statistics.mean(differences)
    lower = mean - T_975_DF7 * statistics.stdev(differences) / math.sqrt(8)
    passed = all(value > 0 for value in differences) and lower > 0
    return {
        "schema": "n83.online-distribution-holdout-selection/v1",
        "admission_status": "PASS",
        "status": "PASS_ALL_EIGHT_SAME_POINT" if passed else "BOUNDED_UNKNOWN",
        "screen_selected_candidate_id": candidate,
        "verified_speedup_candidate_id": candidate if passed else None,
        "point_count": 8, "paired_rho_over_ic_online": ratios,
        "median_paired_ratio": statistics.median(ratios),
        "mean_paired_rho_minus_ic_ms": mean,
        "lower_95_paired_difference_ms": lower,
        "mean_cold_process_ms": statistics.mean(cold),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("screen_selection", type=Path)
    parser.add_argument("holdout_validated", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = select(json.loads(args.screen_selection.read_text()),
                    json.loads(args.holdout_validated.read_text()))
    blob = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output:
        if args.output.exists() and args.output.read_text() != blob:
            raise ValueError("sealed holdout decision differs")
        args.output.write_text(blob)
    else:
        print(blob, end="")


if __name__ == "__main__":
    main()
