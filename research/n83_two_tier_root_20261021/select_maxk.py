#!/usr/bin/env python3
"""Select one N83 factor base from a fully replayed, paired Tier 1 screen."""

import argparse
import json
import math
import re
import statistics
from pathlib import Path


TARGETS_SHA256 = "b99b196b43e69414c101e948ca74ea32703d1ea87ad71e0109e01b37f8cafe38"
SOURCE_SHA256 = "c125b638d89e418936895659f1b3bd856b4e18a6a24f1d307dec99a204833cf0"
LABELS = ("k4300", "k4405", "k5086")
POINT_COUNT = 192
ALPHA_PER_PAIR = 0.05 / 3
ID_PATTERN = re.compile(r"^IC1N83Ckb1a1fb[1-9][0-9]*PDP4rootRCguidedLAgaussTDdirectISO0h[0-9a-f]{12}$")


def exact_two_sided_sign_p(wins, losses):
    """Two-sided binomial sign test after removing ties."""
    n = wins + losses
    if n == 0:
        return 1.0
    tail = sum(math.comb(n, j) for j in range(min(wins, losses) + 1))
    return min(1.0, 2 * tail / (1 << n))


def admitted_matrix(data):
    if (data.get("schema") != "n83.maxk-screen-validated/v1" or
            data.get("source_sha256") != SOURCE_SHA256 or
            data.get("targets_sha256") != TARGETS_SHA256 or
            data.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            data.get("tier_status") != "PASS_TIER1" or
            data.get("stage_gate_status") != "PASS_EXACT_BASE_AND_INDEX" or
            data.get("single_pod") is not True):
        return None, "SOURCE_REPLAY_OR_TIER_GATE"
    candidates = data.get("candidate_ids")
    if (not isinstance(candidates, dict) or set(candidates) != set(LABELS) or
            len(set(candidates.values())) != 3 or
            any(not isinstance(value, str) or not ID_PATTERN.fullmatch(value)
                for value in candidates.values())):
        return None, "CANDIDATE_ID_GATE"
    digests = data.get("candidate_manifest_sha256")
    if (not isinstance(digests, dict) or set(digests) != set(LABELS) or
            any(not isinstance(value, str) or
                    not re.fullmatch(r"[0-9a-f]{64}", value)
                    for value in digests.values())):
        return None, "CANDIDATE_MANIFEST_GATE"
    expected_orders = [list(LABELS), list(reversed(LABELS))]
    if data.get("block_candidate_orders") != expected_orders:
        return None, "ORDER_GATE"
    points = data.get("points")
    if (not isinstance(points, list) or len(points) != POINT_COUNT or
            {point.get("point_index") for point in points
             if isinstance(point, dict)} != set(range(POINT_COUNT))):
        return None, "POINT_COVERAGE_GATE"
    matrix = []
    for point in sorted(points, key=lambda row: row["point_index"]):
        if (point.get("block") != point["point_index"] // 96 or
                point.get("resource_status") != "PASS" or
                point.get("aa_status") != "PASS" or
                not isinstance(point.get("arms"), dict) or
                set(point["arms"]) != set(LABELS)):
            return None, "POINT_RESOURCE_OR_ARM_GATE"
        row = {}
        for label in LABELS:
            arm = point["arms"][label]
            if (not isinstance(arm, dict) or
                    arm.get("candidate_id") != candidates[label] or
                    arm.get("status") != "PASS_INDEPENDENT_REPLAY" or
                    arm.get("group_verified") is not True or
                    arm.get("resource_status") != "PASS" or
                    not isinstance(arm.get("online_ms"), (int, float)) or
                    isinstance(arm["online_ms"], bool) or
                    not math.isfinite(arm["online_ms"]) or
                    arm["online_ms"] <= 0):
                return None, "IC_REPLAY_OR_TIMING_GATE"
            row[label] = arm["online_ms"]
        matrix.append(row)
    return matrix, "PASS"


def select(data):
    matrix, gate = admitted_matrix(data)
    result = {"schema": "n83.maxk-screen-selection/v1",
              "admission_status": gate,
              "status": "BOUNDED_UNKNOWN",
              "selected_candidate_id": None}
    if matrix is None:
        return result
    times = {label: [row[label] for row in matrix] for label in LABELS}
    geometric = {label: statistics.geometric_mean(values)
                 for label, values in times.items()}
    comparisons = []
    for i, first in enumerate(LABELS):
        for second in LABELS[i + 1:]:
            wins = sum(row[first] < row[second] for row in matrix)
            losses = sum(row[second] < row[first] for row in matrix)
            comparisons.append({"first": first, "second": second,
                                "first_wins": wins, "second_wins": losses,
                                "ties": POINT_COUNT - wins - losses,
                                "two_sided_exact_sign_p":
                                exact_two_sided_sign_p(wins, losses)})
    fastest = min(LABELS, key=geometric.__getitem__)
    if all((row["first_wins"] > row["second_wins"]
            if row["first"] == fastest else
            row["second_wins"] > row["first_wins"]) and
           row["two_sided_exact_sign_p"] < ALPHA_PER_PAIR
           for row in comparisons if fastest in (row["first"], row["second"])):
        result["status"] = "SELECTED"
        result["selected_candidate_id"] = data["candidate_ids"][fastest]
    result.update({"candidate_ids": data["candidate_ids"],
                   "point_count": POINT_COUNT,
                   "descriptive_fastest_label": fastest,
                   "geometric_mean_online_ms": geometric,
                   "bonferroni_alpha_per_pair": ALPHA_PER_PAIR,
                   "comparisons": comparisons})
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("validated", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = select(json.loads(args.validated.read_text()))
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        if args.output.exists() and args.output.read_text() != blob:
            raise ValueError("selection output differs from frozen replay")
        args.output.write_text(blob)
    else:
        print(blob, end="")


if __name__ == "__main__":
    main()
