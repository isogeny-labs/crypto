import json, random, sys, itertools, time
from math import comb

def rand_system(n, m, rng, linear_eqs=0):
    x0 = [rng.getrandbits(1) for _ in range(n)]
    eqs = []
    for e in range(m):
        mons = set()
        deg = 1 if e < linear_eqs else 2
        for i in range(n):
            if rng.getrandbits(1): mons.add(1 << i)
        if deg == 2:
            for i in range(n):
                for j in range(i + 1, n):
                    if rng.getrandbits(1): mons.add((1 << i) | (1 << j))
        mons ^= set()  # no-op, keep set
        v = 0
        for mk in mons:
            if all(x0[i] for i in range(n) if mk >> i & 1): v ^= 1
        if v: mons.add(0)
        eqs.append(mons)
    return x0, eqs

def fix(eqs, n, k, x0):
    """fix first k variables to planted values; return eqs over n-k vars (reindexed)."""
    out = []
    for p in eqs:
        q = set()
        for mk in p:
            lo = mk & ((1 << k) - 1)
            if lo & ~sum(x0[i] << i for i in range(k)):  # contains a var fixed to 0
                continue
            r = mk >> k
            q ^= {r}
        out.append(q)
    return out

def solved_at(eqs, nv, D):
    """XL at degree D with x^2=x. True if nv independent combos lead on deg<=1 columns."""
    mons = [m for d in range(D - 1) for c in itertools.combinations(range(nv), d) for m in [sum(1 << i for i in c)]]
    cols = {}
    allm = []
    for d in range(D + 1):
        for c in itertools.combinations(range(nv), d):
            allm.append(sum(1 << i for i in c))
    idx = {m: i for i, m in enumerate(allm)}  # degree-ordered: higher index = higher degree
    piv = {}
    for p in eqs:
        for mu in mons:
            row = 0
            for t in p:
                row ^= 1 << idx[t | mu]
            while row:
                h = row.bit_length() - 1
                if h in piv: row ^= piv[h]
                else:
                    piv[h] = row; break
    lim = 1 + nv  # columns 0..nv are const and degree-1
    cnt = sum(1 for h in piv if 1 <= h < lim)
    return cnt >= nv, len(allm)

def min_degree(eqs, nv, Dmax):
    if nv == 0: return 1, 1
    for D in range(2, Dmax + 1):
        ok, ncols = solved_at(eqs, nv, D)
        if ok: return D, ncols
    return None, None

def cost(k, ncols):
    return None if ncols is None else (2 ** k) * ncols ** 2.37

def cell(n, m, ks, seeds, Dmax, base, linear=0):
    res = {k: [] for k in ks}
    for s in seeds:
        rng = random.Random(base + 1000 * n + s)
        x0, eqs = rand_system(n, m, rng, linear)
        for k in ks:
            D, nc = min_degree(fix(eqs, n, k, x0), n - k, Dmax)
            res[k].append({"D": D, "cost": cost(k, nc)})
    return res

def med(v):
    v = sorted(v); return v[len(v) // 2]

def summarize(res):
    out = {}
    for k, rows in res.items():
        ds = [r["D"] if r["D"] is not None else 99 for r in rows]
        cs = [r["cost"] if r["cost"] is not None else float("inf") for r in rows]
        out[k] = {"medD": med(ds), "medcost": med(cs) if med(cs) != float("inf") else "inf", "censored": sum(r["D"] is None for r in rows)}
    return out

def best_k(summ):
    b = None
    for k, v in summ.items():
        c = float("inf") if v["medcost"] == "inf" else v["medcost"]
        if b is None or c < b[1]: b = (k, c)
    return b[0] if b[1] != float("inf") else None

def main():
    smoke = "--smoke" in sys.argv
    t0 = time.time()
    if smoke: sizes, ks, seeds, Dmax = [8], [0, 1, 2], range(2), 3
    else: sizes, ks, seeds, Dmax = [10, 12, 14], [0, 1, 2, 3, 4], range(4), 4
    SEED = 20261110
    main_r, aa_r, neg_r, pos_r = {}, {}, {}, {}
    for n in sizes:
        main_r[n] = summarize(cell(n, n, ks, seeds, Dmax, SEED))
        aa_r[n] = summarize(cell(n, n, ks, seeds, Dmax, SEED + 7777))  # A/A: disjoint seed stream
    nneg = sizes[0]
    neg_r = summarize(cell(nneg, nneg // 2, [0], seeds, Dmax, SEED + 5))  # underdetermined null
    pos_r = summarize(cell(nneg, 3 * nneg, [0], seeds, Dmax, SEED + 9, linear=3 * nneg))  # all-linear: D=2
    kstar = {n: best_k(main_r[n]) for n in sizes}
    aastar = {n: best_k(aa_r[n]) for n in sizes}
    pos_ok = pos_r[0]["medD"] <= 2
    neg_ok = neg_r[0]["censored"] == len(list(seeds))
    big = [n for n in sizes if n >= 12]
    if smoke:
        verdict = "inconclusive"
    elif not (pos_ok and neg_ok):
        verdict = "inconclusive"
    elif big and all(kstar[n] is not None and kstar[n] >= 1 for n in big) and all(
            (aastar[n] is not None and abs(aastar[n] - kstar[n]) <= 1) for n in big):
        verdict = "consistent_with_prediction"
    elif big and all(kstar[n] == 0 for n in big):
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    # JSON keys must be strings
    f = lambda d: {str(a): ({str(b): c for b, c in v.items()} if isinstance(v, dict) and all(isinstance(b, int) for b in v) else v) for a, v in d.items()}
    print(json.dumps({"lead": "LEAD-ICE-hybrid-guess-determine-l7", "smoke": smoke, "seed_base": SEED,
        "params": {"sizes": sizes, "ks": ks, "n_seeds": len(list(seeds)), "Dmax": Dmax, "omega": 2.37},
        "statistic": "kstar(n) = argmin_k median 2^k*cols(D_min)^2.37", "kstar": {str(a): b for a, b in kstar.items()},
        "kstar_AA": {str(a): b for a, b in aastar.items()},
        "main": f(main_r), "AA": f(aa_r), "negative_control_underdetermined": neg_r, "positive_control_linear": pos_r,
        "controls_ok": {"positive": pos_ok, "negative": neg_ok}, "seconds": round(time.time() - t0, 2),
        "verdict": verdict}))
main()
