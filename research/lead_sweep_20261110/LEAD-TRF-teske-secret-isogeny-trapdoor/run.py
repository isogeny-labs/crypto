#!/usr/bin/env python3
"""Toy test: does a non-invariant structure (2-torsion rank) inside one F_p isogeny class
(fixed trace t => fixed N, Tate) change generic rho cost on the largest prime-order subgroup?"""
import json, math, random, sys

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

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def sqrt_mod(n, p, rt):
    return rt.get(n)

def rho(P, Q, l, a, p, rng):
    """Floyd rho, 16-way r-adding walk; returns (steps, ok). Q = kP, subgroup order l prime."""
    for _ in range(20):
        M = []
        for _ in range(16):
            u, v = rng.randrange(l), rng.randrange(l)
            M.append((u, v, add(mul(u, P, a, p), mul(v, Q, a, p), a, p)))
        def step(X, A, B):
            i = 0 if X is None else X[0] & 15
            u, v, T = M[i]
            return add(X, T, a, p), (A + u) % l, (B + v) % l
        a0, b0 = rng.randrange(l), rng.randrange(l)
        X0 = add(mul(a0, P, a, p), mul(b0, Q, a, p), a, p)
        x, A, B = X0, a0, b0
        y, C, D = X0, a0, b0
        n = 0
        while n < 40 * l:
            x, A, B = step(x, A, B)
            y, C, D = step(*step(y, C, D))
            n += 1
            if x == y: break
        else:
            continue
        dd = (B - D) % l
        if dd == 0: continue
        k = (C - A) * pow(dd, -1, l) % l
        return n, k
    return None, None

def build(p, ncurves, rng):
    chi = [0] * p
    rt = {}
    for y in range(p):
        chi[y * y % p] = 1
    sq = {y * y % p: y for y in range(p)}
    cubes = [x * x * x % p for x in range(p)]
    out = []
    while len(out) < ncurves:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        vals = [(cubes[x] + a * x + b) % p for x in range(p)]
        N = p + 1 + sum((1 if chi[v] else -1) if v else 0 for v in vals)
        roots = sum(1 for v in vals if v == 0)
        out.append((a, b, N, roots))
    return out, sq

def rand_point(a, b, p, sq, rng):
    while True:
        x = rng.randrange(p); v = (x ** 3 + a * x + b) % p
        if v in sq: return (x, sq[v])

def cost(curve, p, sq, rng, trials, order_l=None):
    a, b, N, _ = curve
    l = order_l or max(factor(N))
    ratios = []
    for _ in range(trials):
        while True:
            G = rand_point(a, b, p, sq, rng)
            P = mul(N // l, G, a, p)
            if P is not None: break
        k0 = rng.randrange(1, l)
        Q = mul(k0, P, a, p)
        n, k = rho(P, Q, l, a, p, rng)
        if n is None or k != k0: return None
        ratios.append(n)
    return sum(ratios) / len(ratios), l

def boot(xs, rng, B=2000):
    n = len(xs); ms = []
    for _ in range(B):
        ms.append(sum(xs[rng.randrange(n)] for _ in range(n)) / n)
    ms.sort()
    return ms[int(.025 * B)], ms[int(.975 * B)]

def main():
    smoke = "--smoke" in sys.argv
    SEED = 20261110
    p = 211 if smoke else 4093
    ncurves = 400 if smoke else 5000
    trials = 3 if smoke else 10
    maxpairs = 12 if smoke else 120
    margin = 0.15
    rng = random.Random(SEED)
    curves, sq = build(p, ncurves, rng)
    # class = fixed N (trace). arms by 2-torsion rank: roots==3 (rank2) vs roots==1 (rank1), N even
    byN = {}
    for c in curves:
        if c[2] % 2 == 0 and c[3] in (1, 3) and max(factor(c[2])) > 7:
            byN.setdefault(c[2], {1: [], 3: []})[c[3]].append(c)
    pairs = []
    for N in sorted(byN):
        d = byN[N]
        for i in range(min(len(d[1]), len(d[3]))):
            pairs.append((d[3][i], d[1][i]))
    rng.shuffle(pairs); pairs = pairs[:maxpairs]
    diffs, aa, fails = [], [], 0
    for c3, c1 in pairs:
        r3 = cost(c3, p, sq, rng, trials); r1 = cost(c1, p, sq, rng, trials)
        r1b = cost(c1, p, sq, rng, trials)  # A/A: same curve, independent seeds
        if not (r3 and r1 and r1b): fails += 1; continue
        l = r3[1]
        diffs.append(math.log(r3[0] / r1[0]))
        aa.append(math.log(r1b[0] / r1[0]))
    # positive control: smallest vs largest prime subgroup on the same curves (ratio of sqrt(l) known)
    pc = []
    for c3, _ in pairs[:max(8, maxpairs // 4)]:
        fs = sorted(factor(c3[2]))
        if len(fs) < 2 or fs[-1] < 4 * fs[-2]: continue
        hi = cost(c3, p, sq, rng, trials, fs[-1]); lo = cost(c3, p, sq, rng, trials, fs[-2])
        if hi and lo and fs[-2] >= 5:
            pc.append((math.log(hi[0] / lo[0]), 0.5 * math.log(fs[-1] / fs[-2])))
    pos_ok = bool(pc) and (sum(x for x, _ in pc) / len(pc)) > 0.5 * (sum(y for _, y in pc) / len(pc))
    res = {"lead": "LEAD-TRF-teske-secret-isogeny-trapdoor", "seed": SEED, "p": p,
           "curves_sampled": ncurves, "pairs_used": len(diffs), "trials_per_curve": trials,
           "failures": fails, "margin": margin, "smoke": smoke}
    if len(diffs) >= 3:
        m = sum(diffs) / len(diffs); lo_, hi_ = boot(diffs, rng)
        am = sum(aa) / len(aa); alo, ahi = boot(aa, rng)
        res.update({"statistic_mean_log_ratio_rank2_over_rank1": m, "ci95": [lo_, hi_],
                    "aa_mean": am, "aa_ci95": [alo, ahi],
                    "positive_control_points": len(pc), "positive_control_pass": pos_ok})
        aa_ok = alo <= 0 <= ahi
        if not (pos_ok and aa_ok): v = "inconclusive"
        elif -margin <= lo_ and hi_ <= margin: v = "consistent_with_prediction"
        elif lo_ > margin or hi_ < -margin: v = "falsified"
        else: v = "inconclusive"
    else:
        v = "inconclusive"
    res["verdict"] = v
    print(json.dumps(res))

if __name__=="__main__": main()
