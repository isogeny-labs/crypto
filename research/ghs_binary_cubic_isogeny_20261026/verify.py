#!/usr/bin/env sage -python
"""Independent curve-point and source-subgroup replay for screen results."""

import hashlib
import json
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "results.json"


def curve_from_coefficients(field, embed, small_field, coeffs):
    ring = PolynomialRing(field, names=("X", "Y", "Z"))
    X, Y, Z = ring.gens()
    terms = []
    for i in range(4, -1, -1):
        for j in range(4 - i, -1, -1):
            terms.append(X**i * Y**j * Z**(4 - i - j))
    return Curve(sum((embed(small_field.from_integer(c)) * m
                      for c, m in zip(coeffs, terms)), ring.zero()))


def point_record(P):
    return [int(P[0].to_integer()), int(P[1].to_integer())]


def main():
    raw = json.loads(RAW.read_text())
    assert raw["schema"] == "binary-cubic-quartic-screen/v1"
    k = GF(4, "a", modulus="conway")
    k2 = GF(16, "u", modulus="conway")
    K = GF(64, "w", modulus="conway")
    embeddings = [(k, lambda c: c), (k2, k.embeddings(k2)[0]),
                  (K, k.embeddings(K)[0])]
    assert len(raw["source_population"]) == 126
    recounted_by_order = {}
    for row in raw["source_population"]:
        a = K.from_integer(row["a"])
        b = K.from_integer(row["b"])
        E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
        assert len(E.points()) == row["order"]
        direct_class = "B" if b + b**4 + b**16 == 0 else ("A" if b**4 == b else "C")
        assert direct_class == row["direct_class"]
        recounted_by_order.setdefault(row["order"], []).append(row)
    checked = []
    for witness in raw["witnesses"]:
        order = witness["group_order"]
        counts = []
        for field, embed in embeddings:
            C = curve_from_coefficients(field, embed, k, witness["coefficients"])
            assert C.is_smooth()
            counts.append(len(C.rational_points()))
        assert counts == witness["N1_N2_N3"]
        t = (65 - counts[2]) // 3
        assert 65 - counts[2] == 3 * t
        assert [1, 0, 0, -t, 0, 0, 64] == witness["weil_polynomial_coefficients"]
        assert order == 65 - t
        assert recounted_by_order[order] == witness["sources"]
        # Check every source model attached to this class by independent point
        # enumeration. The group generator below is selected for the first.
        for row in witness["sources"]:
            a = K.from_integer(row["a"])
            b = K.from_integer(row["b"])
            E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
            assert len(E.points()) == order
            assert (b + b**4 + b**16 == 0) == (row["direct_class"] == "B")
        row = witness["sources"][0]
        E = EllipticCurve(K, [K.one(), K.from_integer(row["a"]), K.zero(),
                                K.zero(), K.from_integer(row["b"])])
        r = row["largest_prime_subgroup"]
        h = row["cofactor"]
        G = next((h * P for P in E.points() if not (h * P).is_zero()), None)
        assert G is not None and (r * G).is_zero()
        scalar = max(1, min(7, r - 2))
        Q = scalar * G
        assert not Q.is_zero() and (r * Q).is_zero()
        C = curve_from_coefficients(k, embeddings[0][1], k, witness["coefficients"])
        rational_points = C.rational_points()
        base = rational_points[0].place().divisor()
        jacobian_group = C.jacobian(model="hess", base_div=3 * base).group()
        assert jacobian_group.order() == order
        point_classes = [jacobian_group.point(P.place().divisor() - base)
                         for P in rational_points[1:]]
        factor_base_reaches_r = any(not (h * D).is_zero() for D in point_classes)
        k_elements = list(k)
        line_forms = ([(k.one(), u, v) for u in k_elements for v in k_elements]
                      + [(k.zero(), k.one(), v) for v in k_elements]
                      + [(k.zero(), k.zero(), k.one())])
        fully_split_distinct_lines = sum(
            sum(A * P[0] + B * P[1] + Cc * P[2] == 0 for P in rational_points) == 4
            for A, B, Cc in line_forms)
        checked.append({"group_order": order, "subgroup_order": r,
                        "cofactor": h, "source_a": row["a"], "source_b": row["b"],
                        "source_class": row["direct_class"], "G": point_record(G),
                        "Q": point_record(Q), "check_scalar": scalar,
                        "quartic_point_counts": counts,
                        "rational_point_factor_base_size": len(rational_points),
                        "rational_point_classes_reach_prime_subgroup": factor_base_reaches_r,
                        "fully_split_distinct_rational_lines": fully_split_distinct_lines,
                        "source_models_recounted": len(witness["sources"]),
                        "quartic_smooth": True,
                        "source_to_quartic_map": "unconstructed"})
    result = {"schema": "binary-cubic-quartic-independent-replay/v1",
              "screen_sha256": hashlib.sha256(RAW.read_bytes()).hexdigest(),
              "verifier_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              "source_models_recounted_total": len(raw["source_population"]),
              "checked": checked}
    (ROOT / "verification.json").write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"checked_orders": [r["group_order"] for r in checked],
                      "source_models_recounted": sum(r["source_models_recounted"] for r in checked)}))


if __name__ == "__main__":
    main()
