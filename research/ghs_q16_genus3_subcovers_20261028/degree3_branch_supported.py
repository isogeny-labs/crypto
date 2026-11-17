#!/usr/bin/env sage -python
"""Exclude a precise branch-supported degree-three elliptic-cover ansatz.

For each cataloged genus-three model, test X=lambda*U(x), where U has
branch-root divisor a permutation of (2,1,-2,-1). The AS coordinate is
z_E=z_C+H(x), so matching reduced simple-pole residues is necessary.
The remaining degree-three branch divisors have a pole of order three and
are excluded by the same odd-pole obstruction used for degree four.
"""

import hashlib
import json
from itertools import permutations
from pathlib import Path

from sage.all import GF, LaurentSeriesRing, PolynomialRing, matrix, vector, sage_eval


HERE = Path(__file__).resolve().parent
CATALOG = HERE / "catalog_models.json"


def bits(z):
    return [(int(z.to_integer()) >> i) & 1 for i in range(12)]


def main():
    raw = CATALOG.read_bytes()
    catalog = json.loads(raw)
    assert catalog["schema"] == "ghs-q16-lmfdb-genus3-three-classes-v1"
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embedding = k.embeddings(K)[0]
    a = k.gen()
    R2 = PolynomialRing(k, names=("x", "y"))
    x, y = R2.gens()
    RK = PolynomialRing(K, "x")
    L = LaurentSeriesRing(K, "t", default_prec=7)
    t = L.gen()
    F2 = GF(2)
    patterns = sorted(set(permutations((2, 1, -2, -1))))
    assert len(patterns) == 24
    models = []
    for class_row in catalog["rows"]:
        for index, equation in enumerate(class_row["curves"]):
            lhs, rhs = equation.split("=")
            polynomial = sage_eval(lhs, locals={"a": a, "x": x, "y": y}) - sage_eval(
                rhs, locals={"a": a, "x": x, "y": y})
            h0 = R2(polynomial.coefficient({y: 1}))
            f0 = R2(-polynomial.coefficient({y: 0}))
            h = RK([embedding(h0.coefficient({x: i})) for i in range(h0.degree(x) + 1)])
            f = RK([embedding(f0.coefficient({x: i})) for i in range(f0.degree(x) + 1)])
            roots = [z for z, multiplicity in h.roots() if multiplicity == 1]
            assert len(roots) == 4 and h.degree() == 4 and h.gcd(f) == 1
            residues = [f.derivative()(z) / h.derivative()(z)**2
                        + (f(z) / h.derivative()(z)**2).sqrt() for z in roots]
            rhs_bits = sum((bits(z) for z in residues), [])
            rhs_mask = sum(bit << i for i, bit in enumerate(rhs_bits))
            cases = []
            for exponents in patterns:
                local = []
                for z in roots:
                    U = L(1)
                    for zi, exponent in zip(roots, exponents):
                        U *= (t + L(z - zi))**exponent
                    V = 1 / U
                    local.append((U[-2], U[-1], V[-2], V[-1]))

                def residues_of(lam, mu):
                    return [lam*u1 + mu*v1 + (lam*u2 + mu*v2).sqrt()
                            for u2, u1, v2, v1 in local]

                columns = []
                for j in range(24):
                    basis = K.from_integer(1 << (j % 12))
                    output = residues_of(basis, K(0)) if j < 12 else residues_of(K(0), basis)
                    output_bits = sum((bits(z) for z in output), [])
                    columns.append(sum(bit << i for i, bit in enumerate(output_bits)))
                M = matrix(F2, 48, 24, lambda i, j: (columns[j] >> i) & 1)
                rhs_vector = vector(F2, rhs_bits)
                rank = M.rank()
                augmented_rank = M.augment(rhs_vector).rank()
                assert (rank, augmented_rank) == (24, 25)
                cases.append({"exponents": list(exponents),
                              "matrix_column_bitmasks": columns,
                              "rank": rank, "augmented_rank": augmented_rank})
            models.append({"class": class_row["label"], "model_index": index,
                           "equation": equation,
                           "branch_roots_masks": [int(z.to_integer()) for z in roots],
                           "reduced_residue_masks": [int(z.to_integer()) for z in residues],
                           "rhs_bitmask": rhs_mask, "cases": cases})
    assert len(models) == 60
    receipt = {"schema": "ghs-q16-degree3-branch-supported-all60-v1",
               "scope": "60 cataloged models in three source-order classes; 24 permutations of (2,1,-2,-1); arbitrary lambda,mu in GF(4096)",
               "basis_modulus": str(K.modulus()),
               "base_generator_embedding_mask": int(embedding(a).to_integer()),
               "catalog_sha256": hashlib.sha256(raw).hexdigest(),
               "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
               "models": models}
    (HERE / "degree3_branch_supported_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("60 models; 1,440 rank-24 systems; every augmented rank 25; zero residue matches")


if __name__ == "__main__":
    main()
