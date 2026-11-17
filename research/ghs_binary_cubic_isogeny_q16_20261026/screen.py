#!/usr/bin/env sage -python
"""Frozen GF(16)/GF(4096) cubic binary quartic class screen."""

import hashlib
import json
import random
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing, factor


ROOT = Path(__file__).resolve().parent
SEED = 2026102616
DRAW_COUNT = 2048
EXPONENTS = [(i, j, 4 - i - j) for i in range(4, -1, -1)
             for j in range(4 - i, -1, -1)]
assert len(EXPONENTS) == 15


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def equation(coefficients, small):
    ring = PolynomialRing(small, names=("x", "y", "z"))
    x, y, z = ring.gens()
    return sum((small.from_integer(c) * x**i * y**j * z**h
                for c, (i, j, h) in zip(coefficients, EXPONENTS)), ring.zero())


def point_monomials(small):
    one, zero = small.one(), small.zero()
    points = [(one, y, z) for y in small for z in small]
    points += [(zero, one, z) for z in small]
    points.append((zero, zero, one))
    return [[x**i * y**j * z**h for i, j, h in EXPONENTS]
            for x, y, z in points]


def count_n1(coefficients, matrix, small):
    coefficients = [small.from_integer(c) for c in coefficients]
    return sum(sum((c * m for c, m in zip(coefficients, row)), small.zero()) == 0
               for row in matrix)


def count_by_roots(coefficients, small, large, embed):
    """Projective charts (1:u:v), (0:1:v), and (0:0:1)."""
    mapped = [embed(small.from_integer(c)) for c in coefficients]
    ring = PolynomialRing(large, "v")
    v = ring.gen()
    count = 0
    for u in large:
        powers = [large.one(), u, u**2, u**3, u**4]
        polynomial = sum((mapped[n] * powers[j] * v**h
                          for n, (_, j, h) in enumerate(EXPONENTS)), ring.zero())
        assert polynomial != 0
        count += len(polynomial.roots(multiplicities=False))
    infinity = sum((mapped[n] * v**h for n, (i, _, h) in enumerate(EXPONENTS)
                    if i == 0), ring.zero())
    assert infinity != 0
    count += len(infinity.roots(multiplicities=False))
    count += int(mapped[-1] == 0)
    return count


def source_population(K):
    a1 = next(K.from_integer(i) for i in range(1, K.order())
              if K.from_integer(i).trace() == 1)
    rows, by_order = [], {}
    for a in (K.zero(), a1):
        for bi in range(1, K.order()):
            b = K.from_integer(bi)
            E = EllipticCurve(K, [K.one(), a, K.zero(), K.zero(), b])
            order = int(E.cardinality())
            r = max(int(p) for p, _ in factor(order))
            trace_b = b + b**16 + b**256
            cls = "B" if trace_b == 0 else ("A" if b**16 == b else "C")
            row = {"a": int(a.to_integer()), "b": bi, "order": order,
                   "largest_prime_subgroup": r, "cofactor": order // r,
                   "direct_class": cls, "relative_trace_b": int(trace_b.to_integer())}
            rows.append(row)
            by_order.setdefault(order, []).append(row)
    assert len(rows) == 8190
    return rows, by_order


def main():
    started = time.monotonic()
    k = GF(16, "a", modulus="conway")
    L2 = GF(256, "u", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    emb2 = k.embeddings(L2)[0]
    emb3 = k.embeddings(K)[0]
    population, by_order = source_population(K)
    matrix1 = point_monomials(k)
    rng = random.Random(SEED)
    tallies = {"drawn": 0, "N1_17": 0, "N2_257": 0,
               "smooth": 0, "integral_trace": 0, "source_order_match": 0}
    witnesses = {}
    for index in range(DRAW_COUNT):
        coefficients = [1] + [rng.randrange(16) for _ in range(14)]
        tallies["drawn"] += 1
        if count_n1(coefficients, matrix1, k) != 17:
            continue
        tallies["N1_17"] += 1
        if count_by_roots(coefficients, k, L2, emb2) != 257:
            continue
        tallies["N2_257"] += 1
        f = equation(coefficients, k)
        if not Curve(f).is_smooth():
            continue
        tallies["smooth"] += 1
        N3 = count_by_roots(coefficients, k, K, emb3)
        if (4097 - N3) % 3:
            continue
        tallies["integral_trace"] += 1
        t = (4097 - N3) // 3
        order = 4097 - t
        if order not in by_order:
            continue
        tallies["source_order_match"] += 1
        if order in witnesses:
            continue
        sources = by_order[order]
        witnesses[order] = {
            "draw_index": index, "coefficients": coefficients,
            "equation": str(f), "N1_N2_N3": [17, 257, N3],
            "weil_polynomial_coefficients": [1, 0, 0, -t, 0, 0, 4096],
            "group_order": order, "sources": sources,
            "class_B_source_present": any(r["direct_class"] == "B" for r in sources),
        }
    lead = [order for order, witness in sorted(witnesses.items())
            if not witness["class_B_source_present"]
            and any(row["largest_prime_subgroup"] >= 1000
                    and row["cofactor"] <= 4 for row in witness["sources"])]
    result = {
        "schema": "binary-cubic-quartic-q16-screen/v1",
        "seed": SEED, "draw_count": DRAW_COUNT,
        "fields": {"k": str(k.modulus()), "GF256": str(L2.modulus()),
                   "K": str(K.modulus()), "k_to_GF256": str(emb2(k.gen())),
                   "k_to_K": str(emb3(k.gen()))},
        "source_population": population, "tallies": tallies,
        "witnesses": [witnesses[n] for n in sorted(witnesses)],
        "highlighted_orders": lead,
        "protocol_sha256": digest(ROOT / "PROTOCOL.md"),
        "script_sha256": digest(Path(__file__)),
        "elapsed_seconds_diagnostic": time.monotonic() - started,
        "limits": "Weil-polynomial matches only; no evaluated subgroup transfer or DLP cost",
    }
    (ROOT / "results.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"tallies": tallies, "witness_orders": sorted(witnesses),
                      "highlighted_orders": lead}))


if __name__ == "__main__":
    main()
