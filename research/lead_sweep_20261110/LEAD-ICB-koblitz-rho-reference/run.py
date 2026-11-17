#!/usr/bin/env python3
"""Toy: signed-Frobenius canonicalisation in Pollard rho on Koblitz curve y^2+xy=x^3+x^2+1 over GF(2^m).
Stdlib only, single core, deterministic seeds."""
import sys, json, random, math

PRIM = {7: 0x83, 11: 0x805, 13: 0x201B, 17: 0x20009}

class Field:
    def __init__(s, m):
        s.m = m; s.q = 1 << m
        poly = PRIM[m]
        s.exp = [0] * (2 * (s.q - 1)); s.log = [0] * s.q
        x = 1
        for i in range(s.q - 1):
            s.exp[i] = x; s.log[x] = i
            x <<= 1
            if x & s.q: x ^= poly
        assert x == 1, "poly not primitive"
        for i in range(s.q - 1, 2 * (s.q - 1)): s.exp[i] = s.exp[i - (s.q - 1)]
    def mul(s, a, b):
        return 0 if a == 0 or b == 0 else s.exp[s.log[a] + s.log[b]]
    def inv(s, a): return s.exp[(s.q - 1 - s.log[a]) % (s.q - 1)]
    def sq(s, a): return 0 if a == 0 else s.exp[(2 * s.log[a]) % (s.q - 1)]

def lucas_order(m):
    v0, v1 = 2, 1
    for _ in range(m - 1): v0, v1 = v1, v1 - 2 * v0
    return (1 << m) + 1 - (v1 if m >= 1 else 2)

def factor(n):
    f = []; d = 2
    while d * d <= n:
        while n % d == 0: f.append(d); n //= d
        d += 1
    if n > 1: f.append(n)
    return f

