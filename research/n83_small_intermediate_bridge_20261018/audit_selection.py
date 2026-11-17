#!/usr/bin/env python3
"""Recompute the frozen N83 bridge paired selector outside the Rust producer."""

from __future__ import annotations

import argparse
from collections import defaultdict
import json
import math
from pathlib import Path
import statistics

N = 32
T_975_DF31 = 2.0395134463964077


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def close(actual: float, expected: float, label: str) -> None:
    require(math.isclose(actual, expected, rel_tol=1e-10, abs_tol=1e-7),
            f"{label} differs: {actual} versus {expected}")


def metric(row: dict, name: str) -> float:
    value = {"online": row["ic"]["target_online_ms"],
             "cold": row["ic"]["cold_process_ms"],
             "child_wall": row["ic_child_wall_ms"]}[name]
    require(isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and value > 0, f"{name} is invalid")
    return float(value)


def compare_metric(rows: list[dict], name: str, ids: list[str], native: dict) -> dict:
    first = [metric(pair[1107], name) for pair in rows]
    second = [metric(pair[2304], name) for pair in rows]
    differences = [math.log(b / a) if name == "online" else b - a
                   for a, b in zip(first, second)]
    positive = sum(value > 0 for value in differences)
    negative = sum(value < 0 for value in differences)
    ties = N - positive - negative
    tail = sum(math.comb(N, k) for k in range(max(positive, negative), N + 1))
    p = None if ties else min(1.0, 2 * tail / (1 << N))
    mean = math.fsum(differences) / N
    variance = math.fsum((value - mean) ** 2 for value in differences) / (N - 1)
    halfwidth = T_975_DF31 * math.sqrt(variance / N)
    selected = None
    if p is not None and p <= 0.05:
        if positive >= 23 and mean > 0:
            selected = ids[0]
        elif negative >= 23 and mean < 0:
            selected = ids[1]
    require(native["status"] == ("SELECTED" if selected else "BOUNDED_UNKNOWN")
            and native["selected_candidate_id"] == selected
            and native["first_candidate_id"] == ids[0]
            and native["second_candidate_id"] == ids[1]
            and native["positive_second_minus_first"] == positive
            and native["negative_second_minus_first"] == negative
            and native["ties"] == ties,
            f"{name} native sign or selected ID differs")
    for key, expected in (("mean_difference", mean),
                          ("lower_95", mean - halfwidth),
                          ("upper_95", mean + halfwidth),
                          ("first_arithmetic_mean_ms", math.fsum(first) / N),
                          ("second_arithmetic_mean_ms", math.fsum(second) / N)):
        close(native[key], expected, f"{name}.{key}")
    if p is None:
        require(native["two_sided_exact_sign_p"] is None,
                f"{name} native tie p differs")
    else:
        require(math.isclose(native["two_sided_exact_sign_p"], p,
                             rel_tol=1e-12, abs_tol=0),
                f"{name} exact sign p differs")
    require(len(native["per_point_differences"]) == N,
            f"{name} native per-point coverage differs")
    for index, (actual, expected) in enumerate(zip(native["per_point_differences"],
                                                    differences)):
        close(actual, expected, f"{name}.point{index}")
    if name == "online":
        close(native["geometric_second_over_first"], math.exp(mean),
              "online geometric ratio")
    else:
        require(native["geometric_second_over_first"] is None,
                f"{name} unexpected geometric ratio")
    return {"metric": name, "selected_candidate_id": selected,
            "positive_second_minus_first": positive,
            "negative_second_minus_first": negative, "ties": ties,
            "two_sided_exact_sign_p": p, "mean_difference": mean,
            "lower_95": mean - halfwidth, "upper_95": mean + halfwidth}


