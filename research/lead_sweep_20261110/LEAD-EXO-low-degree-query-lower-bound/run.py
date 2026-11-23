#!/usr/bin/env python3
"""Rank of low-pole-order function evaluations on E(F_p). Stdlib only."""
import json, random, sys

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def curve_points(p, a, b):
    sq = {}
    for y in range(p):
        sq.setdefault(y * y % p, []).append(y)
    pts = []
    for x in range(p):
        for y in sq.get((x ** 3 + a * x + b) % p, []):
            pts.append((x, y))
    return pts  # affine points; group order N = len+1

def basis_row(x, y, D, p):
    # pole orders 0,2,3,...,D: n=2m -> x^m ; n=2m+3 -> x^m y
    row = [1]
    xp = [1]
    for n in range(2, D + 1):
        if n % 2 == 0:
            m = n // 2
            while len(xp) <= m: xp.append(xp[-1] * x % p)
            row.append(xp[m])
        else:
            m = (n - 3) // 2
            while len(xp) <= m: xp.append(xp[-1] * x % p)
            row.append(xp[m] * y % p)
    return row

def rank_mod(rows, p):
    rows = [r[:] for r in rows]
    rk = 0
    ncol = len(rows[0]) if rows else 0
    for c in range(ncol):
        piv = None
        for i in range(rk, len(rows)):
            if rows[i][c] % p:
                piv = i; break
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        inv = pow(rows[rk][c], p - 2, p)
        pr = [v * inv % p for v in rows[rk]]
        rows[rk] = pr
        for i in range(rk + 1, len(rows)):
            f = rows[i][c]
            if f:
                ri = rows[i]
                rows[i] = [(u - f * v) % p for u, v in zip(ri, pr)]
        rk += 1
    return rk

def cell(p, a, b, D, extra=None):
    pts = curve_points(p, a, b)
    rows = []
    for (x, y) in pts:
        r = basis_row(x, y, D, p)
        if extra == "dep":  # null: append x^3+ax+b-y^2 (identically 0 on points) and a copy of x column
            r = r + [(x ** 3 + a * x + b - y * y) % p, r[1] if len(r) > 1 else 0]
        rows.append(r)
    dim = D
    return dict(p=p, a=a, b=b, N=len(pts) + 1, D=D, dim=dim,
                rank=rank_mod(rows, p), cols=len(rows[0]) if rows else 0)

def pick_curve(rng, plo, phi):
    while True:
        p = rng.randrange(plo, phi)
        if not is_prime(p) or p < 5: continue
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p: return p, a, b

def main():
    smoke = "--smoke" in sys.argv
    seed = 20261110
    rng = random.Random(seed)
    ncell = 2 if smoke else 12
    prange = (50, 80) if smoke else (2039, 4001)
    Ds = [8, 16] if smoke else [8, 16, 32, 48]
    main_cells = []
    for _ in range(ncell):
        p, a, b = pick_curve(rng, *prange)
        D = rng.choice(Ds)
        c = cell(p, a, b, D)
        c["expected"] = min(c["dim"], c["N"] - 1)
        main_cells.append(c)
    # A/A: independent seed, same statistic
    rng2 = random.Random(seed + 1)
    aa = []
    for _ in range(ncell):
        p, a, b = pick_curve(rng2, *prange)
        c = cell(p, a, b, rng2.choice(Ds)); c["expected"] = min(c["dim"], c["N"] - 1); aa.append(c)
    # positive control: saturation, D beyond N-1 must cap rank at N-1
    rng3 = random.Random(seed + 2)
    pos = []
    for _ in range(1 if smoke else 4):
        p, a, b = pick_curve(rng3, 23, 40 if smoke else 71)
        pts = curve_points(p, a, b)
        D = len(pts) + 6
        c = cell(p, a, b, D); c["expected"] = len(pts); pos.append(c)
    # negative control: planted dependencies must lower rank by exactly 2
    neg = []
    rng4 = random.Random(seed + 3)
    for _ in range(2 if smoke else 4):
        p, a, b = pick_curve(rng4, *prange)
        c = cell(p, a, b, 12, extra="dep"); c["expected"] = 12
        neg.append(c)
    mm = sum(c["rank"] != c["expected"] for c in main_cells)
    am = sum(c["rank"] != c["expected"] for c in aa)
    pm = sum(c["rank"] != c["expected"] for c in pos)
    nm = sum(c["rank"] != c["expected"] for c in neg)
    controls_ok = pm == 0 and nm == 0 and am == 0
    if not controls_ok: verdict = "inconclusive"
    elif mm == 0: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps(dict(lead="LEAD-EXO-low-degree-query-lower-bound", smoke=smoke, seed=seed,
        statistic="number of cells with rank != min(dim, N-1)", threshold=0,
        main_mismatches=mm, aa_mismatches=am, positive_control_mismatches=pm,
        negative_control_mismatches=nm, main=main_cells, aa=aa, positive=pos, negative=neg,
        parameters=dict(prange=prange, Ds=Ds, ncell=ncell), verdict=verdict)))
main()
