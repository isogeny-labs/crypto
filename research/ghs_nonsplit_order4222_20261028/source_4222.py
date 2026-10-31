#!/usr/bin/env sage -python
"""Exact order-4222 source in the quadratic-pole cubic GHS family."""
import hashlib
import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, PolynomialRing, is_prime

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent/"ghs_nonsplit_v4_cubic_20261027"
sys.path.insert(0,str(PARENT))
from nonsplit import L, K, k2, q, dec, code, absolute_trace_K


def run():
    a = dec(5522909)
    b = dec(1610778)
    t = dec(16191718)
    assert a**q == a and absolute_trace_K(a) == 1
    assert b**(q**3) == b and t+t**q == 1
    g = L.multiplicative_generator()
    W0 = g+g**(q*q)
    line = {c*W0 for c in k2}
    W1 = next(z+z**(q*q) for z in (g**i for i in range(1,50))
              if z+z**(q*q) not in line)
    A = next(w for c in k2 for h in k2 if (w:=c*W0+h*W1)
             and (w*w**(q**3))**2 == b)
    assert A+A**(q*q)+A**(q**4) == 0
    assert A*A**(q**3) == b.sqrt()
    Kfield = L.subfield(12)
    E = EllipticCurve(Kfield,[Kfield(1),Kfield(a),Kfield(0),Kfield(0),Kfield(b)])
    assert E.cardinality() == 4222 and is_prime(2111)
    G = None
    for xx in sorted(K,key=code):
        if not xx:
            continue
        for P in E.lift_x(Kfield(xx),all=True):
            H = 2*P
            if H != E(0) and 2111*H == E(0):
                G = H
                break
        if G is not None:
            break
    assert G is not None
    Q = 7*G
    assert 2111*Q == E(0) and Q != E(0)

    RX = PolynomialRing(L,"x")
    x = RX.gen()
    f0 = A/(x+t)+A**(q**3)/(x+t**q)
    def frob(f):
        num = sum((c**q*x**i for i,c in enumerate(f.numerator())),L(0))
        den = sum((c**q*x**i for i,c in enumerate(f.denominator())),L(0))
        return num/den
    f1 = frob(f0)
    f2 = frob(f1)
    assert f0+f1+f2 == 0 and frob(f2) == f0
    assert all(c**(q**3)==c for c in list(f0.numerator())+list(f0.denominator()))

    alpha = next(z for z in sorted(K,key=code)
                 if z and z+z**q+z**(q*q) == 0)
    p = alpha+alpha**(q*q)
    v = alpha**q+alpha**(q*q)
    assert len({L(0),p,v,p+v}) == 4
    RU = PolynomialRing(L,"U")
    U = RU.gen()
    Lpoly = U*(U+p)*(U+v)*(U+p+v)
    F = p**4*(f0+f0**2)+v**4*(f1+f1**2)
    F += Lpoly[2]*(p**2*f0+v**2*f1)
    num,den = F.numerator(),F.denominator()
    assert all(c**q==c for c in list(num)+list(den)+[Lpoly[1]])
    result = {"schema":"ghs-nonsplit-order4222-source-v1",
              "parent_script_sha256":hashlib.sha256((PARENT/"nonsplit.py").read_bytes()).hexdigest(),
              "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              "field_modulus":str(L.modulus()),"base_q":q,
              "source_a":code(a),"source_b":code(b),
              "source_order":4222,"subgroup_prime":2111,
              "source_G":[code(L(G[0])),code(L(G[1]))],
              "source_Q":[code(L(Q[0])),code(L(Q[1]))],
              "pole_t":code(t),"pole_tq":code(t**q),
              "residue_A":code(A),"residue_Aq3":code(A**(q**3)),
              "sqrt_b":code(A*A**(q**3)),
              "f0":str(f0),"f1":str(f1),
              "descent_alpha":code(alpha),"descent_p":code(p),
              "descent_v":code(v),
              "additive_quartic_coefficients_low_to_high":[code(Lpoly[i]) for i in range(5)],
              "rhs_numerator_coefficients_low_to_high":list(map(code,num.list())),
              "rhs_denominator_coefficients_low_to_high":list(map(code,den.list()))}
    (HERE/"source_4222.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps({k:result[k] for k in
                      ("source_a","source_b","source_order","subgroup_prime",
                       "source_G","source_Q","residue_A",
                       "additive_quartic_coefficients_low_to_high")},indent=2))


if __name__ == "__main__":
    run()
