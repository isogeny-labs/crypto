#!/usr/bin/env python3
"""Independent brute-force check of the A2 witness {YZ + 1, XZ + X + 1} over F_2.

Enumerates every admissible multiplier tuple (no linear algebra, no code shared with the
Rust instrument). Polynomials are frozensets of exponent triples; the field equations
X_i^2 + X_i are inside the generating set for d_F(inside). The fake degree uses the
multilinear quotient (16 elements per multiplier), where the field equations are 0.
Standard library only.
"""
import itertools
import sys

def mono_deg(m): return sum(m)
def deg(p): return max((mono_deg(m) for m in p), default=-1)
def add(p, q): return p ^ q
def mul(p, q):
    out = set()
    for a in p:
        for b in q:
            m = tuple(x + y for x, y in zip(a, b))
            out ^= {m}
    return frozenset(out)
def red(p):
    out = set()
    for m in p:
        out ^= {tuple(min(e, 1) for e in m)}
    return frozenset(out)
def P(*monos): return frozenset(monos)

ONE, X, Y, Z = (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1)
f1 = P((0, 1, 1), ONE)            # YZ + 1
f2 = P((1, 0, 1), X, ONE)         # XZ + X + 1
fe = [P((2, 0, 0), X), P((0, 2, 0), Y), P((0, 0, 2), Z)]

def monos_upto(k):
    return [m for m in itertools.product(range(k + 1), repeat=3) if sum(m) <= k]

def polys_upto(k):
    ms = monos_upto(k)
    for bits in range(1 << len(ms)):
        yield frozenset(m for i, m in enumerate(ms) if bits >> i & 1)

def is_fall(prods, d):
    nz = [p for p in prods if p]
    if not nz or max(deg(p) for p in nz) != d:
        return False
    s = frozenset()
    for p in prods:
        s = add(s, p)
    return bool(s) and deg(s) < d

def true_fall_at(gens, d):
    choices = []
    for f in gens:
        df = deg(f)
        choices.append([frozenset()] if df > d else list(polys_upto(d - df)))
    for gs in itertools.product(*choices):
        if is_fall([mul(g, f) for g, f in zip(gs, gens)], d):
            return gs
    return None

ML = [m for m in itertools.product((0, 1), repeat=3)]
QELEMS = [frozenset(m for i, m in enumerate(ML) if bits >> i & 1) for bits in range(1 << 8)]

def fake_fall_at(gens, d):
    rg = [red(f) for f in gens]
    for gs in itertools.product(QELEMS, repeat=len(rg)):
        prods = [red(mul(g, f)) for g, f in zip(gs, rg)]
        if is_fall(prods, d):
            return gs
    return None

def first(fn, gens, dmax):
    for d in range(0, dmax + 1):
        w = fn(gens, d)
        if w is not None:
            return d, w
    return None, None

inside = [f1, f2] + fe
d_in, w_in = first(true_fall_at, inside, 3)
d_fake, w_fake = first(fake_fall_at, [f1, f2], 3)
print(f"d_F(inside) = {d_in}  witness multipliers = {[sorted(g) for g in w_in] if w_in else None}")
print(f"d'_F = {d_fake}  witness multipliers = {[sorted(g) for g in w_fake] if w_fake else None}")
ok = d_in == 3 and d_fake == 1
print("RESULT: d_F(inside) = 3, d'_F = 1, gap 2 ->", "CONFIRMED" if ok else "NOT CONFIRMED")
sys.exit(0 if ok else 1)
