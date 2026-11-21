#!/usr/bin/env python3
"""Prime31 y-coordinate cubic fiber census (requires SymPy).
Producer-local [pcc:unverified]; not a solver or cost claim.
"""
import itertools
import json
from collections import Counter
from sympy import Poly, factor_list, symbols

P, A, B = 2147483647, 847514846, 2087444724
X = symbols("X")


def sqrt(v):
    v %= P
    r = pow(v, (P + 1) // 4, P)
    return r if r * r % P == v else None


def dickson(depth=6):
    for seed in range(2, 10000):
        c = seed
        for _ in range(depth):
            c = (c*c - 2) % P
        if c in (2, P-2):
            continue
        values = [c]
        for _ in range(depth):
            out = []
            for v in values:
                r = sqrt(v+2)
                if r is not None:
                    out.extend((r, (-r) % P))
            values = sorted(set(out))
        if len(values) != 1 << depth:
            continue
        pts = []
        for x in values:
            y = sqrt((x*x*x + A*x + B) % P)
            if y not in (None, 0):
                pts.append((x, min(y, P-y)))
        if len(pts) >= 2:
            return seed, pts
    raise AssertionError("base unavailable")


def add(p, q):
    if p is None: return q
    if q is None: return p
    x, y = p
    z, w = q
    if x == z and (y+w) % P == 0: return None
    m = ((3*x*x+A)*pow(2*y, -1, P) if x == z
         else (w-y)*pow((z-x) % P, -1, P)) % P
    xx = (m*m-x-z) % P
    return xx, (m*(x-xx)-y) % P


def run():
    seed, pts = dickson()
    assert seed == 3 and len(pts) == 31
    fibers = {}
    for x, y in pts:
        if y not in fibers:
            _, factors = factor_list(X**3 + A*X + B - y*y, modulus=P)
            roots = []
            for f, multiplicity in factors:
                f = Poly(f, X, modulus=P)
                assert multiplicity == 1
                if f.degree() == 1:
                    a, b = map(int, f.all_coeffs())
                    roots.append((-b*pow(a % P, -1, P)) % P)
            fibers[y] = sorted(set(roots))
        assert x in fibers[y]
        assert all((z*z*z+A*z+B-y*y) % P == 0 for z in fibers[y])
    assert len(fibers) == 31
    hist = Counter(len(fibers[y]) for _,y in pts)
    assert hist == {1:14, 3:17}
    oriented = 0
    exceptions = 0
    for p,q in itertools.product(pts[:8], repeat=2):
        x,y = p
        z,w = q
        if x == z:
            exceptions += 1
            continue
        t = ((w-y)*pow((z-x) % P, -1, P)) % P
        xx = (t*t-x-z) % P
        yy = (t*(x-xx)-y) % P
        assert add(p,q) == (xx,yy)
        oriented += 1
    assert oriented == 56 and exceptions == 8
    return dict(standing="[pcc:unverified]",p=P,a=A,b=B,
                seed=seed,base_size=len(pts),unique_base_y=len(fibers),
                rational_x_fiber_histogram={str(k):v for k,v in sorted(hist.items())},
                total_rational_x_lifts=sum(map(len,fibers.values())),
                oriented_nonvertical_identities=oriented,
                equal_x_exceptions=exceptions,
                roots_by_y={str(y):roots for y,roots in sorted(fibers.items())})


if __name__ == "__main__":
    print(json.dumps(run(),indent=2,sort_keys=True))
