"""Price the completed mapped F257 fixed-fixture DLP against counted source rho."""

from __future__ import annotations

import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
P = 257


def read(name: str) -> dict:
    return json.loads((ROOT / "correctness" / name).read_text())


def counts(row: dict) -> tuple[float, float, float, float]:
    return tuple(float(row[key]) for key in ("m", "s", "i", "field_adds"))


def add(*rows: tuple[float, float, float, float]) -> tuple[float, float, float, float]:
    return tuple(sum(row[i] for row in rows) for i in range(4))


def price(row: tuple[float, float, float, float], s: float, i: float, a: float) -> float:
    return row[0] + s * row[1] + i * row[2] + a * row[3]


def formula_counts() -> tuple[float, float, float, float]:
    fixture = json.loads((ROOT / "fixtures/tian-genus3-f257-map-v1.json").read_text())
    assert fixture["field"] == "F_257" and len(fixture["functions"]) == 6
    # mapped_cuts uses one Horner step (M+A) per stored coefficient,
    # then two M and one A per C_i, and one inversion per source point.
    horner = len(fixture["denominator"]) + sum(
        len(row["x_only"]) + len(row["y_factor"]) for row in fixture["functions"])
    return (float(2 * (horner + 12)), 0.0, 2.0, float(2 * (horner + 6)))


def main() -> None:
    dlp = read("tian-genus3-f257-split-bsgs-twist0.json")
    source = read("tian-genus3-f257-source-rho-v1.json")
    map_gate = read("tian-genus3-f257-map-gate-v1.json")
    transfer = read("tian-genus3-f257-transfer-gate-v1.json")
    assert dlp["schema"] == "tian-genus3-f257-split-bsgs-dlp/v1"
    assert source["schema"] == "tian-genus3-f257-source-rho/v1"
    assert map_gate["result"]["printed_examples_checked"] == 2
    assert transfer["destination_scalar_verified"]
    assert dlp["recovered"] == 86241 and dlp["destination_verified"]
    assert source["transfer_source_verified"]
    assert len(source["rho_trials"]) == 5
    assert all(row["source_verified"] and row["recovered"] == dlp["recovered"]
               for row in source["rho_trials"])
    formulas = formula_counts()
    verification = counts(source["transfer_source_verification_counts"])
    destination = counts(dlp["after_search_counts"])
    candidate = add(formulas, destination, verification)
    rho_runs = [counts(row["field_counts"]) for row in source["rho_trials"]]
    mean_rho = tuple(sum(row[i] for row in rho_runs) / len(rho_runs) for i in range(4))
    discovery = counts(source["order_discovery_field_counts"])
    mean_rho_with_discovery = add(mean_rho, discovery)
    prices = []
    for s in (0.0, 0.5, 1.0):
        for i in (1.0, 10.0, 100.0, 1000.0):
            for a in (0.0, 0.1, 1.0):
                c = price(candidate, s, i, a)
                r = price(mean_rho, s, i, a)
                rd = price(mean_rho_with_discovery, s, i, a)
                prices.append({"S_per_M": s, "I_per_M": i, "add_per_M": a,
                               "mapped_bsgs_GAE": c, "mean_source_rho_GAE": r,
                               "mapped_over_rho": c / r,
                               "mean_source_rho_with_order_discovery_GAE": rd,
                               "mapped_over_rho_with_order_discovery": c / rd})
    grid_path = ROOT / "results/tian-genus3-f257-cost-grid.csv"
    with grid_path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(prices[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(prices)
    representative = next(row for row in prices if row["S_per_M"] == 0.5
                          and row["I_per_M"] == 10 and row["add_per_M"] == 0.1)
    result = {
        "schema": "tian-genus3-f257-cost-analysis/v1",
        "fixture": "published F257 P1,P2 and corrected six-function map",
        "source_order": source["source_order"],
        "source_order_factors": source["source_order_factors"],
        "recovered": dlp["recovered"],
        "source_and_destination_verified": True,
        "source_map_executed": True,
        "bounded_generic_destination_dlp_executed": True,
        "source_rho_runs": len(rho_runs),
        "destination_baby_steps": dlp["baby_steps"],
        "destination_giant_steps": dlp["giant_steps"],
        "destination_split_fingerprints": [dlp["baby_split_fingerprints"],
                                           dlp["giant_split_fingerprints"]],
        "count_order": ["M", "S", "I", "field_adds"],
        "formula_evaluation_counts": formulas,
        "destination_setup_and_bsgs_counts": destination,
        "transfer_source_verification_counts": verification,
        "mapped_cold_total_counts": candidate,
        "rho_run_counts": rho_runs,
        "mean_source_rho_counts": mean_rho,
        "source_order_discovery_counts": discovery,
        "mean_rho_with_order_discovery_counts": mean_rho_with_discovery,
        "representative": representative,
        "mapped_over_rho_band": [min(row["mapped_over_rho"] for row in prices),
                                 max(row["mapped_over_rho"] for row in prices)],
        "mapped_over_rho_with_discovery_band": [
            min(row["mapped_over_rho_with_order_discovery"] for row in prices),
            max(row["mapped_over_rho_with_order_discovery"] for row in prices)],
        "mapped_losses": sum(row["mapped_over_rho"] > 1 for row in prices),
        "mapped_wins": sum(row["mapped_over_rho"] < 1 for row in prices),
        "scope": "one F257 published fixture; not a Smith-negative holdout result",
        "unpriced": "memory, hashing, comparison, control flow and qualified wall time",
    }
    (ROOT / "results/tian-genus3-f257-cost-summary.json").write_text(
        json.dumps(result, indent=2) + "\n")
    print(json.dumps(result["representative"], indent=2))


if __name__ == "__main__":
    main()
