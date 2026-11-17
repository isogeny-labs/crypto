#!/usr/bin/env sage -python
"""Enumerate rational maximal isotropic 3-torsion modules for the cubic restriction."""
import hashlib
import json
from pathlib import Path

from sage.all import GF, VectorSpace, block_diagonal_matrix, matrix

HERE = Path(__file__).resolve().parent
F = GF(3)
V = VectorSpace(F, 6)
J2 = matrix(F, [[0, 1], [-1, 0]])
J = block_diagonal_matrix([J2] * 3)
FACTORS = [V.subspace([V.gen(2*i), V.gen(2*i+1)]) for i in range(3)]


def transfer_matrix(M):
    T = matrix(F, 6, 6)
    for i in range(2):
        for j in range(2):
            T[i, 4+j] = M[i, j]
    for i in range(4):
        T[i+2, i] = 1
    assert T.transpose() * J * T == J
    return T


def row(L):
    return [[int(z) for z in v] for v in L.basis()]


def examine(M):
    T = transfer_matrix(M)
    lagrangians = []
    rational = []
    product = []
    mixed = []
    for L in V.subspaces(3):
        B = L.basis()
        if any(v * J * w != 0 for v in B for w in B):
            continue
        lagrangians.append(L)
        if not all(T * v in L for v in B):
            continue
        rational.append(L)
        if all(L.intersection(H).dimension() == 1 for H in FACTORS):
            product.append(L)
        else:
            mixed.append(L)
    return {
        "M": [[int(z) for z in r] for r in M.rows()],
        "T": [[int(z) for z in r] for r in T.rows()],
        "lagrangian_count": len(lagrangians),
        "rational_lagrangian_count": len(rational),
        "product_count": len(product),
        "mixed_count": len(mixed),
        "first_product_basis": row(product[0]) if product else None,
        "first_mixed_basis": row(mixed[0]) if mixed else None,
        "kernel_dimensions": [int(((T + matrix.identity(F, 6))**i).right_kernel().dimension()) for i in range(1, 7)],
    }


def main():
    cases = {
        "scalar_minus": examine(matrix(F, [[2, 0], [0, 2]])),
        "jordan_minus": examine(matrix(F, [[2, 1], [0, 2]])),
        "scalar_plus": examine(matrix(F, [[1, 0], [0, 1]])),
        "irreducible": examine(matrix(F, [[0, 2], [1, 0]])),
    }
    assert cases["scalar_minus"]["rational_lagrangian_count"] == 16
    assert cases["scalar_minus"]["product_count"] == 4
    assert cases["scalar_minus"]["mixed_count"] == 12
    assert cases["jordan_minus"]["rational_lagrangian_count"] == 1
    assert cases["jordan_minus"]["product_count"] == 1
    assert cases["irreducible"]["rational_lagrangian_count"] == 0
    assert all(c["lagrangian_count"] == 1120 for c in cases.values())
    out = {"schema": "cubic-restriction-lagrangian-enumeration-v1",
           "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
           "cases": cases}
    (HERE / "lagrangians.json").write_text(json.dumps(out, indent=2) + "\n")
    print({k: (v["rational_lagrangian_count"], v["product_count"], v["mixed_count"])
           for k, v in cases.items()})


if __name__ == "__main__":
    main()
