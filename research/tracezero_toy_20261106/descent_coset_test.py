#!/usr/bin/env python3
"""Post hoc: 2-isogeny descent class of the trace image (k = 2).

Hypothesis (derived after the b = 0 finding): if E has a rational 2-torsion point (r, 0), the
map delta(P) = x(P) - r mod squares is a homomorphism E(F_p) -> F_p*/squares and
delta(Tr Q) = N(x(Q) - r). For the affine line V = r + F_p*g this is t^2 N(g), a constant:
the trace image lies in one coset of ker(delta), index 2 when N(g) is a square.
Output: per N(g) class, the distribution of (delta constant on S?, index of <S>).
Run: python3 -I descent_coset_test.py
"""
import importlib.util, os
from collections import Counter
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("tz", os.path.join(here, "tracezero.py"))
tz = importlib.util.module_from_spec(spec); spec.loader.exec_module(tz)

def gen_subgroup(curve, S):
    H = {0}; frontier = [0]
    S = list(S)
    while frontier:
        h = frontier.pop()
        for s in S:
            n = curve.T[h][s]
            if n not in H: H.add(n); frontier.append(n)
    return len(H)

def chi(x, p): return 0 if x % p == 0 else (1 if pow(x, (p - 1) // 2, p) == 1 else -1)

def run(p, ncurves):
    F = tz.Field(p, 2)
    tab = Counter(); used = 0
    for a, b in tz.curves_for(p, 2, 400):
        roots = [r for r in range(p) if (r ** 3 + a * r + b) % p == 0]
        if not roots or used >= ncurves: continue
        used += 1
        curve = tz.Curve(p, a, b)
        r = roots[0]
        for c0 in range(p):                                   # lines g = c0 + t (non-rational)
            g = c0 + p
            if F.frob(g) in (g, F.neg(g)): continue
            V = sorted({F.add(r, F.mul(t, g)) if t else r for t in range(p)})   # r + F_p g
            n0, tr = tz.lifts_trace(F, curve, V, a, b)
            S = {t for t in tr if t}
            pts = [curve.pts[t] for t in S if curve.pts[t][0] != r]
            if not pts: continue
            cls = {chi(P[0] - r, p) for P in pts}
            Ng = chi(F.mul(g, F.frob(g)), p)
            idx = curve.n // gen_subgroup(curve, S)
            tab[("N(g) square" if Ng == 1 else "N(g) nonsquare",
                 "delta const=%s" % (sorted(cls),) if len(cls) == 1 else "delta mixed", "index %d" % idx)] += 1
    return dict(sorted(tab.items()))

for p in (23, 37, 53):
    print("p=%d" % p)
    for k, v in run(p, 40).items(): print("   ", k, v)
