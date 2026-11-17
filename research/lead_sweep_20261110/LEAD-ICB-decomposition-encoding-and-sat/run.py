import sys, json, math, random, itertools
SMOKE = "--smoke" in sys.argv
N, POLY = (7, 0b10000011) if SMOKE else (13, (1 << 13) | 0b11011)  # x^7+x+1 ; x^13+x^4+x^3+x+1
LS = [2, 3] if SMOKE else [2, 3, 4, 5, 6]
MS = [2, 3] if SMOKE else [2, 3, 4]
Q = 1 << N
# GF(2^N) log tables (find generator by search)
def mulraw(a, b):
    r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if a >> N: a ^= POLY
    return r
def tables():
    for g in range(2, Q):
        e = [1]
        for _ in range(Q - 2): e.append(mulraw(e[-1], g))
        if len(set(e)) == Q - 1 and mulraw(e[-1], g) == 1:
            lg = {v: i for i, v in enumerate(e)}
            return e, lg
EXP, LOG = tables()
def mul(a, b): return 0 if a == 0 or b == 0 else EXP[(LOG[a] + LOG[b]) % (Q - 1)]
def inv(a): return EXP[(-LOG[a]) % (Q - 1)]
def tr(a):
    t = 0; x = a
    for _ in range(N): t ^= x; x = mul(x, x)
    return t
def halftrace(c):
    z = 0; x = c
    for _ in range((N + 1) // 2): z ^= x; x = mul(mul(x, x), mul(x, x))
    return z
A = 1; B = None
OPS = [0]
def add(P, R):
    OPS[0] += 1
    if P is None: return R
    if R is None: return P
    x1, y1 = P; x2, y2 = R
    if x1 == x2:
        if y1 != y2 or x1 == 0: return None
        l = x1 ^ mul(y1, inv(x1)); x3 = mul(l, l) ^ l ^ A
    else:
        l = mul(y1 ^ y2, inv(x1 ^ x2)); x3 = mul(l, l) ^ l ^ x1 ^ x2 ^ A
    return (x3, mul(l, x1 ^ x3) ^ x3 ^ y1)
def neg(P): return None if P is None else (P[0], P[1] ^ P[0])
def points_at(x):
    if x == 0: return []
    rhs = x ^ A ^ mul(B, inv(mul(x, x)))
    if tr(rhs): return []
    z = halftrace(rhs)
    return [(x, mul(x, z)), (x, mul(x, z) ^ x)]
def fb_subspace(rng, l):
    while True:
        bs = [rng.randrange(1, Q) for _ in range(l)]
        xs = {0}
        for b in bs: xs |= {v ^ b for v in xs}
        if len(xs) == 1 << l: break
    F = []
    for x in sorted(xs): F += points_at(x)
    return F
def fb_random(rng, size):
    F = []
    xs = list(range(1, Q)); rng.shuffle(xs)
    for x in xs:
        F += points_at(x)
        if len(F) >= size: break
    return F[:size]
def direct(F, R, m):
    # exhaustive: (m-1)-multisets, check R - S in F; returns (found, adds)
    OPS[0] = 0; S = set(F); found = [None]
    def rec(start, k, acc, chosen):
        if k == 0:
            need = add(R, neg(acc))
            if need in S: found[0] = chosen + [need]
            return
        for i in range(start, len(F)):
            rec(i, k - 1, add(acc, F[i]), chosen + [F[i]])
    rec(0, m - 1, None, [])
    return found[0] is not None, OPS[0]
def mitm(F, R, m):
    OPS[0] = 0; h1 = (m + 1) // 2; h2 = m - h1
    tab = {}
    def build(start, k, acc):
        if k == 0: tab.setdefault(acc, True); return
        for i in range(start, len(F)): build(i, k - 1, add(acc, F[i]))
    build(0, h1, None)
    hit = [False]
    def probe(start, k, acc):
        if k == 0:
            if add(R, neg(acc)) in tab: hit[0] = True
            return
        for i in range(start, len(F)): probe(i, k - 1, add(acc, F[i]))
    probe(0, h2, None)
    return hit[0], OPS[0]
SOLVERS = {"direct": direct, "mitm": mitm}
def unsat_target(F, m, rng):
    Fset = set(F)
    for _ in range(50):
        R = rng.choice(points_at(rng.randrange(1, Q)) or [None])
        if R is None: continue
        if not direct(F, R, m)[0]: return R   # exhaustive proof of UNSAT for this FB
    return None
def slope(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
def main():
    global B
    seed = 20261110; rng = random.Random(seed)
    while True:
        B = rng.randrange(1, Q)
        if tr(B) == 1 - 0 or True: break   # b with Tr(b)=1 gives cyclic-type order; any nonzero b is nonsingular
    arms = {"subspace_A": ("sub", 101), "subspace_B_AA": ("sub", 202), "random_null": ("rnd", 303)}
    res = {}; sub_pts = {}
    for arm, (kind, s) in arms.items():
        res[arm] = {}
        for m in MS:
            res[arm][m] = {}
            for sv in SOLVERS:
                xs, ys = [], []
                for l in LS:
                    r = random.Random(s * 1000 + l)
                    F = fb_subspace(r, l) if kind == "sub" else fb_random(r, len(fb_subspace(random.Random(101 * 1000 + l), l)))
                    R = unsat_target(F, m, random.Random(s + m * 7 + l))
                    if R is None: continue
                    ok, ops = SOLVERS[sv](F, R, m)
                    assert not ok
                    xs.append(math.log2(len(F))); ys.append(math.log2(max(ops, 1)))
                res[arm][m][sv] = {"log2F": xs, "log2cost": ys, "slope_per_log2F": slope(xs, ys) if len(xs) > 1 else None}
    # positive control: planted instance found by both solvers
    pos = {}
    r = random.Random(7); l = LS[-1] if SMOKE else 4
    F = fb_subspace(r, l)
    for m in MS:
        R = None
        for i in range(m): R = add(R, r.choice(F))
        pos[m] = {sv: SOLVERS[sv](F, R, m)[0] for sv in SOLVERS}
    pos_ok = all(all(v.values()) for v in pos.values())
    verdict = "inconclusive"; detail = {}
    if pos_ok:
        bad = False; low = False
        for m in MS:
            if m < 3: continue
            sl = [res["subspace_A"][m][sv]["slope_per_log2F"] for sv in SOLVERS]
            sl = [x for x in sl if x is not None]
            if not sl: continue
            best = min(sl); thr = m / 2 - 1
            detail[m] = {"best_slope": best, "threshold": thr}
            if best < thr - 0.25: low = True
        aa = max(abs((res["subspace_A"][m][sv]["slope_per_log2F"] or 0) - (res["subspace_B_AA"][m][sv]["slope_per_log2F"] or 0)) for m in MS for sv in SOLVERS)
        detail["aa_max_slope_diff"] = aa
        if low: verdict = "falsified"
        elif aa <= 0.3 and detail: verdict = "consistent_with_prediction"
    print(json.dumps({"lead": "LEAD-ICB-decomposition-encoding-and-sat", "smoke": SMOKE, "n": N, "poly": POLY, "b": B, "a": A,
        "ls": LS, "ms": MS, "seed": seed, "arm_seeds": {k: v[1] for k, v in arms.items()},
        "positive_control_found": pos, "results": res, "decision_detail": detail, "verdict": verdict}))
main()
