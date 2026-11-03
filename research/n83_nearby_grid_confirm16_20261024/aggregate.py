#!/usr/bin/env python3
"""Pair 16 replayed one-target N83 rows across two Tier 1 strata."""

import argparse
import itertools
import json
import math
from pathlib import Path
import statistics

from research.n83_nearby_grid_confirm16_20261024 import protocol
from research.n83_nearby_grid_confirm16_20261024 import verify
from research.n83_nearby_grid_tier1_20261024 import protocol as screen


def exact_sign_flip_p(differences):
    """Two-sided exact randomization p for frozen paired differences."""
    if len(differences) != 16 or any(not math.isfinite(x) for x in differences):
        raise ValueError("exact paired test requires 16 finite differences")
    observed = abs(math.fsum(differences))
    extreme = 0
    for signs in itertools.product((-1, 1), repeat=16):
        if abs(math.fsum(x * sign for x, sign in zip(differences, signs))) >= \
                observed - 1e-10:
            extreme += 1
    return extreme / (1 << 16)


def comparison(first, second, values, metric, alpha):
    if metric == "online":
        paired = [math.log(values[second][i] / values[first][i])
                  for i in range(16)]
        effect = math.exp(statistics.mean(paired))
        effect_name = "geometric_second_over_first"
    else:
        paired = [values[second][i] - values[first][i] for i in range(16)]
        effect = statistics.mean(paired)
        effect_name = "mean_second_minus_first_ms"
    p = exact_sign_flip_p(paired)
    mean = statistics.mean(paired)
    sd = statistics.stdev(paired)
    half_width = 2.131449545559323 * sd / 4
    return {"first_k": first, "second_k": second, "metric": metric,
            effect_name: effect, "paired_mean": mean, "paired_sd": sd,
            "paired_mean_95_t_interval": [mean - half_width, mean + half_width],
            "two_sided_exact_sign_flip_p": p,
            "alpha_bonferroni": alpha,
            "status": "PASS_THRESHOLD" if mean > 0 and p <= alpha else
            "NOT_ESTABLISHED"}


def select_metric(selected_ks, values, metric, alpha):
    scores = ({k: math.exp(statistics.mean(math.log(x) for x in values[k].values()))
               for k in selected_ks} if metric == "online" else
              {k: statistics.mean(values[k].values()) for k in selected_ks})
    fastest = min(selected_ks, key=lambda k: (scores[k], k))
    comparisons = [comparison(fastest, k, values, metric, alpha)
                   for k in selected_ks if k != fastest]
    winner = fastest if all(row["status"] == "PASS_THRESHOLD"
                            for row in comparisons) else None
    return {"metric": metric, "descriptive_fastest_k": fastest,
            "score_by_k": {str(k): scores[k] for k in selected_ks},
            "comparisons_to_descriptive_fastest": comparisons,
            "selected_k": winner,
            "selected_candidate_id": screen.CANDIDATES[winner][0]
            if winner is not None else None,
            "status": "PASS_SELECTED" if winner is not None else
            "BOUNDED_UNKNOWN"}


def aggregate(outputs, raw_screen):
    analyses = []
    for path in outputs:
        try:
            analyses.append(verify.verify(path, raw_screen))
        except (OSError, ValueError, KeyError, TypeError, IndexError,
                json.JSONDecodeError) as error:
            analyses.append({"status": "BOUNDED_UNKNOWN", "stratum": None,
                             "selected_ks": None, "host_instance_id": None,
                             "runner_name": None, "git_head": None,
                             "rows": [], "reason": f"MISSING_OR_INVALID_STRATUM: {error}"})
    selected = analyses[0].get("selected_ks")
    strata = {row.get("stratum") for row in analyses}
    hosts = {row.get("host_instance_id") for row in analyses}
    runner_names = {row.get("runner_name") for row in analyses}
    git_heads = {row.get("git_head") for row in analyses}
    all_rows = [dict(row, stratum=analysis.get("stratum"),
                     host_instance_id=analysis.get("host_instance_id"))
                for analysis in analyses for row in analysis.get("rows", [])]
    result = {"schema": "n83.nearby-grid-confirm16-aggregate/v1",
              "status": "BOUNDED_UNKNOWN", "tier": "Tier 1 GitHub Actions",
              "screen_run_url": analyses[0].get("screen_run_url"),
              "selected_ks": selected,
              "stratum_statuses": [row.get("status") for row in analyses],
              "host_instance_ids": sorted(str(host) for host in hosts),
              "runner_names": sorted(str(name) for name in runner_names),
              "rows": all_rows, "online": None, "cold": None,
              "alpha_per_unordered_pair": None}
    if (len(outputs) != 2 or strata != {0, 1} or len(hosts) != 2 or
            None in hosts or len(runner_names) != 2 or
            None in runner_names or len(git_heads) != 1 or
            None in git_heads or not selected or
            any(row.get("status") != "STRATUM_ADMITTED" or
                row.get("selected_ks") != selected or
                row.get("screen_run_url") != result["screen_run_url"]
                for row in analyses)):
        result["reason"] = "INCOMPLETE_OR_RESOURCE_FAILED_STRATUM"
        return result
    expected = {(index, k) for index in range(16) for k in selected}
    observed = {(row["point_index"], row["k"]) for row in all_rows}
    keys = {(row["candidate_id"], row["workload_id"], row["run_id"])
            for row in all_rows}
    if (observed != expected or len(observed) != len(all_rows) or
            len(keys) != len(all_rows) or
            any(row["status"] != "PASS_INDEPENDENT_REPLAY" for row in all_rows)):
        result["reason"] = "CANDIDATE_POINT_COVERAGE_OR_REPLAY_FAILED"
        return result
    online = {k: {} for k in selected}
    cold = {k: {} for k in selected}
    rho_ratios = {k: {} for k in selected}
    for row in all_rows:
        k, index = row["k"], row["point_index"]
        for key in ("ic_online_ms", "rho_online_ms", "ic_cold_process_ms",
                    "rho_over_ic_online"):
            value = row[key]
            if (not isinstance(value, (int, float)) or isinstance(value, bool) or
                    not math.isfinite(value) or value <= 0):
                result["reason"] = "MISSING_OR_INVALID_COST"
                return result
        online[k][index] = row["ic_online_ms"]
        cold[k][index] = row["ic_cold_process_ms"]
        rho_ratios[k][index] = row["rho_over_ic_online"]
    alpha = 0.05 / (len(selected) * (len(selected) - 1) // 2)
    result.update({
        "status": "PASS_TIER1_CONFIRM16",
        "alpha_per_unordered_pair": alpha,
        "online": select_metric(selected, online, "online", alpha),
        "cold": select_metric(selected, cold, "cold", alpha),
        "median_same_point_rho_over_ic_by_k": {
            str(k): statistics.median(rho_ratios[k].values()) for k in selected},
        "one_target_rows_verified": len(all_rows),
    })
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("stratum0", type=Path)
    parser.add_argument("stratum1", type=Path)
    parser.add_argument("--screen-output", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = aggregate([args.stratum0.resolve(), args.stratum1.resolve()],
                       args.screen_output.resolve())
    args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"status": result["status"],
                      "online_selected_k": (result["online"] or {}).get("selected_k"),
                      "cold_selected_k": (result["cold"] or {}).get("selected_k")},
                     sort_keys=True))
    return 0 if result["status"] == "PASS_TIER1_CONFIRM16" else 2


if __name__ == "__main__":
    raise SystemExit(main())
