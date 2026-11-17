#!/usr/bin/env python3
"""E5: subspace factor base V = {x in t*GF(p)} inside GF(p^2) (x not in GF(p)), m=2. [pcc:unverified]
Measures base size, m=2 yield per random target R' in E(GF(p^2)), and the implied relation-phase
work for IC in E(GF(p^2)) versus sqrt(#E(GF(p))) for rho on the subgroup the DLP actually lives in."""
import sys, random, json
sys.path.insert(0, '.')
from descent import Ext, Curve, O, count_base_points
rng = random.Random(5)
for p in (101, 211, 401):
    n = next(z for z in range(2, p) if pow(z, (p - 1) // 2, p) == p - 1)
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p: break
    F = Ext(p, [(-n) % p, 0]); E = Curve(F, a, b)
    base = []
    for s in range(1, p):
        x = (0, s); yy = F.sqrt(E.rhs(x))
        if yy is None: continue
        base += [(x, yy), (x, F.neg(yy))]
    sums = {}
    for i, P in enumerate(base):
        for Q in base[i:]:
            S = E.add(P, Q)
            if S is not O: sums[S] = sums.get(S, 0) + 1
    NE = count_base_points(p, a, b); tr = p + 1 - NE
    NE2 = p * p + 1 - (tr * tr - 2 * p)
    hits = 0; trials = 2000
    for _ in range(trials):
        while True:
            x = (rng.randrange(p), rng.randrange(p)); yy = F.sqrt(E.rhs(x))
            if yy is not None: break
        if (x, yy) in sums: hits += 1
    yld = hits / trials
    need = len(base) // 2  # unknowns after negation folding
    print(json.dumps({"p": p, "#E(Fp)": NE, "#E(Fp2)": NE2, "|V-base|": len(base),
        "distinct_unordered_2sums": len(sums), "m2_yield_per_target": yld,
        "unknowns(neg-folded)": need, "targets_tried_for_need_relations": round(need / yld) if yld else None,
        "sqrt(#E(Fp))": round(NE ** 0.5, 2),
        "ratio_targets/sqrt(#E(Fp))": round(need / yld / NE ** 0.5, 2) if yld else None}), flush=True)
