#!/usr/bin/env python3
"""Macaulay-rank degeneration of chained S_3 system at j=0/1728 vs generic j. stdlib only."""
import sys, json, random, itertools, math, time

def pmul(A, B, p):
    R = {}
    for ma, ca in A.items():
        for mb, cb in B.items():
            m = tuple(x + y for x, y in zip(ma, mb))
            R[m] = (R.get(m, 0) + ca * cb) % p
    return {m: c for m, c in R.items() if c}

def padd(A, B, p, s=1):
    R = dict(A)
    for m, c in B.items():
        R[m] = (R.get(m, 0) + s * c) % p
    return {m: c for m, c in R.items() if c}

def const(c, p, n=4):
    return {(0,) * n: c % p} if c % p else {}

def var(i, n=4):
    return {tuple(1 if k == i else 0 for k in range(n)): 1}

def S3(X, Y, Z, a, b, p):
    # S3 = (x-y)^2 z^2 - 2((x+y)(xy+a)+2b) z + (xy-a)^2 - 4b(x+y)
    xy = pmul(X, Y, p)
    d = padd(X, Y, p, -1)
    s = padd(X, Y, p)
    t1 = pmul(pmul(d, d, p), pmul(Z, Z, p), p)
    inner = padd(pmul(s, padd(xy, const(a, p), p), p), const(2 * b, p), p)
    t2 = pmul(inner, Z, p)
    t3 = padd(xy, const(a, p), p, -1)
    t3 = pmul(t3, t3, p)
    t4 = pmul(const(4 * b, p), s, p)
    r = padd(t1, t2, p, -2)
    r = padd(r, t3, p)
    return padd(r, t4, p, -1)

def system(a, b, xR, p):
    x1, x2, x3, u = var(0), var(1), var(2), var(3)
    return [S3(x1, x2, u, a, b, p), S3(u, x3, const(xR, p), a, b, p)]

def monos(n, d):
    out = []
    for e in itertools.product(range(d + 1), repeat=n):
        if sum(e) <= d:
            out.append(e)
    return out

def macaulay_rank(polys, d, p, n=4):
    rows = []
    for f in polys:
        df = max(sum(m) for m in f)
        for e in monos(n, d - df):
            rows.append({tuple(x + y for x, y in zip(m, e)): c for m, c in f.items()})
    piv = {}  # leading monomial -> row
    rank = 0
    for r in rows:
        r = dict(r)
        while r:
            lm = max(r)
            if lm in piv:
                q = piv[lm]
                f = r[lm]
                for m, c in q.items():
                    v = (r.get(m, 0) - f * c) % p
                    if v: r[m] = v
                    else: r.pop(m, None)
            else:
                inv = pow(r[lm], p - 2, p)
                piv[lm] = {m: c * inv % p for m, c in r.items()}
                rank += 1
                break
    return rank, len(rows)

def is_prime(n):
    return n > 1 and all(n % q for q in range(2, int(n ** .5) + 1))

def rank_of(a, b, xR, p, d):
    return macaulay_rank(system(a, b, xR, p), d, p)

def deficit(a, b, xR, p, d):
    r, nr = rank_of(a, b, xR, p, d)
    return nr - r

def nonsingular(a, b, p):
    return (4 * a ** 3 + 27 * b * b) % p != 0

def study(p, d, ngen, nxr, rng):
    """returns dict arm -> list of mean deficits (over nxr random xR) per curve"""
    def arm_vals(curves):
        vals = []
        for a, b in curves:
            vals.append(sum(deficit(a, b, rng.randrange(p), p, d) for _ in range(nxr)) / nxr)
        return vals
    gen = []
    while len(gen) < 2 * ngen:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if nonsingular(a, b, p): gen.append((a, b))
    # special curves: j=0 (a=0), j=1728 (b=0), several b / a scalings (twists)
    j0 = [(0, rng.randrange(1, p)) for _ in range(ngen)]
    j1728 = [(rng.randrange(1, p), 0) for _ in range(ngen)]
    # coefficient-block null: generic curves but with a replaced by 0 only in coefficient pattern
    # is the special curve itself; instead use random-support control: generic (a,b) with a,b in
    # a "small" set {1,2,3} (same sparsity, non-special j)
    smallc = [(rng.choice([1, 2, 3]), rng.choice([1, 2, 3])) for _ in range(ngen)]
    smallc = [c for c in smallc if nonsingular(*c, p)]
    return {"genA": arm_vals(gen[:ngen]), "genB": arm_vals(gen[ngen:]),
            "j0": arm_vals(j0), "j1728": arm_vals(j1728), "smallcoef": arm_vals(smallc)}

