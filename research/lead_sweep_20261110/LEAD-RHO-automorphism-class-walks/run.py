#!/usr/bin/env python3
"""Preregistered toy: rho step counts under Aut-orbit canonicalisation (m=1,2,6). stdlib only."""
import json, math, random, sys, time
SMOKE = "--smoke" in sys.argv
T0 = time.time()

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

class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            l = (3 * P[0] * P[0] + s.a) * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)
    def mul(s, k, P):
        R = None
        while k:
            if k & 1: R = s.add(R, P)
            P = s.add(P, P); k >>= 1
        return R
    def order(s):
        p = s.p
        sq = [0] * p
        for y in range(p): sq[y * y % p] += 1
        return p + 1 + sum(-(sq[(x * x * x + s.a * x + s.b) % p] - 1) for x in range(p)) * -1 * -1 if False else \
            1 + sum(sq[(x ** 3 + s.a * x + s.b) % p] for x in range(p))

def find_setup(rng, lo, j0):
    """Return (curve, P, q) with P of prime order q (q = 1 mod 3 for j0)."""
    while True:
        p = rng.randrange(lo, 2 * lo)
        if not isprime(p) or (j0 and p % 3 != 1): continue
        for _ in range(40):
            if j0: C = Curve(p, 0, rng.randrange(1, p))
            else:
                a, b = rng.randrange(1, p), rng.randrange(1, p)
                if (4 * a ** 3 + 27 * b * b) % p == 0: continue
                C = Curve(p, a, b)
            N = C.order()
            fs = factor(N); q = max(fs)
            if q < N // 8 or q < lo // 2 or (j0 and q % 3 != 1): continue
            h = N // q
            while True:
                x = rng.randrange(p); r = (x ** 3 + C.a * x + C.b) % p
                if pow(r, (p - 1) // 2, p) != 1: continue
                y = pow(r, (p + 1) // 4, p) if p % 4 == 3 else None
                if y is None:
                    for yy in range(p):
                        if yy * yy % p == r: y = yy; break
                P = C.mul(h, (x, y))
                if P is not None: return C, P, q

def automorphisms(C, P, q):
    """Return list of (zeta_pow, sign, scalar) images; m=6 for j0."""
    p = C.p
    # find zeta of order 3 in F_p and mu of order 3 mod q matching [mu]P=(zeta x, y)
    g = 2
    while True:
        z = pow(g, (p - 1) // 3, p)
        if z != 1: break
        g += 1
    for zeta in (z, z * z % p):
        for t in range(2, q):
            pass
            break
        # mu = w^((q-1)/3) for a w with nontrivial result
        w = 2
        while True:
            mu = pow(w, (q - 1) // 3, q)
            if mu != 1: break
            w += 1
        for m_ in (mu, mu * mu % q):
            R = C.mul(m_, P)
            if R == (zeta * P[0] % p, P[1]): return zeta, m_
    raise RuntimeError("no endomorphism match")

def run_arm(C, P, q, k, Q, mode, seed, zeta=None, mu=None, nj=32, cap_f=300):
    rng = random.Random(seed); p = C.p
    if mode == 6:
        imgs = [(pow(zeta, i, p), s, (s * pow(mu, i, q)) % q) for i in range(3) for s in (1, -1)]
    elif mode == 2:
        imgs = [(1, 1, 1), (1, -1, q - 1)]
    else:
        imgs = [(1, 1, 1)]
    def canon(pt, a, b):
        best = None
        for zz, s, m in imgs:
            c = (pt[0] * zz % p, pt[1] if s == 1 else (-pt[1]) % p)
            if best is None or c < best[0]: best = (c, m)
        c, m = best
        return c, a * m % q, b * m % q
    J = []
    for _ in range(nj):
        u, v = rng.randrange(1, q), rng.randrange(1, q)
        J.append((C.add(C.mul(u, P), C.mul(v, Q)), u, v))
    seen = {}; steps = 0; cap = cap_f * int(math.isqrt(q)) + 100
    walks = 0; abandoned = 0
    while steps < cap:
        walks += 1
        a, b = rng.randrange(q), rng.randrange(q)
        pt = C.add(C.mul(a, P), C.mul(b, Q))
        if pt is None: continue
        pt, a, b = canon(pt, a, b)
        mine = set()
        while steps < cap:
            if pt in seen:
                a2, b2 = seen[pt]
                if (a, b) != (a2, b2) and (b - b2) % q:
                    kk = (a2 - a) * pow(b - b2, -1, q) % q
                    return {"steps": steps, "ok": kk == k, "walks": walks, "abandoned": abandoned}
                if pt in mine:  # fruitless cycle
                    abandoned += 1; break
                break  # merged with fruitless-equal state from earlier walk: restart
            seen[pt] = (a, b); mine.add(pt)
            R, u, v = J[pt[0] % nj]
            nxt = C.add(pt, R)
            steps += 1
            if nxt is None: abandoned += 1; break
            pt, a, b = canon(nxt, (a + u) % q, (b + v) % q)
    return {"steps": steps, "ok": False, "walks": walks, "abandoned": abandoned, "capped": True}

def stats(xs):
    n = len(xs); m = sum(xs) / n
    sd = math.sqrt(sum((x - m) ** 2 for x in xs) / max(1, n - 1))
    return m, sd / math.sqrt(n)

def main():
    master = random.Random(20261110)
    sizes = [300] if SMOKE else [2 ** 13, 2 ** 16, 2 ** 19]
    trials = 6 if SMOKE else 60
    out = {"protocol": "LEAD-RHO-automorphism-class-walks", "smoke": SMOKE, "seed": 20261110,
           "sizes_lo_p": sizes, "trials": trials, "per_size": [], "limits": "stdlib, 1 core"}
    ratios6, ratios2, aa, ratios2g = [], [], [], []
    for lo in sizes:
        rng = random.Random(master.randrange(2 ** 62))
        C, P, q = find_setup(rng, lo, True)
        zeta, mu = automorphisms(C, P, q)
        k = rng.randrange(2, q - 1); Q = C.mul(k, P)
        G, GP, gq = find_setup(rng, lo, False)
        gk = rng.randrange(2, gq - 1); GQ = G.mul(gk, GP)
        rec = {"p": C.p, "q": q, "gen_p": G.p, "gen_q": gq}
        arms = {"plain_a": [], "plain_b": [], "neg": [], "aut6": [], "g_plain": [], "g_neg": []}
        bad = 0; aband6 = 0
        for t in range(trials):  # interleaved arm order, derived seeds
            order = list(arms); rng.shuffle(order)
            for arm in order:
                sd = master.randrange(2 ** 62) if False else (hash((lo, t, arm)) & 0) + 1000003 * t + 7919 * len(arm) + lo
                if arm in ("plain_a", "plain_b"): r = run_arm(C, P, q, k, Q, 1, sd + (1 if arm == "plain_b" else 0))
                elif arm == "neg": r = run_arm(C, P, q, k, Q, 2, sd)
                elif arm == "aut6": r = run_arm(C, P, q, k, Q, 6, sd, zeta, mu)
                elif arm == "g_plain": r = run_arm(G, GP, gq, gk, GQ, 1, sd)
                else: r = run_arm(G, GP, gq, gk, GQ, 2, sd)
                if not r["ok"]: bad += 1
                if arm == "aut6": aband6 += r["abandoned"]
                arms[arm].append(r["steps"])
            if time.time() - T0 > 500: break
        mp = {a: stats(v) for a, v in arms.items() if v}
        rec.update({"mean_steps": {a: round(m[0], 1) for a, m in mp.items()},
                    "ideal_plain": round(math.sqrt(math.pi * q / 2), 1), "failures_or_capped": bad,
                    "aut6_abandoned_walks_total": aband6, "n": len(arms["aut6"])})
        pl = [(x + y) / 2 for x, y in zip(arms["plain_a"], arms["plain_b"])]
        n = min(len(arms["aut6"]), len(pl))
        mpl = sum(pl[:n]) / n
        ratios6.append(sum(arms["aut6"][:n]) / n / mpl)
        ratios2.append(sum(arms["neg"][:n]) / n / mpl)
        aa.append(sum(arms["plain_a"][:n]) / sum(arms["plain_b"][:n]))
        gp = sum(arms["g_plain"][:n]); ratios2g.append(sum(arms["g_neg"][:n]) / gp)
        rec.update({"R6": ratios6[-1], "R2": ratios2[-1], "AA": aa[-1], "R2_generic": ratios2g[-1]})
        out["per_size"].append(rec)
    R6 = sum(ratios6) / len(ratios6); R2 = sum(ratios2) / len(ratios2)
    AA = sum(aa) / len(aa); R2g = sum(ratios2g) / len(ratios2g)
    out.update({"R6_mean": R6, "R2_mean": R2, "AA_mean": AA, "R2_generic_mean": R2g,
                "ideal_R6": 1 / math.sqrt(6), "ideal_R2": 1 / math.sqrt(2),
                "thresholds": {"consistent": "0.41<=R6<=0.60 and 0.65<=R2<=0.85 and 0.9<=AA<=1.1 and 0.65<=R2_generic<=0.85",
                               "falsified": "R6<0.38 or R6>0.70 (given AA ok)"}})
    aa_ok = 0.9 <= AA <= 1.1
    if SMOKE or not aa_ok: v = "inconclusive"
    elif R6 < 0.38 or R6 > 0.70: v = "falsified"
    elif 0.41 <= R6 <= 0.60 and 0.65 <= R2 <= 0.85 and 0.65 <= R2g <= 0.85: v = "consistent_with_prediction"
    else: v = "inconclusive"
    out["verdict"] = v; out["wall_s"] = round(time.time() - T0, 2)
    print(json.dumps(out))

main()
