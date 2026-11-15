#!/usr/bin/env python3
"""Re-derive the recorded equal-bound Coppersmith reach bound for Semaev S_3 blocks. stdlib only."""
import sys, json, itertools, math, random
from fractions import Fraction as F

def pmul(A, B):
    R = {}
    for ea, ca in A.items():
        for eb, cb in B.items():
            e = tuple(x + y for x, y in zip(ea, eb)); R[e] = R.get(e, 0) + ca * cb
    return {e: c for e, c in R.items() if c}
def padd(*Ps):
    R = {}
    for P in Ps:
        for e, c in P.items(): R[e] = R.get(e, 0) + c
    return {e: c for e, c in R.items() if c}
def sc(P, k): return {e: c * k for e, c in P.items()}
# variables (x1,x2,x3,a,b)
def v(i):
    e = [0] * 5; e[i] = 1; return {tuple(e): 1}
def const(k): return {(0,) * 5: k}
x1, x2, x3, a, b = (v(i) for i in range(5))
def S3():
    d = padd(x1, sc(x2, -1)); t1 = pmul(pmul(d, d), pmul(x3, x3))
    mid = padd(pmul(padd(x1, x2), padd(pmul(x1, x2), a)), sc(b, 2))
    t2 = sc(pmul(mid, x3), -2)
    q = padd(pmul(x1, x2), sc(a, -1))
    t3 = padd(pmul(q, q), sc(pmul(b, padd(x1, x2)), -4))
    return padd(t1, t2, t3)
def evalp(P, vals, p):
    return sum(c * math.prod(pow(vv, e, p) for vv, e in zip(vals, ex)) for ex, c in P.items()) % p
def support(P, k): return sorted({ex[:k] for ex in P})

def sub(u, w): return tuple(x - y for x, y in zip(u, w))
def cross(u, w): return (u[1]*w[2]-u[2]*w[1], u[2]*w[0]-u[0]*w[2], u[0]*w[1]-u[1]*w[0])
def dot(u, w): return sum(x * y for x, y in zip(u, w))
def vol3(pts):
    pts = list(set(pts)); cen = tuple(F(sum(p[i] for p in pts), len(pts)) for i in range(3))
    facets = {}
    for A, B, C in itertools.combinations(pts, 3):
        n = cross(sub(B, A), sub(C, A))
        if n == (0, 0, 0): continue
        s = [dot(n, sub(P, A)) for P in pts]
        if all(x <= 0 for x in s) or all(x >= 0 for x in s):
            on = frozenset(P for P, x in zip(pts, s) if x == 0); facets[on] = n
    V = F(0)
    for on, n in facets.items():
        on = list(on); c = tuple(F(sum(p[i] for p in on), len(on)) for i in range(3))
        # order by angle in the plane
        u = sub(on[0], tuple(int(0) for _ in range(3)))
        e1 = next([x - y for x, y in zip(P0, c)] for P0 in on if tuple(P0) != tuple(c))
        e2 = cross(n, e1)
        prj = {(dot(sub(P, c), e1), dot(sub(P, c), e2)): P for P in on}
        keys = sorted(prj); lo = []; up = []
        def cr(o, p_, q): return (p_[0]-o[0])*(q[1]-o[1])-(p_[1]-o[1])*(q[0]-o[0])
        for k in keys:
            while len(lo) > 1 and cr(lo[-2], lo[-1], k) <= 0: lo.pop()
            lo.append(k)
        for k in reversed(keys):
            while len(up) > 1 and cr(up[-2], up[-1], k) <= 0: up.pop()
            up.append(k)
        on = [prj[k] for k in lo[:-1] + up[:-1]]
        for i in range(len(on)):
            P, Q = on[i], on[(i + 1) % len(on)]
            V += abs(dot(sub(tuple(c), cen), (0, 0, 0)) if False else dot(sub(P, cen), cross(sub(Q, cen), sub(c, cen)))) / 6
    return V
