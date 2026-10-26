#!/usr/bin/env sage -python
"""Exact q16 ordinary-source census and simple-root certificates."""

import hashlib
import json
import math
from collections import Counter
from pathlib import Path

from sage.all import GF, is_prime


HERE = Path(__file__).resolve().parent
PARENT = HERE.parent / "ghs_binary_cubic_isogeny_q16_20261026"
RAW = PARENT / "results.json"
REPLAY = PARENT / "verification.json"
PRIOR = PARENT / "prior_work.json"
EXPECTED = {
    4036: {"a": 0, "r": 1009, "cofactor": 4, "models": 96},
    4124: {"a": 0, "r": 1031, "cofactor": 4, "models": 96},
    4126: {"a": 32, "r": 2063, "cofactor": 2, "models": 108},
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def all_trace_sums(field):
    """Walsh transform of the trace-of-inverse signs, built from scratch."""
    size = int(field.order())
    powers = [field.gen() ** j for j in range(12)]
    values = [0] * size
    for integer in range(1, size):
        u = field.from_integer(integer)
        index = 0
        for j, power in enumerate(powers):
            index |= int((u * power).trace()) << j
        values[index] = 1 - 2 * int((u ** -1).trace())
    assert values.count(0) == 1
    stride = 1
    while stride < size:
        for block in range(0, size, 2 * stride):
            for j in range(stride):
                left = values[block + j]
                right = values[block + j + stride]
                values[block + j] = left + right
                values[block + j + stride] = left - right
        stride *= 2
    return values


def main():
    if not RAW.is_file():
        raise SystemExit(
            "missing managed input results.json; run "
            "python3 tools/artifacts/artifacts.py pull "
            "research/ghs_binary_cubic_isogeny_q16_20261026"
        )
    raw = json.loads(RAW.read_text())
    replay = json.loads(REPLAY.read_text())
    prior = json.loads(PRIOR.read_text())
    assert raw["schema"] == "binary-cubic-quartic-q16-screen/v1"
    assert replay["schema"] == "binary-cubic-quartic-q16-replay/v1"
    assert replay["screen_sha256"] == digest(RAW)
    assert replay["source_models_walsh_verified"] == 8190
    field = GF(4096, "w", modulus="conway")
    q = 16
    q3 = q ** 3
    sums = all_trace_sums(field)
    rows = raw["source_population"]
    assert len(rows) == 2 * (q3 - 1) == 8190
    assert len({(x["a"], x["b"]) for x in rows}) == len(rows)
    assert {x["a"] for x in rows} == {0, 32}
    assert {x["b"] for x in rows} == set(range(1, q3))
    class_counts = Counter()
    for row in rows:
        a = field.from_integer(row["a"])
        b = field.from_integer(row["b"])
        sign = 1 - 2 * int(a.trace())
        assert row["order"] == q3 + 1 + sign * sums[row["b"]]
        classification = ("B" if b + b ** q + b ** (q * q) == 0
                          else "A" if b ** q == b else "C")
        assert row["direct_class"] == classification
        class_counts[classification] += 1
    assert class_counts == {"A": 2 * (q - 1),
                            "B": 2 * (q * q - 1),
                            "C": 2 * (q - 1) * (q * q - 1)}
    # Two elliptic curves over K are K-isogenous exactly when they have the
    # same trace, equivalently the same group order. This is a structural
    # isogeny-closure census, not an attack-cost or isogeny-walk census.
    direct_B_orders = sorted({x["order"] for x in rows if x["direct_class"] == "B"})
    direct_B_order_set = set(direct_B_orders)
    closure = [x for x in rows if x["order"] in direct_B_order_set]
    outside = [x for x in rows if x["order"] not in direct_B_order_set]
    closure_classes = Counter(x["direct_class"] for x in closure)
    outside_classes = Counter(x["direct_class"] for x in outside)
    assert len(direct_B_orders) == 36
    assert len(closure) == 2608 and len(outside) == 5582
    assert closure_classes == {"A": 10, "B": 510, "C": 2088}
    assert outside_classes == {"A": 20, "C": 5562}

    witnesses = {x["group_order"]: x for x in raw["witnesses"]}
    checks = {x["group_order"]: x for x in replay["checked"]}
    prior_classes = {x["source_order"]: x for x in prior["classes"]}
    assert set(EXPECTED) <= set(raw["highlighted_orders"])
    families = []
    for order, expected in EXPECTED.items():
        group = [x for x in rows if x["order"] == order]
        assert len(group) == expected["models"]
        assert all(x["a"] == expected["a"] and x["direct_class"] == "C"
                   for x in group)
        assert order not in direct_B_order_set
        assert all(x["largest_prime_subgroup"] == expected["r"]
                   and x["cofactor"] == expected["cofactor"] for x in group)
        assert order == expected["r"] * expected["cofactor"]
        assert bool(is_prime(expected["r"]))
        t = q3 + 1 - order
        derivative = 6 - 3 * t
        assert math.gcd(expected["r"], derivative) == 1
        assert math.gcd(expected["r"], q3 - 1) == 1
        witness = witnesses[order]
        prior_class = prior_classes[order]
        assert prior_class["catalogued_models"] == prior_class["hyperelliptic_models"]
        assert witness["weil_polynomial_coefficients"] == [1, 0, 0, -t, 0, 0, q3]
        assert witness["N1_N2_N3"] == [q + 1, q * q + 1, q3 + 1 - 3 * t]
        check = checks[order]
        assert check["jacobian_order"] == order
        assert check["subgroup_order"] == expected["r"]
        assert check["point_classes_reach_prime_subgroup"]
        values = sorted(x["b"] for x in group)
        assert len(values) == len(set(values))
        families.append({
            "group_order": order, "source_a": expected["a"],
            "source_b_values": values, "normalized_isomorphism_class_count": len(values),
            "direct_B_count": 0,
            "outside_direct_B_K_isogeny_closure": True,
            "selected_prime": expected["r"],
            "cofactor": expected["cofactor"], "trace_t": t,
            "frobenius_polynomial_coefficients": [1, 0, 0, -t, 0, 0, q3],
            "P_at_one": order, "P_derivative_at_one": derivative,
            "gcd_r_derivative": 1, "gcd_r_q3_minus_one": 1,
            "quartic_coefficients": witness["coefficients"],
            "prior_lmfdb_label": prior_class["label"],
            "prior_catalogued_models": prior_class["catalogued_models"],
            "quartic_point_counts": witness["N1_N2_N3"],
            "jacobian_order_replayed": check["jacobian_order"],
            "isogeny_exists_by_tate": True,
            "some_nonzero_prime_subgroup_isogeny_exists_by_tate": True,
            "evaluated_isogeny": False,
            "full_attack_cost": None,
        })
    assert sum(f["normalized_isomorphism_class_count"] for f in families) == 300
    out = {
        "schema": "ghs-cubic-tate-bridge-q16/v1",
        "field_order": q3, "base_field_order": q,
        "source_normal_forms_checked": len(rows),
        "direct_class_counts": dict(sorted(class_counts.items())),
        "direct_B_isogeny_closure": {
            "criterion": "source order belongs to the direct-B order set",
            "direct_B_source_orders": direct_B_orders,
            "isogeny_class_count": len(direct_B_orders),
            "normalized_models_in_closure": len(closure),
            "class_counts_in_closure": dict(sorted(closure_classes.items())),
            "normalized_models_outside_closure": len(outside),
            "class_counts_outside_closure": dict(sorted(outside_classes.items())),
        },
        "family_count": len(families), "eligible_normal_form_count": 300,
        "raw_sha256": digest(RAW), "replay_sha256": digest(REPLAY),
        "prior_work_sha256": digest(PRIOR),
        "protocol_sha256": digest(HERE / "PROTOCOL.md"),
        "audit_script_sha256": digest(Path(__file__)),
        "families": families,
    }
    (HERE / "certificate.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"verified": len(rows), "class_counts": out["direct_class_counts"],
                      "direct_B_isogeny_closure": len(closure),
                      "outside_direct_B_isogeny_closure": len(outside),
                      "eligible_normal_forms": 300,
                      "orders": [f["group_order"] for f in families]}))


if __name__ == "__main__":
    main()
