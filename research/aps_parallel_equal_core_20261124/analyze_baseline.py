#!/usr/bin/env python3
"""Recompute descriptive phase totals and idealized bounds from archived JSON.

This reads an existing public-target receipt. It does not run either solver or
turn an idealized bound into a measured parallel result.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import statistics
import sys
from pathlib import Path


DEFAULT_INPUT = (
    Path(__file__).resolve().parents[1]
    / "prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_EXACT_CI_20261116.json"
)
APS = "incumbent_joint_APS"
RHO = "unfolded_rho64_dp5"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def analyze(raw: bytes) -> dict:
    data = json.loads(raw)
    cases = data["cases"]
    arms = data["arm_names"]
    require(len(cases) == data["cases_count"] == 48, "unexpected case count")
    require(APS in arms and RHO in arms, "required historical arms absent")
    require(len(set(case["case"] for case in cases)) == 48, "duplicate case index")

    totals = {arm: sum(int(case[arm]["cold_wall_ns"]) for case in cases) for arm in arms}
    for arm in arms:
        require(
            all(case[arm]["complete_target_scalar_recovered"] for case in cases),
            f"incomplete historical arm: {arm}",
        )
        require(
            totals[arm] == int(data["fully_cold_ns"][arms.index(arm)]),
            f"summary disagrees with case rows: {arm}",
        )
        require(
            data["successful_complete_calls"][arms.index(arm)] == 48,
            f"summary success count disagrees: {arm}",
        )

    attempts = [case[APS]["attempts"] for case in cases]
    first_attempt_successes = sum(
        len(rows) == 1 and rows[0]["solution"] is not None for rows in attempts
    )
    source_ns = sum(int(row["source_setup_ns"]) for rows in attempts for row in rows)
    search_ns = sum(int(row["s3_query_ns"]) for rows in attempts for row in rows)
    aps_ns = totals[APS]
    rho_ns = totals[RHO]
    require(source_ns + search_ns <= aps_ns, "phase sums exceed cold total")
    aps_case_ns = [int(case[APS]["cold_wall_ns"]) for case in cases]
    rho_case_ns = [int(case[RHO]["cold_wall_ns"]) for case in cases]
    search_case_ns = [
        sum(int(row["s3_query_ns"]) for row in rows) for rows in attempts
    ]
    require(
        all(search <= cold for search, cold in zip(search_case_ns, aps_case_ns)),
        "case search exceeds cold total",
    )

    ideal = []
    for cores in (2, 4, 8):
        # Only the recorded S3 phase is divided. Every other cost stays serial.
        lower_bound_ns = aps_ns - search_ns + search_ns / cores
        case_bounds = [
            cold - search + search / cores
            for cold, search in zip(aps_case_ns, search_case_ns)
        ]
        ideal.append(
            {
                "cores": cores,
                "ideal_aps_cold_ns": round(lower_bound_ns),
                "maximum_aps_speedup": round(aps_ns / lower_bound_ns, 6),
                "rho_speedup_ceiling_for_crossover": round(rho_ns / lower_bound_ns, 6),
                "ideal_aps_paired_wins_vs_serial_rho": sum(
                    aps < rho for aps, rho in zip(case_bounds, rho_case_ns)
                ),
                "median_case_ideal_aps_over_serial_rho": round(
                    statistics.median(
                        aps / rho for aps, rho in zip(case_bounds, rho_case_ns)
                    ),
                    6,
                ),
            }
        )

    return {
        "standing": "archived serial development receipts; derived bounds, no parallel measurement",
        "source_sha256": hashlib.sha256(raw).hexdigest(),
        "cases": len(cases),
        "first_attempt_successes": first_attempt_successes,
        "aps_cold_ns": aps_ns,
        "unfolded_rho64_cold_ns": rho_ns,
        "aps_source_setup_ns": source_ns,
        "aps_s3_query_ns": search_ns,
        "aps_s3_fraction_of_cold": round(search_ns / aps_ns, 9),
        "median_case_s3_fraction_of_cold": round(
            statistics.median(search / cold for search, cold in zip(search_case_ns, aps_case_ns)),
            9,
        ),
        "serial_aps_paired_wins_vs_rho": sum(
            aps < rho for aps, rho in zip(aps_case_ns, rho_case_ns)
        ),
        "serial_aps_over_rho_ratio": round(aps_ns / rho_ns, 9),
        "ideal_s3_only_parallel_bounds": ideal,
        "cpu_time_in_receipt": False,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "receipt", nargs="?", default=str(DEFAULT_INPUT), help="JSON path or - for stdin"
    )
    args = parser.parse_args()
    raw = sys.stdin.buffer.read() if args.receipt == "-" else Path(args.receipt).read_bytes()
    print(json.dumps(analyze(raw), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