def vol2(pts):
    pts = sorted(set(pts))
    def cr(o, p, q): return (p[0]-o[0])*(q[1]-o[1])-(p[1]-o[1])*(q[0]-o[0])
    lo = []; up = []
    for p in pts:
        while len(lo) > 1 and cr(lo[-2], lo[-1], p) <= 0: lo.pop()
        lo.append(p)
    for p in reversed(pts):
        while len(up) > 1 and cr(up[-2], up[-1], p) <= 0: up.pop()
        up.append(p)
    h = lo[:-1] + up[:-1]
    return abs(sum(h[i][0]*h[(i+1) % len(h)][1]-h[(i+1) % len(h)][0]*h[i][1] for i in range(len(h)))) / F(2)
def reach(vol, n): return float((math.factorial(n) * vol)) ** (-1.0 / n)

def curve_check(seed, p, trials):
    """S_3 must vanish on x-coords of P+Q+R=0 (validates the expansion)."""
    rng = random.Random(seed); bad = 0; tested = 0
    while tested < trials:
        A, B = rng.randrange(p), rng.randrange(p)
        if (4*A**3 + 27*B**2) % p == 0: continue
        pts = []
        for x in range(p):
            r = (x**3 + A*x + B) % p
            if r == 0: continue
            if pow(r, (p-1)//2, p) == 1: pts.append((x, next(y for y in range(1, p) if y*y % p == r)))
        if len(pts) < 3: continue
        def add(P, Q):
            (xa, ya), (xb, yb) = P, Q
            if xa == xb:
                if (ya + yb) % p == 0: return None
                l = (3*xa*xa + A) * pow(2*ya, -1, p) % p
            else: l = (yb - ya) * pow(xb - xa, -1, p) % p
            xc = (l*l - xa - xb) % p; return (xc, (l*(xa - xc) - ya) % p)
        for _ in range(20):
            P, Q = rng.choice(pts), rng.choice(pts); R = add(P, Q)
            if R is None: continue
            tested += 1
            if evalp(S3(), [P[0], Q[0], R[0], A, B], p) != 0: bad += 1
    return bad, tested

def main():
    smoke = "--smoke" in sys.argv
    seeds = [20261110, 7, 99]
    sp = S3(); sup3 = support(sp, 3)
    # n=3 unknowns (x1,x2,x3); n=2: x3 fixed -> project to (x1,x2) with generic coefficient
    sup2 = sorted({s[:2] for s in sup3})
    V3, V2 = vol3(sup3), vol2(sup2)
    r3, r2 = reach(V3, 3), reach(V2, 2)
    # positive controls: univariate deg 2 -> 1/2 ; bivariate linear x1+x2+1 -> 1
    pc_uni = reach(F(2), 1); pc_lin = reach(vol2([(0, 0), (1, 0), (0, 1)]), 2)
    # negative control: full cube [0,2]^3 support (generic, no structure): n!vol = 48
    nc_cube = reach(vol3(list(itertools.product(range(3), repeat=3))), 3)
    # A/A: identical derivation replayed, plus curve validity under different seeds
    p = 31 if smoke else 1009
    cc = [curve_check(s, p, 20 if smoke else 400) for s in seeds]
    r3b = reach(vol3(sup3), 3)
    demand = 8 / 9
    ok_vals = abs(r2 - 0.35355) < 5e-4 and abs(r3 - 0.3293) < 5e-4
    ok_ctl = abs(pc_uni - 0.5) < 1e-12 and abs(pc_lin - 1.0) < 1e-9
    ok_curve = all(b == 0 for b, _ in cc) and r3 == r3b
    if ok_vals and ok_ctl and ok_curve and max(r2, r3) < demand: verdict = "consistent_with_prediction"
    elif ok_ctl and ok_curve and not ok_vals: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-coppersmith-smallroot-semaev", "smoke": smoke, "seeds": seeds,
        "params": {"curve_prime": p, "support_S3_x": sup3, "support_S3_x1x2": sup2},
        "statistic": {"vol_newt_S3_n3": str(V3), "vol_newt_S3_n2": str(V2), "reach_n3": r3, "reach_n2": r2, "demand_m4_w2": demand,
                      "margin_n2": r2 - demand, "margin_n3": r3 - demand},
        "controls": {"positive_univariate_deg2": pc_uni, "positive_bivariate_linear": pc_lin,
                     "negative_full_cube_3var": nc_cube, "AA_replay_equal": r3 == r3b,
                     "S3_curve_check_bad_tested": cc},
        "verdict": verdict}))
main()
