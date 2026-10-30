#!/usr/bin/env python3
"""Recompute cost headroom from the frozen five-lead receipts."""

import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1] / "five_leads_e2e_20261019"
S, I, NATIVE = 0.5, 10.0, 0.1


def vector_price(vector):
    return vector[0] + S * vector[1] + I * vector[2] + NATIVE * vector[3]


def main():
    totals = {"genus_projective": Counter(), "rho_neg": Counter()}
    rows = Counter()
    prices = Counter()
    additions_price = Counter()
    logical_price = Counter()
    divisors = Counter()
    for bits in (16, 20, 24, 28, 32, 36, 40):
        path = ROOT / "sessions_projective_chart_v2" / f"order-genus-{bits}" / "records.jsonl"
        for line in path.open():
            record = json.loads(line)
            arm = record["arm"]
            if arm not in totals:
                continue
            assert record["outcome"]["status"] == "verified"
            c = record["counters"]
            unit = c["source_add_multiplications"] + S * c["source_add_squarings"] + I * c["source_add_inversions"]
            assert unit > 0
            arithmetic = (c["field_multiplications"] + S * c["field_squarings"] + I * c["field_inversions"]) / unit
            logical = NATIVE * sum(value for key, value in c.items() if key.endswith("_uncharged"))
            totals[arm].update(c)
            rows[arm] += 1
            prices[arm] += arithmetic + logical
            logical_price[arm] += logical
            additions_price[arm] += NATIVE * c["field_additions_uncharged"]
            divisors[arm] += c.get("processed_divisors", 0)
    assert rows == {"genus_projective": 1260, "rho_neg": 1260}
    reference = json.loads((ROOT / "results/genus2-projective-summary.json").read_text())["summary"][0]["representative"]
    assert abs(prices["genus_projective"] - reference["genus_projective_GAE"]) < 0.01
    assert abs(prices["rho_neg"] - reference["rho_neg_GAE"]) < 0.01
    candidate = prices["genus_projective"]
    rho = prices["rho_neg"]
    quartic = json.loads((ROOT / "results/tian-genus3-f257-line-sieve-cost-summary.json").read_text())
    raw = json.loads((ROOT / "correctness/tian-genus3-f257-line-sieve-mapped-dlp-v1.json").read_text())
    initial = quartic["initial_curve_counts"]
    mapped = quartic["mapped_setup_descent_and_destination_verification_counts"]
    scalar = quartic["scalar_relation_and_recovery_counts"]
    source_check = quartic["source_verification_counts"]
    cold = quartic["mapped_cold_total_counts"]
    assert [sum(parts) for parts in zip(initial, mapped, scalar, source_check)] == cold
    after_descent = raw["descent"]["after_descent_curve_field_counts"]
    keys = ("m", "s", "i", "field_adds")
    descent_setup = [after_descent[k] for k in keys]
    after_verification = raw["descent"]["after_verification_curve_field_counts"]
    assert [after_verification[k] for k in keys] == mapped
    verification_increment = [mapped[j] - descent_setup[j] for j in range(4)]
    cold_price = vector_price(cold)
    output = {
        "schema": "next-leads-cost-headroom/v1",
        "inputs": [
            "research/five_leads_e2e_20261019/sessions_projective_chart_v2/order-genus-{16,20,24,28,32,36,40}/records.jsonl",
            "research/five_leads_e2e_20261019/results/genus2-projective-summary.json",
            "research/five_leads_e2e_20261019/results/tian-genus3-f257-line-sieve-cost-summary.json",
            "research/five_leads_e2e_20261019/correctness/tian-genus3-f257-line-sieve-mapped-dlp-v1.json",
        ],
        "price": {"S_per_M": S, "I_per_M": I, "native_per_logical": NATIVE},
        "genus2": {
            "paired_targets": rows["genus_projective"],
            "projective_GAE": candidate,
            "rho_GAE": rho,
            "projective_over_rho": candidate / rho,
            "processed_divisors": divisors["genus_projective"],
            "projective_field_additions": totals["genus_projective"]["field_additions_uncharged"],
            "projective_field_additions_only_GAE": additions_price["genus_projective"],
            "field_additions_only_over_rho": additions_price["genus_projective"] / rho,
            "projective_all_logical_GAE": logical_price["genus_projective"],
            "all_logical_over_rho": logical_price["genus_projective"] / rho,
            "max_total_cost_fraction_to_tie_rho": rho / candidate,
            "minimum_total_cost_reduction_fraction_to_tie_rho": 1 - rho / candidate,
        },
        "genus3_f257": {
            "cold_GAE": cold_price,
            "initial_relation_GAE": vector_price(initial),
            "initial_relation_fraction_of_cold": vector_price(initial) / cold_price,
            "mapped_setup_and_descent_GAE": vector_price(descent_setup),
            "destination_verification_increment_GAE": vector_price(verification_increment),
            "scalar_relation_and_recovery_GAE": vector_price(scalar),
            "source_verification_GAE": vector_price(source_check),
            "mapped_setup_descent_destination_verification_GAE": vector_price(mapped),
        },
        "interpretation": "Algebraic counterfactuals from observed counters, not measured implementations. Genus-2 floors hold the existing logical-operation counts fixed. Genus-3 mapped setup and descent are inseparable in these snapshots.",
    }
    print(json.dumps(output, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
