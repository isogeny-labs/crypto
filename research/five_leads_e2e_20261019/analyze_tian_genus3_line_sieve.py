"""Price Tian's complete F257 mapped line-sieve DLP against source rho."""

from __future__ import annotations

import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def read(directory: str, name: str) -> dict:
    return json.loads((ROOT / directory / name).read_text())


def vector(row: dict) -> tuple[float, float, float, float]:
    return tuple(float(row[key]) for key in ("m", "s", "i", "field_adds"))


def add(*rows: tuple[float, float, float, float]) -> tuple[float, float, float, float]:
    return tuple(sum(row[i] for row in rows) for i in range(4))


def price(row: tuple[float, float, float, float], s: float, i: float, a: float) -> float:
    return row[0] + s * row[1] + i * row[2] + a * row[3]


def mean_trials(rows: list[dict]) -> tuple[float, float, float, float]:
    assert len(rows) == 5 and all(row["source_verified"] for row in rows)
    return tuple(sum(vector(row["field_counts"])[j] for row in rows) / 5 for j in range(4))


def main() -> None:
    solve = read("correctness", "tian-genus3-f257-line-sieve-mapped-dlp-v1.json")
    rho = read("correctness", "tian-genus3-f257-source-rho-v1.json")
    bsgs = read("results", "tian-genus3-f257-cost-summary.json")
    assert solve["schema"] == "tian-genus3-f257-line-sieve-mapped-dlp/v1"
    assert solve["r"] == 172553 and solve["relation_rank"] == 249
    assert solve["nullity"] == 1 and solve["fully_split_lines"] == 2458
    descent = solve["descent"]
    assert descent["recovered"] == 86241
    assert descent["destination_verified"] and descent["source_verified"]
    assert descent["index_calculus_executed"] and descent["source_map_executed"]
    assert rho["transfer_source_verified"] and rho["source_order"] == 4 * solve["r"]
    assert bsgs["recovered"] == descent["recovered"]
    relation = solve["scalar_field_counts"]
    scalar = (float(relation["multiplications"] + descent["scalar_kernel_multiplications"]),
              0.0, float(relation["inversions"] + descent["scalar_recovery_inversions"]),
              float(relation["additions"] + descent["scalar_kernel_additions"]))
    initial_curve = vector(solve["curve_field_counts"])
    mapped_descent_curve = vector(descent["after_verification_curve_field_counts"])
    source_verification = vector(rho["transfer_source_verification_counts"])
    candidate = add(initial_curve, mapped_descent_curve, scalar, source_verification)
    conventional = mean_trials(rho["rho_trials"])
    negation = mean_trials(rho["rho_negation_trials"])
    order_discovery = vector(rho["order_discovery_field_counts"])
    bsgs_candidate = tuple(bsgs["mapped_cold_total_counts"])
    rows = []
    for s in (0.0, 0.5, 1.0):
        for i in (1.0, 10.0, 100.0, 1000.0):
            for a in (0.0, 0.1, 1.0):
                c = price(candidate, s, i, a)
                r = price(conventional, s, i, a)
                rn = price(negation, s, i, a)
                best = min(r, rn)
                rd = best + price(order_discovery, s, i, a)
                rows.append({"S_per_M": s, "I_per_M": i, "add_per_M": a,
                             "mapped_line_sieve_GAE": c, "source_rho_GAE": r,
                             "source_negation_walk_GAE": rn,
                             "best_source_rho_GAE": best,
                             "mapped_over_best_rho": c / best,
                             "best_source_rho_with_order_discovery_GAE": rd,
                             "mapped_over_rho_with_order_discovery": c / rd,
                             "mapped_over_mapped_bsgs": c / price(bsgs_candidate, s, i, a)})
    with (ROOT / "results/tian-genus3-f257-line-sieve-cost-grid.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    representative = next(row for row in rows if row["S_per_M"] == 0.5
                          and row["I_per_M"] == 10 and row["add_per_M"] == 0.1)
    summary = {
        "schema": "tian-genus3-f257-line-sieve-cost-analysis/v1",
        "fixture": "one published F257 P1/P2 pair; mapped source formulas evaluated in solver",
        "source_order": rho["source_order"],
        "prime_relation_modulus": solve["r"],
        "source_order_cofactor": 4,
        "fully_split_lines": solve["fully_split_lines"],
        "factor_base_size": solve["affine_rational_points"],
        "relation_rank": solve["relation_rank"],
        "recovered": descent["recovered"],
        "source_and_destination_verified": True,
        "lift_tests": descent["tested_lifts"],
        "count_order": ["M", "S", "I", "field_adds"],
        "initial_curve_counts": initial_curve,
        "mapped_setup_descent_and_destination_verification_counts": mapped_descent_curve,
        "scalar_relation_and_recovery_counts": scalar,
        "source_verification_counts": source_verification,
        "mapped_cold_total_counts": candidate,
        "mean_conventional_rho_counts": conventional,
        "mean_negation_walk_counts": negation,
        "source_order_discovery_counts": order_discovery,
        "representative": representative,
        "mapped_over_best_rho_band": [min(row["mapped_over_best_rho"] for row in rows),
                                      max(row["mapped_over_best_rho"] for row in rows)],
        "mapped_over_rho_with_discovery_band": [
            min(row["mapped_over_rho_with_order_discovery"] for row in rows),
            max(row["mapped_over_rho_with_order_discovery"] for row in rows)],
        "mapped_over_bsgs_band": [min(row["mapped_over_mapped_bsgs"] for row in rows),
                                  max(row["mapped_over_mapped_bsgs"] for row in rows)],
        "rho_losses": sum(row["mapped_over_best_rho"] > 1 for row in rows),
        "rho_wins": sum(row["mapped_over_best_rho"] < 1 for row in rows),
        "bsgs_losses": sum(row["mapped_over_mapped_bsgs"] > 1 for row in rows),
        "bsgs_wins": sum(row["mapped_over_mapped_bsgs"] < 1 for row in rows),
        "scope": "fixed published positive control only; Smith-negative holdout untested",
        "unpriced": "memory, hashing, comparisons, control flow and qualified wall time; scalar-field M/I priced as single-word base-field M/I",
    }
    (ROOT / "results/tian-genus3-f257-line-sieve-cost-summary.json").write_text(
        json.dumps(summary, indent=2) + "\n")
    print(json.dumps(representative, indent=2))


if __name__ == "__main__":
    main()
