#!/usr/bin/env sage -python
"""Exact two-common-pole V4 eligibility gate and one q=16 witness."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, PolynomialRing, is_prime

from triangle import K, code, dec, trace

HERE = Path(__file__).resolve().parent
q = 16


def relative_trace(z):
    return z + z**q + z**(q*q)


def counts():
    V = [z for z in K if z and relative_trace(z) == 0]
    products = {code(A*B) for A in V for B in V}
    d = pow(2*(q+1), -1, q**3-1)
    outside = sum(1 for sc in products if trace(dec(sc)**d) == 1)
    assert len(V) == q*q-1
    assert len(products) == (q*q-1)*(q+2)//2 == 2295
    assert outside == q*q*(q+1)//4 == 1088
    return {"trace_zero_nonzero":len(V),"two_pole_b":len(products),
            "two_pole_outside_triangle":outside,
            "triangle_b":q**3-1,"union_models":q**3-1+outside,
            "all_normalized_ordinary_models":2*(q**3-1)}


def witness():
    ac,bc = 1079,2852
    A,B = dec(ac),dec(bc)
    b = (A*B)**2
    assert relative_trace(A) == relative_trace(B) == 0
    assert code(b) == 3399
    assert trace(b**pow(2*(q+1),-1,q**3-1)) == 1
    E = EllipticCurve(K,[K(1),K(0),K(0),K(0),b])
    assert E.cardinality() == 4132 and is_prime(1033)
    G = None
    for xc in range(4096):
        for P in sorted(E.lift_x(dec(xc),all=True),key=lambda t:code(t[1])):
            H = 4*P
            if H != E(0) and 1033*H == E(0):
                G = H
                break
        if G is not None:
            break
    assert G is not None
    Q = 7*G

    RX = PolynomialRing(K,"x")
    x = RX.gen()
    f0 = A*x+B/x
    f1 = A**q*x+B**q/x
    f2 = A**(q*q)*x+B**(q*q)/x
    assert f0+f1+f2 == 0
    X = A*x
    s = A*B
    assert s*s == b and f0 == X+s/X
    # E point (X,Y) corresponds to y0=(Y+s)/X.
    assert X*X*f0+s*s+X*s == X**3+b

    alpha = dec(14)
    assert relative_trace(alpha) == 0
    p = alpha+alpha**(q*q)
    v = alpha**q+alpha**(q*q)
    assert len({K(0),p,v,p+v}) == 4
    RU = PolynomialRing(K,"U")
    U = RU.gen()
    L = U*(U+p)*(U+v)*(U+p+v)
    l1,l2 = L[1],L[2]
    assert L[3] == 0 and all(z**q == z for z in (l1,l2))
    F = p**4*(f0+f0**2)+v**4*(f1+f1**2)+l2*(p**2*f0+v**2*f1)
    num,den = F.numerator(),F.denominator()
    assert all(z**q == z for z in list(num)+list(den))
    assert den.degree() == 2
    assert num.degree() <= 4

    def fiber(P):
        xx,yy = P[0],P[1]
        xx0 = xx/A
        y0 = (yy+s)/xx
        assert y0*y0+y0 == f0(xx0)
        val = f1(xx0)
        ys = [z for z in K if z*z+z == val]
        assert len(ys) in (0,2)
        return {"x":code(xx0),"y0":code(y0),"f1":code(val),
                "y1_roots":list(map(code,ys)),
                "pushforward_twice":list(map(code,2*P))}

    out={"schema":"ghs-two-common-poles-v1",
         "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
         "field_modulus":"w^12+w^7+w^6+w^5+w^3+w+1",
         "base_q":q,"source_a":0,"source_b":code(b),
         "source_order":4132,"subgroup_prime":1033,
         "source_G":list(map(code,G[:2])),"source_Q":list(map(code,Q[:2])),
         "A":ac,"B":bc,"sqrt_b":code(s),
         "alpha":code(alpha),"p":code(p),"v":code(v),
         "additive_quartic_coefficients_low_to_high":[code(L[i]) for i in range(5)],
         "rhs_numerator_coefficients_low_to_high":list(map(code,num.list())),
         "rhs_denominator_coefficients_low_to_high":list(map(code,den.list())),
         "G_fiber":fiber(G),"Q_fiber":fiber(Q),
         "count":counts()}
    (HERE/"two_pole_4132.json").write_text(json.dumps(out,indent=2)+"\n")
    print(json.dumps({"count":out["count"],"source_b":code(b),
                      "source_order":4132,"subgroup_prime":1033,
                      "G":out["source_G"],"Q":out["source_Q"],
                      "L":out["additive_quartic_coefficients_low_to_high"],
                      "N":out["rhs_numerator_coefficients_low_to_high"],
                      "D":out["rhs_denominator_coefficients_low_to_high"]}))


if __name__ == "__main__":
    witness()
