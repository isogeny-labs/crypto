#!/usr/bin/env python3
"""Select the fixed31 base only from independently replayed Tier 1 pairs."""

import argparse
from decimal import Decimal, localcontext
import json
import math
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
POINT_COUNT = 1536
ALPHA_PER_PAIR = "1/60"


def exact_two_sided_sign_p(wins: int, losses: int) -> Decimal:
    count = wins + losses
    if count == 0:
        return Decimal(1)
    tail = sum(math.comb(count, k)
               for k in range(min(wins, losses) + 1))
    with localcontext() as ctx:
        ctx.prec = 60
        return min(Decimal(1), Decimal(2 * tail) / Decimal(1 << count))


def significant_sign(wins: int, losses: int) -> bool:
    count = wins + losses
    if count == 0:
        return False
    tail = sum(math.comb(count, k)
               for k in range(min(wins, losses) + 1))
    return 120 * tail < (1 << count)


def select(data: dict, dispatch: dict) -> dict:
    arms = dispatch["arms"]
    all_ids = [a["candidate_id"] for a in arms]
    fixed_ids = [a["candidate_id"] for a in arms if a["fixed31"]]
    result = {
        "schema": "n83.fixed31-screen-selection/v1",
        "status": "BOUNDED_UNKNOWN",
        "selected_candidate_id": None,
        "candidate_ids": all_ids,
        "point_count": POINT_COUNT,
    }
    gate = None
    if (data.get("schema") != "n83.fixed31-screen-validated/v1" or
            data.get("source_sha256") != dispatch["source_sha256"] or
            data.get("screen_targets_sha256") != dispatch["screen_targets_sha256"] or
            data.get("workload_manifest_sha256") !=
            dispatch["workload_manifest_sha256"] or
            data.get("candidate_ids") != all_ids or
            data.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            data.get("layout_control_status") !=
            "PASS_IDENTICAL_BASE_ROWS_RANK_ANSWERS"):
        gate = "IDENTITY_REPLAY_OR_LAYOUT_CONTROL"
    host = data.get("host")
    if gate is None and (
            not isinstance(host, dict) or
            host.get("tier_status") != "PASS_TIER1" or
            host.get("aa_status") != "PASS_20_PERCENT_PER_CHILD" or
            host.get("point_indices") != list(range(POINT_COUNT)) or
            data.get("block_candidate_orders") !=
            [all_ids, list(reversed(all_ids))]):
        gate = "HOST_ORDER_OR_CALIBRATION"
    points = data.get("points")
    if gate is None and (
            not isinstance(points, list) or len(points) != POINT_COUNT or
            {row.get("point_index") for row in points} !=
            set(range(POINT_COUNT))):
        gate = "POINT_COVERAGE"
    matrix = []
    if gate is None:
        for point in sorted(points, key=lambda row: row["point_index"]):
            rows = point.get("arms")
            if not isinstance(rows, list) or len(rows) != 4:
                gate = "ARM_COVERAGE"
                break
            by_id = {row.get("candidate_id"): row for row in rows}
            if set(by_id) != set(all_ids) or len(by_id) != 4:
                gate = "ARM_IDENTITY"
                break
            if any(row.get("status") != "PASS_INDEPENDENT_REPLAY" or
                   row.get("resource_status") != "PASS" or
                   not isinstance(row.get("one_target_online_ms"), (int, float)) or
                   isinstance(row.get("one_target_online_ms"), bool) or
                   not math.isfinite(row["one_target_online_ms"]) or
                   row["one_target_online_ms"] <= 0
                   for row in rows):
                gate = "ARM_REPLAY_RESOURCE_OR_TIMING"
                break
            matrix.append(by_id)
    if gate is not None:
        result["admission_status"] = gate
        return result
    result["admission_status"] = "PASS"
    times = {candidate: [row[candidate]["one_target_online_ms"]
                         for row in matrix] for candidate in fixed_ids}
    geometric = {candidate: statistics.geometric_mean(times[candidate])
                 for candidate in fixed_ids}
    comparisons = []
    for i, first in enumerate(fixed_ids):
        for second in fixed_ids[i + 1:]:
            pairs = zip(times[first], times[second])
            first_wins = sum(a < b for a, b in pairs)
            second_wins = sum(b < a for a, b in
                              zip(times[first], times[second]))
            p_value = exact_two_sided_sign_p(first_wins, second_wins)
            paired_logs = [math.log(b / a) for a, b in
                           zip(times[first], times[second])]
            mean_log = statistics.mean(paired_logs)
            sd_log = statistics.stdev(paired_logs)
            margin = 1.96 * sd_log / math.sqrt(POINT_COUNT)
            comparisons.append({
                "first": first,
                "second": second,
                "first_wins": first_wins,
                "second_wins": second_wins,
                "ties": POINT_COUNT - first_wins - second_wins,
                "two_sided_exact_sign_p": str(p_value),
                "significant_at_one_over_60": significant_sign(
                    first_wins, second_wins),
                "mean_log_second_over_first": mean_log,
                "median_log_second_over_first": statistics.median(paired_logs),
                "sample_sd_log_difference": sd_log,
                "normal_approx_95_mean_log_interval": [
                    mean_log - margin, mean_log + margin],
            })
    fastest = min(fixed_ids, key=geometric.__getitem__)
    selected = all(
        ((row["first"] == fastest and
          row["first_wins"] > row["second_wins"]) or
         (row["second"] == fastest and
          row["second_wins"] > row["first_wins"])) and
        row["significant_at_one_over_60"]
        for row in comparisons if fastest in (row["first"], row["second"])
    )
    result.update({
        "status": "SELECTED" if selected else "BOUNDED_UNKNOWN",
        "selected_candidate_id": fastest if selected else None,
        "descriptive_fastest_candidate_id": fastest,
        "geometric_mean_online_ms": geometric,
        "median_online_ms": {
            candidate: statistics.median(times[candidate])
            for candidate in fixed_ids
        },
        "bonferroni_alpha_per_pair": ALPHA_PER_PAIR,
        "comparisons": comparisons,
    })
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("validated", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    dispatch = json.loads((HERE / "dispatch.json").read_text())
    result = select(json.loads(args.validated.read_text()), dispatch)
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        if args.output.exists() and args.output.read_text() != blob:
            raise ValueError("screen selection differs")
        args.output.write_text(blob)
    else:
        print(blob, end="")


if __name__ == "__main__":
    main()
