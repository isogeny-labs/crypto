#!/usr/bin/env sage -python
"""Solve F16 descent equations for the six square-rational degree-four covers."""

import hashlib
import json
from itertools import permutations, product
from pathlib import Path

from sage.all import GF, LaurentSeriesRing, Matrix, PolynomialRing


HERE = Path(__file__).resolve().parent
FRONTIER = HERE.parent / "ghs_nonsplit_order4222_20261028" / "frontier_q16.json"


def main():
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    a = k.embeddings(K)[0](k.gen())
    F2 = GF(2)
    R = PolynomialRing(K, "x")
    x = R.gen()
    h = a*a*x**4 + a*x
    roots = [r for r, _ in h.roots()]
    frob = [roots.index(r**16) for r in roots]
    L = LaurentSeriesRing(K, "t", default_prec=12)
    t = L.gen()
    patterns = sorted(set(permutations((2, 2, -2, -2))))
    frontier = json.loads(FRONTIER.read_text())
    source_masks = set().union(*(set(c["all_source_b_masks"])
                                 for c in frontier["candidates"]))
    rows = []
    for exponents in patterns:
        coeffs = []
        for r in roots:
            U = L(1)
            for ri, exponent in zip(roots, exponents):
                U *= (t + L(r-ri))**exponent
            V = 1/U
            coeffs.append((U[-2], V[-2]))

        columns = []
        for j in range(24):
            lam = K.from_integer(1 << j) if j < 12 else K(0)
            mu = K(0) if j < 12 else K.from_integer(1 << (j-12))
            c = [lam*u + mu*v for u, v in coeffs]
            residual = [c[frob[i]] + c[i]**16 for i in range(4)]
            columns.append([F2((int(z.to_integer()) >> bit) & 1)
                            for z in residual for bit in range(12)])
        M = Matrix(F2, 48, 24, lambda i, j: columns[j][i])
        kernel = M.right_kernel()
        assert M.rank() == 20 and kernel.dimension() == 4
        pairs = []
        for bits in product((0, 1), repeat=4):
            vector = sum((bits[i]*kernel.basis()[i] for i in range(4)),
                         kernel.zero())
            lam_mask = sum(int(vector[j]) << j for j in range(12))
            mu_mask = sum(int(vector[j+12]) << j for j in range(12))
            if lam_mask == 0 or mu_mask == 0:
                continue
            lam, mu = K.from_integer(lam_mask), K.from_integer(mu_mask)
            beta = lam*mu
            b = beta**2
            pairs.append({"lambda_mask": lam_mask, "mu_mask": mu_mask,
                          "b_mask": int(b.to_integer()),
                          "lambda_equals_mu": bool(lam == mu),
                          "lambda_in_base": bool(lam**16 == lam),
                          "b_in_base": bool(b**16 == b)})
        assert len(pairs) == 15
        rows.append({
            "exponents": list(exponents), "rank": int(M.rank()),
            "nullity": int(kernel.dimension()),
            "matrix_column_bitmasks": [sum(int(bit) << i for i, bit in enumerate(col))
                                       for col in columns],
            "kernel_basis_bitmasks": [sum(int(bit) << i for i, bit in enumerate(v))
                                      for v in kernel.basis()],
            "pairs": pairs,
        })
        print(exponents, "rank", M.rank(), "nonzero pairs", len(pairs),
              "nonbase b", [p["b_mask"] for p in pairs if not p["b_in_base"]],
              flush=True)

    b_sets = [set(p["b_mask"] for p in row["pairs"]) for row in rows]
    assert all(b_set == b_sets[0] for b_set in b_sets)
    overlap = sorted(b_sets[0].intersection(source_masks))
    receipt = {
        "schema": "ghs-q16-square-pattern-f16-descent-v1",
        "field_modulus": str(K.modulus()),
        "base_generator_embedding_mask": int(a.to_integer()),
        "branch_roots_masks": [int(r.to_integer()) for r in roots],
        "frobenius_branch_permutation": frob,
        "rows": rows,
        "eligible_b_masks": sorted(b_sets[0]),
        "all_66_frontier_b_masks": sorted(source_masks),
        "intersection": overlap,
        "frontier_sha256": hashlib.sha256(FRONTIER.read_bytes()).hexdigest(),
        "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    (HERE/"descent_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print("eligible b", sorted(b_sets[0]), "frontier intersection", overlap,
          flush=True)


if __name__ == "__main__":
    main()
