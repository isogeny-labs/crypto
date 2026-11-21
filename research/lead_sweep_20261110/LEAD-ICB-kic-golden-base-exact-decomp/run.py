#!/usr/bin/env python3
"""Toy: does an affine 2-flat x-coordinate factor base change exact m=2 sumset coverage vs random base of equal size?"""
import sys, json, random, math
POLY = {9: (1<<9)|(1<<4)|1, 11: (1<<11)|(1<<2)|1, 13: (1<<13)|(1<<4)|(1<<3)|(1<<1)|1}

def build(n):
    p = POLY[n]; N = 1 << n
    exp = [0]*(2*N); log = [0]*N
    # find primitive element
    for g in range(2, N):
        x = 1; seen = 0; ok = True
        for i in range(N-1):
            exp[i] = x; log[x] = i
            x <<= 1
            if x >> n: x ^= p
            if x == 1 and i < N-2: ok = False; break
        if ok and x == 1: break
    for i in range(N-1, 2*N): exp[i] = exp[i-(N-1)]
    return exp, log

def run(n, l, trials, seed):
    rng = random.Random(seed); N = 1 << n
    exp, log = build(n)
    def mul(a, b): return 0 if a == 0 or b == 0 else exp[log[a]+log[b]]
    def inv(a): return exp[(N-1)-log[a]]
    def tr(a):
        t = 0; x = a
        for _ in range(n): t ^= x; x = mul(x, x)
        return t
    def halftrace_solve(c):  # t^2+t=c, n odd: t=sum c^(2^(2i)), i=0..(n-1)/2
        t = 0; x = c
        for _ in range((n+1)//2): t ^= x; x = mul(mul(x, x), mul(x, x))
        return t
    def curve(a, b):
        pts = {}
        for x in range(1, N):
            c = x ^ a ^ mul(b, inv(mul(x, x)))
            if tr(c) == 0:
                t = halftrace_solve(c)
                y = mul(t, x); pts[x] = (y, y ^ x)
        return pts
    def add(P, Q, a):
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            lam = x1 ^ mul(y1, inv(x1)); x3 = mul(lam, lam) ^ lam ^ a
        else:
            lam = mul(y1 ^ y2, inv(x1 ^ x2)); x3 = mul(lam, lam) ^ lam ^ x1 ^ x2 ^ a
        y3 = mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)
    def sumset(base, a):
        s = set()
        for i in range(len(base)):
            for j in range(i+1, len(base)):
                s.add(add(base[i], base[j], a))
        return len(s)
    def flat_xs(valid):
        while True:
            basis = [rng.randrange(1, N) for _ in range(l)]
            xs = {rng.randrange(N)}
            for v in basis: xs |= {x ^ v for x in xs}
            if len(xs) == (1 << l): return [x for x in xs if x in valid]
    D, DAA, POS, EF = [], [], [], []
    for _ in range(trials):
        a = rng.randrange(2); b = rng.randrange(1, N)
        pts = curve(a, b); valid = sorted(pts)
        if len(valid) < 4*(1 << l): continue
        fx = flat_xs(pts); k = len(fx)
        if k < 4: continue
        bf = [(x, pts[x][s]) for x in fx for s in (0, 1)]
        r1 = rng.sample(valid, k); r2 = rng.sample(valid, k)
        b1 = [(x, pts[x][s]) for x in r1 for s in (0, 1)]
        b2 = [(x, pts[x][s]) for x in r2 for s in (0, 1)]
        sf, s1, s2 = sumset(bf, a), sumset(b1, a), sumset(b2, a)
        D.append(sf/s1 - 1); DAA.append(s2/s1 - 1)
        EF.append(len(bf)*(len(bf)-1)/2)
        # positive control: +-kG progression of same point count
        G = pts[rng.choice(valid)][0:2]; G = None
        x0 = rng.choice(valid); Gp = (x0, pts[x0][0]); mult = [Gp]; cur = Gp
        need = len(bf)//2
        for _ in range(need-1):
            nxt = add(cur, Gp, a) if cur[0] != Gp[0] else None
            if nxt is None:
                if cur == Gp:  # double
                    x1, y1 = Gp; lam = x1 ^ mul(y1, inv(x1)); x3 = mul(lam, lam)^lam^a
                    nxt = (x3, mul(lam, x1^x3)^x3^y1)
                else: break
            mult.append(nxt); cur = nxt
        pb = mult + [(x, y ^ x) for x, y in mult]
        POS.append(sumset(pb, a)/s1 - 1)
    def ms(v):
        m = sum(v)/len(v); sd = math.sqrt(sum((u-m)**2 for u in v)/max(1, len(v)-1))
        return m, sd/math.sqrt(len(v)), (m/(sd/math.sqrt(len(v))) if sd > 0 else 0.0)
    return D, DAA, POS, ms

def main():
    smoke = '--smoke' in sys.argv
    n, l, trials, seed = (9, 3, 6, 20261110) if smoke else (13, 6, 120, 20261110)
    D, DAA, POS, ms = run(n, l, trials, seed)
    if len(D) < 3:
        print(json.dumps({"verdict": "inconclusive", "reason": "too few usable trials", "n_trials": len(D)})); return
    md, sed, zd = ms(D); ma, sea, za = ms(DAA); mp, sep, zp = ms(POS)
    if mp > -0.3: v = "inconclusive"          # detector failed positive control
    elif abs(za) > 4: v = "inconclusive"      # noise floor/null broken
    elif abs(zd) > 4 and abs(md) > 0.01: v = "falsified"
    elif abs(md) < 0.02: v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICB-kic-golden-base-exact-decomp", "smoke": smoke,
        "statistic": "mean paired relative sumset-size difference flat vs random base (m=2 exact decomposition coverage)",
        "flat_vs_null": {"mean": md, "se": sed, "z": zd},
        "AA_null_vs_null": {"mean": ma, "se": sea, "z": za},
        "positive_control_progression": {"mean": mp, "se": sep, "z": zp},
        "params": {"n": n, "flat_dim": l, "trials_used": len(D), "trials_requested": trials, "poly": POLY[n]},
        "seeds": {"master": seed}, "thresholds": {"consistent": "|mean|<0.02", "falsified": "|z|>4 and |mean|>0.01", "pos_control": "mean<-0.3", "AA": "|z|<=4"},
        "verdict": v}))
main()
