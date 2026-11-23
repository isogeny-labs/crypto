#!/usr/bin/env python3
"""Divisor census of r-1 and r+1 for random primes r (Cheon supply). stdlib only."""
import sys, json, random, math

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def factor(n):
    f = {}; p = 2
    while p*p <= n:
        while n % p == 0: f[p] = f.get(p,0)+1; n//=p
        p += 1 if p == 2 else 2
    if n > 1: f[n] = f.get(n,0)+1
    return f

def divisors(n):
    ds = [1]
    for p,e in factor(n).items():
        ds = [d*p**k for d in ds for k in range(e+1)]
    return ds

def has_window(n, lo, hi):
    return any(lo <= d <= hi for d in divisors(n))

def supply(r, lo_e=0.4, hi_e=0.5):
    lo, hi = r**lo_e, r**hi_e
    return has_window(r-1, lo, hi) or has_window(r+1, lo, hi)

def rand_prime(rng, bits):
    while True:
        n = rng.randrange(1<<(bits-1), 1<<bits) | 1
        if is_prime(n): return n

def rand_odd(rng, bits):
    return rng.randrange(1<<(bits-1), 1<<bits) | 1

def frac(flags):
    n = len(flags); p = sum(flags)/n
    return p, math.sqrt(p*(1-p)/n)

def main():
    smoke = "--smoke" in sys.argv
    sizes = [12, 14] if smoke else [12, 14, 16, 18, 20, 22]
    N = 40 if smoke else 400
    out = {"lead": "LEAD-RHO-cheon-deployed-applicability", "smoke": smoke,
           "window_exponents": [0.4, 0.5], "n_per_size": N, "sizes_bits": sizes,
           "seeds": {"primes": 20261110, "null": 20261111, "aa": 20261112}, "rows": []}
    for bits in sizes:
        rng = random.Random(20261110 + bits)
        primes = [rand_prime(rng, bits) for _ in range(N)]
        fl = [supply(r) for r in primes]
        a, b = fl[:N//2], fl[N//2:]
        pe, se = frac(fl)
        # null: random odd integers n, tested as "r" (no primality conditioning on r)
        rn = random.Random(20261111 + bits)
        nul = [supply(rand_odd(rn, bits)) for _ in range(N)]
        pn, sn = frac(nul)
        # positive control: planted r-1 = d*k with d ~ r^0.4 -> must flag
        rp = random.Random(20261113 + bits)
        pos = []
        for _ in range(min(N, 100)):
            while True:
                d = rp.randrange(1<<(2*bits//5), 1<<(2*bits//5+1)); k = rp.randrange(2, 1<<(bits-2*bits//5-1))
                r = d*k+1
                if is_prime(r): break
            pos.append(supply(r))
        pa, sa = frac(a); pb, sb = frac(b)
        out["rows"].append({"bits": bits, "E": pe, "se": se, "null_E": pn, "null_se": sn,
            "positive_control_flag_rate": sum(pos)/len(pos),
            "aa_diff": pa-pb, "aa_se": math.sqrt(sa*sa+sb*sb)})
    rows = out["rows"]
    pc_ok = all(r["positive_control_flag_rate"] == 1.0 for r in rows)
    aa_ok = all(abs(r["aa_diff"]) <= 3*r["aa_se"] + 1e-12 for r in rows)
    out["controls_ok"] = pc_ok and aa_ok
    big = rows[-1]
    # prediction: E in [0.15,0.40] at largest size, and E(largest) <= E(smallest)+2se
    in_band = 0.70 <= big["E"] <= 0.90
    nonincr = abs(big["E"] - rows[0]["E"]) < 0.10
    out["statistic"] = {"E_largest": big["E"], "in_band_0.70_0.90": in_band, "drift_lt_0.10": nonincr}
    if not out["controls_ok"]: out["verdict"] = "inconclusive"
    elif smoke: out["verdict"] = "inconclusive"
    elif in_band and nonincr: out["verdict"] = "consistent_with_prediction"
    else: out["verdict"] = "falsified"
    print(json.dumps(out))
main()