class Curve:
    def __init__(s, F): s.F = F
    def neg(s, P): return None if P is None else (P[0], P[1] ^ P[0])
    def add(s, P, Q):
        F = s.F
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None   # Q = -P (or doubling 2-torsion)
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ 1             # a=1
        else:
            lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
            x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ 1
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)
    def mulk(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R
    def frob(s, P): return None if P is None else (s.F.sq(P[0]), s.F.sq(P[1]))
    def find_point(s, rng):
        F = s.F; m = F.m
        while True:
            x = rng.randrange(1, F.q)
            c = F.mul(F.mul(F.mul(x, x), x) ^ F.sq(x) ^ 1, F.inv(F.sq(x)))
            t = 0; c2 = c
            for _ in range(m): t ^= c2; c2 = F.sq(c2)
            if t & 0: pass
            # trace is the xor-sum as field element; trace in {0,1}
            if t != 0: continue
            h = 0; c2 = c                      # half trace (m odd)
            for _ in range((m + 1) // 2): h ^= c2; c2 = F.sq(F.sq(c2))
            return (x, F.mul(x, h))

def setup(m, seed):
    rng = random.Random(seed)
    F = Field(m); E = Curve(F)
    N = lucas_order(m); fs = factor(N); n = max(fs); h = N // n
    while True:
        P = E.find_point(rng)
        assert E.mulk(N, P) is None, "order formula wrong"
        G = E.mulk(h, P)
        if G is not None: break
    fG = E.frob(G); lam = None
    for l in range(n):
        if (l * l - l + 2) % n == 0 and E.mulk(l, G) == fG: lam = l; break
    assert lam is not None
    return F, E, N, n, G, lam

def make_arm(E, n, lam, m, kind):
    F = E.F
    lampow = [pow(lam, j, n) for j in range(m)]
    def canon(P):
        """returns (canonical point, multiplier mod n)"""
        if P is None: return None, 1
        if kind == "none": return P, 1
        best = None
        if kind == "neg":
            Q = E.neg(P)
            return (P, 1) if P <= Q else (Q, n - 1)
        x, y = P
        for j in range(m):
            for s in (0, 1):
                c = (x, y ^ x) if s else (x, y)
                if best is None or c < best[0]: best = (c, j, s)
            x = F.sq(x); y = F.sq(y)
        c, j, s = best
        mult = lampow[j] if not s else (n - lampow[j]) % n
        return c, mult
    return canon

def solve_one(E, n, G, Q, canon, rng, cap):
    """Floyd walk(s) with restarts. returns (f_evals, restarts, k or None)"""
    r = 16
    tabs = []
    for _ in range(r):
        u, v = rng.randrange(n), rng.randrange(n)
        tabs.append((E.add(E.mulk(u, G), E.mulk(v, Q)), u, v))
    def f(st):
        P, a, b = st
        if P is None: return st
        R, u, v = tabs[(P[0] * 2654435761 >> 7) & (r - 1)]
        P2, mult = canon(E.add(P, R))
        return (P2, (a + u) * mult % n, (b + v) * mult % n)
    evals = 0; restarts = 0
    while evals < cap:
        a, b = rng.randrange(n), rng.randrange(n)
        P0 = E.add(E.mulk(a, G), E.mulk(b, Q))
        P0, mult = canon(P0); st = (P0, a * mult % n, b * mult % n)
        tor, har = st, st
        found = None
        while evals < cap:
            tor = f(tor); har = f(f(har)); evals += 3
            if tor[0] == har[0]:
                found = (tor, har); break
        if found:
            (_, a1, b1), (_, a2, b2) = found
            db = (b2 - b1) % n
            if db != 0:
                k = (a1 - a2) * pow(db, -1, n) % n
                if E.mulk(k, G) == Q: return evals, restarts, k
        restarts += 1
    return evals, restarts, None

def boot_ratio(x, y, rng, B=500):
    rs = []
    nx, ny = len(x), len(y)
    for _ in range(B):
        mx = sum(x[rng.randrange(nx)] for _ in range(nx)) / nx
        my = sum(y[rng.randrange(ny)] for _ in range(ny)) / ny
        rs.append(mx / my)
    rs.sort()
    return rs[int(0.025 * B)], rs[int(0.975 * B)]

def run(ms, T, base_seed):
    out = {}
    for m in ms:
        F, E, N, n, G, lam = setup(m, base_seed + m)
        cap = int(60 * 3 * math.sqrt(n))
        data = {}
        # interleaved arms, derived seeds; A/A = second "neg" arm with different seed stream
        arms = [("none", "none", 0), ("neg", "neg", 1), ("neg_AA", "neg", 2), ("frob", "frob", 3)]
        canons = {a: make_arm(E, n, lam, m, kind) for a, kind, _ in arms}
        for a, _, _ in arms: data[a] = []
        restarts = {a: 0 for a, _, _ in arms}; fails = {a: 0 for a, _, _ in arms}
        for t in range(T):
            krng = random.Random(base_seed * 1000003 + m * 7919 + t)
            k = krng.randrange(1, n); Q = E.mulk(k, G)
            for a, kind, off in arms:
                rng = random.Random(f"{base_seed}/{m}/{t}/{off}")
                ev, rs, kk = solve_one(E, n, G, Q, canons[a], rng, cap)
                data[a].append(ev); restarts[a] += rs
                if kk is None or E.mulk(kk, G) != Q: fails[a] += 1
        mean = {a: sum(v) / len(v) for a, v in data.items()}
        brng = random.Random(base_seed + 99)
        ideal = math.sqrt(2 * m)
        R = mean["neg"] / mean["frob"]; Rci = boot_ratio(data["neg"], data["frob"], brng)
        aa = mean["neg"] / mean["neg_AA"]; aaci = boot_ratio(data["neg"], data["neg_AA"], brng)
        pc = mean["none"] / mean["neg"]; pcci = boot_ratio(data["none"], data["neg"], brng)
        out[m] = dict(N=N, n=n, lam=lam, cap=cap, mean_f_evals=mean, restarts=restarts, failures=fails,
                      ideal_sqrt_2m=ideal, R_neg_over_frob=R, R_ci95=Rci,
                      AA_ratio=aa, AA_ci95=aaci, posctrl_none_over_neg=pc, posctrl_ci95=pcci,
                      threshold_0p6_ideal=0.6 * ideal)
    return out

def verdict(out, smoke):
    ctrl_ok = all(o["AA_ci95"][0] <= 1 <= o["AA_ci95"][1] and 1.1 <= o["posctrl_none_over_neg"] <= 1.8
                  and o["failures"]["frob"] == 0 for o in out.values())
    if smoke or not ctrl_ok: return "inconclusive"
    if all(o["R_neg_over_frob"] >= o["threshold_0p6_ideal"] for o in out.values()): return "consistent_with_prediction"
    if any(o["R_ci95"][1] < o["threshold_0p6_ideal"] for o in out.values()): return "falsified"
    return "inconclusive"

if __name__ == "__main__":
    smoke = "--smoke" in sys.argv
    ms, T = ([7, 11], 4) if smoke else ([11, 13, 17], 200)
    out = run(ms, T, 20261110)
    print(json.dumps(dict(lead="LEAD-ICB-koblitz-rho-reference", smoke=smoke, seed=20261110, trials=T,
                          sizes_m=ms, walk_partitions=16, results={str(k): v for k, v in out.items()},
                          verdict=verdict(out, smoke))))
