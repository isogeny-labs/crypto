#!/usr/bin/env sage -python
"""Bounded search for direct branch-supported C_3 -> E_b maps.

The branch support and exponent patterns are explicit search restrictions.
An empty result does not exclude other degree-four maps or correspondences.
"""

import argparse
import hashlib
import json
from itertools import permutations
from pathlib import Path

from sage.all import GF, LaurentSeriesRing, PolynomialRing


HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_nonsplit_order4222_20261028" / "frontier_q16.json"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--order", type=int, choices=(3974,), default=3974)
    args = parser.parse_args()

    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    a = k.embeddings(K)[0](k.gen())
    ring = PolynomialRing(K, "x")
    x = ring.gen()
    h = a * a * x**4 + a * x
    f = x**8 + x**3 + a * x**2 + (a * a + a + 1) * x + a**3 + a + 1
    roots = [r for r, _ in h.roots()]
    assert len(roots) == 4 and h.gcd(f) == 1
    residues = [f.derivative()(r) / a**2 + (f(r) / a**2).sqrt() for r in roots]

    data = json.loads(FRONTIER.read_text())
    row = next(c for c in data["candidates"] if c["source_order"] == args.order)
    b_masks = row["all_source_b_masks"]
    patterns = sorted(
        set(permutations((4, -2, -1, -1)))
        | set(permutations((-4, 2, 1, 1)))
        | set(permutations((2, 2, -2, -2)))
    )
    L = LaurentSeriesRing(K, "t", default_prec=12)
    t = L.gen()
    matches = []
    tested = 0
    unavoidable_cubic_pole_patterns = []
    square_patterns = []
    for exponents in patterns:
        local = []
        for r in roots:
            U = L(1)
            for ri, exponent in zip(roots, exponents):
                U *= (t + L(r - ri)) ** exponent
            V = 1 / U
            local.append(([U[j] for j in range(-4, 0)], [V[j] for j in range(-4, 0)]))
        if all(e % 2 == 0 for e in exponents):
            square_patterns.append(list(exponents))
        elif any((uc[1] != 0 and vc[1] == 0) or
                 (vc[1] != 0 and uc[1] == 0) for uc, vc in local):
            unavoidable_cubic_pole_patterns.append(list(exponents))
        for b_mask in b_masks:
            beta = K.from_integer(b_mask).sqrt()
            for lam in K:
                if lam == 0:
                    continue
                mu = beta / lam
                passes = True
                for i in range(4):
                    u_coeffs, v_coeffs = local[i]
                    c = [lam * u_coeffs[j] + mu * v_coeffs[j] for j in range(4)]
                    if c[1] != 0:  # An odd order-three pole cannot be removed by H^2+H.
                        passes = False
                        break
                    c[2] += c[0].sqrt()  # Eliminate order four.
                    c[3] += c[2].sqrt()  # Eliminate order two.
                    if c[3] != residues[i]:
                        passes = False
                        break
                tested += 1
                if passes:
                    matches.append({"b_mask": b_mask, "exponents": exponents,
                                    "lambda_mask": int(lam.to_integer())})
        print(f"pattern {exponents}: {len(matches)} matches so far", flush=True)

    receipt = {
        "schema": "ghs-q16-genus3-branch-supported-degree4-v1",
        "source_order": args.order,
        "field_modulus": str(K.modulus()),
        "base_field_modulus": str(k.modulus()),
        "base_generator_embedding_mask": int(a.to_integer()),
        "source_b_masks": b_masks,
        "curve_h": str(h),
        "curve_f": str(f),
        "branch_roots_masks": [int(r.to_integer()) for r in roots],
        "branch_residues_masks": [int(r.to_integer()) for r in residues],
        "patterns": [list(v) for v in patterns],
        "unavoidable_cubic_pole_patterns": unavoidable_cubic_pole_patterns,
        "square_patterns": square_patterns,
        "tested": tested,
        "matches": matches,
        "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
        "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "scope": "X=lambda*product((x-root_i)^e_i), 30 listed exponent patterns, all nonzero lambda in GF(4096), all listed order-3974 source b masks",
    }
    output = HERE / "receipt_3974.json"
    assert len(unavoidable_cubic_pole_patterns) == 24
    assert len(square_patterns) == 6
    output.write_text(json.dumps(receipt, indent=2) + "\n")
    print(f"tested={tested} matches={len(matches)} output={output}")


if __name__ == "__main__":
    main()
