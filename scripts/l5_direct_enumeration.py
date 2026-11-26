#!/usr/bin/env python3
"""L5: direct membership enumeration for the Semaev m=3 cell at k=50.

The generic F4 route (msolve) does not finish k=50 in 15 min.  For m=3 the
base is an explicit x-set, so the decision/count problem can be solved by
enumerating the k^3 x-space triples directly:

  * enumerate all ordered point triples and their target x-coordinates;
  * pick the target with the fewest point triples;
  * count x-space solutions by evaluating S4 over all k^3 x-triples.

This is O(k^3) field work and does not build a Groebner basis.  It answers
"is this cell solvable / how many x-solutions" for m=3, not the m>=4 regime.
"""
import json
import sys
import time

P, A, B = 4099, 1, 1


def curve_points():
    from sympy import sqrt_mod
    pts = []
    for x in range(P):
        r = (x * x * x + A * x + B) % P
        y = sqrt_mod(r, P)
        if y is not None:
            pts.append((x, y))
            if y:
                pts.append((x, (-y) % P))
    return pts


def eadd(p1, p2):
    if p1 is None:
        return p2
    if p2 is None:
        return p1
    x1, y1 = p1
    x2, y2 = p2
    if x1 == x2:
        if (y1 + y2) % P == 0:
            return None
        lam = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (lam * lam - x1 - x2) % P
    return (x3, (lam * (x1 - x3) - y1) % P)


def s4_eval(x1, x2, x3, x4):
    """S4 at concrete field elements, via the two-quadratic resultant."""
    def abc(u, v):
        a_ = (u - v) * (u - v) % P
        b_ = (-2 * (((u + v) * (u * v + A) + 2 * B) % P)) % P
        c_ = (((u * v - A) % P) ** 2 - 4 * B * (u + v)) % P
        return a_, b_, c_
    a1, b1, c1 = abc(x1, x2)
    a2, b2, c2 = abc(x3, x4)
    ac = (a1 * c2 - a2 * c1) % P
    ab = (a1 * b2 - a2 * b1) % P
    bc = (b1 * c2 - b2 * c1) % P
    return (ac * ac - ab * bc) % P


def base_set(by_x, k, family):
    xs = sorted(by_x)
    if family == "window":
        return xs[:k]
    tset, xset = set(), []
    for x in (v for v in xs if v):
        tset.add(x * x % P)
        pre = [y for y in xs if (y * y % P) in tset]
        if len(pre) >= k:
            return pre[:k]
    raise SystemExit(f"pullback k={k}: only {len(pre)} on-curve x-values")


def count_targets(by_x, xset):
    pts = [by_x[x] for x in xset]
    cnt = {}
    for p1 in pts:
        for p2 in pts:
            s2 = eadd(p1, p2)
            if s2 is None:
                continue
            for p3 in pts:
                s = eadd(s2, p3)
                if s is not None:
                    cnt[s[0]] = cnt.get(s[0], 0) + 1
    return cnt


def xspace_count(xset, xq):
    n = 0
    for a in xset:
        for b in xset:
            for c in xset:
                if s4_eval(a, b, c, xq) == 0:
                    n += 1
    return n


def run(k):
    by_x = {}
    for q in curve_points():
        by_x.setdefault(q[0], q)
    out = {}
    for family in ("window", "pullback"):
        xset = base_set(by_x, k, family)
        t0 = time.time()
        cnt = count_targets(by_x, xset)
        t_enum = time.time() - t0
        xq, c = min(cnt.items(), key=lambda kv: (kv[1], kv[0]))
        t0 = time.time()
        num = xspace_count(xset, xq)
        t_count = time.time() - t0
        out[family] = dict(k=k, actual_k=len(xset), n_targets=len(cnt),
                           min_point_count=c, min_xq=xq,
                           enum_seconds=round(t_enum, 4),
                           xspace_count=num, xspace_seconds=round(t_count, 4))
        print(out[family], flush=True)
    return out


if __name__ == "__main__":
    k = int(sys.argv[1]) if len(sys.argv) > 1 else 50
    res = run(k)
    json.dump(res, open(f"l5_enum_k{k}.json", "w"), indent=1)
