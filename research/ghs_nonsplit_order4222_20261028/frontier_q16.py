#!/usr/bin/env sage -python
"""Exact q16 isogeny closure of three published binary cubic V4 screens."""

import hashlib
import json
import math
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, factor, is_prime

HERE = Path(__file__).resolve().parent
Q = 16
K = GF(Q**3, "w", modulus="conway")
L = GF(Q**6, "z", modulus="conway")
EMBED = K.embeddings(L)[0]


def walsh_trace_inverse_sums():
    """All binary Kloosterman sums via an exact trace-dual Walsh transform."""
    powers = [K.gen()**j for j in range(12)]
    values = [0] * (Q**3)
    for integer in range(1, Q**3):
        u = K.from_integer(integer)
        index = sum(int((u * power).trace()) << j
                    for j, power in enumerate(powers))
        values[index] = 1 - 2 * int((u**-1).trace())
    assert values.count(0) == 1
    stride = 1
    while stride < Q**3:
        for block in range(0, Q**3, 2 * stride):
            for j in range(stride):
                a, b = values[block + j], values[block + j + stride]
                values[block + j], values[block + j + stride] = a + b, a - b
        stride *= 2
    return values


def source_order(twist, b_mask, sums):
    return Q**3 + 1 + (1 - 2 * twist) * sums[b_mask]


def screen_sets():
    d = pow(2 * (Q + 1), -1, Q**3 - 1)
    triangle = {(int((K.from_integer(b)**d).trace()), b)
                for b in range(1, Q**3)}

    trace_zero = [u for i in range(Q**3)
                  if (u := K.from_integer(i)) + u**Q + u**(Q * Q) == 0]
    assert len(trace_zero) == Q * Q
    hess = {(0, int(((u * v)**2).to_integer()))
            for u in trace_zero[1:] for v in trace_zero[1:]}

    to_k = {EMBED(K.from_integer(i)): i for i in range(1, Q**3)}
    assert len(to_k) == Q**3 - 1
    generator = L.multiplicative_generator()
    k2_generator = generator**((Q**6 - 1) // (Q * Q - 1))
    k2 = [L(0)] + [k2_generator**i for i in range(Q * Q - 1)]
    w0 = next(x + x**(Q * Q)
              for x in (generator**i for i in range(1, 30))
              if x + x**(Q * Q))
    w0_line = {c * w0 for c in k2}
    w1 = next(x + x**(Q * Q)
              for x in (generator**i for i in range(1, 50))
              if x + x**(Q * Q) not in w0_line)
    assert len({c * w0 + h * w1 for c in k2 for h in k2}) == Q**4
    norms = {w * w**(Q**3)
             for c in k2 for h in k2 if (w := c * w0 + h * w1)}
    assert len(norms) == (Q**3 + Q - 2) // 2
    nonsplit = {(1, to_k[s * s]) for s in norms}
    return triangle, hess, nonsplit


def subgroup_receipt(order, b_mask):
    factors = factor(order)
    r = int(max(p for p, _ in factors))
    assert bool(is_prime(r))
    cofactor = order // r
    a = K.from_integer(32)
    assert int(a.trace()) == 1
    curve = EllipticCurve(K, [1, a, 0, 0, K.from_integer(b_mask)])
    assert curve.cardinality() == order  # Independent of Walsh order formula.
    zero = curve(0)
    for x_mask in range(Q**3):
        points = sorted(curve.lift_x(K.from_integer(x_mask), all=True),
                        key=lambda p: int(p[1].to_integer()))
        for point in points:
            base = cofactor * point
            if base != zero and r * base == zero:
                target = 7 * base
                assert target != zero and r * target == zero
                return {
                    "source_a": 32, "source_b": b_mask,
                    "source_order": order, "subgroup_prime": r,
                    "cofactor": cofactor, "selected_x_mask": x_mask,
                    "G": [int(base[0].to_integer()), int(base[1].to_integer())],
                    "Q_equals_7G": [int(target[0].to_integer()),
                                     int(target[1].to_integer())],
                    "rho_group_order_sqrt_approx": math.sqrt(r),
                    "simple_root_gcd": math.gcd(r, 3 * (1 - Q**3)),
                    "weil_polynomial_low_to_high":
                        [1, 0, 0, order - Q**3 - 1, 0, 0, Q**3],
                }
    raise AssertionError("prime-order point not found")


def main():
    triangle, hess, nonsplit = screen_sets()
    direct = triangle | hess | nonsplit
    assert (len(triangle), len(hess), len(nonsplit), len(direct)) == (
        4095, 2295, 2055, 6158)
    sums = walsh_trace_inverse_sums()
    all_models = {(twist, b) for twist in (0, 1) for b in range(1, Q**3)}
    order_set = {source_order(*row, sums) for row in direct}
    all_orders = {source_order(*row, sums) for row in all_models}
    closure = {row for row in all_models
               if source_order(*row, sums) in order_set}
    outside = all_models - closure
    outside_orders = sorted(all_orders - order_set)
    assert len(all_orders) == 128
    assert len(order_set) == 125
    assert len(closure) == 8124 and len(outside) == 66
    assert outside_orders == [3974, 3978, 4178]
    assert all(twist == 1 for twist, _ in outside)

    candidates = []
    for order in outside_orders:
        group = sorted(b for twist, b in outside
                       if source_order(twist, b, sums) == order)
        receipt = subgroup_receipt(order, group[0])
        assert receipt["simple_root_gcd"] == 1
        candidates.append({**receipt, "model_count": len(group),
                           "all_source_b_masks": group})
    assert [row["model_count"] for row in candidates] == [12, 30, 24]
    output = {
        "schema": "ghs-q16-three-v4-screen-isogeny-closure-v1",
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "protocol_sha256": hashlib.sha256(
            (HERE / "Q16_FRONTIER_PROTOCOL.md").read_bytes()).hexdigest(),
        "field": {"K_modulus": str(K.modulus()),
                  "L_modulus": str(L.modulus()),
                  "embedding": "first element of K.embeddings(L)",
                  "mask_encoding": "Conway polynomial basis"},
        "counts": {"all_normalized_ordinary_models": len(all_models),
                   "triangle_models": len(triangle),
                   "hess_models": len(hess),
                   "nonsplit_models": len(nonsplit),
                   "three_screen_union_models": len(direct),
                   "all_source_orders": len(all_orders),
                   "three_screen_source_orders": len(order_set),
                   "models_in_K_isogeny_closure": len(closure),
                   "models_outside_K_isogeny_closure": len(outside)},
        "outside_orders": outside_orders,
        "candidates": candidates,
        "claim_scope": "Exact exclusion from the K-isogeny closure of the three specified direct V4 genus-three screens; other covers and correspondences remain possible.",
    }
    (HERE / "frontier_q16.json").write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps({"counts": output["counts"],
                      "candidates": [(x["source_order"], x["model_count"],
                                      x["subgroup_prime"])
                                     for x in candidates]}))


if __name__ == "__main__":
    main()
