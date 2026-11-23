#!/usr/bin/env python3
"""Protocol LEAD-TRF-second-torsion-direction (stdlib only, deterministic)."""
import sys, json, random, time
SMOKE = "--smoke" in sys.argv
T0 = time.time()
def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
def nextprime(n):
    while not isprime(n): n += 1
    return n
def lpf(n):
    f = 1; i = 2
    while i*i <= n:
        while n % i == 0: f = i; n //= i
        i += 1
    return max(f, n) if n > 1 else f
def order(p, a, b, tab):
    chi, x3 = tab
    return p + 1 + sum(chi[(x3[x] + a*x + b) % p] for x in range(p))
def mk(p):
    chi = [-1]*p; chi[0] = 0
    for y in range(1, p): chi[y*y % p] = 1
    return chi, [x*x*x % p for x in range(p)]
def embdeg(p, n, cap=64):
    v = 1
    for k in range(1, cap+1):
        v = v*p % n
        if v == 1: return k
    return cap+1
def main():
    P = [nextprime(1200)] if SMOKE else [nextprime(6000), nextprime(9000), nextprime(14000)]
    ncurve = 60 if SMOKE else 5500
    rng = random.Random(20261110)
    rows = []  # (arm_half, k, n)
    pos = []
    for p in P:
        tab = mk(p)
        for i in range(ncurve):
            a = rng.randrange(p); b = rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = order(p, a, b, tab)
            n = lpf(N)
            if n < 100 and not SMOKE: continue
            if n < 11: continue
            rows.append((i % 2, embdeg(p, n)))
        # positive control: prime n | p-1 with n | #E (forced second direction, k=1)
        prng = random.Random(p)
        ns = [q for q in range(11, 4*int(p**.5)) if isprime(q) and (p-1) % q == 0]
        tries = 0; found = 0
        while ns and found < (3 if SMOKE else 20) and tries < (4000 if SMOKE else 40000):
            tries += 1
            a = prng.randrange(p); b = prng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            N = order(p, a, b, tab)
            for q in ns:
                if N % q == 0:
                    pos.append(embdeg(p, q)); found += 1; break
        if time.time() - T0 > 400: break
    def frac(r): return sum(1 for _, k in r if k <= 12) / max(1, len(r))
    f = frac(rows)
    h0 = [r for r in rows if r[0] == 0]; h1 = [r for r in rows if r[0] == 1]
    f0, f1 = frac(h0), frac(h1)
    m = len(rows); import math
    sd = math.sqrt(max(f*(1-f), 1e-9)*(1/max(1, len(h0)) + 1/max(1, len(h1))))
    aa_ok = abs(f0 - f1) <= max(3*sd, 2.0/max(1, len(h0)))
    pos_ok = len(pos) > 0 and all(k == 1 for k in pos)
    if not pos_ok or not aa_ok: verdict = "inconclusive"
    elif f <= 0.005: verdict = "consistent_with_prediction"
    elif f > 0.02: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-TRF-second-torsion-direction", "smoke": SMOKE, "primes": P,
        "seed": 20261110, "curves_scored": m, "statistic_frac_k_le_12": f,
        "null_threshold_consistent": 0.005, "falsify_threshold": 0.02,
        "aa_halves": [f0, f1], "aa_ok": aa_ok,
        "positive_control_n": len(pos), "positive_control_all_k1": pos_ok,
        "verdict": verdict, "elapsed_s": round(time.time()-T0, 2)}))
main()
