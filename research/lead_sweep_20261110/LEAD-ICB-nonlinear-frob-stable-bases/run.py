#!/usr/bin/env python3
"""Preregistered toy for LEAD-ICB-nonlinear-frob-stable-bases (stdlib only)."""
import sys, json, random, statistics

class GF:
    def __init__(s, n):
        s.n = n; s.q = 1 << n; M = s.q - 1
        for low in range(1, 1 << n, 2):
            poly = (1 << n) | low
            exp = [0] * (2 * M); x = 1; ok = True
            for i in range(M):
                exp[i] = x
                x <<= 1
                if x >> n: x ^= poly
                if x == 1 and i < M - 1: ok = False; break
            if ok and x == 1: break
        for i in range(M, 2 * M): exp[i] = exp[i - M]
        s.exp = exp; s.log = [0] * s.q
        for i in range(M): s.log[exp[i]] = i
        s.M = M
    def mul(s, a, b):
        return 0 if a == 0 or b == 0 else s.exp[s.log[a] + s.log[b]]
    def inv(s, a): return s.exp[s.M - s.log[a]]
    def sq(s, a): return 0 if a == 0 else s.exp[(2 * s.log[a]) % s.M]

def normal_basis(F, rng):
    n = F.n
    while True:
        t = rng.randrange(2, F.q); b = []; x = t
        for _ in range(n): b.append(x); x = F.sq(x)
        rows = list(b); r = 0
        for bit in range(n):
            piv = next((i for i in range(r, n) if rows[i] >> bit & 1), None)
            if piv is None: break
            rows[r], rows[piv] = rows[piv], rows[r]
            for i in range(n):
                if i != r and rows[i] >> bit & 1: rows[i] ^= rows[r]
            r += 1
        if r == n: return b

def points(F):
    """x -> y for x != 0 on y^2+xy=x^3+x^2+1 (odd n, half-trace)."""
    n = F.n; P = {}
    for x in range(1, F.q):
        xi = F.inv(x); c = F.sq(F.mul(x, x) ^ F.mul(x, x) * 0 ^ 0) if False else None
        rhs = F.mul(F.mul(x, F.sq(x)) ^ F.sq(x) ^ 1, F.sq(xi))  # (x^3+x^2+1)/x^2
        z = 0; c = rhs
        for _ in range((n + 1) // 2): z ^= c; c = F.sq(F.sq(c))
        if F.sq(z) ^ z == rhs: P[x] = F.mul(z, x)
    return P

def sumx(F, x1, y1, x2, y2):
    if x1 == x2:
        if y1 != y2: return None  # P + (-P)
        lam = x1 ^ F.mul(y1, F.inv(x1)); return F.sq(lam) ^ lam ^ 1
    lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
    return F.sq(lam) ^ lam ^ x1 ^ x2 ^ 1

def efficiency(F, pts, base):
    base = [x for x in base if x in pts]; S = set()
    for i in range(len(base)):
        x1 = base[i]; y1 = pts[x1]
        for j in range(i + 1, len(base)):
            x2 = base[j]; y2 = pts[x2]
            for yy in (y2, y2 ^ x2):  # +/- second point
                r = sumx(F, x1, y1, x2, yy)
                if r is not None: S.add(r)
    pairs = len(base) * (len(base) - 1)
    return len(S) / pairs, len(base)

def orbit(F, x):
    o = [x]; y = F.sq(x)
    while y != x: o.append(y); y = F.sq(y)
    return o

def ball(F, basis, w):
    from itertools import combinations
    out = set()
    for k in range(1, w + 1):
        for c in combinations(range(F.n), k):
            v = 0
            for i in c: v ^= basis[i]
            out.add(v)
    return out

def orbit_control(F, pts, k, rng):
    keys = list(pts); base = set()
    while len(base) < k:
        base.update(o for o in orbit(F, rng.choice(keys)) if o in pts)
    return sorted(base)

def arith_progression(F, pts, k, rng):
    # positive control: x-coords of multiples jG, j=1..k (known tiny sumset)
    x, y = rng.choice(list(pts.items())); out = [x]; cx, cy = x, y
    # compute multiples via chord/tangent with full y
    def add(x1, y1, x2, y2):
        if x1 == x2:
            lam = x1 ^ F.mul(y1, F.inv(x1)); x3 = F.sq(lam) ^ lam ^ 1
            return x3, F.sq(x1) ^ F.mul(lam ^ 1, x3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2)); x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ 1
        return x3, F.mul(lam, x1 ^ x3) ^ x3 ^ y1
    px, py = x, y
    for _ in range(k - 1):
        try: px, py = add(px, py, x, y)
        except ZeroDivisionError: break
        if px == 0: break
        out.append(px)
    return out

def run(n, w, ncontrol, seed):
    rng = random.Random(seed * 1000 + n); F = GF(n); pts = points(F)
    basis = normal_basis(F, rng)
    B = sorted(x for x in ball(F, basis, w) if x in pts)
    eB, k = efficiency(F, pts, B)
    ctrl = [efficiency(F, pts, orbit_control(F, pts, k, rng))[0] for _ in range(ncontrol)]
    half = ncontrol // 2
    aa = statistics.mean(ctrl[:half]) / statistics.mean(ctrl[half:2 * half])
    mc = statistics.mean(ctrl); sd = statistics.pstdev(ctrl)
    pos = efficiency(F, pts, arith_progression(F, pts, k, rng))[0] / mc
    return dict(n=n, weight=w, k=k, E_ball=eB, E_ctrl_mean=mc, E_ctrl_sd=sd,
                ratio=eB / mc, z=(eB - mc) / (sd or 1e-12), aa_ratio=aa, positive_ratio=pos)

def main():
    smoke = "--smoke" in sys.argv
    plan = [(9, 2, 4)] if smoke else [(11, 3, 16), (13, 3, 16), (15, 3, 12), (17, 3, 8)]
    seed = 20261110; rows = [run(n, w, c, seed) for n, w, c in plan]
    pos_ok = all(r["positive_ratio"] < 0.2 for r in rows)
    aa_ok = all(abs(r["aa_ratio"] - 1) < 0.05 for r in rows)
    if not smoke and not (pos_ok and aa_ok): v = "inconclusive"
    elif any(r["ratio"] > 1.10 and r["z"] > 3 for r in rows): v = "falsified"
    elif all(0.90 <= r["ratio"] <= 1.10 for r in rows): v = "consistent_with_prediction"
    else: v = "falsified" if any(r["ratio"] > 1.10 for r in rows) else "inconclusive"
    if smoke and not (pos_ok and aa_ok): v = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICB-nonlinear-frob-stable-bases", smoke=smoke, seed=seed,
        curve="y^2+xy=x^3+x^2+1 over GF(2^n)", rows=rows, positive_control_ok=pos_ok,
        aa_ok=aa_ok, verdict=v)))
main()