def mean(v): return sum(v) / len(v)
def sd(v):
    m = mean(v); return math.sqrt(sum((x - m) ** 2 for x in v) / max(1, len(v) - 1))

def positive_control(p, d):
    # forced rank drop: second polynomial = x1 * first (+ itself), true deficit increases
    a, b = 3, 5
    f1, f2 = system(a, b, 7, p)
    g = [f1, pmul(var(0), f1, p)]
    return deficit_poly(g, d + 1, p) - deficit_poly([f1, f2], d + 1, p)

def deficit_poly(polys, d, p):
    r, nr = macaulay_rank(polys, d, p)
    return nr - r

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke:
        primes, ds, ngen, nxr = [31], [4], 3, 1
    else:
        primes, ds, ngen, nxr = [97, 109, 157], [8, 9, 10, 11, 12], 30, 3
    # p = 1 mod 12 would make both j=0,1728 curves have extra automorphisms over F_p; require 1 mod 12
    primes = [q for q in primes if is_prime(q)]
    rng = random.Random(20261110)
    res = []
    for p in primes:
        for d in ds:
            s = study(p, d, ngen, nxr, rng)
            gm, gs = mean(s["genA"] + s["genB"]), max(sd(s["genA"] + s["genB"]), 1e-9)
            row = {"p": p, "d": d, "p_mod_12": p % 12,
                   "mean_deficit": {k: round(mean(v), 4) for k, v in s.items()},
                   "gen_sd": round(gs, 4)}
            nulld = abs(mean(s["genA"]) - mean(s["genB"])) / (gs / math.sqrt(ngen) * math.sqrt(2) + 1e-9)
            row["AA_z"] = round(nulld, 3)
            for k in ("j0", "j1728", "smallcoef"):
                eff = mean(s[k]) - gm
                row["z_" + k] = round(eff / (gs / math.sqrt(ngen) * math.sqrt(1.5) + 1e-9), 3)
                row["eff_" + k] = round(eff, 4)
                row["hit_" + k] = abs(eff) >= max(3 * gs / math.sqrt(ngen) * math.sqrt(1.5), 0.5)
            res.append(row)
    pc = positive_control(primes[0], ds[0])
    # decision: drop = special arm mean deficit differs from generic by z>=3 at >=2 of the cells,
    # in same direction, AND smallcoef null z <3 AND AA_z<3 AND positive control > 0
    ok_ctrl = pc > 0 and all(abs(r["AA_z"]) < 3 for r in res)
    hits = [r for r in res if (r["hit_j0"] or r["hit_j1728"]) and not r["hit_smallcoef"]]
    if not ok_ctrl:
        verdict = "inconclusive"
    elif smoke:
        verdict = "inconclusive"
    elif len(hits) >= 2 and len(hits) * 2 >= len(res):
        verdict = "falsified"  # prediction: no degeneration beyond null
    elif not hits:
        verdict = "consistent_with_prediction"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-xline-symmetry-special-j", "smoke": smoke,
                      "seed": 20261110, "primes": primes, "degrees": ds, "n_curves_per_arm": ngen,
                      "n_xR_per_curve": nxr, "cells": res, "positive_control_extra_deficit": pc,
                      "n_hit_cells": len(hits), "controls_ok": ok_ctrl,
                      "elapsed_s": round(time.time() - t0, 2), "verdict": verdict}))

if __name__ == "__main__":
    main()
