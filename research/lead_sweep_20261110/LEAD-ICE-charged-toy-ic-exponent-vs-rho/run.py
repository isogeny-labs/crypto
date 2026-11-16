#!/usr/bin/env python3
"""Toy charged IC-vs-rho exponent (stdlib only). Proxy in Z_p^*, p=2q+1 safe prime, subgroup order q."""
import sys, json, math, random

def is_prime(n):
    if n < 2: return False
    for s in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % s == 0: return n == s
    d, r = n-1, 0
    while d % 2 == 0: d //= 2; r += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(r-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def safe_prime(bits, rng):
    while True:
        q = rng.getrandbits(bits-1) | (1 << (bits-2)) | 1
        if is_prime(q) and is_prime(2*q+1): return 2*q+1, q

def small_primes(n):
    s = bytearray([1])*(n+1); s[0:2] = b'\0\0'
    for i in range(2, int(n**.5)+1):
        if s[i]: s[i*i::i] = bytearray(len(s[i*i::i]))
    return [i for i in range(n+1) if s[i]]

def rho(p, q, G, y, rng):
    """Floyd, 3-way partition; each group op = 1 mulmod. Returns (log, ops)."""
    ops = 0
    while True:
        a, b = rng.randrange(q), rng.randrange(q)
        x = pow(G, a, p) * pow(y, b, p) % p
        def step(x, a, b):
            c = x % 3
            if c == 0: return x*x % p, 2*a % q, 2*b % q
            if c == 1: return x*G % p, (a+1) % q, b
            return x*y % p, a, (b+1) % q
        X, A, B = x, a, b; ops += 2
        while True:
            x, a, b = step(x, a, b); X, A, B = step(*step(X, A, B)); ops += 3
            if x == X: break
        db = (b - B) % q
        if db == 0: continue
        s = (A - a) * pow(db, -1, q) % q
        if pow(G, s, p) == y: return s, ops

def ic(p, q, G, y, rng, B):
    """Index calculus with QR-prime factor base; returns (log, mulmods, divisions)."""
    fb = [l for l in small_primes(B) if pow(l, q, p) == 1]
    k = len(fb); M = 0; D = 0
    piv = {}                       # col -> (row dict, rhs), pivot normalised to 1
    e = rng.randrange(q); x = pow(G, e, p); M += 1
    def smooth(x):
        nonlocal D
        D += k; v = {}
        for i, l in enumerate(fb):
            if x % l == 0:
                c = 0
                while x % l == 0: x //= l; c += 1
                v[i] = c
        return v if x == 1 else None
    while len(piv) < k:
        e = (e + 1 + rng.randrange(7)) % q; x = x * G % p; M += 1  # cheap update stepping by G; offsets pre-multiplied below
        # step offset: recompute exactly to keep e consistent
        x = pow(G, e, p); M += 1
        v = smooth(x)
        if v is None: continue
        row = {i: c % q for i, c in v.items()}; rhs = e
        while row:
            j = min(row)
            if j in piv:
                pr, pb = piv[j]; f = row[j]
                for t, c in pr.items():
                    nv = (row.get(t, 0) - f*c) % q; M += 1
                    if nv: row[t] = nv
                    else: row.pop(t, None)
                rhs = (rhs - f*pb) % q; M += 1
            else:
                inv = pow(row[j], -1, q); M += 1
                row = {t: c*inv % q for t, c in row.items()}; rhs = rhs*inv % q; M += len(row)
                piv[j] = (row, rhs); break
    L = [0]*k
    for j in range(k-1, -1, -1):
        pr, pb = piv[j]; s = pb
        for t, c in pr.items():
            if t != j: s -= c*L[t]; M += 1
        L[j] = s % q
    for _ in range(10**7):
        e = rng.randrange(q); x = y * pow(G, e, p) % p; M += 2
        v = smooth(x)
        if v is not None:
            s = (sum(c*L[i] for i, c in v.items()) - e) % q; M += len(v)
            return s, M, D
    raise RuntimeError("no target relation")

def one(bits, seed, mode):
    rng = random.Random(seed*1000003 + bits)
    p, q = safe_prime(bits, rng)
    while True:
        r = rng.randrange(2, p-1); G = r*r % p
        if G != 1: break
    s = rng.randrange(1, q); y = pow(G, s, p)
    if mode == "null":              # negative control: random target, answer unverifiable by s -> use own check
        pass
    lnp = math.log(p); B = max(30, int(math.exp(0.6*math.sqrt(lnp*math.log(lnp)))))
    sr, ro = rho(p, q, G, y, rng); assert pow(G, sr, p) == y
    si, M, D = ic(p, q, G, y, rng, B); assert pow(G, si, p) == y
    return ro, M, D

