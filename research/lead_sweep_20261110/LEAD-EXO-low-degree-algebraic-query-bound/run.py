#!/usr/bin/env python3
"""Toy: how many k can a low-degree polynomial test of (a_i+b_i k)P x-coordinates vanish on?"""
import json, sys, random, time
SMOKE = "--smoke" in sys.argv
def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
def curve(p, rng):
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    for _ in range(4000):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3+27*b*b) % p == 0: continue
        n = 1 + sum(len(sq.get((x**3+a*x+b) % p, ())) for x in range(p))
        if isprime(n): return a, b, n, sq
    return None
def xs_of_multiples(p, a, b, n, sq):
    for x in range(p):
        ys = sq.get((x**3+a*x+b) % p)
        if ys: P = (x, ys[0]); break
    xs = [None]*n; Q = None
    for k in range(n):
        xs[k] = None if Q is None else Q[0]
        if Q is None: Q = P
        else:
            (x1, y1), (x2, y2) = Q, P
            if x1 == x2:
                if (y1+y2) % p == 0: Q = None; continue
                l = (3*x1*x1+a)*pow(2*y1, -1, p) % p
            else: l = (y2-y1)*pow(x2-x1, -1, p) % p
            x3 = (l*l-x1-x2) % p
            Q = (x3, (l*(x1-x3)-y1) % p)
    return xs
def nullvec(rows, ncols, p):
    rows = [r[:] for r in rows]; piv = []; r = 0
    for c in range(ncols):
        s = next((i for i in range(r, len(rows)) if rows[i][c]), None)
        if s is None: continue
        rows[r], rows[s] = rows[s], rows[r]
        inv = pow(rows[r][c], -1, p); rows[r] = [v*inv % p for v in rows[r]]
        for i in range(len(rows)):
            if i != r and rows[i][c]:
                f = rows[i][c]; rows[i] = [(u-f*v) % p for u, v in zip(rows[i], rows[r])]
        piv.append(c); r += 1
    free = [c for c in range(ncols) if c not in piv]
    if not free: return None
    v = [0]*ncols; v[free[0]] = 1
    for i, c in enumerate(piv): v[c] = (-rows[i][free[0]]) % p
    return v
def monos(D): return [(i, j) for i in range(D+1) for j in range(D+1-i)]
def biv_zeros(xs, n, p, D, rng, pair, ctrl):
    a1, b1, a2, b2 = pair; M = monos(D)
    ks = [k for k in range(n) if xs[(a1+b1*k) % n] is not None and xs[(a2+b2*k) % n] is not None]
    X1 = lambda k: xs[(a1+b1*k) % n]; X2 = lambda k: xs[(a2+b2*k) % n]
    if ctrl == "null": f = [rng.randrange(p) for _ in M]
    else:
        pick = rng.sample(ks, len(M)-1)
        rows = [[pow(X1(k), i, p)*pow(X2(k), j, p) % p for i, j in M] for k in pick]
        f = nullvec(rows, len(M), p)
    z = 0
    for k in ks:
        x1, x2 = X1(k), X2(k)
        if sum(c*pow(x1, i, p)*pow(x2, j, p) for c, (i, j) in zip(f, M)) % p == 0: z += 1
    return z, len(M)-1
def run(sizes, Ds, reps, seed):
    out = {"cells": []}; bez_viol = 0; t0 = time.time()
    for lo in sizes:
        rng = random.Random(seed*1000+lo)
        p0 = rng.randrange(1 << lo, 1 << (lo+1)); c = None
        while c is None:
            p0 += 1
            if isprime(p0): c = curve(p0, rng)
        p = p0; a, b, n, sq = c
        xs = xs_of_multiples(p, a, b, n, sq)
        # univariate Bezout check: f(x((a+bk)P))=0 has <= 2*deg f solutions (x-values) -> k count <= 2D
        uni_max = 0
        for D in Ds:
            roots = [rng.randrange(p) for _ in range(D)]
            rs = set(roots)
            for _ in range(reps):
                aa, bb = rng.randrange(n), rng.randrange(1, n)
                z = sum(1 for k in range(n) if (v := xs[(aa+bb*k) % n]) is not None and v in rs)
                uni_max = max(uni_max, z/(2*D)); bez_viol += z > 2*D
        for D in Ds:
            arms = {}
            for ctrl in ("fit", "fit_AA", "null"):
                R = []
                for r in range(reps):
                    r2 = random.Random(rng.randrange(1 << 60))
                    pair = (r2.randrange(n), r2.randrange(1, n), r2.randrange(n), r2.randrange(1, n))
                    if (pair[1]-pair[3]) % n == 0: continue
                    z, m = biv_zeros(xs, n, p, D, r2, pair, ctrl)
                    R.append(z/m)
                arms[ctrl] = {"max": max(R), "mean": sum(R)/len(R)}
            out["cells"].append({"log2N": n.bit_length(), "p": p, "N": n, "D": D, "arms": arms})
        out["cells"][-1]["uni_max_over_2D"] = uni_max
    out["bezout_violations"] = bez_viol; out["secs"] = round(time.time()-t0, 1)
    return out
def main():
    sizes, Ds, reps = ([6], [2, 3], 3) if SMOKE else ([11, 13, 15], [2, 3, 4, 5], 8)
    seed = 20261110
    o = run(sizes, Ds, reps, seed)
    big = [c for c in o["cells"] if c["log2N"] == max(x["log2N"] for x in o["cells"])]
    small = [c for c in o["cells"] if c["log2N"] == min(x["log2N"] for x in o["cells"])]
    maxR = max(c["arms"]["fit"]["max"] for c in o["cells"])
    meanbig = sum(c["arms"]["fit"]["mean"] for c in big)/len(big)
    meansmall = sum(c["arms"]["fit"]["mean"] for c in small)/len(small)
    nullmax = max(c["arms"]["null"]["max"] for c in o["cells"])
    aa = max(abs(c["arms"]["fit"]["mean"]-c["arms"]["fit_AA"]["mean"]) for c in o["cells"])
    if o["bezout_violations"] or maxR > 4 or meanbig > 1.5*max(meansmall, 1e-9) + 0.5: v = "falsified"
    elif maxR <= 1.5 and nullmax <= 1.0 and not SMOKE: v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps({"lead": "LEAD-EXO-low-degree-algebraic-query-bound", "smoke": SMOKE, "seed": seed,
        "sizes_log2p": sizes, "degrees": Ds, "reps": reps, "max_fit_R": maxR, "mean_fit_R_small": meansmall,
        "mean_fit_R_big": meanbig, "null_max_R": nullmax, "AA_max_mean_diff": aa,
        "bezout_violations": o["bezout_violations"], "result": o, "verdict": v}))
main()
