#!/usr/bin/env sage -python
"""Deterministic cubic-Weil-restriction/plane-quartic structural screen."""

from __future__ import annotations

import hashlib
import json
import random
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing, factor


ROOT = Path(__file__).resolve().parent
SEED = 20261026
DRAW_COUNT = 2048
EXPONENTS = [(i, j, 4 - i - j) for i in range(4, -1, -1) for j in range(4 - i, -1, -1)]
assert len(EXPONENTS) == 15 and EXPONENTS[0] == (4, 0, 0)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def point_monomials(field):
    elements = list(field)
    one, zero = field.one(), field.zero()
    points = [(one, u, v) for u in elements for v in elements]
    points += [(zero, one, v) for v in elements]
    points.append((zero, zero, one))
    assert len(points) == field.order() ** 2 + field.order() + 1
    return [[X**i * Y**j * Z**k for i, j, k in EXPONENTS] for X, Y, Z in points]


def count_zero(coeffs, matrix, embedding, k):
    mapped = [embedding(k.from_integer(c)) for c in coeffs]
    zero = mapped[0].parent().zero()
    return sum(sum((a * m for a, m in zip(mapped, row)), zero) == zero for row in matrix)


def quartic(coeffs, field, embedding, k):
    ring = PolynomialRing(field, names=("x", "y", "z"))
    x, y, z = ring.gens()
    return sum((embedding(k.from_integer(c)) * x**i * y**j * z**h
                for c, (i, j, h) in zip(coeffs, EXPONENTS)), ring.zero())


def source_curves(K):
    twist_a = next(K.from_integer(i) for i in range(1, K.order())
                   if K.from_integer(i).trace() == 1)
    rows = []
    by_order = {}
    for a in (K.zero(), twist_a):
        for b_int in range(1, K.order()):
            b = K.from_integer(b_int)
            E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
            order = int(E.cardinality())
            prime = max(int(p) for p, _ in factor(order))
            trace_b = b + b**4 + b**16
            direct_class = "B" if trace_b == 0 else ("A" if b**4 == b else "C")
            row = {"a": int(a.to_integer()), "b": b_int, "order": order,
                   "largest_prime_subgroup": prime, "cofactor": order // prime,
                   "direct_class": direct_class,
                   "relative_trace_b": int(trace_b.to_integer())}
            rows.append(row)
            by_order.setdefault(order, []).append(row)
    assert len(rows) == 126
    assert {r["direct_class"] for r in rows} == {"A", "B", "C"}
    return rows, by_order


def main():
    started = time.monotonic()
    k = GF(2**2, "a", modulus="conway")
    L2 = GF(2**4, "u", modulus="conway")
    K = GF(2**6, "w", modulus="conway")
    emb1 = lambda x: x
    emb2 = k.embeddings(L2)[0]
    emb3 = k.embeddings(K)[0]
    source, by_order = source_curves(K)
    mats = {1: point_monomials(k), 2: point_monomials(L2), 3: point_monomials(K)}
    rng = random.Random(SEED)
    tallies = {"drawn": 0, "N1_5": 0, "N2_17": 0, "smooth": 0,
               "integral_trace": 0, "source_order_match": 0}
    witnesses = {}
    for index in range(DRAW_COUNT):
        coeffs = [1] + [rng.randrange(4) for _ in range(14)]
        tallies["drawn"] += 1
        if count_zero(coeffs, mats[1], emb1, k) != 5:
            continue
        tallies["N1_5"] += 1
        if count_zero(coeffs, mats[2], emb2, k) != 17:
            continue
        tallies["N2_17"] += 1
        f = quartic(coeffs, k, emb1, k)
        if not Curve(f).is_smooth():
            continue
        tallies["smooth"] += 1
        N3 = count_zero(coeffs, mats[3], emb3, k)
        if (65 - N3) % 3:
            continue
        tallies["integral_trace"] += 1
        t = (65 - N3) // 3
        order = 65 - t
        if order not in by_order:
            continue
        tallies["source_order_match"] += 1
        if order in witnesses:
            continue
        sources = by_order[order]
        independent = []
        for field, emb in ((k, emb1), (L2, emb2), (K, emb3)):
            independent.append(len(Curve(quartic(coeffs, field, emb, k)).rational_points()))
        assert independent == [5, 17, N3]
        witnesses[order] = {"draw_index": index, "coefficients": coeffs,
                            "equation": str(f), "N1_N2_N3": independent,
                            "weil_polynomial_coefficients": [1, 0, 0, -t, 0, 0, 64],
                            "group_order": order,
                            "class_B_source_present": any(s["direct_class"] == "B" for s in sources),
                            "sources": sources}
    order_classes = {str(order): {"source_models": len(rows),
                                  "classes": sorted({r["direct_class"] for r in rows}),
                                  "best_subgroup_prime": max(r["largest_prime_subgroup"] for r in rows),
                                  "witness": order in witnesses}
                     for order, rows in sorted(by_order.items())}
    result = {"schema": "binary-cubic-quartic-screen/v1", "seed": SEED,
              "draw_count": DRAW_COUNT, "sage_version": __import__("sage.version").version.version,
              "fields": {"k": str(k.modulus()), "K": str(K.modulus()),
                         "k_to_GF16": str(emb2(k.gen())), "k_to_K": str(emb3(k.gen()))},
              "source_population": source, "source_order_classes": order_classes,
              "tallies": tallies, "witnesses": [witnesses[n] for n in sorted(witnesses)],
              "elapsed_seconds_diagnostic": time.monotonic() - started,
              "protocol_sha256": sha256(ROOT / "PROTOCOL.md"),
              "script_sha256": sha256(Path(__file__)),
              "limits": "Weil-polynomial witnesses only; no evaluated source-to-quartic isogeny, subgroup map, DLP, or attack cost"}
    (ROOT / "results.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"tallies": tallies, "source_orders": len(by_order),
                      "witness_orders": sorted(witnesses),
                      "class_C_only_witness_orders": [n for n in sorted(witnesses)
                                                      if not witnesses[n]["class_B_source_present"]]}))


if __name__ == "__main__":
    main()