def audit(fixture: Path, analyses: list[Path], native_path: Path) -> dict:
    require(len(analyses) == 4, "four analyses required")
    protocol = read(fixture / "protocol.json")
    workloads = read(fixture / "workloads.json")["rows"]
    native = read(native_path)
    ids = [protocol["candidate_k1107"], protocol["candidate_k2304"]]
    require(native["schema"] == "n83.native-bridge-selection/v1"
            and native["status"] == "PASS_TIER1"
            and native["point_count"] == N
            and native["candidate_ids"] == ids
            and len(workloads) == N,
            "native selection or frozen fixture differs")
    by_point: dict[int, dict[int, dict]] = defaultdict(dict)
    for stratum, path in enumerate(analyses):
        analysis = read(path)
        require(analysis["schema"] == "n83.native-bridge-stratum-analysis/v1"
                and analysis["status"] == "PASS_TIER1"
                and analysis["stratum"] == stratum
                and analysis["source_sha256"] == protocol["source_sha256"]
                and analysis["targets_sha256"] == protocol["targets_sha256"]
                and analysis["host"]["git_sha"] == native["head_sha"]
                and analysis["host"]["github_run_id"] == native["run_id"]
                and analysis["steal_jiffies_before"] ==
                analysis["steal_jiffies_after"]
                and len(analysis["rows"]) == 16,
                f"stratum {stratum} admission differs")
        for row in analysis["rows"]:
            point, k = row["point_index"], row["k"]
            require(type(point) is int and point // 8 == stratum
                    and k in (1107, 2304) and k not in by_point[point]
                    and row["candidate_id"] == ids[0 if k == 1107 else 1]
                    and row["workload_id"] == workloads[point]["workload_id"]
                    and row["target"] ==
                    workloads[point]["canonical_record"]["public_target_q"]
                    and row["ic"]["status"] == "PASS_INDEPENDENT_REPLAY"
                    and row["ic"]["rank"] == k
                    and row["rho"]["status"] == "PASS_INDEPENDENT_REPLAY"
                    and row["calibration_status"] == "PASS_10_PERCENT"
                    and isinstance(row["calibration_relative_spread"], (int, float))
                    and 0 <= row["calibration_relative_spread"] <= 0.10
                    and math.isfinite(row["calibration_relative_spread"])
                    and isinstance(row["rho"]["online_ms"], (int, float))
                    and row["rho"]["online_ms"] > 0
                    and math.isfinite(row["rho"]["online_ms"]),
                    f"point {point} candidate {k} admission differs")
            by_point[point][k] = row
    require(set(by_point) == set(range(N))
            and all(set(pair) == {1107, 2304} for pair in by_point.values()),
            "32 complete point pairs required")
    ordered = [by_point[index] for index in range(N)]
    results = [compare_metric(ordered, name, ids, native[name])
               for name in ("online", "cold", "child_wall")]
    ratios = [[pair[k]["rho"]["online_ms"] / metric(pair[k], "online")
               for k in (1107, 2304)] for pair in ordered]
    require(len(native["paired_rho_over_ic_online"]) == N,
            "native rho ratio coverage differs")
    for index, (expected, actual) in enumerate(zip(ratios,
                                                   native["paired_rho_over_ic_online"])):
        for arm in (0, 1):
            close(actual[arm], expected[arm], f"rho ratio point{index} arm{arm}")
    require(native["all_32_same_point_ic_faster_than_rho_by_candidate"] ==
            [all(pair[arm] > 1 for pair in ratios) for arm in (0, 1)],
            "native same-point rho flags differ")
    cold_over_rho = []
    for k in (1107, 2304):
        observed = []
        for pair in ordered:
            rho_wall = pair[k]["rho"]["child_wall_ms"]
            require(isinstance(rho_wall, (int, float))
                    and math.isfinite(rho_wall) and rho_wall > 0,
                    "matched rho child wall is invalid")
            observed.append(metric(pair[k], "child_wall") / rho_wall)
        cold_over_rho.append({"candidate_id": ids[0 if k == 1107 else 1],
                              "point_count": N,
                              "ic_child_wall_faster_count":
                              sum(value < 1 for value in observed),
                              "median_ic_over_rho_child_wall":
                              statistics.median(observed)})
    return {"schema": "n83.native-bridge-independent-selection-audit/v1",
            "status": "PASS_INDEPENDENT_PAIRED_SELECTION",
            "run_id": native["run_id"], "point_count": N,
            "metrics": results, "cold_over_matched_rho": cold_over_rho,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixture", type=Path, required=True)
    parser.add_argument("--analyses", type=Path, nargs=4, required=True)
    parser.add_argument("--native-selection", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.fixture, args.analyses, args.native_selection)
    text = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output.exists():
        require(args.output.read_text() == text, "sealed independent audit differs")
    else:
        args.output.write_text(text)
    print(json.dumps({"status": result["status"], "run_id": result["run_id"]}))


if __name__ == "__main__":
    main()
