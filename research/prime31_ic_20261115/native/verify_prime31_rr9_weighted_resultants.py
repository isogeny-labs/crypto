#!/usr/bin/env python3
"""Independent Python BigInt checker of Prime31 weighted-slot RR9 resultants.

The checker deliberately does not import or trust Rust field/group arithmetic;
it rebuilds every fixture from the public deterministic test seed and validates
the entire producer JSON. This is NOT independently signed PCC validation.
"""
import json
import sys
from collections import Counter

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
G = (625037715, 2100627251)
W = (1, 3, 5, 7, 11, 13, 17, 19)


def inv(a):
    assert a % P
    return pow(a, -1, P)


def add(q, r):
    if q is None:
        return r
    if r is None:
        return q
    x, y = q
    u, v = r
    if x == u and (y + v) % P == 0:
        return None
    m = ((3 * x * x + A) * inv(2 * y) if q == r
         else (v - y) * inv(u - x)) % P
    z = (m*m - x-u) % P
    return z, (m * (x-z) - y) % P


def scalar(k, q):
    total = None
    while k:
        if k & 1:
            total = add(total, q)
        q = add(q, q)
        k >>= 1
    return total


def curve(q):
    return q is None or (q[1]**2 - q[0]**3 - A*q[0] - B) % P == 0


def base():
    out = []
    for x in range(10000):
        rhs = (x**3 + A*x + B) % P
        y = pow(rhs, (P + 1)//4, P)
        if y and y*y % P == rhs:
            pt = (x, min(y, P-y))
            assert curve(pt)
            out.append(pt)
            if len(out) == 31:
                break
    assert len(out) == 31
    return out


def solve(matrix):
    m = [row[:] for row in matrix]
    for col in range(7):
        pivot = next(k for k in range(col, 8) if m[k][col] != 0)
        m[col], m[pivot] = m[pivot], m[col]
        u = inv(m[col][col])
        m[col] = [v*u % P for v in m[col]]
        for k in range(8):
            if k != col:
                f = m[k][col]
                m[k] = [(a-f*b) % P for a, b in zip(m[k], m[col])]
    assert m[7] == [0]*8
    return [m[k][7] for k in range(7)]


def chart(r, points):
    rows = []
    for x, y in points:
        powers = [pow(x, j, P) for j in range(4)]
        v = (y+r[1]) % P
        rows.append([(x-r[0])*z % P for z in powers] +
                    [v*z % P for z in powers[:3]] +
                    [-v*powers[3] % P])
    return solve(rows)


def polymul(a, b):
    z = [0]*(len(a)+len(b)-1)
    for i, u in enumerate(a):
        for j, v in enumerate(b):
            z[i+j] = (z[i+j]+u*v) % P
    return z


def polyadd(a, b, sign):
    return [((a[i] if i < len(a) else 0) +
             sign*(b[i] if i < len(b) else 0)) % P
            for i in range(max(len(a), len(b)))]


def norm_m(r, c):
    x, y = r
    v = c[4:] + [1]
    d = [(x*x+A) % P, x, 1]
    m = polymul(d, polymul(v, v))
    m = polyadd(m, polymul([-x % P, 1], polymul(c[:4], c[:4])), -1)
    m = polyadd(m, [2*y*t % P for t in polymul(c[:4], v)], -1)
    assert len(m) == 9 and m[-1] == 1
    return m


def value(f, x):
    z = 0
    for c in reversed(f):
        z = (z*x+c) % P
    return z


def signed_f(r, c, q):
    x,y = q
    a = value(c[:4], x)
    v = value(c[4:]+[1], x)
    return ((x-r[0])*a+(y+r[1])*v) % P


def validate(record):
    assert record["schema"] == "prime31.rr9.weighted-resultants.v1"
    for field, expected in [("p",P),("a",A),("b",B),("n",N),("cofactor",1),
                          ("unique_weighted_x",248),("planted_certificates_verified",16),
                          ("complete_unknown_ecdlp_solves",0),
                          ("unknown_public_chart_extractions",0)]:
        assert record[field] == expected, field
    assert record["weights"] == list(W)
    assert scalar(N,G) is None and curve(G)
    points = base()
    assert record["base_x"] == [q[0] for q in points]
    slots = [[scalar(w, q) for q in points] for w in W]
    assert all(curve(q) and q is not None for row in slots for q in row)
    xcoords = [q[0] for row in slots for q in row]
    assert len(set(xcoords)) == 248
    assert len(record["fixtures"]) == 16
    for t, row in enumerate(record["fixtures"]):
        indices = [(3*t+7*j+j*j) % 31 for j in range(8)]
        signs = [1 if (t+3*j) % 4 < 2 else -1 for j in range(8)]
        signed = []
        r = None
        for j in range(8):
            x,y = slots[j][indices[j]]
            q = (x, y if signs[j] == 1 else (-y) % P)
            signed.append(q)
            r = add(r, q)
        assert r is not None
        assert row["fixture"] == t and row["target"] == list(r)
        c = chart(r, signed)
        assert row["chart"] == c
        assert signed_f(r, c, (r[0], (-r[1]) % P)) == 0
        m = norm_m(r,c)
        roots = [[i for i,q in enumerate(group) if value(m,q[0]) == 0]
                 for group in slots]
        assert all(len(roots_j) == 1 for roots_j in roots)
        got = []
        recovered_signs = []
        replay = None
        for j in range(8):
            i = roots[j][0]
            x,y = slots[j][i]
            found = [(sgn,(x,y if sgn == 1 else (-y) % P))
                     for sgn in (1,-1)
                     if signed_f(r,c,(x,y if sgn == 1 else (-y) % P)) == 0]
            assert len(found) == 1
            sgn,q = found[0]
            got.append(i)
            recovered_signs.append(sgn)
            replay=add(replay,q)
        assert replay == r
        assert got == indices == row["root_indices"]
        assert recovered_signs == signs == row["root_signs"]
        assert row["valid_eight_resultants"] is True
        assert row["replayed_sum"] is True
        mutated=c[:]
        mutated[0] = (mutated[0]+1)%P
        bad=norm_m(r,mutated)
        hitcounts=[sum(value(bad,q[0])==0 for q in group) for group in slots]
        assert hitcounts == row["mutated_root_counts"] and not all(hitcounts)
    roots=[slots[0][0][0],slots[1][0][0],slots[1][1][0]]+[
        slots[j][0][0] for j in range(2,7)
    ]
    assert len(set(roots))==8
    m=[1]
    for x in roots:
        m=polymul(m,[(-x)%P,1])
    modified=slots[:7]+[slots[0]]
    hits=[sum(value(m,q[0])==0 for q in group) for group in modified]
    assert hits == [1,2,1,1,1,1,1,1]
    assert record["hall_overlap_all_resultants_zero"] is True
    assert record["hall_overlap_injective_matching"] is False
    assert record["all_mutations_rejected"] is True
    assert record["matched_complete_solver_benchmarks"] == 0
    print("PASS independent BigInt: original curve, 248 distinct x, 16 charts,",
          "128 signed roots, 16 mutated negatives, Hall-overlap false positive.")


if __name__ == "__main__":
    with open(sys.argv[1], "r", encoding="utf-8") as fh:
        validate(json.load(fh))
