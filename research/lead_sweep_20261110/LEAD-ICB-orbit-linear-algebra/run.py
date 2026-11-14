#!/usr/bin/env python3
"""Toy: orbit-folded vs unfolded relation-matrix solve over Z_r (stdlib only)."""
import json, random, sys, statistics

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def find_r(start):
    r = start
    while not (r % 3 == 1 and isprime(r)): r += 1
    return r

def cube_root_unity(r):
    for g in range(2, 100):
        l = pow(g, (r - 1) // 3, r)
        if l != 1: return l

def solve(rows, n, r):
    """Gauss elimination mod r. Returns (solution normalised x[0]=1 or None, mulmod count)."""
    A = [row[:] for row in rows]; ops = 0; piv = []; rk = 0
    for c in range(n):
        p = next((i for i in range(rk, len(A)) if A[i][c]), None)
        if p is None: continue
        A[rk], A[p] = A[p], A[rk]
        inv = pow(A[rk][c], -1, r); pr = A[rk]
        pr[c:] = [v * inv % r for v in pr[c:]]; ops += n - c
        for i in range(len(A)):
            if i != rk and A[i][c]:
                f = A[i][c]; ri = A[i]
                ri[c:] = [(a - f * b) % r for a, b in zip(ri[c:], pr[c:])]; ops += n - c
        piv.append(c); rk += 1
    if rk != n - 1 or piv != list(range(n - 1)): return None, ops, rk
    x = [(-A[i][n - 1]) % r for i in range(n - 1)] + [1]
    return x, ops, rk

def trial(m, seed, r, lam, w=4, twist=None, extra=8):
    rnd = random.Random(seed)
    y = [rnd.randrange(1, r) for _ in range(m)]            # hidden orbit-rep logs
    lp = [pow(lam, k, r) for k in range(3)]
    # unfolded columns: (i,k,s) -> index; hidden log s*lam^k*y_i
    idx = [(i, k, s) for i in range(m) for k in range(3) for s in (1, -1)]
    n = len(idx); logs = [s * lp[k] * y[i] % r for i, k, s in idx]
    nrel = n + extra
    unf, fold = [], []
    mu = twist if twist is not None else lam
    for _ in range(nrel):
        cols = rnd.sample(range(n), w); co = [rnd.randrange(1, r) for _ in range(w - 1)]
        last = (-sum(c * logs[j] for c, j in zip(co, cols[:-1]))) * pow(logs[cols[-1]], -1, r) % r
        co.append(last)
        u = [0] * n; f = [0] * m
        for c, j in zip(co, cols):
            u[j] = (u[j] + c) % r
            i, k, s = idx[j]; f[i] = (f[i] + c * s * pow(mu, k, r)) % r
        unf.append(u); fold.append(f)
    xu, opsu, rku = solve(unf, n, r)
    xf, opsf, rkf = solve(fold, m, r)
    okf = xf is not None and all(xf[i] * pow(y[m - 1], -1, r) * y[m-1] % r * 0 + (xf[i] * y[m-1] - y[i]) % r == 0 for i in range(m)) if xf else False
    oku = xu is not None and all((xu[j] * logs[n-1] - logs[j]) % r == 0 for j in range(n))
    return dict(n=n, m=m, ops_unf=opsu, ops_fold=opsf, ok_unf=bool(oku), ok_fold=bool(okf), rk_unf=rku, rk_fold=rkf)

def main():
    smoke = "--smoke" in sys.argv
    r = find_r(1 << 20); lam = cube_root_unity(r)
    assert (lam * lam + lam + 1) % r == 0
    sizes = [3] if smoke else [6, 12, 24, 36]
    seeds = [11, 12, 13] if smoke else [101, 102, 103, 104, 105]
    out = dict(lead="LEAD-ICB-orbit-linear-algebra", r=r, lam=lam, aut_order=6, sizes=sizes, seeds=seeds, arms={})
    arms = {"main": (seeds, None), "AA_other_seeds": ([s + 1000 for s in seeds], None),
            "null_wrong_eigenvalue": (seeds[:3], 12345 % r)}
    for name, (sd, tw) in arms.items():
        res = {}
        for m in sizes:
            rows = [trial(m, s, r, lam, twist=tw) for s in sd]
            ratios = [x["ops_unf"] / max(1, x["ops_fold"]) for x in rows]
            res[m] = dict(median_ratio=statistics.median(ratios), ratios=[round(v, 2) for v in ratios],
                          all_ok_unf=all(x["ok_unf"] for x in rows), all_ok_fold=all(x["ok_fold"] for x in rows),
                          ops_fold_med=statistics.median(x["ops_fold"] for x in rows))
        out["arms"][name] = res
    big = sizes[-1]; mn = out["arms"]["main"][big]; aa = out["arms"]["AA_other_seeds"][big]
    nl = out["arms"]["null_wrong_eigenvalue"][big]
    pred = 6 ** 3
    out["prediction"] = f"median dense-cost ratio unfolded/folded within [0.7,1.3]*{pred} at largest size; folded solve recovers all logs"
    out["statistic"] = mn["median_ratio"]; out["aa_statistic"] = aa["median_ratio"]
    out["aa_rel_diff"] = abs(aa["median_ratio"] - mn["median_ratio"]) / mn["median_ratio"]
    pos = mn["all_ok_fold"] and mn["all_ok_unf"]
    null_fails = not nl["all_ok_fold"]
    aa_ok = out["aa_rel_diff"] <= 0.1
    if not (pos and null_fails and aa_ok): v = "inconclusive"
    elif 0.7 * pred <= mn["median_ratio"] <= 1.3 * pred: v = "consistent_with_prediction"
    elif smoke: v = "inconclusive"
    else: v = "falsified"
    out["controls_ok"] = dict(positive_recovery=pos, null_fails=null_fails, aa_within_10pct=aa_ok)
    out["verdict"] = v
    print(json.dumps(out)); return 0

if __name__ == "__main__": sys.exit(main())
