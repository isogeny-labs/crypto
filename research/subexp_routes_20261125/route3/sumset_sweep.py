#!/usr/bin/env python3
"""T-sweep of the sumset (pruning) question, on the public 13-bit toy curve.

For a base of |V| abscissae on E (generic x-values, the shape of the
prime-isogeny base), count
    s2 = |x(V +- V)|    (distinct pair-sum abscissae)
    s3 = |x(V +- V +- V)|
and the exact m=2 / m=3 relation yield per target.  A coset base V = R + H is
the positive control for collapse: s2/s3 then grow like |V|, not |V|^2, |V|^3.

Decision cost in EC additions for one target (each addition = 1 field add +
1 mult, priced at the binary's measured ns_per_add):
    m=2 exact : |V| probes after one hash table of V        -> |V| adds
    m=3 exact : |V|^2 pair sums + |V| probes per target     -> |V|^2 adds
    pruned    : only if s2 ~ |V| (coset)                     -> |V| adds
"""
import itertools
import json
import random

P = 8191
A, B = -3 % P, 4830        # the toy:13,1 curve the binary reports (order 8277)


def ec_add(Pt, Q, a, p):
    if Pt is None:
        return Q
    if Q is None:
        return Pt
    x1, y1 = Pt
    x2, y2 = Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if Pt == Q:
        m = (3 * x1 * x1 + a) * pow(2 * y1, p - 2, p) % p
    else:
        m = (y2 - y1) * pow(x2 - x1, p - 2, p) % p
    x3 = (m * m - x1 - x2) % p
    return (x3, (m * (x1 - x3) - y1) % p)


def lift(x, a, b, p):
    rhs = (x * x * x + a * x + b) % p
    if rhs == 0:
        return (x, 0)
    ls = pow(rhs, (p - 1) // 2, p)
    if ls != 1:
        return None
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(rhs, q, p), pow(rhs, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, bb * bb % p, t * bb * bb % p, r * bb % p
    return (x, r)


def base(p, a, b, n):
    out, x = [], 1
    while len(out) < n:
        Pt = lift(x, a, b, p)
        if Pt is not None:
            out.append(Pt)
        x += 1
    return out


def sums(points, a, p, k):
    """distinct x(Q) over all signed sums of k distinct-or-not base points."""
    xs = set()
    for idx in itertools.combinations_with_replacement(range(len(points)), k):
        for sg in itertools.product((1, -1), repeat=k):
            R, ok = None, True
            for i, s in zip(idx, sg):
                Q = points[i] if s == 1 else (points[i][0], (-points[i][1]) % p)
                R = ec_add(R, Q, a, p)
                if R is None:
                    ok = False
                    break
            if ok and R is not None:
                xs.add(R[0])
    return xs


def main():
    out = {"curve": f"y^2=x^3+{A}x+{B} over F_{P}", "p": P, "rows": []}
    # Measured 13-bit base sizes: t=4 -> 10 abscissae, t=5 -> 18, t=6 -> 26.
    # 34 is a T=7 proxy: the true t=7 interval base is 68 abscissae, whose triple
    # sumset (68^3) is out of reach in this counting script; 34 keeps the trend.
    for T, n in ((4, 10), (5, 18), (6, 26), (7, 34)):
        V = base(P, A, B, n)
        s2 = sums(V, A, P, 2)
        s3 = sums(V, A, P, 3)
        n = len(V)
        out["rows"].append({
            "T": T, "abs": n, "points": 2 * n,
            "s2": len(s2), "s2_over_n2": len(s2) / n ** 2,
            "s3": len(s3), "s3_over_n3": len(s3) / n ** 3,
            "yield_m2": 2 * len(s2) / P, "yield_m3": 2 * len(s3) / P,
        })
        print(json.dumps(out["rows"][-1]), flush=True)
    with open("sumset_sweep.json", "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)


if __name__ == "__main__":
    main()