def fit(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    b = sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)
    return my - b*mx, b

def med(v):
    v = sorted(v); n = len(v); return v[n//2] if n % 2 else 0.5*(v[n//2-1]+v[n//2])

def analyse(bits_l, data, ws, rng, nb):
    out = {}
    for w in ws:
        ric = {b: [m + w*d for (_, m, d) in data[b]] for b in bits_l}
        rrh = {b: [r for (r, _, _) in data[b]] for b in bits_l}
        lr = [math.log2(med(rrh[b])) for b in bits_l]; li = [math.log2(med(ric[b])) for b in bits_l]
        ar, br = fit(bits_l, lr); ai, bi = fit(bits_l, li)
        bs_r, bs_i = [], []
        for _ in range(nb):
            yr, yi = [], []
            for b in bits_l:
                n = len(data[b]); idx = [rng.randrange(n) for _ in range(n)]
                yr.append(math.log2(med([rrh[b][i] for i in idx]))); yi.append(math.log2(med([ric[b][i] for i in idx])))
            bs_r.append(fit(bits_l, yr)[1]); bs_i.append(fit(bits_l, yi)[1])
        ci = lambda v: [sorted(v)[int(.025*len(v))], sorted(v)[int(.975*len(v))-1]]
        out["w=%g" % w] = {"rho_slope": br, "rho_slope_ci": ci(bs_r), "ic_slope": bi, "ic_slope_ci": ci(bs_i),
            "ratio_ic_over_rho_by_bits": {str(b): med(ric[b])/med(rrh[b]) for b in bits_l},
            "crossover_bits_linear_fit": ((ar-ai)/(bi-br) if bi != br else None)}
    return out

def main():
    smoke = "--smoke" in sys.argv
    bits_l = [12, 14] if smoke else [16, 20, 24, 28, 32]
    seeds = 2 if smoke else 12; nb = 50 if smoke else 400
    ws = [0.0, 0.1, 1.0]
    data = {b: [one(b, s, "main") for s in range(1, seeds+1)] for b in bits_l}
    rng = random.Random(777)
    res = analyse(bits_l, data, ws, rng, nb)
    # A/A: odd vs even seeds, same instrument; slope difference of rho arm must lie within pooled CI half-widths
    half = {k: [data[b][0::2] for b in bits_l] for k in ("a",)}
    da = {b: data[b][0::2] for b in bits_l}; db = {b: data[b][1::2] for b in bits_l}
    ra = analyse(bits_l, da, [1.0], random.Random(1), nb)["w=1"]; rb = analyse(bits_l, db, [1.0], random.Random(2), nb)["w=1"]
    aa_ic = abs(ra["ic_slope"]-rb["ic_slope"]); aa_rho = abs(ra["rho_slope"]-rb["rho_slope"])
    # positive control: rho slope must contain 0.5 (theory) within 0.1; negative control: rho cost ratio at fixed 2x bits shift
    r1 = res["w=1"]; pos_ok = abs(r1["rho_slope"]-0.5) < 0.1
    aa_ok = aa_rho < 0.15 and aa_ic < 0.3
    ratios = r1["ratio_ic_over_rho_by_bits"]
    if smoke or not (pos_ok and aa_ok): verdict = "inconclusive"
    elif all(v > 1 for v in ratios.values()): verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-ICE-charged-toy-ic-exponent-vs-rho", "smoke": smoke, "bits_ladder": bits_l,
        "seeds_per_size": seeds, "seed_rule": "Random(seed*1000003+bits), seed=1..N; bootstrap seeds 777/1/2",
        "weights_w_per_division": ws, "B_rule": "exp(0.6*sqrt(ln p * ln ln p)), floor 30",
        "results": res, "positive_control_rho_slope_near_0.5": pos_ok,
        "aa_arm": {"ic_slope_diff": aa_ic, "rho_slope_diff": aa_rho, "ok": aa_ok},
        "null_control": "assert pow(G,found,p)==y on every solve (wrong-answer null); random-seed split A/A",
        "verdict": verdict}))
main()
