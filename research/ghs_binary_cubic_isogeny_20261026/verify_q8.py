#!/usr/bin/env sage -python
"""Independent Sage replay of the GF(8) structural witnesses."""

import hashlib
import json
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "results_q8.json"
PRIORITY = {502, 514, 526, 542}


def quartic(field, small, embed, coeffs):
    ring = PolynomialRing(field, names=("X", "Y", "Z"))
    X, Y, Z = ring.gens()
    terms = [X**i * Y**j * Z**(4-i-j)
             for i in range(4, -1, -1) for j in range(4-i, -1, -1)]
    return Curve(sum((embed(small.from_integer(c)) * term
                      for c, term in zip(coeffs, terms)), ring.zero()))


def coordinates(P):
    return [int(P[0].to_integer()), int(P[1].to_integer())]


def main():
    raw = json.loads(RAW.read_text())
    assert raw["schema"] == "binary-cubic-quartic-q8-screen/v1"
    k = GF(8, "a", modulus="conway")
    L2 = GF(64, "u", modulus="conway")
    K = GF(512, "w", modulus="conway")
    embeds = ((k, lambda x: x), (L2, k.embeddings(L2)[0]),
              (K, k.embeddings(K)[0]))
    by_order = {}
    for row in raw["source_population"]:
        a, b = K.from_integer(row["a"]), K.from_integer(row["b"])
        E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
        assert len(E.points()) == row["order"]
        direct_class = "B" if b + b**8 + b**64 == 0 else ("A" if b**8 == b else "C")
        assert direct_class == row["direct_class"]
        by_order.setdefault(row["order"], []).append(row)
    assert len(raw["source_population"]) == 1022
    checked = []
    for witness in raw["witnesses"]:
        order = witness["group_order"]
        assert by_order[order] == witness["sources"]
        counts = []
        curves = []
        for field, embed in embeds:
            C = quartic(field, k, embed, witness["coefficients"])
            assert C.is_smooth()
            curves.append(C)
            counts.append(len(C.rational_points()))
        assert counts == witness["N1_N2_N3"]
        assert (513 - counts[2]) % 3 == 0
        t = (513 - counts[2]) // 3
        assert [1, 0, 0, -t, 0, 0, 512] == witness["weil_polynomial_coefficients"]
        assert order == 513 - t
        rec = {"group_order": order, "quartic_point_counts": counts,
               "source_models_recounted": len(witness["sources"]),
               "quartic_smooth": True,
               "source_to_quartic_map": "unconstructed"}
        if order in PRIORITY:
            source = witness["sources"][0]
            E = EllipticCurve(K, [K.one(), K.from_integer(source["a"]), K.zero(),
                                    K.zero(), K.from_integer(source["b"])])
            r, h = source["largest_prime_subgroup"], source["cofactor"]
            G = next((h * P for P in E.points() if not (h * P).is_zero()), None)
            assert G is not None and (r * G).is_zero()
            Q = 7 * G
            assert not Q.is_zero() and (r * Q).is_zero()
            C = curves[0]
            rational_points = C.rational_points()
            base = rational_points[0].place().divisor()
            jacobian_group = C.jacobian(model="hess", base_div=3 * base).group()
            assert jacobian_group.order() == order
            point_classes = [jacobian_group.point(P.place().divisor() - base)
                             for P in rational_points[1:]]
            k_elements = list(k)
            line_forms = ([(k.one(), u, v) for u in k_elements for v in k_elements]
                          + [(k.zero(), k.one(), v) for v in k_elements]
                          + [(k.zero(), k.zero(), k.one())])
            split_lines = sum(
                sum(A * P[0] + B * P[1] + D * P[2] == 0 for P in rational_points) == 4
                for A, B, D in line_forms)
            rec.update({"subgroup_order": r, "cofactor": h,
                        "source_a": source["a"], "source_b": source["b"],
                        "G": coordinates(G), "Q": coordinates(Q), "check_scalar": 7,
                        "rational_point_factor_base_size": len(rational_points),
                        "rational_point_classes_reach_prime_subgroup":
                            any(not (h * D).is_zero() for D in point_classes),
                        "fully_split_distinct_rational_lines": split_lines})
        checked.append(rec)
    out = {"schema": "binary-cubic-quartic-q8-independent-replay/v1",
           "source_models_recounted_total": 1022,
           "screen_sha256": hashlib.sha256(RAW.read_bytes()).hexdigest(),
           "verifier_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
           "checked": checked}
    (ROOT / "verification_q8.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"checked_orders": [r["group_order"] for r in checked],
                      "priority": [r for r in checked if r["group_order"] in PRIORITY]}))


if __name__ == "__main__":
    main()
