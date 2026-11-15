"""Exact degree-three elliptic quotient screen for the eight odd-five carriers."""
import json
import time
from sage.all import GF, PolynomialRing, EllipticCurve, power_mod

F = GF(3)
R0 = PolynomialRing(F, 'z')
z = R0.gen()
K = F.extension(z**5 + 2*z + 1, 'a')
R = PolynomialRing(K, 'x')
x = R.gen()
ROWS = [(-26, 28967), (-22, 28869), (-14, 9735), (-10, 2801),
        (10, 2872), (14, 10172), (22, 32280), (26, 35518)]

def curve_poly(mask):
    f = x**11
    for i in range(11):
        f += (mask % 3)*x**i
        mask //= 3
    assert mask == 0
    return f

def find_maps(f, trace):
    factors = list(f.factor())
    assert sorted((int(q.degree()), int(e)) for q,e in factors) == [(1,1)] + [(2,1)]*5
    out = []
    trials = 0
    for B,e in factors:
        if B.degree() != 2:
            continue
        assert e == 1 and B.is_monic()
        C = f // B
        assert C*B == f
        A0 = power_mod(C, 3**9, B)
        assert (A0**3 - C) % B == 0
        for v in K:
            trials += 1
            A = A0 + B*(x+v)
            assert A.is_monic() and A.degree() == 3 and A.gcd(B) == 1
            Q,rem = (C-A**3).quo_rem(B)
            assert rem == 0
            # Q = c2*A^2 + c1*A*B + c0*B^2 (degree at most 6).
            # Read c2 at degree 6, then c1 at degree 5, and c0 at degree 4.
            c2 = Q[6]
            Q -= c2*A**2
            c1 = Q[5]
            Q -= c1*A*B
            c0 = Q[4]
            if Q != c0*B**2:
                continue
            E = EllipticCurve(K, [0,c2,0,c1,c0])
            if E.discriminant() == 0:
                continue
            order = int(E.cardinality())
            assert order == 244-trace, (trace,order)
            out.append(dict(A=str(A), B=str(B), c2=str(c2), c1=str(c1),
                            c0=str(c0), order=order))
    assert trials == 5*243
    return out,trials

start = time.monotonic()
results = []
for trace,mask in ROWS:
    f = curve_poly(mask)
    maps,trials = find_maps(f,trace)
    results.append(dict(trace=trace,mask=mask,degree_two_factors=5,
                        cubic_candidates=trials,maps=maps))
print(json.dumps(dict(field='F3[a]/(a^5+2*a+1)',rows=results,
                      elapsed_sec=time.monotonic()-start),indent=2))
