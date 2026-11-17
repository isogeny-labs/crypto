#!/usr/bin/env sage -python
"""Independent Walsh point-count and alternate-chart quartic replay."""

import hashlib
import json
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "results.json"
EXPONENTS = [(i, j, 4 - i - j) for i in range(4, -1, -1)
             for j in range(4 - i, -1, -1)]


def walsh_orders(K):
    """All #E(a,b) from the additive transform of Tr(u^-1)."""
    q = K.order()
    basis = [K.gen()**i for i in range(12)]
    table = [0] * q
    for integer in range(1, q):
        u = K.from_integer(integer)
        dual_bits = sum(int((u * basis[i]).trace()) << i for i in range(12))
        table[dual_bits] = 1 if int((u**-1).trace()) == 0 else -1
    assert table.count(0) == 1
    span = 1
    while span < q:
        for base in range(0, q, 2 * span):
            for offset in range(span):
                i, j = base + offset, base + offset + span
                x, y = table[i], table[j]
                table[i], table[j] = x + y, x - y
        span *= 2
    return table


def quartic(field, small, embed, coefficients):
    ring = PolynomialRing(field, names=("X", "Y", "Z"))
    X, Y, Z = ring.gens()
    return Curve(sum((embed(small.from_integer(c)) * X**i * Y**j * Z**h
                      for c, (i, j, h) in zip(coefficients, EXPONENTS)), ring.zero()))


def count_alternate_chart(field, small, embed, coefficients):
    """Charts (u:1:v), (1:0:v), (0:0:1), unlike the screen."""
    mapped = [embed(small.from_integer(c)) for c in coefficients]
    ring = PolynomialRing(field, "v")
    v = ring.gen()
    total = 0
    for u in field:
        powers = [field.one(), u, u**2, u**3, u**4]
        polynomial = sum((mapped[n] * powers[i] * v**h
                          for n, (i, _, h) in enumerate(EXPONENTS)), ring.zero())
        assert polynomial != 0
        total += len(polynomial.roots(multiplicities=False))
    boundary = sum((mapped[n] * v**h for n, (_, j, h) in enumerate(EXPONENTS)
                    if j == 0), ring.zero())
    assert boundary != 0
    total += len(boundary.roots(multiplicities=False))
    total += int(mapped[-1] == 0)
    return total


def coords(P):
    return [int(P[0].to_integer()), int(P[1].to_integer())]


def main():
    raw = json.loads(RAW.read_text())
    assert raw["schema"] == "binary-cubic-quartic-q16-screen/v1"
    k = GF(16, "a", modulus="conway")
    L2 = GF(256, "u", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    transforms = walsh_orders(K)
    for row in raw["source_population"]:
        a = K.from_integer(row["a"])
        sign = 1 if int(a.trace()) == 0 else -1
        assert row["order"] == 4097 + sign * transforms[row["b"]]
        b = K.from_integer(row["b"])
        assert (b + b**16 + b**256 == 0) == (row["direct_class"] == "B")
    assert len(raw["source_population"]) == 8190
    embeddings = ((k, lambda x: x), (L2, k.embeddings(L2)[0]),
                  (K, k.embeddings(K)[0]))
    checked = []
    for witness in raw["witnesses"]:
        order = witness["group_order"]
        counts = []
        for field, embed in embeddings:
            C = quartic(field, k, embed, witness["coefficients"])
            assert C.is_smooth()
            counts.append(count_alternate_chart(field, k, embed,
                                                witness["coefficients"]))
        assert counts == witness["N1_N2_N3"]
        t = (4097 - counts[2]) // 3
        assert counts[2] == 4097 - 3 * t
        assert order == 4097 - t
        assert witness["weil_polynomial_coefficients"] == [1, 0, 0, -t, 0, 0, 4096]
        rec = {"group_order": order, "point_counts": counts,
               "quartic_smooth": True, "source_to_quartic_map": "unconstructed"}
        if order in raw["highlighted_orders"]:
            source = next(r for r in witness["sources"]
                          if r["largest_prime_subgroup"] >= 1000
                          and r["cofactor"] <= 4)
            a = K.from_integer(source["a"])
            b = K.from_integer(source["b"])
            E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
            r, h = source["largest_prime_subgroup"], source["cofactor"]
            G = next((h * P for P in E.points() if not (h * P).is_zero()), None)
            assert G is not None and (r * G).is_zero()
            Q = 7 * G
            assert not Q.is_zero() and (r * Q).is_zero()
            C = quartic(k, k, lambda x: x, witness["coefficients"])
            points = C.rational_points()
            base = points[0].place().divisor()
            J = C.jacobian(model="hess", base_div=3 * base).group()
            assert J.order() == order
            classes = [J.point(P.place().divisor() - base) for P in points[1:]]
            elements = list(k)
            lines = ([(k.one(), u, v) for u in elements for v in elements]
                     + [(k.zero(), k.one(), v) for v in elements]
                     + [(k.zero(), k.zero(), k.one())])
            split_lines = sum(
                sum(A * P[0] + B * P[1] + D * P[2] == 0 for P in points) == 4
                for A, B, D in lines)
            rec.update({"source_a": source["a"], "source_b": source["b"],
                        "subgroup_order": r, "cofactor": h,
                        "G": coords(G), "Q": coords(Q), "check_scalar": 7,
                        "jacobian_order": int(J.order()),
                        "rational_point_class_count": len(points),
                        "fully_split_distinct_rational_lines": split_lines,
                        "point_classes_reach_prime_subgroup":
                            any(not (h * D).is_zero() for D in classes)})
        checked.append(rec)
    out = {"schema": "binary-cubic-quartic-q16-replay/v1",
           "source_models_walsh_verified": 8190,
           "screen_sha256": hashlib.sha256(RAW.read_bytes()).hexdigest(),
           "verifier_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
           "checked": checked}
    (ROOT / "verification.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"walsh_verified": 8190,
                      "checked_orders": [x["group_order"] for x in checked],
                      "highlighted": [x for x in checked
                                      if x["group_order"] in raw["highlighted_orders"]]}))


if __name__ == "__main__":
    main()
