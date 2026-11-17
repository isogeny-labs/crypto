#!/usr/bin/env python3
"""Preregistered toy: negation-map rho with fruitless-cycle escape vs plain rho. stdlib only."""
import argparse, json, math, random, time

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13):
        if n % q == 0: return n == q
    i = 17
    while i*i <= n:
        if n % i == 0: return False
        i += 2
    return True

def make_curve(bits, rng):
    while True:
        p = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if not is_prime(p) or p % 3 != 1: continue
        sq = bytearray(p)
        for y in range(p): sq[y*y % p] = 1
        for _ in range(60):
            a, b = rng.randrange(p), rng.randrange(p)
            if (4*a**3 + 27*b*b) % p == 0: continue
            cnt = 1
            for x in range(p):
                r = (x*x*x + a*x + b) % p
                if r == 0: cnt += 1
                elif sq[r]: cnt += 2
            if is_prime(cnt) and cnt > 100:
                # find generator point
                while True:
                    x = rng.randrange(p); r = (x**3 + a*x + b) % p
                    if r and sq[r]:
                        y = next(y for y in _sqrt_iter(r, p))
                        return p, a, b, cnt, (x, y)

def _sqrt_iter(r, p):
    # Tonelli-Shanks
    if p % 4 == 3:
        y = pow(r, (p+1)//4, p); yield y; return
    q, s = p-1, 0
    while q % 2 == 0: q //= 2; s += 1
    z = 2
    while pow(z, (p-1)//2, p) != p-1: z += 1
    m, c, t, R = s, pow(z, q, p), pow(r, q, p), pow(r, (q+1)//2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1: t2 = t2*t2 % p; i += 1
        b = pow(c, 1 << (m-i-1), p)
        m, c, t, R = i, b*b % p, t*b*b % p, R*b % p
    yield R

class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        p = s.p; x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            l = (3*x1*x1 + s.a) * pow(2*y1, -1, p) % p
        else:
            l = (y2-y1) * pow(x2-x1, -1, p) % p
        x3 = (l*l - x1 - x2) % p
        return (x3, (l*(x1-x3) - y1) % p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R

R_PART = 16

def run_walk(E, n, P, Q, x_true, mode, rng, cap):
    """mode: plain | neg_noescape | neg_escape. Returns (steps, ok, fruitless_count)."""
    p = E.p
    M = []
    for _ in range(R_PART):
        u, v = rng.randrange(1, n), rng.randrange(1, n)
        M.append((u, v, E.add(E.mul(u, P), E.mul(v, Q))))
    a, b = rng.randrange(1, n), rng.randrange(1, n)
    X = E.add(E.mul(a, P), E.mul(b, Q))
    neg = mode != "plain"
    seen = {}
    fruitless = 0
    for step in range(1, cap+1):
        if X is None:
            a, b = rng.randrange(1, n), rng.randrange(1, n)
            X = E.add(E.mul(a, P), E.mul(b, Q)); continue
        if neg and X[1] > p - X[1]:
            X = (X[0], p - X[1]); a, b = (-a) % n, (-b) % n
        key = X if not neg else X[0]
        if key in seen:
            a0, b0 = seen[key]
            if (a0, b0) == (a, b):
                fruitless += 1
                if mode == "neg_noescape":
                    return step, False, fruitless
                X = E.add(X, X); a, b = 2*a % n, 2*b % n   # escape by doubling
                continue
            db = (b - b0) % n
            if db == 0:
                fruitless += 1
                if mode == "neg_noescape": return step, False, fruitless
                X = E.add(X, X); a, b = 2*a % n, 2*b % n; continue
            xs = (a0 - a) * pow(db, -1, n) % n
            return step, xs == x_true, fruitless
        seen[key] = (a, b)
        u, v, T = M[X[0] % R_PART]
        X = E.add(X, T); a, b = (a+u) % n, (b+v) % n
    return cap, False, fruitless

def boot_ci(xs, ys, rng, B=400):
    rs = []
    for _ in range(B):
        sx = [xs[rng.randrange(len(xs))] for _ in xs]
        sy = [ys[rng.randrange(len(ys))] for _ in ys]
        rs.append(sum(sx)/sum(sy)*len(ys)/len(xs))
    rs.sort()
    return rs[int(.025*B)], rs[int(.975*B)]

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--smoke", action="store_true")
    A = ap.parse_args()
    t0 = time.time()
    SEED = 20261110
    bits_list = [10] if A.smoke else [14, 16, 18, 20]
    runs = 20 if A.smoke else {14: 600, 16: 500, 18: 400, 20: 250}
    primary_min_bits = 10 if A.smoke else 18
    rng = random.Random(SEED)
    per_size, pooled = [], {"plain": [], "plain2": [], "neg_escape": [], "neg_noescape": []}
    pooled_n = []
    for bits in bits_list:
        p, a, b, n, G = make_curve(bits, rng)
        E = Curve(p, a, b)
        x_true = rng.randrange(2, n-1); Q = E.mul(x_true, G)
        nr = runs if A.smoke else runs[bits]
        res = {k: [] for k in pooled}; fr = 0; fails = 0; bad = 0
        for i in range(nr):                      # interleaved arms, derived seeds
            for mode in ("plain", "plain2", "neg_escape", "neg_noescape"):
                m = "plain" if mode == "plain2" else mode
                r = random.Random(f"{SEED}-{bits}-{i}-{mode}")
                s, ok, f = run_walk(E, n, G, Q, x_true, m, r, int(60*math.sqrt(n))+2000)
                if mode == "neg_noescape":
                    fails += (not ok); fr += f
                    if ok: res[mode].append(s)
                else:
                    if ok: res[mode].append(s)
                    else: bad += 1
        row = {"bits": bits, "p": p, "n": n, "runs": nr, "wrong_or_capped": bad,
               "noescape_fail_rate": fails/nr, "escape_events_noescape_arm": fr}
        for k in res: row["mean_" + k] = sum(res[k])/max(1, len(res[k]))
        row["mean_over_sqrt_pi_n_4"] = {k: row["mean_"+k]/math.sqrt(math.pi*n/4) for k in res}
        per_size.append(row)
        if bits >= primary_min_bits:
            for k in res: pooled[k] += [s/math.sqrt(n) for s in res[k]]
    brng = random.Random(SEED + 1)
    prim = boot_ci(pooled["neg_escape"], pooled["plain"], brng) if pooled["plain"] else (0, 0)
    aa = boot_ci(pooled["plain2"], pooled["plain"], brng) if pooled["plain"] else (0, 0)
    ratio = (sum(pooled["neg_escape"])/len(pooled["neg_escape"])) / (sum(pooled["plain"])/len(pooled["plain"])) if pooled["plain"] else 0
    aa_ratio = (sum(pooled["plain2"])/len(pooled["plain2"])) / (sum(pooled["plain"])/len(pooled["plain"])) if pooled["plain"] else 0
    # positive control: ideal random mapping on n vs n/2 states (birthday), expected ratio 1/sqrt2
    prng = random.Random(SEED + 2); N = 1 << 16
    def birthday(M, k):
        t = 0
        for _ in range(k):
            s = set(); c = 0
            while True:
                c += 1; v = prng.randrange(M)
                if v in s: break
                s.add(v)
            t += c
        return t/k
    k = 300 if not A.smoke else 20
    pos_ratio = birthday(N//2, k) / birthday(N, k)
    pos_ok = abs(pos_ratio - 0.7071) < 0.12
    aa_ok = aa[0] <= 1.0 <= aa[1]
    lo, hi = 0.62, 0.85
    if not (pos_ok and aa_ok): verdict = "inconclusive"
    elif prim[0] >= lo and prim[1] <= hi: verdict = "consistent_with_prediction"
    elif prim[0] > hi or prim[1] < lo: verdict = "falsified"
    else: verdict = "inconclusive"
    if A.smoke: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-RHO-negation-map-fruitless-cycles", "smoke": A.smoke,
        "seed": SEED, "r_partition": R_PART, "sizes_bits": bits_list,
        "statistic": "mean steps neg_escape / mean steps plain (pooled, steps/sqrt(n), sizes>=primary)",
        "ratio": ratio, "ratio_ci95": prim, "threshold_band": [lo, hi],
        "A_A": {"ratio": aa_ratio, "ci95": aa, "ok": aa_ok},
        "positive_control_birthday_half_space": {"ratio": pos_ratio, "ok": pos_ok},
        "negative_control": "plain vs plain2 (A/A) and neg_noescape fail rate",
        "per_size": per_size, "verdict": verdict, "elapsed_s": time.time()-t0}))
main()
