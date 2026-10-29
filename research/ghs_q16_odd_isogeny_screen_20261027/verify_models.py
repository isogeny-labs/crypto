#!/usr/bin/env sage -python
"""Independently count the two displayed exact elliptic curve representatives."""
from sage.all import EllipticCurve, GF, PolynomialRing

R = PolynomialRing(GF(2), "w")
w = R.gen()
K = GF(2**12, modulus=w**12+w**7+w**6+w**5+w**3+w+1, name="w")
u = K.gen()


def decode(code):
    return sum(((code >> i) & 1) * u**i for i in range(12))


for a_code, expected in ((0, 4076), (32, 4118)):
    E = EllipticCurve(K, [K(1), decode(a_code), K(0), K(0), decode(106)])
    assert E.cardinality() == expected
    print(f"a={a_code}, b=106: #E(K)={expected}")
