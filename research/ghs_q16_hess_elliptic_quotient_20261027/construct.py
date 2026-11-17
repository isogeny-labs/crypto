#!/usr/bin/env sage -python
"""Construct exact degree-four elliptic quotients of four Hess genus-four models."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, Matrix, PolynomialRing, vector

HERE = Path(__file__).resolve().parent
MODELS = HERE.parent / "ghs_q16_4126_hess_g4_20261027/EXACT_MODELS.md"
F2 = GF(2)
R2 = PolynomialRing(F2, "t")
t = R2.gen()
k = GF(16, "a", modulus=t**4 + t + 1)
a = k.gen()
R = PolynomialRing(k, "x")
x = R.gen()

# Transcribed verbatim from EXACT_MODELS.md. The independent auditor uses the
# certificate's polynomial-basis encodings and does not reuse these expressions.
CURVES = {
    4006: (
        a**8*x**4+a**10*x**2+a**11*x+a**4,
        a**5*x**9+a**4*x**8+a**9*x**5+x**4+a**11*x**3+a**2*x**2+a**14*x+a**4,
        88132,
    ),
    4054: (
        a**8*x**4+a**13*x+a**9,
        a**10*x**9+a**12*x**8+a**10*x**4+a**5*x**3+a**7*x**2+a**11*x+a**10,
        56756,
    ),
    4078: (
        a**8*x**4+a**14*x**2+x+a**9,
        a**14*x**9+x**8+a**11*x**5+a**11*x**4+a**13*x**3+a**8*x**2+a**14*x+a**5,
        57092,
    ),
    4126: (
        a**8*x**4+a**3*x**2+x+a**4,
        a**11*x**9+a**13*x**8+a*x**5+a**10*x**3+a*x**2+a**12*x+a**12,
        57764,
    ),
}


def encode(element):
    p = element.polynomial()
    return sum(int(p[i]) << i for i in range(4))


def bits(poly, degree):
    return [int(poly[i].polynomial()[j]) for i in range(degree + 1) for j in range(4)]


def construct(h, f):
    L = h - h[0]
    assert L.degree() == 4 and L[1] != 0
    # Solve f+B^2+hB = F(L), B of degree <=6 and F of degree <=3.
    # This is linear over GF(2), including the squaring terms.
    columns = []
    for i in range(7):
        for j in range(4):
            v = a**j * x**i
            columns.append(bits(v*v+h*v, 12))
    for i in range(4):
        for j in range(4):
            columns.append(bits(a**j * L**i, 12))
    M = Matrix(F2, 52, 44, lambda row, col: columns[col][row])
    rhs = vector(F2, bits(f, 12))
    solution = M.solve_right(rhs)
    B = sum(a**j*x**i for i in range(7) for j in range(4) if solution[4*i+j])
    F = sum(a**j*x**i for i in range(4) for j in range(4) if solution[28+4*i+j])
    assert f+B*B+h*B == F(L)
    assert B.degree() <= 6 and F.degree() == 3
    return L, B, F


def quotient_curve(F, h0):
    X = R.gen()
    shifted = F(X+h0)
    c3 = shifted[3]
    assert c3 != 0
    # X_ell = c3*(z+h0), Y_ell = c3*(y+B(x)).
    E = EllipticCurve(k, [1, shifted[2], 0, c3*shifted[1], c3*c3*shifted[0]])
    assert E.discriminant() != 0
    return E


def main():
    records = []
    for order, (h, f, jac_order) in CURVES.items():
        L, B, F = construct(h, f)
        E = quotient_curve(F, h[0])
        expected, rem = divmod(jac_order, order)
        assert rem == 0 and E.cardinality() == expected
        roots = [c for c in L.change_ring(k.extension(3, "u")).roots(multiplicities=False)]
        assert len(roots) == 4
        records.append({
            "source_order": order,
            "hess_jacobian_order": jac_order,
            "h": [encode(h[i]) for i in range(5)],
            "f": [encode(f[i]) for i in range(10)],
            "L": [encode(L[i]) for i in range(5)],
            "B": [encode(B[i]) for i in range(7)],
            "F": [encode(F[i]) for i in range(4)],
            "elliptic_weierstrass_a": [encode(v) for v in E.a_invariants()],
            "elliptic_order": int(E.cardinality()),
            "elliptic_trace": 17 - int(E.cardinality()),
            "translation_kernel_size_over_GF4096": len(roots),
            "quotient_map": "z=L(x), v=y+B(x); X=F_3*(z+h_0), Y=F_3*v",
        })
    output = {
        "schema": "ghs-q16-hess-elliptic-quotient-v1",
        "model_ledger_sha256": hashlib.sha256(MODELS.read_bytes()).hexdigest(),
        "constructor_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "field": "GF(2)[a]/(a^4+a+1)",
        "records": records,
    }
    (HERE / "certificate.json").write_text(json.dumps(output, indent=2) + "\n")
    for row in records:
        print(row["source_order"], row["elliptic_order"], row["elliptic_trace"], row["B"], row["F"])


if __name__ == "__main__":
    main()
