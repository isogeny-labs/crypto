#!/usr/bin/env python3
"""LEAD-EXO-hnp-advice-leakage: Cheon auxiliary-input DLP vs generic rho, counted ops, stdlib only."""
import sys, json, math, random

def factor(m):
    f = {}; p = 2
    while p * p <= m:
        while m % p == 0: f[p] = f.get(p, 0) + 1; m //= p
        p += 1 if p == 2 else 2
    if m > 1: f[m] = f.get(m, 0) + 1
    return f

def isprime(m):
    if m < 2: return False
    i = 2
    while i * i <= m:
        if m % i == 0: return False
        i += 1
    return True

def divisors(f):
    ds = [1]
    for p, e in f.items(): ds = [d * p ** k for d in ds for k in range(e + 1)]
    return sorted(ds)

def pick_prime(bits, rng):
    """prime n ~2^bits with a divisor d of n-1 closest to sqrt(n), d>=4, k=(n-1)/d."""
    best = None
    c = (1 << bits) + 1 + 2 * rng.randrange(1 << (bits - 3))
    tries = 0
    while tries < 60:
        c += 2
        if not isprime(c): continue
        tries += 1
        ds = divisors(factor(c - 1)); t = math.isqrt(c)
        d = min((x for x in ds if x >= 4), key=lambda x: abs(math.log(x / t)))
        sc = abs(math.log(d / t))
        if best is None or sc < best[0]: best = (sc, c, d)
        if sc < 0.15: break
    return best[1], best[2]

