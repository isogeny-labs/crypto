#!/usr/bin/env python3
"""Exact, familywise-controlled secondary N83 online screen selector."""

import argparse
import json
import math
import statistics
from pathlib import Path

SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
TARGETS_SHA256 = "0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3"
IDS = (
    "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e",
    "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea",
    "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586",
    "IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77",
    "IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7",
)
POINT_COUNT = 192
BLOCKS = (tuple(range(96)), tuple(range(96, 192)))
ALPHA_PER_PAIR = 0.05 / 10


def finite_positive(value):
    return (isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and value > 0)


def exact_two_sided_sign_p(wins, losses):
    """Exact binomial tail; ties have already been removed."""
    n = wins + losses
    if n == 0:
        return 1.0
    tail = sum(math.comb(n, j) for j in range(min(wins, losses) + 1))
    return min(1.0, 2 * tail / (1 << n))


def admitted_matrix(data):
    if (data.get("schema") != "n83.online-distribution-screen-validated/v1" or
            data.get("source_sha256") != SOURCE_SHA256 or
            data.get("targets_sha256") != TARGETS_SHA256 or
            data.get("candidate_ids") != list(IDS) or
            data.get("screen_mode") != "secondary_shared_setup_per_point_intervals" or
            data.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            data.get("single_cloud_instance") is not True or
            data.get("same_candidate_bases") is not True):
        return None, "SOURCE_IDENTITY_OR_REPLAY_GATE"
    host = data.get("host_strata")
    if (not isinstance(host, list) or len(host) != 1 or
            host[0].get("point_indices") != list(range(POINT_COUNT)) or
            host[0].get("tier_status") != "PASS_TIER1" or
            host[0].get("aa_status") != "PASS_20_PERCENT_PER_CHILD" or
            data.get("block_point_indices") != [list(b) for b in BLOCKS] or
            data.get("block_candidate_orders") !=
            [list(IDS), list(reversed(IDS))]):
        return None, "HOST_ORDER_OR_CALIBRATION_GATE"
    points = data.get("points")
    if (not isinstance(points, list) or len(points) != POINT_COUNT or
            {p.get("point_index") for p in points} !=
            set(range(POINT_COUNT))):
        return None, "POINT_COVERAGE_GATE"
    matrix = []
    for point in sorted(points, key=lambda p: p["point_index"]):
        arms, rho = point.get("arms"), point.get("rho")
        if (not isinstance(arms, list) or len(arms) != len(IDS) or
                not isinstance(rho, dict) or
                rho.get("status") != "PASS_INDEPENDENT_REPLAY" or
                rho.get("resource_status") != "PASS" or
                not finite_positive(rho.get("online_ms"))):
            return None, "PAIR_OR_RHO_GATE"
        by_id = {arm.get("candidate_id"): arm for arm in arms}
        if set(by_id) != set(IDS) or len(by_id) != len(IDS):
            return None, "IC_CANDIDATE_ID_GATE"
        if any(arm.get("status") != "PASS_INDEPENDENT_REPLAY" or
               arm.get("resource_status") != "PASS" or
               not finite_positive(arm.get("one_target_online_ms"))
               for arm in arms):
            return None, "IC_REPLAY_RESOURCE_OR_TIMING_GATE"
        matrix.append(by_id)
    return matrix, "PASS"


def select(data):
    matrix, gate = admitted_matrix(data)
    if matrix is None:
        return {"schema": "n83.online-distribution-screen-selection/v1",
                "admission_status": gate, "status": "BOUNDED_UNKNOWN",
                "selected_candidate_id": None}
    times = {candidate: [row[candidate]["one_target_online_ms"]
                         for row in matrix] for candidate in IDS}
    geometric = {candidate: statistics.geometric_mean(times[candidate])
                 for candidate in IDS}
    medians = {candidate: statistics.median(times[candidate])
               for candidate in IDS}
    comparisons = []
    for i, first in enumerate(IDS):
        for second in IDS[i + 1:]:
            paired = list(zip(times[first], times[second]))
            first_wins = sum(a < b for a, b in paired)
            second_wins = sum(b < a for a, b in paired)
            ties = POINT_COUNT - first_wins - second_wins
            logs = [math.log(b / a) for a, b in paired]
            comparisons.append({
                "first": first, "second": second,
                "first_wins": first_wins, "second_wins": second_wins,
                "ties": ties,
                "two_sided_exact_sign_p": exact_two_sided_sign_p(
                    first_wins, second_wins),
                "mean_log_second_over_first": statistics.mean(logs),
                "median_log_second_over_first": statistics.median(logs),
                "sample_sd_log_difference": statistics.stdev(logs),
            })
    winners = [candidate for candidate in IDS if
               candidate == min(IDS, key=geometric.__getitem__) and all(
                   ((row["first"] == candidate and
                     row["first_wins"] > row["second_wins"]) or
                    (row["second"] == candidate and
                     row["second_wins"] > row["first_wins"])) and
                   row["two_sided_exact_sign_p"] < ALPHA_PER_PAIR
                   for row in comparisons
                   if candidate in (row["first"], row["second"]))]
    return {
        "schema": "n83.online-distribution-screen-selection/v1",
        "admission_status": gate,
        "status": "SELECTED" if len(winners) == 1 else "BOUNDED_UNKNOWN",
        "selected_candidate_id": winners[0] if len(winners) == 1 else None,
        "candidate_ids": list(IDS), "point_count": POINT_COUNT,
        "screen_mode": "secondary_shared_setup_per_point_intervals",
        "descriptive_fastest_candidate_id": min(IDS, key=geometric.__getitem__),
        "geometric_mean_online_ms": geometric,
        "median_online_ms": medians,
        "bonferroni_alpha_per_pair": ALPHA_PER_PAIR,
        "comparisons": comparisons,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("validated", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = select(json.loads(args.validated.read_text()))
    blob = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output:
        if args.output.exists() and args.output.read_text() != blob:
            raise ValueError("sealed screen selection differs")
        args.output.write_text(blob)
    else:
        print(blob, end="")


if __name__ == "__main__":
    main()
