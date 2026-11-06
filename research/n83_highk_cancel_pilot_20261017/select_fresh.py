#!/usr/bin/env python3
"""Frozen exact paired selector for the fresh high-K cancellation factorial."""

import argparse
import bisect
import json
import math
import statistics
from pathlib import Path

SCHEMA = "n83.highk-cancel-factorial-validated/v1"
SOURCE_SHA256 = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829"
TARGETS_SHA256 = "5a234d93d748fafdf6ee0d00714d42446a309bfbbdfba9669b75831a87ad8090"
T_975_DF23 = 2.06865761041904
IDS = (
    "IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h974bf560d5c0",
    "IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993",
    "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hbf1f87cc02bf",
    "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27",
)
N = 24
PER_HOST = 6


def positive(value):
    return (isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and value > 0)


def signed_sums(values):
    sums = [0.0]
    for value in values:
        sums = [x + value for x in sums] + [x - value for x in sums]
    return sums


def sign_flip_p(differences):
    """Count all 2^24 sign assignments by two exact 2^12 half lists."""
    if len(differences) != N or not all(math.isfinite(x) for x in differences):
        raise ValueError("24 finite paired differences required")
    observed = abs(math.fsum(differences))
    tolerance = 1e-10 * max(1.0, math.fsum(abs(x) for x in differences))
    if observed <= tolerance:
        return 1.0
    left = signed_sums(differences[:N // 2])
    right = sorted(signed_sums(differences[N // 2:]))
    count = 0
    for value in left:
        count += bisect.bisect_right(right, -observed + tolerance - value)
        count += len(right) - bisect.bisect_left(
            right, observed - tolerance - value)
    return count / (1 << N)


def summarize(differences, definition, alpha):
    mean = statistics.mean(differences)
    sd = statistics.stdev(differences)
    halfwidth = T_975_DF23 * sd / math.sqrt(N)
    p = sign_flip_p(differences)
    return {"difference_definition": definition, "point_count": N,
            "mean": mean, "median": statistics.median(differences),
            "sample_sd": sd, "lower_95": mean - halfwidth,
            "upper_95": mean + halfwidth,
            "positive_count": sum(value > 0 for value in differences),
            "negative_count": sum(value < 0 for value in differences),
            "tie_count": sum(value == 0 for value in differences),
            "two_sided_sign_flip_p": p, "alpha": alpha,
            "status": "PASS_THRESHOLD" if p < alpha else "BOUNDED_UNKNOWN"}


def admitted_matrix(data):
    if (data.get("schema") != SCHEMA or
            data.get("source_sha256") != SOURCE_SHA256 or
            data.get("targets_sha256") != TARGETS_SHA256 or
            data.get("candidate_ids") != list(IDS) or
            data.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            data.get("different_cloud_instances") is not True or
            data.get("same_candidate_bases_across_strata") is not True):
        return None, "IDENTITY_OR_REPLAY_GATE"
    hosts = data.get("host_strata")
    if (not isinstance(hosts, list) or len(hosts) != 4 or
            {tuple(host.get("point_indices", [])) for host in hosts} !=
            {tuple(range(i * PER_HOST, (i + 1) * PER_HOST)) for i in range(4)} or
            any(host.get("tier_status") != "PASS_TIER1" or
                host.get("aa_status") != "PASS_10_PERCENT_PER_PAIR"
                for host in hosts)):
        return None, "HOST_STRATA_GATE"
    points = data.get("points")
    if (not isinstance(points, list) or len(points) != N or
            {point.get("point_index") for point in points} != set(range(N))):
        return None, "POINT_COVERAGE_GATE"
    matrix = []
    for point in sorted(points, key=lambda row: row["point_index"]):
        arms, rho, calibration = (point.get(key) for key in
                                  ("arms", "rho_by_candidate",
                                   "calibration_by_candidate"))
        if (not isinstance(arms, list) or len(arms) != 4 or
                not isinstance(rho, dict) or set(rho) != set(IDS) or
                not isinstance(calibration, dict) or
                set(calibration) != set(IDS)):
            return None, "PAIR_COVERAGE_GATE"
        by_id = {arm.get("candidate_id"): arm for arm in arms}
        if len(by_id) != 4 or set(by_id) != set(IDS):
            return None, "IC_ID_GATE"
        if any(arm.get("status") != "PASS_INDEPENDENT_REPLAY" or
               arm.get("resource_status") != "PASS" or
               not positive(arm.get("one_target_online_ms")) or
               not positive(arm.get("cold_process_ms"))
               for arm in arms):
            return None, "IC_REPLAY_RESOURCE_OR_TIMING_GATE"
        if any(item.get("status") != "PASS_INDEPENDENT_REPLAY" or
               item.get("resource_status") != "PASS" or
               not positive(item.get("online_ms")) for item in rho.values()):
            return None, "RHO_REPLAY_RESOURCE_OR_TIMING_GATE"
        if any(item.get("status") != "PASS_10_PERCENT"
               for item in calibration.values()):
            return None, "CALIBRATION_GATE"
        matrix.append({"point_index": point["point_index"],
                       "arms": by_id, "rho": rho})
    return matrix, "PASS"


def choose_pair(matrix, first, second, metric):
    online = metric == "one_target_online_ms"
    differences = [
        math.log(row["arms"][second][metric] /
                 row["arms"][first][metric]) if online else
        row["arms"][second][metric] - row["arms"][first][metric]
        for row in matrix]
    result = summarize(differences,
                       "log_second_over_first" if online else
                       "second_minus_first_ms", 0.05)
    selected = (first if result["mean"] > 0 else
                second if result["mean"] < 0 else None)
    result.update({"first_candidate_id": first, "second_candidate_id": second,
                   "geometric_second_over_first":
                   math.exp(result["mean"]) if online else None,
                   "selected_candidate_id": selected if
                   result["status"] == "PASS_THRESHOLD" else None,
                   "status": "SELECTED" if selected is not None and
                   result["status"] == "PASS_THRESHOLD" else
                   "BOUNDED_UNKNOWN"})
    return result


def policy_contrasts(matrix):
    log_pairs = {
        2816: [math.log(row["arms"][IDS[1]]["one_target_online_ms"] /
                        row["arms"][IDS[0]]["one_target_online_ms"])
               for row in matrix],
        3072: [math.log(row["arms"][IDS[3]]["one_target_online_ms"] /
                        row["arms"][IDS[2]]["one_target_online_ms"])
               for row in matrix],
    }
    interaction = [b - a for a, b in zip(log_pairs[2816], log_pairs[3072])]
    alpha = 0.05 / 3
    return {
        "k2816_cancel_over_join": summarize(
            log_pairs[2816], "log_cancel_over_join_K2816", alpha),
        "k3072_cancel_over_join": summarize(
            log_pairs[3072], "log_cancel_over_join_K3072", alpha),
        "policy_by_base_interaction": summarize(
            interaction, "log_cancel_over_join_K3072_minus_K2816", alpha),
    }


def select(data):
    matrix, admission = admitted_matrix(data)
    if matrix is None:
        return {"schema": "n83.highk-cancel-factorial-selection/v1",
                "admission_status": admission,
                "online": {"status": "BOUNDED_UNKNOWN",
                           "selected_candidate_id": None},
                "cold": {"status": "BOUNDED_UNKNOWN",
                         "selected_candidate_id": None}}
    speedup = {}
    for stratum in range(4):
        rows = matrix[stratum * PER_HOST:(stratum + 1) * PER_HOST]
        speedup[str(stratum)] = {}
        for candidate in IDS:
            ratios = [row["rho"][candidate]["online_ms"] /
                      row["arms"][candidate]["one_target_online_ms"]
                      for row in rows]
            speedup[str(stratum)][candidate] = {
                "status": "PASS_ALL_SIX_SAME_POINT" if all(x > 1 for x in ratios)
                          else "NO_ALL_POINT_SPEEDUP",
                "paired_rho_over_ic": ratios,
                "median_ratio": statistics.median(ratios)}
    return {"schema": "n83.highk-cancel-factorial-selection/v1",
            "admission_status": admission,
            "candidate_ids": list(IDS), "point_count": N,
            "online": choose_pair(matrix, IDS[1], IDS[3],
                                  "one_target_online_ms"),
            "cold": choose_pair(matrix, IDS[1], IDS[3], "cold_process_ms"),
            "policy_contrasts": policy_contrasts(matrix),
            "tier1_speedup_by_host": speedup}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("validated_rows", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    blob = json.dumps(select(json.loads(args.validated_rows.read_text())),
                      indent=2, sort_keys=True) + "\n"
    if args.output:
        if args.output.exists() and args.output.read_text() != blob:
            raise ValueError("sealed selection differs")
        args.output.write_text(blob)
    else:
        print(blob, end="")


if __name__ == "__main__":
    main()