def primroot(n):
    fs = list(factor(n - 1))
    g = 2
    while any(pow(g, (n - 1) // q, n) == 1 for q in fs): g += 1
    return g

def bsgs_sub(base, order, target_fn, start_fn):
    raise NotImplementedError

def cheon(n, d, alpha, G, cnt):
    """Group is Z_n (points are ints, scalar mult = x*P mod n; each counted as one scalar mult).
    Oracle gives G, aG, a^d G. Returns alpha."""
    k = (n - 1) // d
    g = primroot(n)
    P1 = alpha * G % n; Pd = pow(alpha, d, n) * G % n
    # stage 1: find m with zeta^m G = a^d G, zeta = g^d of order k
    zeta = pow(g, d, n); t = math.isqrt(k) + 1
    baby = {}; z = 1
    for b in range(t):
        baby.setdefault(z * G % n, b); z = z * zeta % n; cnt[0] += 1
    zt_inv = pow(pow(zeta, t, n), -1, n); cur = Pd; m = None
    for a in range(t + 1):
        if cur in baby: m = (a * t + baby[cur]) % k; break
        cur = zt_inv * cur % n; cnt[0] += 1
    if m is None: return None
    # stage 2: alpha = g^(m+k u), u in [0,d); rho = g^k order d
    rho = pow(g, k, n); t2 = math.isqrt(d) + 1
    base = pow(g, m, n) * G % n; cnt[0] += 1
    baby = {}; z = 1
    for b in range(t2):
        baby.setdefault(z * base % n, b); z = z * rho % n; cnt[0] += 1
    ri = pow(pow(rho, t2, n), -1, n); cur = P1
    for a in range(t2 + 1):
        if cur in baby:
            u = a * t2 + baby[cur]; return pow(g, m + k * u, n)
        cur = ri * cur % n; cnt[0] += 1
    return None

def bsgs_plain(n, alpha, G):
    """No-advice control (d=1): plain BSGS on the same group; returns scalar-mult count (group adds)."""
    t = math.isqrt(n) + 1; ops = 0; baby = {}; P = alpha * G % n
    for b in range(t): baby.setdefault(b * G % n, b); ops += 1
    for a in range(t + 1):
        q = (P - a * t * G) % n; ops += 1
        if q in baby: return (a * t + baby[q]) % n, ops
    return None, ops

def rho(n, alpha, G, rng):
    """Generic Pollard rho (Floyd), 16-way partition; point tracked as (x,a,b) with x=aG+b(alphaG); 1 group add/step."""
    P = alpha * G % n
    while True:
        r = [(rng.randrange(n), rng.randrange(n)) for _ in range(16)]
        rt = [(( ai + bi * alpha) * G) % n for ai, bi in r]
        def step(s):
            x, a, b = s; j = x % 16
            return ((x + rt[j]) % n, (a + r[j][0]) % n, (b + r[j][1]) % n)
        a0, b0 = rng.randrange(n), rng.randrange(n)
        s0 = ((a0 + b0 * alpha) * G % n, a0, b0); T = H = s0; ops = 0
        while True:
            T = step(T); H = step(step(H)); ops += 3
            if T[0] == H[0]: break
        db = (T[2] - H[2]) % n
        if db == 0: continue
        return ((H[1] - T[1]) * pow(db, -1, n)) % n, ops

def med(v): v = sorted(v); return v[len(v) // 2]

def run(bits_list, trials, seed):
    rng = random.Random(seed); rows = []; fails = 0; wrong = 0
    for bits in bits_list:
        n, d = pick_prime(bits, rng); cs, rs, bs, aa = [], [], [], []
        for _ in range(trials):
            G = rng.randrange(1, n); alpha = rng.randrange(2, n - 1)
            cnt = [0]; a = cheon(n, d, alpha, G, cnt)
            if a is None: fails += 1
            elif a * G % n != alpha * G % n: wrong += 1
            else: cs.append(cnt[0])
            x, o1 = rho(n, alpha, G, random.Random(rng.random())); rs.append(o1)
            x2, o2 = rho(n, alpha, G, random.Random(rng.random())); aa.append(o2)
            if x * G % n != alpha * G % n: wrong += 1
            xb, ob = bsgs_plain(n, alpha, G); bs.append(ob)
            if xb is None or xb * G % n != alpha * G % n: wrong += 1
        rows.append(dict(bits=bits, n=n, d=d, k=(n - 1) // d, cheon_med=med(cs) if cs else None,
                         rho_med=med(rs), rho_aa_med=med(aa), bsgs_med=med(bs),
                         ratio_cheon_rho_unit=(med(cs) / med(rs)) if cs else None,
                         ratio_cheon_rho_logn=(med(cs) * math.log2(n) / med(rs)) if cs else None,
                         aa_ratio=med(rs) / med(aa)))
    return rows, fails, wrong

def slope(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def main():
    smoke = "--smoke" in sys.argv
    bits = [10, 12] if smoke else [12, 14, 16, 18, 20, 22]
    trials = 3 if smoke else 25
    seed = 20261110
    rows, fails, wrong = run(bits, trials, seed)
    ok = [r for r in rows if r["cheon_med"]]
    out = dict(lead="LEAD-EXO-hnp-advice-leakage", smoke=smoke, seed=seed, trials=trials, rows=rows,
               recovery_failures=fails, wrong_answers=wrong,
               predicted_exponent_cheon=0.25, rho_exponent=0.5)
    if len(ok) >= 2:
        xs = [r["bits"] for r in ok]
        ec = slope(xs, [math.log2(r["cheon_med"]) for r in ok]) 
        er = slope(xs, [math.log2(r["rho_med"]) for r in ok])
        eb = slope(xs, [math.log2(r["bsgs_med"]) for r in ok])
        out.update(fit_exponent_cheon=ec, fit_exponent_rho=er, fit_exponent_bsgs_null=eb,
                   aa_ratio_median=med([r["aa_ratio"] for r in rows]))
        pos = fails == 0 and wrong == 0
        if smoke or not pos: v = "inconclusive"
        elif ec <= 0.35 and er >= 0.42 and eb >= 0.42 and 0.7 <= out["aa_ratio_median"] <= 1.4: v = "consistent_with_prediction"
        elif ec > 0.35: v = "falsified"
        else: v = "inconclusive"
    else: v = "inconclusive"
    out["verdict"] = v
    print(json.dumps(out))

main()
