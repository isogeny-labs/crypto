import json, sys, random
SMOKE = "--smoke" in sys.argv

def primes_in(lo, hi):
    s = bytearray([1]) * (hi + 1); s[0:2] = b"\0\0"
    for i in range(2, int(hi ** .5) + 1):
        if s[i]: s[i*i::i] = bytearray(len(s[i*i::i]))
    return [i for i in range(lo, hi + 1) if s[i]]

def largest_pf(n):
    m = 1; d = 2
    while d * d <= n:
        while n % d == 0: m = d; n //= d
        d += 1
    return max(m, n) if n > 1 else m

def mult_order(p, r):
    # order of p mod prime r: divide r-1
    n = r - 1; o = n; f = []; m = n; d = 2
    while d * d <= m:
        if m % d == 0:
            f.append(d)
            while m % d == 0: m //= d
        d += 1
    if m > 1: f.append(m)
    for q in f:
        while o % q == 0 and pow(p, o // q, r) == 1: o //= q
    return o

def order(p, a, b, chi):
    return p + 1 + sum(chi[(x*x*x + a*x + b) % p] for x in range(p))

def chi_table(p):
    c = [-1] * p; c[0] = 0
    for y in range(1, p): c[y*y % p] = 1
    return c

def arm(seed, plist, ncurves):
    rng = random.Random(seed); ks = []; ratios = []
    for _ in range(ncurves):
        p = rng.choice(plist); chi = chi_table(p)
        while True:
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p: break
        n = order(p, a, b, chi); r = largest_pf(n)
        if r < p // 8 or r < 5 or r == p: continue  # keep cryptographic-like, skip anomalous
        k = mult_order(p, r); ks.append(k); ratios.append(k / (r - 1))
    return ks, ratios

def stats(ks, ratios, K=10):
    s = sorted(ratios)
    return {"n": len(ks), "frac_k_le_K": sum(k <= K for k in ks) / max(1, len(ks)),
            "median_k_over_rminus1": s[len(s)//2] if s else None}

def pos_control(plist):
    out = []
    for p in plist:
        if p % 3 != 2: continue
        n = order(p, 0, 1, chi_table(p)); r = largest_pf(n)
        if r > 3 and r != p: out.append((p, n, r, mult_order(p, r)))
    return out

def main():
    if SMOKE: plist = primes_in(300, 600); nc = 60; npos = 5
    else: plist = primes_in(2**12, 2**13 + 600); nc = 700; npos = 40
    A1 = arm(1001, plist, nc); A2 = arm(1002, plist, nc)  # A/A: same instrument, different seeds
    sA1, sA2 = stats(*A1), stats(*A2)
    pos = pos_control(plist)[:npos]
    pos_ok = bool(pos) and all(k == 2 for *_, k in pos)
    # negative control / null: random-order model: random prime r in [p/8,p], random p mod r
    rng = random.Random(7); nullr = []
    rs = primes_in(plist[0] // 8, plist[-1])
    for _ in range(len(A1[0])):
        r = rng.choice(rs); pp = rng.choice(plist)
        if pp % r == 0: continue
        nullr.append(mult_order(pp, r) / (r - 1))
    nullmed = sorted(nullr)[len(nullr)//2]
    aa_ok = abs(sA1["frac_k_le_K"] - sA2["frac_k_le_K"]) <= 0.01
    pooled = (sA1["frac_k_le_K"] + sA2["frac_k_le_K"]) / 2
    med = (sA1["median_k_over_rminus1"] + sA2["median_k_over_rminus1"]) / 2
    pred = pooled <= 0.01 and med >= 0.1 and abs(med - nullmed) <= 0.15
    if not (pos_ok and aa_ok): v = "inconclusive"
    elif pred: v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps({"lead": "LEAD-TRF-mov-small-embedding-degree", "smoke": SMOKE,
      "seeds": {"A1": 1001, "A2": 1002, "null": 7}, "params": {"p_range": [plist[0], plist[-1]], "curves_per_arm": nc, "K": 10},
      "arm_A1": sA1, "arm_A2": sA2, "pooled_frac_k_le_10": pooled, "pooled_median_ratio": med,
      "null_median_ratio": nullmed, "positive_control": {"supersingular_y2=x3+1": len(pos), "all_k_eq_2": pos_ok},
      "aa_ok": aa_ok, "verdict": v}))
main()
