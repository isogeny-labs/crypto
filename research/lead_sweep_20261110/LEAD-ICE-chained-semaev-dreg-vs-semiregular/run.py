#!/usr/bin/env python3
"""Toy: Macaulay-rank deficit of a Boolean chained binary-Semaev system vs a semi-regular null.
Stdlib only. See PROTOCOL.md."""
import sys, json, random, itertools, math, time

IRRED = {4: 0b10011, 5: 0b100101, 6: 0b1000011}

# bit-polynomials: frozenset of monomials; monomial = frozenset of variable indices (multilinear)
def padd(a, b): return a ^ b
def pmul(a, b):
    out = set()
    for m1 in a:
        for m2 in b:
            out ^= {m1 | m2}
    return frozenset(out)
ONE = frozenset([frozenset()])
ZERO = frozenset()

def fmul(x, y, n):
    # x,y lists of n bit-polys; polynomial basis mult mod IRRED[n]
    prod = [ZERO] * (2 * n - 1)
    for i in range(n):
        if not x[i]: continue
        for j in range(n):
            if y[j]: prod[i + j] = padd(prod[i + j], pmul(x[i], y[j]))
    red = IRRED[n]
    for d in range(2 * n - 2, n - 1, -1):
        if prod[d]:
            for k in range(n):
                if (red >> k) & 1: prod[d - n + k] = padd(prod[d - n + k], prod[d])
            prod[d] = ZERO
    return prod[:n]
def fadd(x, y): return [padd(a, b) for a, b in zip(x, y)]
def const(c, n): return [ONE if (c >> i) & 1 else ZERO for i in range(n)]

def s3(x1, x2, x3, b, n):
    e = fadd(fadd(fmul(x1, x2, n), fmul(x1, x3, n)), fmul(x2, x3, n))
    return fadd(fadd(fmul(e, e, n), fmul(fmul(x1, x2, n), x3, n)), b)

def chained(n, l, rng):
    """vars: x1,x2,x3 in V (low l bits free, rest 0), u full n bits. Eq: S3(x1,x2,u)=0, S3(u,x3,R)=0."""
    v = 0
    def var():
        nonlocal v; v += 1; return v - 1
    def elt(free):
        return [frozenset([frozenset([var()])]) if i < free else ZERO for i in range(n)]
    x1, x2, x3, u = elt(l), elt(l), elt(l), elt(n)
    b = const(rng.randrange(1, 1 << n), n); R = const(rng.randrange(1, 1 << n), n)
    eqs = [p for p in s3(x1, x2, u, b, n) + s3(u, x3, R, b, n) if p]
    return v, eqs

def rand_poly(N, deg, rng):
    ms = [frozenset(c) for d in range(deg + 1) for c in itertools.combinations(range(N), d)]
    return frozenset(m for m in ms if rng.random() < 0.5)

def null_system(N, M, rng):
    return N, [rand_poly(N, 3, rng) for _ in range(M)]

def monos(N, D):
    out = []
    for d in range(D + 1): out += [frozenset(c) for c in itertools.combinations(range(N), d)]
    return out

def macaulay_rank(N, eqs, D):
    ms = monos(N, D); idx = {m: i for i, m in enumerate(ms)}
    degs = [max((len(m) for m in p), default=0) for p in eqs]
    piv = {}; inconsistent = False
    for p, dp in zip(eqs, degs):
        if dp > D: continue
        for d in range(max(0, D - dp) + 1):
            for c in itertools.combinations(range(N), d):
                cs = frozenset(c); row = 0
                for m in p:
                    row ^= 1 << idx[m | cs]
                while row:
                    h = row.bit_length() - 1
                    if h in piv: row ^= piv[h]
                    else: piv[h] = row; break
    if 0 in piv and piv[0] == 1: inconsistent = True
    return len(piv), inconsistent

def pred_rank(N, M, D, d=3):
    # series (1+z)^N / (1+z^d)^M truncated; rank pred = dim(<=D monomials) - quotient dim (<=D)
    ser = [math.comb(N, k) for k in range(D + 1)]
    for _ in range(M):  # divide by (1+z^d)
        for k in range(d, D + 1): ser[k] -= ser[k - d]
    ser = [max(s, 0) for s in ser]
    tot = sum(math.comb(N, k) for k in range(D + 1))
    return tot - sum(ser)

def arm_stat(kind, n, l, seeds, Ds):
    res = {D: [] for D in Ds}; incons = 0
    for s in seeds:
        rng = random.Random(s)
        N, eqs = chained(n, l, rng)
        M = len(eqs)
        if kind == "null": N, eqs = null_system(N, M, rng)
        elif kind == "pos":
            N, eqs = null_system(N, M, rng)
            lin = frozenset([frozenset([rng.randrange(N)]), frozenset([rng.randrange(N)])])
            eqs[-1] = pmul(eqs[0], lin)  # planted dependency
        elif kind != "chain": raise ValueError(kind)
        for D in Ds:
            r, inc = macaulay_rank(N, eqs, D)
            res[D].append(r / max(1, pred_rank(N, M, D)))
            if D == Ds[-1] and inc: incons += 1
    return {D: sum(v) / len(v) for D, v in res.items()}, incons

def main():
    smoke = "--smoke" in sys.argv
    n, l, K, Ds = (4, 1, 2, [2, 3]) if smoke else (6, 2, 14, [3, 4, 5])
    t0 = time.time()
    # derived seeds per arm; interleaving is irrelevant (deterministic, no timing arm)
    seeds = {a: [hash_seed(a, i) for i in range(K)] for a in ("chain", "aa1", "aa2", "pos")}
    out = {}
    out["chain"], ic = arm_stat("chain", n, l, seeds["chain"], Ds)
    out["null"], _ = arm_stat("null", n, l, seeds["chain"], Ds)  # paired shape; null randomness from same seed stream
    out["aa1"], _ = arm_stat("null", n, l, seeds["aa1"], Ds)
    out["aa2"], _ = arm_stat("null", n, l, seeds["aa2"], Ds)
    out["pos"], _ = arm_stat("pos", n, l, seeds["pos"], Ds)
    D = Ds[-1]
    diff = out["null"][D] - out["chain"][D]
    aa = abs(out["aa1"][D] - out["aa2"][D])
    pos = out["aa1"][D] - out["pos"][D]
    thr = max(0.02, 3 * aa)
    if pos <= thr: verdict = "inconclusive"  # detector blind: positive control failed
    elif diff > thr: verdict = "consistent_with_prediction"
    elif diff <= aa: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICE-chained-semaev-dreg-vs-semiregular", "smoke": smoke,
        "params": {"n": n, "l": l, "instances_per_arm": K, "degrees": Ds, "irreducible": IRRED[n]},
        "seeds": seeds, "mean_rank_ratio_by_degree": {a: {str(k): v for k, v in d.items()} for a, d in out.items()},
        "statistic_D": D, "diff_null_minus_chain": diff, "aa_noise": aa, "pos_control_effect": pos,
        "threshold": thr, "chain_inconsistent_count_at_maxD": ic, "wall_s": round(time.time() - t0, 2),
        "verdict": verdict}))
def hash_seed(arm, i):
    return {"chain": 1000, "aa1": 2000, "aa2": 3000, "pos": 4000}[arm] + i
if __name__ == "__main__":
    main()
