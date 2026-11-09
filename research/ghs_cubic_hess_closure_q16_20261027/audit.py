#!/usr/bin/env sage -python
"""Exhaustive normalized GF(4096) Hess genus-three closure audit."""

from collections import Counter, defaultdict
from hashlib import sha256
import json
from math import gcd
from pathlib import Path

from sage.all import EllipticCurve, GF, ZZ, is_prime


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SNAPSHOT = ROOT / "ghs_cubic_catalog_q16_20261026/catalog_snapshot.json"
CATCERT = ROOT / "ghs_cubic_catalog_q16_20261026/certificate.json"
PRIOR = ROOT / "ghs_cubic_tate_bridge_20261026/certificate.json"
QUARTIC = ROOT / "ghs_binary_cubic_isogeny_q16_20261026/verification.json"


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    K = GF(4096, "w", modulus="conway")
    q = 16
    zero = K(0)
    trace = lambda x: x + x ** q + x ** (q * q)
    U = {x for x in K if x != zero and trace(x) == zero}
    P = {x * y for x in U for y in U}
    assert len(U) == q * q - 1 == 255
    assert len(P) == (q - 1) * (q + 1) * (q + 2) // 2 == 2295
    assert P & U == U
    assert len(P - U) == 2040
    a_by_code = {0: K.from_integer(0), 32: K.from_integer(32)}
    assert {int(a.trace()) for a in a_by_code.values()} == {0, 1}

    catalog = json.loads(SNAPSHOT.read_text())
    catalog_certificate = json.loads(CATCERT.read_text())
    prior = json.loads(PRIOR.read_text())
    quartic = json.loads(QUARTIC.read_text())
    catalog_by_order = {}
    for row in catalog["rows"]:
        coeff = [int(x) for x in row["poly_str"].split()]
        if len(coeff) != 7 or coeff[:3] != [1, 0, 0] or coeff[4:] != [0, 0, 4096]:
            continue
        order = 4097 + coeff[3]
        assert order not in catalog_by_order
        catalog_by_order[order] = row

    groups = defaultdict(lambda: {0: {"hess": [], "outside": []},
                                  32: {"hess": [], "outside": []}})
    class_counts = Counter()
    full_orders = set()
    for a_code, a in a_by_code.items():
        for b_code in range(1, 4096):
            b = K.from_integer(b_code)
            order = int(EllipticCurve(K, [K(1), a, zero, zero, b]).cardinality())
            assert 3969 <= order <= 4225
            full_orders.add(order)
            delta = b.sqrt()
            hess = (delta in P) if a_code == 0 else (delta in U)
            branch = "hess" if hess else "outside"
            groups[order][a_code][branch].append(b_code)
            cls = "A" if b ** q == b else "B" if trace(b) == zero else "C"
            class_counts[(a_code, cls)] += 1
        print("completed_a", a_code, "orders_so_far", len(full_orders), flush=True)

    assert len(full_orders) == 128
    assert sum(sum(len(v) for v in by_branch.values())
               for by_a in groups.values() for by_branch in by_a.values()) == 8190
    assert all(class_counts[(a, "A")] == 15 and class_counts[(a, "B")] == 255
               and class_counts[(a, "C")] == 3825 for a in (0, 32))
    assert set(full_orders) <= set(catalog_by_order)
    prior_orders = {row["group_order"]: row for row in prior["families"]}
    for order, expected in prior_orders.items():
        a = expected["source_a"]
        all_b = sorted(groups[order][a]["hess"] + groups[order][a]["outside"])
        assert all_b == expected["source_b_values"]
        assert len(all_b) == expected["normalized_isomorphism_class_count"]
        assert catalog_by_order[order]["label"] == expected["prior_lmfdb_label"]

    records = []
    for order in sorted(full_orders):
        models = groups[order]
        status = catalog_by_order[order]["has_jacobian"]
        assert status in (0, 1)
        total = sum(len(v) for by_a in models.values() for v in by_a.values())
        hess_total = sum(len(models[a]["hess"]) for a in (0, 32))
        prime = max(int(p) for p, _ in ZZ(order).factor())
        assert bool(is_prime(prime))
        records.append({
            "order": order,
            "source_models": total,
            "hess_models": hess_total,
            "outside_hess_models": total - hess_total,
            "in_hess_isogeny_closure": hess_total > 0,
            "catalog_has_jacobian": status,
            "catalog_label": catalog_by_order[order]["label"],
            "largest_prime": prime,
            "simple_root_gate": gcd(prime, 3 * (4096 - 1)) == 1,
            "by_a": {str(a): models[a] for a in (0, 32)},
        })
    prior_catalog_orders = {row["order"]: row for row in catalog_certificate["orders"]}
    assert set(prior_catalog_orders) == full_orders
    for row in records:
        prior_row = prior_catalog_orders[row["order"]]
        assert row["source_models"] == prior_row["source_models"]
        assert row["catalog_has_jacobian"] == prior_row["catalog_has_jacobian"]
        assert row["catalog_label"] == prior_row["catalog_label"]
        assert row["simple_root_gate"] == prior_row["simple_root_gate"]

    summary = {
        "total_models": 8190,
        "total_orders": 128,
        "hess_models": sum(row["hess_models"] for row in records),
        "outside_hess_models": sum(row["outside_hess_models"] for row in records),
        "hess_closure_models": sum(row["source_models"] for row in records
                                   if row["in_hess_isogeny_closure"]),
        "outside_hess_closure_models": sum(row["source_models"] for row in records
                                           if not row["in_hess_isogeny_closure"]),
        "outside_hess_closure_orders": sum(not row["in_hess_isogeny_closure"]
                                           for row in records),
        "catalog_positive_outside_hess_closure_models": sum(
            row["source_models"] for row in records
            if not row["in_hess_isogeny_closure"] and row["catalog_has_jacobian"] == 1),
        "catalog_positive_outside_hess_closure_orders": sum(
            1 for row in records
            if not row["in_hess_isogeny_closure"] and row["catalog_has_jacobian"] == 1),
        "simple_root_positive_outside_hess_closure_models": sum(
            row["source_models"] for row in records
            if not row["in_hess_isogeny_closure"] and row["catalog_has_jacobian"] == 1
            and row["simple_root_gate"]),
    }
    focus = {}
    for order in (4036, 4124, 4126):
        row = next(row for row in records if row["order"] == order)
        focus[str(order)] = {
            "source_models": row["source_models"],
            "hess_models": row["hess_models"],
            "outside_hess_models": row["outside_hess_models"],
            "in_hess_isogeny_closure": row["in_hess_isogeny_closure"],
            "by_a": row["by_a"],
        }
    assert [(focus[str(x)]["hess_models"], focus[str(x)]["outside_hess_models"])
            for x in (4036, 4124, 4126)] == [(48, 48), (84, 12), (0, 108)]
    assert not focus["4126"]["in_hess_isogeny_closure"]
    record_by_order = {row["order"]: row for row in records}
    quartic_orders = sorted({int(row["group_order"]) for row in quartic["checked"]})
    assert len(quartic_orders) == 11
    outside_quartic_orders = [order for order in quartic_orders
                              if not record_by_order[order]["in_hess_isogeny_closure"]]
    assert outside_quartic_orders == [3978, 4030, 4094, 4126]
    summary["verified_quartic_outside_hess_closure_orders"] = len(outside_quartic_orders)
    summary["verified_quartic_outside_hess_closure_models"] = sum(
        record_by_order[order]["source_models"]
        for order in outside_quartic_orders)
    assert summary["verified_quartic_outside_hess_closure_models"] == 258

    output = {
        "schema": "ghs-cubic-hess-closure-q16/v1",
        "protocol_sha256": digest(HERE / "PROTOCOL.md"),
        "script_sha256": digest(HERE / "audit.py"),
        "catalog_snapshot_sha256": digest(SNAPSHOT),
        "catalog_certificate_sha256": digest(CATCERT),
        "prior_family_certificate_sha256": digest(PRIOR),
        "quartic_verification_sha256": digest(QUARTIC),
        "field": {"base_q": q, "extension_order": 4096,
                  "conway_polynomial": "w^12+w^7+w^6+w^5+w^3+w+1",
                  "a_codes": [0, 32]},
        "product_set_count": len(P),
        "trace_zero_count": len(U),
        "class_counts": {f"a{a}_{cls}": class_counts[(a, cls)]
                         for a in (0, 32) for cls in ("A", "B", "C")},
        "summary": summary,
        "focus": focus,
        "verified_quartic_outside_hess_closure_orders": outside_quartic_orders,
        "orders": records,
    }
    (HERE / "certificate.json").write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps({"summary": summary,
                      "focus": {order: {k: v for k, v in row.items() if k != "by_a"}
                                for order, row in focus.items()}}, indent=2), flush=True)


if __name__ == "__main__":
    main()
