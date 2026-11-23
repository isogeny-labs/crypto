import json, random, sys, itertools, statistics
SMOKE = "--smoke" in sys.argv
SEED = 20261110

def monos(n, d):
    out = []
    for k in range(d, -1, -1):          # high degree first
        out += [c for c in itertools.combinations(range(n), k)]
    return out

def mul(a, b):
    return tuple(sorted(set(a) ^ set(b))) if False else tuple(sorted(set(a) | set(b)))  # x^2=x (Boolean)

def build(n, m, supports, rng):
    """Macaulay matrix degree 3 of m quadratic Boolean equations; supports fixed, coefficients random."""
    cols = monos(n, 3); idx = {c: i for i, c in enumerate(cols)}
    rows = []
    for e in range(m):
        poly = {s: rng.getrandbits(1) for s in supports[e]}
        for mult in [()] + [(v,) for v in range(n)]:
            r = 0
            for s, c in poly.items():
                if c:
                    r ^= 1 << idx[mul(s, mult)]
            if r: rows.append(r)
    return rows, len(cols)

def supports_for(n, m, rng, dens):
    q = monos(n, 2)
    return [[s for s in q if rng.random() < dens] or [q[0]] for _ in range(m)]

def eliminate(rows, ncols, trace=None):
    """Forward GF(2) elimination, columns high-degree first (bit ncols-1 first).
    cost = xors + pivot-candidate tests. trace: list of (col,row_id) hints."""
    rows = list(rows); used = [False]*len(rows); xors = tests = 0; newtrace = []; miss = 0
    hint = dict(trace) if trace else {}
    for col in range(ncols-1, -1, -1):
        bit = 1 << col; piv = None
        h = hint.get(col)
        if h is not None:
            tests += 1
            if not used[h] and rows[h] & bit: piv = h
            else: miss += 1
        if piv is None and (h is not None or trace is None):
            for i in range(len(rows)):
                if used[i]: continue
                tests += 1
                if rows[i] & bit: piv = i; break
        elif piv is None:
            # column had no pivot in the trace: replay trusts it, one test of all unused rows
            for i in range(len(rows)):
                if not used[i]:
                    tests += 1
                    if rows[i] & bit: piv = i; miss += 1; break
        if piv is None: continue
        used[piv] = True; newtrace.append((col, piv)); p = rows[piv]
        for i in range(len(rows)):
            if not used[i]:
                tests += 1
                if rows[i] & bit: rows[i] ^= p; xors += 1
    rank = sum(used)
    return xors + tests, rank, newtrace, miss

def trial(n, m, dens, rng):
    sup = supports_for(n, m, rng, dens)
    A, nc = build(n, m, sup, rng); B, _ = build(n, m, sup, rng)
    sup2 = supports_for(n, m, rng, dens); U, _ = build(n, m, sup2, rng)
    cF, rF, trB, _ = eliminate(B, nc)                  # fresh, also yields B's own trace
    _, _, trA, _ = eliminate(A, nc)
    cR, rR, _, missR = eliminate(B, nc, trA)           # replay A's trace on same-support B
    cAA, rAA, _, _ = eliminate(B, nc, trB)             # positive control: B's own trace
    _, _, trU, _ = eliminate(U, nc)
    cN, rN, _, _ = eliminate(B, nc, trU)               # null: unrelated-support trace
    # A/A: two independent fresh runs of same B are deterministic -> ratio exactly 1
    cF2, _, _, _ = eliminate(B, nc)
    assert rR == rF == rAA == rN
    return dict(fresh=cF, same_support=cR, own=cAA, unrelated=cN, aa=cF2, miss=missR)

def main():
    rng = random.Random(SEED)
    cfg = [(5, 5, 0.5, 3)] if SMOKE else [(9, 10, 0.3, 12), (10, 12, 0.3, 12), (11, 13, 0.25, 12)]
    res = []
    for n, m, dens, T in cfg:
        ts = [trial(n, m, dens, rng) for _ in range(T)]
        rat = lambda k: statistics.median(t['fresh']/t[k] for t in ts)
        res.append(dict(n=n, m=m, density=dens, trials=T, ratio_same_support=rat('same_support'),
                        ratio_own_trace=rat('own'), ratio_unrelated=rat('unrelated'), ratio_aa=rat('aa'),
                        mean_misses=statistics.mean(t['miss'] for t in ts)))
    r = statistics.median(c['ratio_same_support'] for c in res)
    pos = min(c['ratio_own_trace'] for c in res); aa = max(abs(c['ratio_aa']-1) for c in res)
    nul = max(c['ratio_unrelated'] for c in res)
    if aa > 1e-9 or pos <= 1.0: v = "inconclusive"
    elif r > 2.3: v = "falsified"
    elif 1.0 <= r <= 2.3 and nul <= r: v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICE-boolean-f4-f5-f6-engines", smoke=SMOKE, seed=SEED, cells=res,
        statistic_median_same_support_ratio=r, positive_control_min=pos, aa_max_dev=aa,
        null_max_unrelated_ratio=nul, threshold_upper=2.3, verdict=v)))
main()
