#!/usr/bin/env python3
"""Toy re-derivation of the cryptoscale candidate screen (stdlib only)."""
import sys, json, random, math

OPS = [0]
def inv(x, p): return pow(x, -1, p)
def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    OPS[0] += 1
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        l = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)
def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R
def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True
def factor(n):
    f = {}; d = 2
    while d * d <= n:
        while n % d == 0: f[d] = f.get(d, 0) + 1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0) + 1
    return f
def prime_near(k, rng):
    n = (1 << k) + rng.randrange(1 << (k - 1))
    while not isprime(n): n += 1
    return n
def count(a, b, p):
    sq = {}
    for y in range(p): sq[y * y % p] = sq.get(y * y % p, 0) + 1
    return 1 + sum(sq.get((x * x * x + a * x + b) % p, 0) for x in range(p))
def randpt(a, b, p, rng):
    while True:
        x = rng.randrange(p); s = (x ** 3 + a * x + b) % p
        if s == 0: return (x, 0)
        if pow(s, (p - 1) // 2, p) == 1:
            for y in range(1, p):  # small p: sqrt by Tonelli-free search avoided below
                break
            y = sqrt_mod(s, p)
            return (x, y)
def sqrt_mod(s, p):
    if p % 4 == 3: return pow(s, (p + 1) // 4, p)
    q, e = p - 1, 0
    while q % 2 == 0: q //= 2; e += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = e, pow(z, q, p), pow(s, q, p), pow(s, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r
def curve(k, rng, smooth):
    if smooth:  # planted weakness: y^2=x^3+b over p=2 mod 3 has order p+1; pick p+1 smooth
        for _ in range(200000):
            p = (1 << k) + rng.randrange(1 << (k - 1))
            if p % 3 != 2 or not isprime(p): continue
            f = factor(p + 1)
            if max(f) <= 2 ** (k // 4): return p, 0, rng.randrange(1, p), p + 1, f
        return None
    for _ in range(4000):
        p = prime_near(k, rng)
        a, b = rng.randrange(p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        n = count(a, b, p); f = factor(n); big = max(f)
        if smooth and big <= 2 ** (k * 2 // 5) and n > 2 ** (k - 2): return p, a, b, n, f
        if not smooth and big * 1 == n and n > 2 ** (k - 1): return p, a, b, n, f
    return None
def setup(inst, k, rng, smooth):
    p, a, b, n, f = inst
    G = randpt(a, b, p, rng)
    d = n
    for q in f:
        while d % q == 0 and mul(d // q, G, a, p) is None: d //= q
    x = rng.randrange(1, d)
    return p, a, b, G, d, x, mul(x, G, a, p)
def bsgs(G, Q, d, a, p):
    m = math.isqrt(d) + 1; tab = {}; R = None
    for j in range(m):
        tab.setdefault(R, j); R = add(R, G, a, p)
    S = mul(m, G, a, p); S = None if S is None else (S[0], (-S[1]) % p); T = Q
    for i in range(m + 1):
        if T in tab: return (i * m + tab[T]) % d
        T = add(T, S, a, p)
    return None
def rho(G, Q, d, a, p, rng):
    mask = max(1, int(math.isqrt(d) // 8)); nb = 16
    mult = []
    for _ in range(nb):
        s, t = rng.randrange(d), rng.randrange(d)
        mult.append((s, t, add(mul(s, G, a, p), mul(t, Q, a, p), a, p)))
    seen = {}
    for _ in range(400):
        s, t = rng.randrange(d), rng.randrange(d)
        R = add(mul(s, G, a, p), mul(t, Q, a, p), a, p)
        for _ in range(30 * mask):
            if R is None: break
            if R[0] % mask == 0:
                if R in seen and seen[R] != (s, t):
                    s2, t2 = seen[R]; dt = (t - t2) % d
                    if math.gcd(dt, d) == 1: return (s2 - s) * pow(dt, -1, d) % d
                    break
                seen[R] = (s, t); break
            j = R[0] % nb; ds, dt2, M = mult[j]
            R = add(R, M, a, p); s = (s + ds) % d; t = (t + dt2) % d
    return None
def pohlig(G, Q, d, a, p):
    x, mod = 0, 1
    for q, e in factor(d).items():
        xq = 0
        for i in range(e):
            Gq = mul(d // q, G, a, p)
            h = add(Q, mul((d - xq) % d, G, a, p), a, p)
            h = mul(d // q ** (i + 1), h, a, p)
            xq += (bsgs(Gq, h, q, a, p) or 0) * q ** i
        qe = q ** e
        x = x + mod * ((xq - x) * pow(mod, -1, qe) % qe); mod *= qe
    return x % d
def med(v): v = sorted(v); return v[len(v) // 2]
def boot(v, rng, B=300):
    ms = sorted(med([rng.choice(v) for _ in v]) for _ in range(B))
    return ms[int(0.025 * B)], ms[int(0.975 * B)]
def run(sizes, ninst, seed):
    rng = random.Random(seed); res = {}
    arms = ["rhoA", "rhoB", "bsgs", "ph_smooth_planted", "ph_generic"]
    cost = {s: {a: [] for a in arms} for s in sizes}
    for k in sizes:
        for i in range(ninst):
            for smooth in ((True,) if k >= 20 else (False,)):
                c = curve(k, rng, smooth)
                if c is None: continue
                p, a, b, G, d, x, Q = setup(c, k, rng, smooth)
                sd = math.sqrt(d)
                def meas(fn, name):
                    OPS[0] = 0; r = fn()
                    if r is None or mul(r, G, a, p) != Q: return
                    cost[k][name].append(OPS[0] / sd)
                if smooth: meas(lambda: pohlig(G, Q, d, a, p), "ph_smooth_planted")
                else:
                    meas(lambda: rho(G, Q, d, a, p, random.Random(rng.random())), "rhoA")
                    meas(lambda: rho(G, Q, d, a, p, random.Random(rng.random())), "rhoB")
                    meas(lambda: bsgs(G, Q, d, a, p), "bsgs")
                    meas(lambda: pohlig(G, Q, d, a, p), "ph_generic")
    pooled = {a: [v for k in sizes for v in cost[k][a]] for a in arms}
    st = {}; ref = pooled["bsgs"]; rm = med(ref) if ref else None
    for a in arms:
        v = pooled[a]
        if not v or rm is None: st[a] = {"n": len(v)}; continue
        lo, hi = boot(v, rng)
        st[a] = {"n": len(v), "median_ops_over_sqrt_r": med(v), "ci95": [lo, hi],
                 "ratio_to_bsgs": med(v) / rm, "candidate": hi / rm < 0.8}
    ok = rm is not None and all(len(pooled[a]) >= 3 for a in arms)
    if not ok: verdict = "inconclusive"
    elif not st["ph_smooth_planted"]["candidate"]: verdict = "inconclusive"
    elif any(st[a]["candidate"] for a in ("rhoA", "rhoB", "ph_generic")): verdict = "falsified"
    else: verdict = "consistent_with_prediction"
    return {"lead": "LEAD-EXO-cryptoscale-candidate-screen", "seed": seed, "sizes_log2p": sizes,
            "instances_per_size": ninst, "threshold": "upper CI95 of arm median / bsgs median < 0.8 (bsgs = best generic reference)",
            "arms": st, "negative_controls": ["rhoA", "rhoB (A/A)", "ph_generic"], "AA_rhoB_over_rhoA": (st["rhoB"].get("median_ops_over_sqrt_r", 0) / st["rhoA"]["median_ops_over_sqrt_r"]) if st["rhoA"].get("median_ops_over_sqrt_r") else None,
            "positive_control": "ph_smooth_planted", "verdict": verdict}
if __name__ == "__main__":
    if "--smoke" in sys.argv: out = run([10, 28], 3, 1)
    else: out = run([12, 14, 16, 28, 32], 14, 20261110)
    print(json.dumps(out))
