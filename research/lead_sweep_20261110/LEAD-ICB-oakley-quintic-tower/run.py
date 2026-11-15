import sys, json, random, itertools
POLY = {10: (1 << 10) | 0b1001, 15: (1 << 15) | 0b11, 20: (1 << 20) | 0b1001}

class GF:
    def __init__(s, n):
        s.n = n; s.M = (1 << n) - 1
        s.exp = [0] * (2 * s.M); s.log = [0] * (s.M + 1)
        v = 1; P = POLY[n]
        for i in range(s.M):
            s.exp[i] = v; s.log[v] = i
            v <<= 1
            if v >> n: v ^= P
        assert v == 1, "poly not primitive"
        for i in range(s.M, 2 * s.M): s.exp[i] = s.exp[i - s.M]
        s.half = [-1] * (s.M + 1)  # z^2+z -> z
        for z in range(s.M + 1):
            s.half[s.sq(z) ^ z] = z
    def mul(s, a, b):
        return 0 if a == 0 or b == 0 else s.exp[s.log[a] + s.log[b]]
    def sq(s, a):
        return 0 if a == 0 else s.exp[(2 * s.log[a]) % s.M]
    def inv(s, a):
        return s.exp[s.M - s.log[a]]

class Curve:  # y^2+xy=x^3+a x^2+b ; None = infinity
    def __init__(s, F, a, b): s.F, s.a, s.b = F, a, b
    def neg(s, P): return None if P is None else (P[0], P[1] ^ P[0])
    def add(s, P, Q):
        F = s.F
        if P is None: return Q
        if Q is None: return P
        x1, y1 = P; x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0: return None
            l = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(l) ^ l ^ s.a
            return (x3, F.sq(x1) ^ F.mul(l ^ 1, x3))
        l = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(l) ^ l ^ x1 ^ x2 ^ s.a
        return (x3, F.mul(l, x1 ^ x3) ^ x3 ^ y1)
    def points_at(s, x):
        F = s.F
        if x == 0:
            r = F.exp[(F.log[s.b] * (1 << (F.n - 1))) % F.M]  # sqrt(b)
            return [(0, r)]
        w = x ^ s.a ^ (F.mul(s.b, F.inv(F.sq(x))))
        z = F.half[w]
        if z < 0: return []
        return [(x, F.mul(x, z)), (x, F.mul(x, z ^ 1))]
    def order(s):
        return 2 + sum(len(s.points_at(x)) for x in range(1 << s.F.n)) - 1 + 0 if False else 1 + sum(len(s.points_at(x)) for x in range(1 << s.F.n))

def collisions(C, pts):
    seen = set(); coll = 0
    for sub in itertools.combinations(pts, 5):
        S = None
        for P in sub: S = C.add(S, P)
        if S in seen: coll += 1
        else: seen.add(S)
    return coll

def run(k, nnull, seed, a, b):
    n = 5 * k; q = 1 << k
    F = GF(n); C = Curve(F, a, b)
    rng = random.Random(seed)
    step = F.M // (q - 1)
    sub = [0] + [F.exp[j * step] for j in range(q - 1)]
    fb = []
    for x in sub: fb += C.points_at(x)
    m = (len(fb) - 1) // 2
    allx = [x for x in range(1, 1 << n)]
    def randset():
        pts = C.points_at(0)
        while len(pts) < len(fb):
            p = C.points_at(rng.choice(allx))
            for t in p:
                if t not in pts and len(pts) < len(fb): pts.append(t)
        return pts
    # random-pair null: each selected x contributes both signs
    def nullset():
        pts = C.points_at(0); xs = set()
        while len(xs) < m:
            x = rng.choice(allx)
            if x not in xs and C.points_at(x): xs.add(x); pts += C.points_at(x)
        return pts
    def posset():
        while True:
            x = rng.choice(allx); pp = C.points_at(x)
            if pp: break
        P = pp[0]; pts = C.points_at(0); cur = None
        for _ in range(m):
            cur = C.add(cur, P)
            pts += [cur, C.neg(cur)]
        return pts
    cf = collisions(C, fb)
    nl = [collisions(C, nullset()) for _ in range(nnull)]
    h = nnull // 2
    mA, mB = sum(nl[:h]) / max(h, 1), sum(nl[h:]) / max(nnull - h, 1)
    mn = sum(nl) / nnull
    pos = [collisions(C, posset()) for _ in range(max(2, nnull // 5))]
    mp = sum(pos) / len(pos)
    NN = len(list(itertools.combinations(range(len(fb)), 5))) if len(fb) < 40 else None
    return dict(k=k, field_bits=n, a=a, b=b, fb_size=len(fb), subsets=NN, coll_fb=cf, null_mean=mn,
                aa_means=[mA, mB], pos_mean=mp, ratio=cf / mn if mn else None,
                pos_ratio=mp / mn if mn else None, aa_ratio=mA / mB if mB else None)

def verdict(r):
    if not r["null_mean"] or r["aa_ratio"] is None or not (0.6 <= r["aa_ratio"] <= 1.67): return "inconclusive"
    if r["pos_ratio"] < 3: return "inconclusive"
    if 0.7 <= r["ratio"] <= 1.4: return "consistent_with_prediction"
    return "falsified"

def main():
    smoke = "--smoke" in sys.argv
    cfgs = [(2, 6, 1, 0, 1)] if smoke else [(4, 30, 20261110, 0, 1), (4, 30, 20261111, 1, 1)]
    res = [run(*c) for c in cfgs]
    vs = [verdict(r) for r in res]
    v = "falsified" if "falsified" in vs else ("inconclusive" if "inconclusive" in vs else "consistent_with_prediction")
    print(json.dumps(dict(lead="LEAD-ICB-oakley-quintic-tower", smoke=smoke, statistic="collision ratio FB/null",
        thresholds=dict(consistent=[0.7, 1.4], pos_min=3, aa=[0.6, 1.67]), results=res, per_run=vs, verdict=v)))
main()
