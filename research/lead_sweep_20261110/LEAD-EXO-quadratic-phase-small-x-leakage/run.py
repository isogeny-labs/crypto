import sys, json, math, random, cmath, time

def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def make_curve(rng, plo, phi):
    # random curve, random point; keep it if the point has prime order N >= 1000
    while True:
        p = rng.randrange(plo, phi)
        if not isprime(p) or p % 4 == 1 and False: continue
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a**3 + 27 * b * b) % p == 0: continue
        sq = {}
        for _ in range(60):
            x = rng.randrange(p); r = (x**3 + a * x + b) % p
            if pow(r, (p - 1) // 2, p) == 1:
                y = next(y for y in range(p) if y * y % p == r) if p < 3000 else None
                if y is None:
                    # Tonelli-free for p%4==3
                    if p % 4 == 3: y = pow(r, (p + 1) // 4, p)
                    else: continue
                G = (x, y); pts = {}; Q = None; d = 0
                lim = p + 2 * int(p ** .5) + 3
                while True:
                    Q = add(Q, G, a, p); d += 1
                    if Q is None: break
                    pts[d] = Q[0]
                    if d > lim: d = None; break
                if d and isprime(d) and d >= 1000:
                    return p, a, b, d, pts
        # else retry other curve

def stat(ds, pairs, N, roots):
    best = 0.0
    n = len(ds)
    for (a, b) in pairs:
        sr = si = 0.0
        for d in ds:
            z = roots[(a * d * d + b * d) % N]
            sr += z.real; si += z.imag
        v = math.hypot(sr, si)
        if v > best: best = v
    return best / math.sqrt(n)

def run(smoke):
    seed = 20261110
    rng = random.Random(seed)
    if smoke: ncurve, plo, phi, nnull, npair, frac = 2, 1500, 2500, 6, 20, 4
    else: ncurve, plo, phi, nnull, npair, frac = 8, 8000, 65000, 16, 100, 16
    rows = []; t0 = time.time()
    for ci in range(ncurve):
        p, a, b, N, pts = make_curve(rng, plo, phi)
        roots = [cmath.exp(2j * math.pi * k / N) for k in range(N)]
        B = p // frac
        small = [d for d in range(1, (N - 1) // 2 + 1) if pts[d] < B]
        # negation-closed by construction (x(-P)=x(P)): only half-set d<=(N-1)/2 kept; stat uses +-d
        S = small + [N - d for d in small]
        m = len(small)
        if m < 20: continue
        a0 = rng.randrange(1, N)
        pairs = [(a0, 0)] + [(rng.randrange(1, N), rng.randrange(N)) for _ in range(npair - 1)]
        crng = random.Random(seed + 1000 * (ci + 1))
        # grid for observed/null/positive excludes the planted pair (a0,0) for fairness: use pairs[1:]
        grid = pairs[1:]
        def null_set():
            ds = crng.sample(range(1, (N - 1) // 2 + 1), m)
            return ds + [N - d for d in ds]
        T_obs = stat(S, grid, N, roots)
        nulls = [stat(null_set(), grid, N, roots) for _ in range(nnull)]
        aa = stat(null_set(), grid, N, roots)  # A/A: independent null draw ranked against same null
        # positive control: planted bias a0*d^2 mod N in [0,N/4), same size, grid includes (a0,0)
        pool = [d for d in range(1, (N - 1) // 2 + 1) if (a0 * d * d) % N < N // 4]
        pd = crng.sample(pool, min(m, len(pool)))
        T_pos = stat(pd + [N - d for d in pd], pairs, N, roots)
        def pval(t): return (1 + sum(1 for v in nulls if v >= t)) / (nnull + 1)
        rows.append({"p": p, "N": N, "a": a, "b": b, "m_small": m, "T_obs": round(T_obs, 4),
                     "null_mean": round(sum(nulls) / nnull, 4), "p_obs": pval(T_obs),
                     "p_aa": pval(aa), "p_pos": pval(T_pos)})
        if time.time() - t0 > 480: break
    n = len(rows)
    thr = 1.0 / (nnull + 1) * (1 if smoke else 1)
    alpha = 0.1 if smoke else 0.0625  # <= 1/16 with 16 nulls: p_min=1/17
    rej = sum(1 for r in rows if r["p_obs"] <= alpha)
    aa_rej = sum(1 for r in rows if r["p_aa"] <= alpha)
    pos_det = sum(1 for r in rows if r["p_pos"] <= alpha)
    # exact binomial tail P(X>=rej | n, alpha)
    tail = sum(math.comb(n, k) * alpha**k * (1 - alpha)**(n - k) for k in range(rej, n + 1)) if n else 1.0
    if n == 0 or pos_det < max(1, math.ceil(0.9 * n)): verdict = "inconclusive"
    elif tail < 0.01: verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    return {"lead": "LEAD-EXO-quadratic-phase-small-x-leakage", "smoke": smoke, "seed": seed,
            "params": {"n_curves": ncurve, "p_range": [plo, phi], "small_x_fraction": "x<p/%d" % frac,
                       "n_null": nnull, "n_pairs": npair, "alpha": alpha},
            "statistic": "max over (a,b) grid of |sum_{d in +-S} e((a d^2+b d)/N)|/sqrt(|S|); empirical rank vs negation-closed random subsets",
            "curves_used": n, "n_reject_obs": rej, "n_reject_AA": aa_rej, "n_positive_detected": pos_det,
            "binomial_tail_p": tail, "rows": rows, "verdict": verdict,
            "elapsed_s": round(time.time() - t0, 2)}

if __name__ == "__main__":
    print(json.dumps(run("--smoke" in sys.argv)))
