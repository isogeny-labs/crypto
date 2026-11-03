#!/usr/bin/env sage -python
"""Construct an explicit F_16(x,u) model of each twisted V4 cover."""
import hashlib
import json
from pathlib import Path

from sage.all import PolynomialRing

from triangle import K, code, dec

HERE = Path(__file__).resolve().parent
Q = 16


def one(order):
    source = json.loads((HERE/f"certificate_{order}.json").read_text())
    t0,t1,t2 = map(dec,source["t_orbit"])
    A,A1,A2 = map(dec,source["A_orbit"])
    assert t1 == t0**Q and t2 == t1**Q and t0 == t2**Q
    assert A1 == A**Q and A2 == A1**Q and A == A2**Q

    # u = alpha*y0 + alpha^q*y1 + alpha^(q^2)*y2, with y2=y0+y1.
    alpha = t0
    p = alpha+alpha**(Q*Q)
    v = alpha**Q+alpha**(Q*Q)
    assert len({K(0),p,v,p+v}) == 4
    RU = PolynomialRing(K,"U")
    U = RU.gen()
    L = U*(U+p)*(U+v)*(U+p+v)
    assert L[0] == 0 and L[3] == 0 and L[4] == 1
    l1,l2 = L[1],L[2]
    assert all(z**Q == z for z in (l1,l2))
    assert p**4+l2*p**2+l1*p == 0
    assert v**4+l2*v**2+l1*v == 0

    RX = PolynomialRing(K,"x")
    x = RX.gen()
    f0 = A/(x+t0)+A1/(x+t1)
    f1 = A1/(x+t1)+A2/(x+t2)
    # Reduce L(u) using y_i^2+y_i=f_i. Coefficients of y_i cancel.
    F = p**4*(f0+f0**2)+v**4*(f1+f1**2)+l2*(p**2*f0+v**2*f1)
    num,den = F.numerator(),F.denominator()
    assert all(z**Q == z for z in list(num)+list(den))
    assert den.degree() == 6 and num.degree() <= 5
    k = [z for z in K if z**Q == z]
    assert len(k) == 16
    assert all(den(z) != 0 for z in k)
    lifts = {}
    for z in k:
        vals = [a for a in k if L(a) == F(z)]
        assert len(vals) == 1
        lifts[str(code(z))] = code(vals[0])

    output = {
        "schema":"ghs-twisted-v4-explicit-k-model-v1",
        "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "source_certificate_sha256":hashlib.sha256((HERE/f"certificate_{order}.json").read_bytes()).hexdigest(),
        "source_order":order,"alpha":code(alpha),"p":code(p),"v":code(v),
        "additive_quartic_coefficients_low_to_high":[code(L[i]) for i in range(5)],
        "rhs_numerator_coefficients_low_to_high":[code(z) for z in num.list()],
        "rhs_denominator_coefficients_low_to_high":[code(z) for z in den.list()],
        "F16_affine_lifts":lifts,"F16_total_count":len(lifts)+1,
        "equation":"L(u)=N(x)/D(x), with all displayed coefficients fixed by q-Frobenius",
        "birational_generator":"u=(alpha+alpha^(q^2))*y0+(alpha^q+alpha^(q^2))*y1"
    }
    (HERE/f"descent_{order}.json").write_text(json.dumps(output,indent=2)+"\n")
    print(order,"L",output["additive_quartic_coefficients_low_to_high"],
          "N",output["rhs_numerator_coefficients_low_to_high"],
          "D",output["rhs_denominator_coefficients_low_to_high"])


def main():
    for order in (3972,4006,4198,4222):
        one(order)


if __name__ == "__main__":
    main()
