#!/usr/bin/env python3
"""LEAD-ICB-cofactor-torsion-quotients toy: exact 2-sum decomposition yield on
binary curves y^2+xy=x^3+x^2+b, with and without the 2-torsion quotient.
stdlib only. g = |S u (S+T)| / (2|S|) = per-test yield gain of the quotient."""
import sys, json, random, math
POLY = {12: 0x1053, 14: 0x4443, 16: 0x1002D, 18: 0x40081}

def field(m):
    n = (1 << m) - 1; poly = POLY[m]
    exp = [0] * (2 * n + 2); log = [0] * (n + 1); x = 1
    for i in range(n):
        exp[i] = x; log[x] = i; x <<= 1
        if x >> m: x ^= poly
    assert x == 1 and len(set(exp[:n])) == n, "poly not primitive"
    for i in range(n, 2 * n + 2): exp[i] = exp[i - n]
    return n, exp, log

class Curve:
    def __init__(s, m, b):
        s.m = m; s.n, s.exp, s.log = field(m) if m not in _F else _F[m]; _F[m] = (s.n, s.exp, s.log)
        s.b = b
        zmap = {}
        for z in range(1 << m): zmap[s.sq(z) ^ z] = z
        s.pts = []
        for x in range(1, 1 << m):
            c = x ^ 1 ^ s.mul(b, s.inv(s.sq(x)))   # a=1
            if c in zmap:
                z = zmap[c]; y = s.mul(x, z); s.pts.append((x, y)); s.pts.append((x, x ^ y))
        y = b
        for _ in range(m - 1): y = s.sq(y)
        s.T = (0, y); s.pts.append(s.T)
        s.N = len(s.pts) + 1
    def mul(s, a, b): return 0 if a == 0 or b == 0 else s.exp[s.log[a] + s.log[b]]
    def sq(s, a): return 0 if a == 0 else s.exp[2 * s.log[a]]
    def inv(s, a): return s.exp[s.n - s.log[a]]
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            l = x1 ^ s.mul(y1, s.inv(x1)); x3 = s.sq(l) ^ l ^ 1
            return (x3, s.sq(x1) ^ s.mul(l ^ 1, x3))
        l = s.mul(y1 ^ y2, s.inv(x1 ^ x2)); x3 = s.sq(l) ^ l ^ x1 ^ x2 ^ 1
        return (x3, s.mul(l, x1 ^ x3) ^ x3 ^ y1)
_F = {}

def neg(P): return (P[0], P[0] ^ P[1])

def sums(C, fb):
    S = set(); add = C.add
    for i in range(len(fb)):
        for j in range(i, len(fb)): S.add(add(fb[i], fb[j]))
    return S

def g_of(C, fb):
    S = sums(C, fb); Q = S | {C.add(P, C.T) for P in S}
    return len(Q) / (2 * len(S)), len(S) / C.N

def fb_struct(C, rnd, k):
    vecs = [rnd.randrange(1, 1 << C.m) for _ in range(k)]
    xs = {0}
    for v in vecs: xs |= {x ^ v for x in xs}
    return [P for P in C.pts if P[0] in xs]

def fb_random(C, rnd, size):
    pool = [P for P in C.pts if P[1] and P[0] != 0 and P[1] != P[0]] ; pool = [P for P in C.pts if P[0] != 0]
    out = set()
    while len(out) < size:
        P = rnd.choice(pool); out.add(P); out.add(neg(P))
    return list(out)

def fb_tclosed(C, rnd, size):
    pool = [P for P in C.pts if P[0] != 0]; out = set()
    while len(out) < size:
        P = rnd.choice(pool)
        for R in (P, neg(P), C.add(P, C.T), neg(C.add(P, C.T))): out.add(R)
    return list(out)

def stat(v):
    mu = sum(v) / len(v); sd = math.sqrt(sum((x - mu) ** 2 for x in v) / max(1, len(v) - 1))
    return mu, 2 * sd / math.sqrt(len(v))

def run(ms, ncurves, nseeds):
    TOL = 0.03; res = {}; flags = []
    for m in ms:
        d = {"struct_minus_random": [], "aa_struct_minus_struct": [], "random_minus_tclosed": [], "g_struct": [], "dens": []}
        for ci in range(ncurves):
            rb = random.Random(1000 * m + ci); b = rb.randrange(1, 1 << m)
            C = Curve(m, b); k = round(math.log2(math.sqrt(C.N) / 2))
            for si in range(nseeds):
                r = random.Random((m * 100 + ci) * 1000 + si)
                f1 = fb_struct(C, r, k); f2 = fb_struct(C, r, k)
                fr = fb_random(C, r, len(f1)); ft = fb_tclosed(C, r, len(f1))
                g1, dens = g_of(C, f1); g2, _ = g_of(C, f2); gr, _ = g_of(C, fr); gt, _ = g_of(C, ft)
                d["struct_minus_random"].append(g1 - gr); d["aa_struct_minus_struct"].append(g1 - g2)
                d["random_minus_tclosed"].append(gr - gt); d["g_struct"].append(g1); d["dens"].append(dens)
        out = {kk: stat(v) for kk, v in d.items()}
        res[str(m)] = {kk: {"mean": a, "ci2se": c} for kk, (a, c) in out.items()}
        res[str(m)]["order_last_curve"] = C.N; res[str(m)]["n"] = len(d["g_struct"])
        sm, sc = out["struct_minus_random"]; am, ac = out["aa_struct_minus_struct"]; pm, pc = out["random_minus_tclosed"]
        flags.append(("gain", sm - sc > TOL)); flags.append(("null", abs(sm) + sc <= TOL))
        flags.append(("aa", abs(am) <= max(ac, 0) * 1.0 + 0.01)); flags.append(("pos", pm - pc > 0.1))
    gain = any(f for n, f in flags if n == "gain")
    ok = all(f for n, f in flags if n in ("null", "aa", "pos"))
    posok = all(f for n, f in flags if n == "pos"); aaok = all(f for n, f in flags if n == "aa")
    if not (posok and aaok): v = "inconclusive"
    elif gain: v = "falsified"
    elif ok: v = "consistent_with_prediction"
    else: v = "inconclusive"
    return res, v

if __name__ == "__main__":
    smoke = "--smoke" in sys.argv
    ms, nc, ns = ([12], 1, 3) if smoke else ([12, 14, 16, 18], 3, 10)
    res, v = run(ms, nc, ns)
    print(json.dumps({"lead": "LEAD-ICB-cofactor-torsion-quotients", "smoke": smoke, "m_values": ms,
        "curves_per_m": nc, "seeds_per_curve": ns, "seed_rule": "Random((m*100+ci)*1000+si); b from Random(1000*m+ci)",
        "tolerance": 0.03, "statistic": "g = |S u (S+T)|/(2|S|); paired diffs vs random FB",
        "results": res, "verdict": v}))
