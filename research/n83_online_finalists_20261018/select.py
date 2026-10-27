#!/usr/bin/env python3
"""Frozen exact paired selector for six N83 batched-root factor bases."""

import argparse
import bisect
import json
import math
import statistics
from pathlib import Path

SCHEMA = "n83.online-finalists-validated/v1"
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
TARGETS_SHA256 = "bfd299b20bae05af249dea7941451d5fc6cbbebe44b483c2e84651cc4c2d3478"
T_975_DF31 = 2.0395134463964077
IDS = (
    "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668",
    "IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h00a0a52d463c",
    "IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h3a9440ac9544",
    "IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e",
    "IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1",
    "IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea",
)
N = 32
PER_HOST = 8


def positive(value):
    return (isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and value > 0)


def signed_sums(values):
    sums = [0.0]
    for value in values:
        sums = [x + value for x in sums] + [x - value for x in sums]
    return sums


def sign_flip_count(left_values, right_values, observed, tolerance):
    left = signed_sums(left_values)
    right = sorted(signed_sums(right_values))
    count = 0
    for value in left:
        count += bisect.bisect_right(right, -observed + tolerance - value)
        count += len(right) - bisect.bisect_left(
            right, observed - tolerance - value)
    return count


def sign_flip_p(differences):
    """Count all 2^32 sign assignments with two independent 16/16 splits."""
    if len(differences) != N or not all(math.isfinite(x) for x in differences):
        raise ValueError("32 finite paired differences required")
    observed = abs(math.fsum(differences))
    tolerance = 1e-10 * max(1.0, math.fsum(abs(x) for x in differences))
    if observed <= tolerance:
        return 1.0
    count = sign_flip_count(differences[:N // 2], differences[N // 2:],
                            observed, tolerance)
    alternate = sign_flip_count(differences[::2], differences[1::2],
                                 observed, tolerance)
    if count != alternate:
        raise ValueError("sign-flip count changes with the half-list split")
    return count / (1 << N)


def summarize(differences, definition, alpha):
    mean = statistics.mean(differences)
    sd = statistics.stdev(differences)
    halfwidth = T_975_DF31 * sd / math.sqrt(N)
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
        if (not isinstance(arms, list) or len(arms) != 6 or
                not isinstance(rho, dict) or set(rho) != set(IDS) or
                not isinstance(calibration, dict) or
                set(calibration) != set(IDS)):
            return None, "PAIR_COVERAGE_GATE"
        by_id = {arm.get("candidate_id"): arm for arm in arms}
        if len(by_id) != 6 or set(by_id) != set(IDS):
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


def select_metric(matrix, metric):
    online = metric == "one_target_online_ms"
    alpha = 0.05 / 15
    comparisons = []
    for i, first in enumerate(IDS):
        for second in IDS[i + 1:]:
            differences = [
                math.log(row["arms"][second][metric] /
                         row["arms"][first][metric]) if online else
                row["arms"][second][metric] - row["arms"][first][metric]
                for row in matrix]
            result = summarize(differences,
                               "log_second_over_first" if online else
                               "second_minus_first_ms", alpha)
            result.update({"first_candidate_id": first,
                           "second_candidate_id": second,
                           "geometric_second_over_first":
                           math.exp(result["mean"]) if online else None})
            comparisons.append(result)
    means = {}
    medians = {}
    for candidate in IDS:
        values = [row["arms"][candidate][metric] for row in matrix]
        means[candidate] = (math.exp(statistics.mean(map(math.log, values)))
                            if online else statistics.mean(values))
        medians[candidate] = statistics.median(values)
    fastest = min(IDS, key=means.__getitem__)
    admitted = all(
        (pair["mean"] > 0 if pair["first_candidate_id"] == fastest else
         pair["mean"] < 0) and pair["two_sided_sign_flip_p"] < alpha
        for pair in comparisons
        if fastest in (pair["first_candidate_id"],
                       pair["second_candidate_id"]))
    return {"status": "SELECTED" if admitted else "BOUNDED_UNKNOWN",
            "selected_candidate_id": fastest if admitted else None,
            "descriptive_fastest_candidate_id": fastest,
            "alpha_per_pair": alpha,
            "mean_by_candidate_ms": means,
            "median_by_candidate_ms": medians,
            "comparisons": comparisons}


def select(data):
    matrix, admission = admitted_matrix(data)
    if matrix is None:
        return {"schema": "n83.online-finalists-selection/v1",
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
                "status": "PASS_ALL_EIGHT_SAME_POINT" if all(x > 1 for x in ratios)
                          else "NO_ALL_POINT_SPEEDUP",
                "paired_rho_over_ic": ratios,
                "median_ratio": statistics.median(ratios)}
    try:
        online = select_metric(matrix, "one_target_online_ms")
        cold = select_metric(matrix, "cold_process_ms")
    except ValueError as error:
        return {"schema": "n83.online-finalists-selection/v1",
                "admission_status": "STATISTICAL_REPLAY_NUMERIC_MISMATCH",
                "statistical_error": str(error),
                "candidate_ids": list(IDS), "point_count": N,
                "online": {"status": "BOUNDED_UNKNOWN",
                           "selected_candidate_id": None},
                "cold": {"status": "BOUNDED_UNKNOWN",
                         "selected_candidate_id": None},
                "tier1_speedup_by_host": speedup}
    return {"schema": "n83.online-finalists-selection/v1",
            "admission_status": admission,
            "candidate_ids": list(IDS), "point_count": N,
            "online": online, "cold": cold,
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
