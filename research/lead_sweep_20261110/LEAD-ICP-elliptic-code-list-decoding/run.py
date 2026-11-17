import sys, json, random, math, time
SMOKE = '--smoke' in sys.argv

def is_prime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1]+Q[1]) % p == 0: return None
        l = (3*P[0]*P[0]+a)*pow(2*P[1], -1, p) % p
    else:
        l = (Q[1]-P[1])*pow(Q[0]-P[0], -1, p) % p
    x = (l*l-P[0]-Q[0]) % p
    return (x, (l*(P[0]-x)-P[1]) % p)

def sqrt_tab(p):
    t = {}
    for y in range(p): t.setdefault(y*y % p, y)
    return t

def make_group(p, seed):
    """Random curve; return (n, table point->log, list of FB logs source)."""
    rng = random.Random(seed)
    sq = sqrt_tab(p)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3+27*b*b) % p == 0: continue
        pts = []
        for x in range(p):
            r = (x**3+a*x+b) % p
            if r in sq:
                y = sq[r]; pts.append((x, y))
                if y: pts.append((x, p-y))
        N = len(pts)+1
        P = rng.choice(pts)
        tab = {}; Q = P; k = 1
        while Q is not None:
            tab[Q] = k; Q = add(Q, P, a, p); k += 1
        n = k  # order of P
        if n >= 0.4*p: return n, tab, pts, a

def count_signed_ksubsets(logs, n, kmax, target):
    # dp[j] : dict sum -> count of ordered-by-index signed j-subsets
    dp = [dict() for _ in range(kmax+1)]
    dp[0][0] = 1
    for L in logs:
        for j in range(kmax-1, -1, -1):
            src = dp[j]; dst = dp[j+1]
            for s, c in list(src.items()):
                for e in (L, -L):
                    t = (s+e) % n
                    dst[t] = dst.get(t, 0)+c
    return [dp[k].get(target % n, 0) for k in range(kmax+1)], dp

def cell(p, B, kmax, seed, ntargets, mode):
    n, tab, pts, a = make_group(p, seed)
    rng = random.Random(seed*7+1)
    inside = [Q for Q in pts if Q in tab]
    if mode == 'null':      # uniform random labels, same size
        logs = [rng.randrange(1, n) for _ in range(B)]
    else:
        # IC-style factor base: smallest-x points of the subgroup, one sign each
        seen = set(); fb = []
        for Q in sorted(inside):
            if Q[0] in seen: continue
            seen.add(Q[0]); fb.append(Q)
            if len(fb) == B: break
        logs = [tab[Q] for Q in fb]
    # one DP pass gives the full sum distribution; targets are read off it
    _, dp = count_signed_ksubsets(logs, n, kmax, 0)
    tg = [rng.randrange(n) for _ in range(ntargets)]
    out = {}
    for k in range(2, kmax+1):
        obs = sum(dp[k].get(t, 0) for t in tg)/ntargets
        exp = math.comb(B, k)*2**k/n
        out[k] = {"observed_mean": obs, "expected_mean": exp}
    return n, out

def main():
    t0 = time.time()
    if SMOKE:
        primes = [next_prime(300)]; B = 10; kmax = 3; nt = 50; seeds = [1]
    else:
        primes = [next_prime(x) for x in (4000, 16000, 64000)]
        B = 24; kmax = 5; nt = 2000; seeds = [11, 12]
    cells = []
    for p in primes:
        for s in seeds:
            for mode in ('fb', 'null'):
                n, o = cell(p, B, kmax, s, nt, mode)
                cells.append({"p": p, "n": n, "seed": s, "mode": mode, "B": B, "k": o})
    # statistic: pooled observed/expected over cells with expected >= 1 (well-sampled)
    def pooled(mode):
        so = se = 0.0
        for c in cells:
            if c["mode"] != mode: continue
            for k, v in c["k"].items():
                if v["expected_mean"] >= 1: so += v["observed_mean"]; se += v["expected_mean"]
        return (so/se) if se else None
    r_fb, r_null = pooled('fb'), pooled('null')
    # A/A: seed-11 vs seed-12 ratio spread (fb only)
    aa = {}
    for c in cells:
        if c["mode"] == 'fb':
            for k, v in c["k"].items():
                if v["expected_mean"] >= 1:
                    aa.setdefault((c["p"], k), []).append(v["observed_mean"]/v["expected_mean"])
    aa_spread = max((max(v)-min(v) for v in aa.values() if len(v) > 1), default=None)
    # GS agreement threshold sqrt(B*k) vs weight k
    gs = {str(k): {"sqrt_Bk": math.sqrt(B*k), "k": k, "decoder_reaches_weight_k": math.sqrt(B*k) <= k}
          for k in range(2, kmax+1)}
    gs_fail_all = all(not g["decoder_reaches_weight_k"] for g in gs.values())
    # positive control: plant a k-subset; count must be >= 1
    n, tab, pts, a = make_group(primes[0], 99)
    rng = random.Random(5)
    logs = [rng.randrange(1, n) for _ in range(B)]
    idx = rng.sample(range(B), 3); t = sum(logs[i] for i in idx) % n
    cnt, _ = count_signed_ksubsets(logs, n, 3, t)
    pos_ok = cnt[3] >= 1
    if r_fb is None or not pos_ok:
        verdict = "inconclusive"
    elif gs_fail_all and 0.75 <= r_fb <= 1.33 and abs(r_fb-r_null) <= 0.25*r_null:
        verdict = "consistent_with_prediction"
    elif r_fb > 1.5 or r_fb < 0.5:
        verdict = "falsified"
    else:
        verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICP-elliptic-code-list-decoding", "smoke": SMOKE,
        "statistic_pooled_obs_over_expected_fb": r_fb, "null_pooled_ratio": r_null,
        "aa_max_seed_spread": aa_spread, "gs_threshold": gs, "gs_fails_all_k": gs_fail_all,
        "positive_control_planted_found": pos_ok, "seeds": seeds, "primes": primes,
        "B": B, "kmax": kmax, "ntargets": nt, "cells": cells,
        "elapsed_s": round(time.time()-t0, 2), "verdict": verdict}))
main()
