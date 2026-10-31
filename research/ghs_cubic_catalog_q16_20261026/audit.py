#!/usr/bin/env sage -python
"""Replay the finite cubic Jacobian catalog join and one curve witness."""

import collections
import hashlib
import json
import math
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, is_prime


ROOT = Path(__file__).resolve().parent
PARENT = ROOT.parent


def read(path):
    return json.loads(path.read_text())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source_path = PARENT / "ghs_binary_cubic_isogeny_q16_20261026/results.json"
    closure_path = PARENT / "ghs_cubic_tate_bridge_20261026/certificate.json"
    catalog_path = ROOT / "catalog_snapshot.json"
    source, closure, catalog = map(read, (source_path, closure_path, catalog_path))
    assert catalog["schema"] == "ghs-cubic-q16-catalog-snapshot/v1"
    assert closure["source_normal_forms_checked"] == 8190
    by_trace = {}
    for row in catalog["rows"]:
        c = [int(x) for x in row["poly_str"].split()]
        assert len(c) == 7 and c[:3] == [1, 0, 0] and c[4:] == [0, 0, 4096]
        t = -c[3]
        assert t not in by_trace
        assert row["has_jacobian"] in (0, 1)
        by_trace[t] = row
    assert len(by_trace) == 141

    direct_orders = set(closure["direct_B_isogeny_closure"]["direct_B_source_orders"])
    assert len(direct_orders) == 36
    by_order = collections.defaultdict(list)
    for s in source["source_population"]:
        by_order[s["order"]].append(s)
    assert len(by_order) == 128 and sum(map(len, by_order.values())) == 8190
    assert direct_orders == {n for n, rows in by_order.items()
                             if any(s["direct_class"] == "B" for s in rows)}

    counts = collections.Counter()
    outside_mt = collections.Counter()
    order_rows = []
    for n, rows in sorted(by_order.items()):
        t = 4097 - n
        catalog_row = by_trace[t]
        catalog_yes = bool(catalog_row["has_jacobian"])
        direct = n in direct_orders
        r = rows[0]["largest_prime_subgroup"]
        assert is_prime(r) and all(s["largest_prime_subgroup"] == r for s in rows)
        simple = math.gcd(r, 6 - 3*t) == 1
        for s in rows:
            assert s["order"] == n and s["cofactor"] * r == n
            assert (s["direct_class"] == "B") <= direct
            counts[("catalog_yes" if catalog_yes else "catalog_zero",
                    "direct" if direct else "outside_direct",
                    "simple" if simple else "non_simple")] += 1
            if catalog_yes and n % 8 in (4, 6):
                outside_mt[(s["direct_class"], "simple" if simple else "non_simple")] += 1
        order_rows.append({"order": n, "trace": t, "source_models": len(rows),
                           "catalog_label": catalog_row["label"],
                           "catalog_has_jacobian": int(catalog_yes),
                           "direct_B_order": direct, "largest_prime": r,
                           "simple_root_gate": simple})
    expected = {
        ("catalog_yes", "direct", "simple"): 2156,
        ("catalog_yes", "direct", "non_simple"): 452,
        ("catalog_yes", "outside_direct", "simple"): 5302,
        ("catalog_yes", "outside_direct", "non_simple"): 232,
        ("catalog_zero", "outside_direct", "simple"): 48,
    }
    assert dict(counts) == expected
    assert dict(outside_mt) == {("C", "simple"): 3852,
                                ("C", "non_simple"): 180,
                                ("A", "simple"): 12,
                                ("A", "non_simple"): 4}
    assert [x["order"] for x in order_rows if not x["catalog_has_jacobian"]] == [3972, 4222]

    # A cataloged class outside every direct-B isogeny class, selected before replay.
    K = GF(4096, "w", modulus="conway")
    aE, bE = K.from_integer(32), K.from_integer(306)
    E = EllipticCurve(K, [K.one(), aE, K.zero(), K.zero(), bE])
    assert E.cardinality() == 3974
    P = E(K.from_integer(307), K.from_integer(2825))
    assert P != E(0) and 1987 * P == E(0)
    assert bE + bE**16 + bE**256 != 0
    assert bE**16 != bE
    assert math.gcd(1987, 6 - 3*123) == 1
    k = GF(16, "a", modulus="conway")
    a = k.gen()
    R = PolynomialRing(k, "x")
    x = R.gen()
    h = a**2*x**4 + a*x
    f = x**8 + x**3 + a*x**2 + (a**2+a+1)*x + a**3+a+1
    assert catalog["order_3974_model"] == "y^2+(a^2*x^4+a*x)*y=x^8+x^3+a*x^2+(a^2+a+1)*x+a^3+a+1"
    C = HyperellipticCurve(f, h)
    assert C.genus() == 3
    point_counts = list(map(int, C.count_points(3)))
    assert point_counts == [17, 257, 3728]

    receipt = {
        "schema": "ghs-cubic-q16-catalog-audit/v1",
        "source_sha256": digest(source_path),
        "closure_sha256": digest(closure_path),
        "catalog_sha256": digest(catalog_path),
        "source_models": 8190,
        "source_orders": len(by_order),
        "catalog_positive_orders": sum(x["catalog_has_jacobian"] for x in order_rows),
        "catalog_positive_models": sum(x["source_models"] for x in order_rows if x["catalog_has_jacobian"]),
        "counts": {"/".join(k): v for k, v in sorted(counts.items())},
        "outside_menezes_teske_X": {"/".join(k): v for k, v in sorted(outside_mt.items())},
        "example": {"order": 3974, "trace": 123, "subgroup_prime": 1987,
                    "source_a": 32, "source_b": 306,
                    "subgroup_generator_xy": [307, 2825],
                    "curve_points_k_k2_k3": point_counts,
                    "model": catalog["order_3974_model"]},
        "orders": order_rows,
    }
    out = ROOT / "certificate.json"
    out.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps({k: receipt[k] for k in
                      ("source_models", "source_orders", "catalog_positive_orders",
                       "catalog_positive_models", "counts", "example")}, sort_keys=True))


if __name__ == "__main__":
    main()
