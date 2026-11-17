#!/usr/bin/env python3
"""LEAD-RHO-cheon-aux-acquisition: toy check of the recorded obstruction. stdlib only."""
import json, math, random, sys

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def factor(n):
    f, q = set(), 2
    while q*q <= n:
        while n % q == 0: f.add(q); n //= q
        q += 1
    if n > 1: f.add(n)
    return f

def make_group(bits, d, rng):
    """prime r ~ 2^bits with r = 1 mod d; p = 2kr+1 prime; g of order r in Z_p^*."""
    while True:
        r = rng.randrange(1 << (bits-1), 1 << bits) // d * d + 1
        if is_prime(r): break
    k = 1
    while not is_prime(2*k*r+1): k += 1
    p = 2*k*r+1
    while True:
        g = pow(rng.randrange(2, p-1), (p-1)//r, p)
        if g != 1: return r, p, g

def rho(p, r, g, h, rng):
    """Brent rho with 16-way r-adding walk; returns (x, group mults). Mults counted after precompute."""
    while True:
        M = []
        for _ in range(16):
            u, v = rng.randrange(r), rng.randrange(r)
            M.append((pow(g,u,p)*pow(h,v,p) % p, u, v))
        a, b = rng.randrange(r), rng.randrange(r)
        X = pow(g,a,p)*pow(h,b,p) % p
        ops, cx, ca, cb, power, lam = 0, X, a, b, 1, 0
        while True:
            m, u, v = M[X & 15]
            X = X*m % p; a = (a+u) % r; b = (b+v) % r; ops += 1
            if X == cx: break
            lam += 1
            if lam == power: cx, ca, cb, power, lam = X, a, b, power*2, 0
        db = (b-cb) % r
        if db == 0: continue
        x = (ca-a) * pow(db, -1, r) % r
        if pow(g, x, p) == h: return x, ops

def cheon(p, r, g, d, x, zeta):
    """Cheon-style recovery of x from g, g^x, g^(x^d), d | r-1. Returns (x, group exponentiations)."""
    m = (r-1)//d
    X1, Q = pow(g,x,p), pow(g, pow(x,d,r), p)
    ex = 0
    zd = pow(zeta, d, r)
    s = math.isqrt(m)+1
    baby, c = {}, 1
    for i in range(s):
        baby.setdefault(pow(g,c,p), i); ex += 1; c = c*zd % r
    giant = pow(zd, -s, r); c = 1; t = None
    for j in range(s+1):
        e = pow(Q, c, p); ex += 1
        if e in baby: t = (baby[e] + s*j) % m; break
        c = c*giant % r
    assert t is not None
    zm = pow(zeta, m, r); s2 = math.isqrt(d)+1
    baby2, c = {}, 1
    for i in range(s2):
        baby2.setdefault(pow(g,c,p), i); ex += 1; c = c*zm % r
    gi = pow(zm, -s2, r); zt = pow(zeta, -t, r)
    tgt = pow(X1, zt, p); ex += 1; c = 1
    for j in range(s2+1):
        e = pow(tgt, c, p); ex += 1
        if e in baby2:
            k = (baby2[e] + s2*j) % d
            return pow(zeta, t+m*k, r), ex
        c = c*gi % r
    raise RuntimeError("cheon failed")

def primitive_root(r):
    fs = factor(r-1)
    z = 2
    while any(pow(z, (r-1)//q, r) == 1 for q in fs): z += 1
    return z

def heuristic_hits(p, r, g, trials, rng):
    """Null: try to hit g^(x^2) with random known-span elements g^(a+b x); hits expected ~ trials/r."""
    x = rng.randrange(2, r); target = pow(g, x*x % r, p); X1 = pow(g, x, p)
    hits = 0
    for _ in range(trials):
        a, b = rng.randrange(r), rng.randrange(r)
        if pow(g,a,p)*pow(X1,b,p) % p == target: hits += 1
    return hits

def main():
    smoke = "--smoke" in sys.argv
    sizes = [10] if smoke else [12, 14, 16, 18, 20]
    nt = 4 if smoke else 40
    cheon_n = 3 if smoke else 20
    seed = 20261110
    out = {"lead": "LEAD-RHO-cheon-aux-acquisition", "seed": seed, "smoke": smoke, "sizes_bits": sizes,
           "trials_per_size": nt, "rows": []}
    pts = []; aa_ok = True; heur_bad = False; pos_win = False
    for bits in sizes:
        rng = random.Random(seed + bits)
        d = max(2, 1 << (bits//3))
        r, p, g = make_group(bits, d, rng)
        zeta = primitive_root(r)
        runs = []
        for _ in range(nt):
            x = rng.randrange(2, r); h = pow(g, x, p)
            xs, ops = rho(p, r, g, h, rng); assert xs == x
            runs.append(ops)
        half = nt//2
        mA, mB = sum(runs[:half])/half, sum(runs[half:])/(nt-half)
        mean = sum(runs)/nt
        aa = mA/mB
        if not smoke and not (0.7 <= aa <= 1.4): aa_ok = False
        cm = []
        for _ in range(cheon_n):
            x = rng.randrange(2, r)
            if pow(x, d, r) == 0: continue
            xr, ex = cheon(p, r, g, d, x, zeta); assert xr == x
            cm.append(ex)
        cheon_mults = sum(cm)/len(cm)*1.5*math.log2(r)
        trials = 200 if smoke else 20*math.isqrt(r)
        hits = heuristic_hits(p, r, g, trials, rng)
        exp_hits = trials/r
        if hits > exp_hits + 5*math.sqrt(exp_hits) + 3: heur_bad = True
        if cheon_mults < mean: pos_win = True
        acq_total = mean + 1.5*math.log2(r)   # DL by rho, then one exponentiation gives g^(x^d)
        pts.append((math.log2(r), math.log2(mean)))
        out["rows"].append({"bits": bits, "r": r, "p": p, "d": d, "rho_mean_mults": mean, "rho_over_sqrt_r": mean/math.sqrt(r),
            "aa_ratio_halfA_over_halfB": aa, "cheon_free_aux_mults_equiv": cheon_mults,
            "cheon_over_rho": cheon_mults/mean, "acquire_by_DL_mults": acq_total,
            "self_acquired_cheon_total_over_rho": (acq_total+cheon_mults)/mean,
            "heuristic_trials": trials, "heuristic_hits": hits, "heuristic_expected": exp_hits})
    if len(pts) >= 2:
        n = len(pts); mx = sum(a for a,_ in pts)/n; my = sum(b for _,b in pts)/n
        slope = sum((a-mx)*(b-my) for a,b in pts)/sum((a-mx)**2 for a,_ in pts)
    else: slope = None
    out["statistic"] = {"loglog_slope_acquisition_cost_vs_r": slope, "threshold_band": [0.45, 0.55]}
    out["controls"] = {"positive_free_aux_cheon_beats_rho_at_some_size": pos_win,
                       "negative_heuristic_hits_within_null": not heur_bad,
                       "aa_ratio_within_[0.7,1.4]": aa_ok}
    if smoke: v = "inconclusive"
    elif not (aa_ok and pos_win): v = "inconclusive"
    elif heur_bad or not (0.45 <= slope <= 0.55): v = "falsified"
    else: v = "consistent_with_prediction"
    out["verdict"] = v
    print(json.dumps(out))
    return 0

if __name__ == "__main__":
    sys.exit(main())
