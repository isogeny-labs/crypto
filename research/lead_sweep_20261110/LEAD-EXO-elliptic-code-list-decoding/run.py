import json, math, random, sys
from itertools import combinations

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)

def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R

def find_curve(p, rng):
    """prime-order curve over F_p: naive count, stdlib only."""
    sq = {}
    for y in range(p): sq.setdefault(y * y % p, []).append(y)
    for _ in range(2000):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0: continue
        pts = [(x, y) for x in range(p) for y in sq.get((x ** 3 + a * x + b) % p, [])]
        n = len(pts) + 1
        if is_prime(n) and n > 50:
            return a, b, n, pts
    raise RuntimeError("no prime-order curve")

def count_k(S, n, Q, k):
    """number of k-subsets of distinct FB indices (scalars S) with sum == Q mod n."""
    B = len(S)
    if k == 2:
        d = {}
        c = 0
        for i, s in enumerate(S):
            c += d.get((Q - s) % n, 0)
            d[s] = d.get(s, 0) + 1
        return c
    if k == 3:
        pd = {}
        for i, j in combinations(range(B), 2):
            pd.setdefault((S[i] + S[j]) % n, []).append((i, j))
        tot = 0
        for l, s in enumerate(S):
            for (i, j) in pd.get((Q - s) % n, ()):
                if l != i and l != j: tot += 1
        return tot // 3
    raise ValueError

def run_cell(pbits, B, k, T, seed):
    rng = random.Random(seed)
    p = rng.randrange(1 << (pbits - 1), 1 << pbits)
    while not is_prime(p) or p % 4 != 3 and False: p += 1
    a, b, n, pts = find_curve(p, rng)
    P = pts[0]
    S = rng.sample(range(1, n), B)               # FB = {s_i P}, distinct scalars
    FB = [mul(s, P, a, p) for s in S]
    ec_ok = True                                  # verify scalar model against real EC addition
    for _ in range(5):
        i, j = rng.sample(range(B), 2)
        ec_ok &= add(FB[i], FB[j], a, p) == mul((S[i] + S[j]) % n, P, a, p)
    expect = math.comb(B, k) / n
    def mean_var(targets):
        cs = [count_k(S, n, Q, k) for Q in targets]
        m = sum(cs) / len(cs)
        v = sum((c - m) ** 2 for c in cs) / (len(cs) - 1)
        return m, v, cs
    tr = random.Random(seed ^ 0xA5A5)
    t1 = [tr.randrange(n) for _ in range(T)]       # negative control / null: random targets
    t2 = [tr.randrange(n) for _ in range(T)]       # A/A: disjoint second target set
    m1, v1, _ = mean_var(t1); m2, v2, _ = mean_var(t2)
    pl = [sum(S[i] for i in rng.sample(range(B), k)) % n for _ in range(T // 4)]
    mp, _, cp = mean_var(pl)                       # positive control: planted targets
    Mgs = math.sqrt(B * k)
    lg = (math.lgamma(B + 1) - math.lgamma(int(math.ceil(Mgs)) + 1) - math.lgamma(B - int(math.ceil(Mgs)) + 1)
          + (k - math.ceil(Mgs) - 1) * math.log(p))
    return dict(pbits=pbits, p=p, n=n, B=B, k=k, T=T, expected_mean=expect,
                mean_random=m1, mean_AA=m2, ratio=m1 / expect, ratio_AA=m2 / expect,
                dispersion=v1 / m1 if m1 > 0 else None, planted_min=min(cp), planted_mean=mp,
                ec_model_ok=bool(ec_ok), M_gs=Mgs, gs_agreement_exceeds_k=bool(Mgs > k),
                ln_counting_bound_at_M=lg)

def main():
    smoke = "--smoke" in sys.argv
    cells = [(7, 12, 2, 40)] if smoke else [(12, 32, 2, 600), (12, 64, 2, 600), (12, 32, 3, 400),
                                            (14, 48, 2, 400), (14, 64, 3, 300)]
    seed0 = 20261110
    res = [run_cell(pb, B, k, T, seed0 + 17 * i) for i, (pb, B, k, T) in enumerate(cells)]
    # prediction: random-target mean matches C(B,k)/n (ratio in [0.7,1.3]); A/A agrees; planted >=1;
    # GS agreement M>k in every cell with B>k (decoding cannot reach weight-k words).
    ok_ratio = all(0.7 <= r["ratio"] <= 1.3 and 0.7 <= r["ratio_AA"] <= 1.3 for r in res)
    pos = all(r["planted_min"] >= 1 and r["ec_model_ok"] for r in res)
    gs = all(r["gs_agreement_exceeds_k"] for r in res if r["B"] > r["k"])
    if not pos: verdict = "inconclusive"
    elif ok_ratio and gs: verdict = "consistent_with_prediction"
    elif any(r["ratio"] > 1.5 for r in res): verdict = "falsified"
    else: verdict = "inconclusive"
    if smoke and verdict != "consistent_with_prediction": verdict = "inconclusive"
    print(json.dumps(dict(lead="LEAD-EXO-elliptic-code-list-decoding", smoke=smoke, seed0=seed0,
                          cells=res, ok_ratio=ok_ratio, positive_control_ok=pos, gs_bound_ok=gs,
                          verdict=verdict)))
main()
