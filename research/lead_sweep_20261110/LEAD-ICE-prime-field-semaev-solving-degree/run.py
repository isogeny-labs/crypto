import json, random, sys, time
SMOKE = '--smoke' in sys.argv
T0 = time.time(); BUDGET = 5 if SMOKE else 540

def pmul(f, g, p):
    r = {}
    for (a, b), c in f.items():
        for (e, h), d in g.items():
            k = (a + e, b + h); r[k] = (r.get(k, 0) + c * d) % p
    return r
def padd(f, g, p, s=1):
    r = dict(f)
    for k, c in g.items(): r[k] = (r.get(k, 0) + s * c) % p
    return r
def const(c): return {(0, 0): c}
X1 = {(1, 0): 1}; X2 = {(0, 1): 1}

def s3(p, a, b, xr):
    # Semaev S_3(x1,x2,xr) for y^2=x^3+ax+b, xr constant
    d = padd(X1, X2, p, -1); d2 = pmul(d, d, p)
    sm = padd(X1, X2, p); pr = pmul(X1, X2, p)
    t = pmul(sm, padd(pr, const(a), p), p); t = padd(t, const(2 * b), p)
    f = pmul(d2, const(xr * xr % p), p)
    f = padd(f, pmul(t, const(2 * xr % p), p), p, -1)
    q = padd(pr, const(a), p, -1); q = pmul(q, q, p)
    q = padd(q, pmul(sm, const(4 * b % p), p), p, -1)
    return padd(f, q, p)

def rand_poly(p, rng):  # null: random bivariate polynomial, bidegree <= (2,2)
    return {(i, j): rng.randrange(p) for i in range(3) for j in range(3)}

def univ(roots, var, p):
    f = const(1)
    for r in roots:
        f = pmul(f, padd({(1, 0) if var == 0 else (0, 1): 1}, const(r), p, -1), p)
    return f

def tdeg(f): return max((a + b for (a, b), c in f.items() if c), default=-1)

def dsol(gens, p, dmax):
    # per D: Macaulay rows, echelon, min degree of univariate-in-x1 element
    out = []
    for D in range(1, dmax + 1):
        mons = [(i, j) for i in range(D + 1) for j in range(D + 1 - i)]
        mons.sort(key=lambda m: (m[1] == 0, m[1], m[0] + m[1], m[0]), reverse=True)
        idx = {m: k for k, m in enumerate(mons)}; n = len(mons)
        rows = []
        for g in gens:
            dg = tdeg(g)
            if dg < 0 or dg > D: continue
            for (i, j) in mons:
                if i + j + dg <= D:
                    r = [0] * n
                    for (a, b), c in g.items():
                        if c: r[idx[(a + i, b + j)]] = c
                    rows.append(r)
        piv = {}
        for r in rows:
            for k in range(n):
                if r[k] == 0: continue
                if k in piv:
                    pr = piv[k]; f = r[k]
                    r = [(x - f * y) % p for x, y in zip(r, pr)]
                else:
                    inv = pow(r[k], p - 2, p); r = [x * inv % p for x in r]
                    piv[k] = r; break
        best = None
        for k in piv:
            m = mons[k]
            if m[1] == 0 and (best is None or m[0] < best): best = m[0]
        out.append(best)
    return out

def instance(arm, v, p, rng):
    a, b = rng.randrange(1, p), rng.randrange(1, p)
    xr = rng.randrange(p)
    V = rng.sample(range(p), v)
    F1 = univ(V, 0, p)
    if arm == 'rho_pos':      # positive control: factor base for x2 is a single planted point
        F2 = univ(V[:1], 1, p)
    else:
        F2 = univ(V, 1, p)
    f = rand_poly(p, rng) if arm == 'null' else s3(p, a, b, xr)
    return [f, F1, F2]

def run_arm(arm, vs, p_list, n, seed, deadline):
    res = {}
    for v in vs:
        hits = 0; tot = 0; ds = []
        for t in range(n):
            if time.time() > deadline: break
            rng = random.Random(f"{seed}/{arm}/{v}/{t}")
            p = p_list[t % len(p_list)]
            gens = instance(arm, v, p, rng)
            dm = v + 2
            seq = dsol(gens, p, dm)
            final = seq[-1]
            if final is None:   # no univariate element at all in range
                continue
            d = next(D for D, x in zip(range(1, dm + 1), seq) if x is not None and x <= final)
            ds.append(d); tot += 1; hits += d >= v
        res[v] = {"n": tot, "frac_Dsol_ge_v": hits / tot if tot else None,
                  "mean_Dsol_over_v": sum(ds) / tot / v if tot else None}
    return res

def pooled(r):
    n = sum(x["n"] for x in r.values()); h = sum((x["frac_Dsol_ge_v"] or 0) * x["n"] for x in r.values())
    return h / n if n else None, n

def main():
    if SMOKE: vs, n, pl = [3], 3, [31, 37]
    else: vs, n, pl = [3, 4, 5, 6], 12, [101, 103, 107, 109, 113, 127]
    dl = T0 + BUDGET
    arms = {}
    for arm, seed in [("semaev", 1), ("semaev_AA", 2), ("null", 3), ("rho_pos", 4)]:
        arms[arm] = run_arm(arm, vs, pl, n, 20261110 * 10 + seed, dl)
    fs, ns = pooled(arms["semaev"]); fa, na = pooled(arms["semaev_AA"])
    fn, nn = pooled(arms["null"]); fp, npos = pooled(arms["rho_pos"])
    complete = None not in (fs, fa, fn, fp) and time.time() < dl
    verdict = "inconclusive"
    if complete:
        aa_ok = abs(fs - fa) <= 0.15
        pos_ok = fp < 0.5
        if aa_ok and pos_ok:
            verdict = "consistent_with_prediction" if fs >= 0.8 else ("falsified" if fs < 0.5 else "inconclusive")
    print(json.dumps({"lead": "LEAD-ICE-prime-field-semaev-solving-degree", "smoke": SMOKE,
        "statistic": "pooled fraction of instances with D_sol >= v", "semaev": fs, "semaev_AA": fa,
        "null_random_bidegree22": fn, "positive_control_planted_single_fb": fp,
        "n": {"semaev": ns, "AA": na, "null": nn, "pos": npos}, "arms": arms,
        "params": {"vs": vs, "n_per_v": n, "primes": pl, "dmax": "v+2"},
        "seeds": "random.Random(f'20261110N/arm/v/t'), arm seeds 1..4",
        "elapsed_s": round(time.time() - T0, 2), "verdict": verdict}))
main()
