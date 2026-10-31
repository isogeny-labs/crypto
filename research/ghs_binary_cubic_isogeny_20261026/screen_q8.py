#!/usr/bin/env sage -python
"""Frozen GF(8)/GF(512) binary cubic plane-quartic class screen."""

import hashlib
import json
import random
import time
from pathlib import Path

from sage.all import Curve, EllipticCurve, GF, PolynomialRing, factor

from screen import EXPONENTS, count_zero, point_monomials, quartic


ROOT = Path(__file__).resolve().parent
SEED = 2026102608
DRAW_COUNT = 1024


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def point_count_by_roots(coeffs, small, large, emb):
    """Count the two standard projective charts by univariate root sets."""
    mapped = [emb(small.from_integer(c)) for c in coeffs]
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
            trace_b = b + b**8 + b**64
            cls = "B" if trace_b == 0 else ("A" if b**8 == b else "C")
            row = {"a": int(a.to_integer()), "b": bi, "order": order,
                   "largest_prime_subgroup": r, "cofactor": order // r,
                   "direct_class": cls, "relative_trace_b": int(trace_b.to_integer())}
            rows.append(row)
            by_order.setdefault(order, []).append(row)
    assert len(rows) == 1022
    return rows, by_order


def main():
    started = time.monotonic()
    k = GF(8, "a", modulus="conway")
    L2 = GF(64, "u", modulus="conway")
    K = GF(512, "w", modulus="conway")
    emb2 = k.embeddings(L2)[0]
    emb3 = k.embeddings(K)[0]
    rows, by_order = source_population(K)
    mat1 = point_monomials(k)
    mat2 = point_monomials(L2)
    rng = random.Random(SEED)
    tallies = {"drawn": 0, "N1_9": 0, "N2_65": 0, "smooth": 0,
               "integral_trace": 0, "source_order_match": 0}
    witnesses = {}
    for index in range(DRAW_COUNT):
        coeffs = [1] + [rng.randrange(8) for _ in range(14)]
        tallies["drawn"] += 1
        if count_zero(coeffs, mat1, lambda x: x, k) != 9:
            continue
        tallies["N1_9"] += 1
        if count_zero(coeffs, mat2, emb2, k) != 65:
            continue
        tallies["N2_65"] += 1
        f = quartic(coeffs, k, lambda x: x, k)
        if not Curve(f).is_smooth():
            continue
        tallies["smooth"] += 1
        N3 = point_count_by_roots(coeffs, k, K, emb3)
        if (513 - N3) % 3:
            continue
        tallies["integral_trace"] += 1
        t = (513 - N3) // 3
        order = 513 - t
        if order not in by_order:
            continue
        tallies["source_order_match"] += 1
        if order in witnesses:
            continue
        witnesses[order] = {"draw_index": index, "coefficients": coeffs,
                            "equation": str(f), "N1_N2_N3": [9, 65, N3],
                            "weil_polynomial_coefficients": [1, 0, 0, -t, 0, 0, 512],
                            "group_order": order, "sources": by_order[order],
                            "class_B_source_present": any(r["direct_class"] == "B"
                                                          for r in by_order[order])}
    result = {"schema": "binary-cubic-quartic-q8-screen/v1",
              "seed": SEED, "draw_count": DRAW_COUNT,
              "fields": {"k": str(k.modulus()), "K": str(K.modulus()),
                         "k_to_GF64": str(emb2(k.gen())), "k_to_K": str(emb3(k.gen()))},
              "source_population": rows, "tallies": tallies,
              "witnesses": [witnesses[n] for n in sorted(witnesses)],
              "protocol_sha256": digest(ROOT / "PROTOCOL_Q8.md"),
              "script_sha256": digest(Path(__file__)),
              "elapsed_seconds_diagnostic": time.monotonic() - started,
              "limits": "Weil-polynomial match only; no evaluated source map or DLP cost"}
    (ROOT / "results_q8.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    lead = [n for n, w in sorted(witnesses.items())
            if not w["class_B_source_present"]
            and max(r["largest_prime_subgroup"] for r in w["sources"]) >= 127
            and min(r["cofactor"] for r in w["sources"]) <= 4]
    print(json.dumps({"tallies": tallies, "witness_orders": sorted(witnesses),
                      "high_priority_class_C_only_orders": lead}))


if __name__ == "__main__":
    main()
