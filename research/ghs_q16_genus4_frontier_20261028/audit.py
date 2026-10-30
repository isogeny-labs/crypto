#!/usr/bin/env python3
"""Validate retained calculator receipts and derive a bounded result table."""

import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_nonsplit_order4222_20261028" / "frontier_q16.json"
frontier_bytes = FRONTIER.read_bytes()
frontier = json.loads(frontier_bytes)
classes = {3974: "3.16.a_a_aet", 3978: "3.16.a_a_aep", 4178: "3.16.a_a_dd"}


def receipt(stem, order, schema):
    row = json.loads((HERE / f"{stem}_{order}_receipt.json").read_text())
    assert row["schema"] == schema and row["http_status"] == 200
    assert row["frontier_sha256"] == hashlib.sha256(frontier_bytes).hexdigest()
    assert row["input_sha256"] == hashlib.sha256(
        (HERE / f"{stem}_{order}.magma").read_bytes()).hexdigest()
    answer = row["response"]
    assert answer["success"] is True and answer["exit_code"] == 0
    assert answer["truncated"] is False and answer["magma"]["version"] == "2.30-1"
    out = answer["stdout"]
    assert "User error:" not in out and "Runtime error:" not in out
    return out


def extract(pattern, text, cast=int):
    match = re.search(pattern, text)
    assert match, pattern
    return cast(match.group(1))


report = {"schema": "ghs-q16-genus4-frontier-audit-v1", "classes": []}
for candidate in frontier["candidates"]:
    n = candidate["source_order"]
    r = candidate["subgroup_prime"]
    masks = candidate["all_source_b_masks"]
    sweep = receipt("sweep", n, "ghs-q16-frontier-all-models-all-parameters-v1")
    hits = re.findall(
        r"B (\d+) GENUS_HIST \[ <4, 30>, <6, 450>, <7, 3585> \]\s+ERRORS 30",
        sweep,
    )
    assert [int(x) for x in hits] == masks
    maps = receipt("map", n, "ghs-q16-frontier-all-models-explicit-g4-map-v1")
    map_hits = re.findall(r"B (\d+) J_ORDER (\d+)", maps)
    assert [int(x) for x, _ in map_hits] == masks
    assert maps.count(" CHECK true") == len(masks)
    per_model = []
    for b_text, jac_text in map_hits:
        b, jac = int(b_text), int(jac_text)
        assert jac % n == 0 and jac % r == 0
        per_model.append({
            "a_mask": 32, "b_mask": b, "source_order": n,
            "subgroup_prime": r, "jacobian_order": jac,
            "extra_factor_order": jac // n,
            "lmfdb_class": classes[n],
            "prior_work": {
                "known_isogeny_class": True,
                "lmfdb_url": "https://www.lmfdb.org/Variety/Abelian/Fq/3/16/" + classes[n].split(".")[-1],
                "known_standard_ghs_construction": True,
                "ghs_source": "https://www.kantmath.de/publications/papers/weil_descent.pdf",
                "exact_equation_publication_novelty": "unverified",
            },
            "c_equals_1_genus_four_subgroup_map": True,
            "parameter_histogram": {"genus4": 30, "genus6": 450,
                                    "genus7": 3585, "intrinsic_rejection": 30,
                                    "genus3": 0},
        })
    attack = receipt("order", n, "ghs-q16-genus4-end-to-end-v1")
    assert f"SOURCE {n} PRIME {r} " in attack
    assert "TARGET_FREE false" in attack
    assert "RELATION_MATRIX" in attack and "RELATION_RHS" in attack
    assert "INDEX_LOG 7" in attack and "VERIFIED true" in attack
    assert "RHO_TRIALS 1000" in attack
    ghs_cpu = extract(r"FULL_CPU ([\d.]+)", attack, float)
    rho_cpu = extract(r"RHO_CPU ([\d.]+)", attack, float)
    record = {
        "source_order": n, "representative_b_mask": candidate["source_b"],
        "subgroup_prime": r, "lmfdb_class": classes[n],
        "models": per_model,
        "representative_attack": {
            "index_log": 7, "relation_count": extract(r"RELATIONS (\d+)", attack),
            "relation_walk_steps": extract(r"WALK_STEPS (\d+)", attack),
            "full_ghs_cpu_seconds": ghs_cpu,
            "rho_successes": 1000,
            "rho_attempts": extract(r"RHO_ATTEMPTS (\d+)", attack),
            "rho_total_steps": extract(r"RHO_STEPS (\d+)", attack),
            "rho_total_cpu_seconds": rho_cpu,
            "rho_mean_cpu_seconds": rho_cpu / 1000,
            "ghs_over_mean_rho_cpu": ghs_cpu / (rho_cpu / 1000),
        },
    }
    report["classes"].append(record)

assert sum(len(x["models"]) for x in report["classes"]) == 66
split_receipt = json.loads((HERE / "smooth17_receipt.json").read_text())
assert split_receipt["schema"] == "ghs-q16-order17-all-g4-split-obstruction-v1"
assert split_receipt["frontier_sha256"] == hashlib.sha256(frontier_bytes).hexdigest()
assert split_receipt["input_sha256"] == hashlib.sha256(
    (HERE / "smooth17.magma").read_bytes()).hexdigest()
assert split_receipt["http_status"] == 200
assert split_receipt["response"]["success"] is True
assert split_receipt["response"]["truncated"] is False
assert split_receipt["response"]["stdout"].strip() == (
    "GENUS4_PARAMS 30 ALL_ZERO_SPLIT true GENUS6 450 GENUS7 3585 REJECTIONS 30"
)
irr = json.loads((HERE / "irreducibility_receipt.json").read_text())
assert irr["schema"] == "ghs-q16-simple-weil-polynomial-v1"
assert len(irr["records"]) == 3
for cert, (n, p) in zip(irr["records"], [(3974, 11), (3978, 11), (4178, 5)]):
    trace = 4097 - n
    assert cert["source_order"] == n and cert["trace"] == trace
    assert cert["modulus_prime"] == p
    assert cert["weil_polynomial_high_to_low"] == [1, 0, 0, -trace, 0, 0, 4096]
    assert cert["gcd_x_p2_minus_x"] == cert["gcd_x_p3_minus_x"] == "1"
    assert cert["x_p6_mod_f"] == "z" and cert["irreducible_mod_p"] is True
for row in report["classes"]:
    row["algebraic_transfer_genus_lower_bound"] = 3
(HERE / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
for row in report["classes"]:
    a = row["representative_attack"]
    print(row["source_order"], "models", len(row["models"]),
          "relations", a["relation_count"], "GHS/Rho", round(a["ghs_over_mean_rho_cpu"]))
