#!/usr/bin/env python3
"""LEAD-ICE-linearization-excess-fb-prior: stdlib toy. See PROTOCOL.md."""
import json, random, sys, math, time
SMOKE = "--smoke" in sys.argv
T0 = time.time()

class Field:
    def __init__(s, p, n, rng):
        s.p, s.n, s.q = p, n, p**n
        while True:
            poly = [rng.randrange(p) for _ in range(n)]  # x^n = -sum poly[i] x^i
            if poly[0] == 0: continue
            exp = [0]*(s.q-1); cur = [1]+[0]*(n-1); ok = True
            for i in range(s.q-1):
                v = sum(d*p**k for k, d in enumerate(cur))
                if i > 0 and v == 1: ok = False; break
                exp[i] = v
                top = cur[-1]; cur = [0]+cur[:-1]
                cur = [(cur[k]-top*poly[k]) % p for k in range(n)]
            if ok and sum(d*p**k for k, d in enumerate(cur)) == 1: break
        s.exp = exp; s.log = {v: i for i, v in enumerate(exp)}
        s.pw = [p**k for k in range(n)]
    def dig(s, a): return [(a//s.pw[k]) % s.p for k in range(s.n)]
    def undig(s, d): return sum(x*s.pw[k] for k, x in enumerate(d))
    def add(s, a, b): return s.undig([(x+y) % s.p for x, y in zip(s.dig(a), s.dig(b))])
    def neg(s, a): return s.undig([(-x) % s.p for x in s.dig(a)])
    def sub(s, a, b): return s.add(a, s.neg(b))
    def mul(s, a, b):
        if a == 0 or b == 0: return 0
        return s.exp[(s.log[a]+s.log[b]) % (s.q-1)]
    def inv(s, a): return s.exp[(-s.log[a]) % (s.q-1)]

def make_curve(F, rng):
    while True:
        a, b = rng.randrange(F.q), rng.randrange(F.q)
        pts = []
        for x in range(F.q):
            r = F.add(F.add(F.mul(F.mul(x, x), x), F.mul(a, x)), b)
            if r == 0: pts.append((x, 0))
            elif F.log[r] % 2 == 0:
                y = F.exp[F.log[r]//2]; pts += [(x, y), (x, F.neg(y))]
        N = len(pts)+1
        if N < F.q//2: continue
        def addp(P, Q):
            if P is None: return Q
            if Q is None: return P
            if P[0] == Q[0]:
                if P[1] != Q[1] or P[1] == 0: return None
                num = F.add(F.mul(3, F.mul(P[0], P[0])), a) if False else F.add(F.mul(F.undig([3 % F.p]+[0]*(F.n-1)), F.mul(P[0], P[0])), a)
                lam = F.mul(num, F.inv(F.mul(F.undig([2]+[0]*(F.n-1)), P[1])))
            else:
                lam = F.mul(F.sub(Q[1], P[1]), F.inv(F.sub(Q[0], P[0])))
            x3 = F.sub(F.sub(F.mul(lam, lam), P[0]), Q[0])
            return (x3, F.sub(F.mul(lam, F.sub(P[0], x3)), P[1]))
        for _ in range(8):
            G = rng.choice(pts); tab = {}; cur = None; k = 0
            while True:
                cur = addp(cur, G); k += 1
                if cur is None: break
                tab[cur] = k
                if k > N: break
            if k == N and len(tab) == N-1:
                return a, b, N, tab
        # non-cyclic or small order: try another curve

def rank_mod_p(rows, p):
    rows = [r[:] for r in rows]; rk = 0; n = len(rows[0]) if rows else 0
    for c in range(n):
        piv = next((i for i in range(rk, len(rows)) if rows[i][c] % p), None)
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        iv = pow(rows[rk][c], -1, p)
        rows[rk] = [(x*iv) % p for x in rows[rk]]
        for i in range(len(rows)):
            if i != rk and rows[i][c] % p:
                f = rows[i][c]; rows[i] = [(x-f*y) % p for x, y in zip(rows[i], rows[rk])]
        rk += 1
    return rk

def subspace(F, basis):
    els = {0}
    for b in basis:
        els = {F.add(e, F.mul(F.undig([c]+[0]*(F.n-1)), b)) for e in els for c in range(F.p)}
    return els

def excess(F, basis):
    l = len(basis)
    prods = [F.dig(F.mul(basis[i], basis[j])) for i in range(l) for j in range(i, l)]
    return l*(l+1)//2 - rank_mod_p(prods, F.p)

def coverage(S, N, m):
    S = list(S); S2 = {(a+b) % N for a in S for b in S}
    if m == 3: T = {(a+b) % N for a in S2 for b in S}
    else: T = {(a+b) % N for a in S2 for b in S2}
    return len(T)/N

def unit(p, n, m, seed, nrand, nstruct, l=2):
    rng = random.Random(seed); F = Field(p, n, rng)
    a, b, N, tab = make_curve(F, rng)
    byx = {}
    for (x, y), k in tab.items(): byx.setdefault(x, []).append(k)
    W = [e for e in range(F.q) if e == 0 or F.log[e] % ((F.q-1)//(p*p-1)) == 0] if n % 2 == 0 else None
    items = []
    def add(basis, kind):
        X = subspace(F, basis)
        if len(X) != p**l: return
        S = [k for x in X for k in byx.get(x, [])]
        if len(S) < 4: return
        items.append((excess(F, basis), len(S), coverage(S, N, m), kind))
    for _ in range(nrand): add([rng.randrange(1, F.q) for _ in range(l)], "rand")
    if W:
        wb = [e for e in W if e][:]
        for _ in range(nstruct):
            c = rng.randrange(1, F.q); w1 = rng.choice(wb); w2 = rng.choice(wb)
            add([F.mul(c, w1), F.mul(c, w2)], "struct")
    return N, items

def resid_stat(items, labels):
    base = {}
    for (e, s, c, k), lab in zip(items, labels):
        if not lab: base.setdefault(s, []).append(c)
    if not base: return 0.0
    bm = {s: sum(v)/len(v) for s, v in base.items()}
    def b(s): return bm[min(bm, key=lambda t: abs(t-s))]
    r1 = [c-b(s) for (e, s, c, k), lab in zip(items, labels) if lab]
    return sum(r1)/len(r1) if r1 else 0.0

def spearman(x, y):
    def rk(v):
        o = sorted(range(len(v)), key=lambda i: v[i]); r = [0]*len(v); i = 0
        while i < len(o):
            j = i
            while j+1 < len(o) and v[o[j+1]] == v[o[i]]: j += 1
            for t in range(i, j+1): r[o[t]] = (i+j)/2
            i = j+1
        return r
    a, b = rk(x), rk(y); ma, mb = sum(a)/len(a), sum(b)/len(b)
    cv = sum((u-ma)*(v-mb) for u, v in zip(a, b))
    d = math.sqrt(sum((u-ma)**2 for u in a)*sum((v-mb)**2 for v in b))
    return cv/d if d else 0.0

def main():
    cells = [(5, 6, 3), (5, 6, 4), (7, 4, 3), (7, 4, 4)]
    seeds = [101, 202]; nperm = 1000; nr, ns = 100, 40
    if SMOKE: cells = [(5, 6, 3)]; seeds = [101]; nperm = 100; nr, ns = 8, 6
    units = []; pos = []; diffs = []; perm_stats = []; aa = []
    for (p, n, m) in cells:
        for sd in seeds:
            N, items = unit(p, n, m, sd*1000+p*10+m, nr, ns)
            lab = [1 if it[0] >= 1 else 0 for it in items]
            d = resid_stat(items, lab); diffs.append(d)
            rho_size = spearman([it[1] for it in items], [it[2] for it in items])
            pos.append(rho_size)
            rg = random.Random(sd+p+m)
            ps = []
            for _ in range(nperm):
                L = lab[:]; rg.shuffle(L); ps.append(resid_stat(items, L))
            perm_stats.append(ps)
            e0 = [it for it in items if it[0] == 0]; h = len(e0)//2
            aa.append(abs(resid_stat(e0, [1]*h+[0]*(len(e0)-h))) if h > 1 else 0.0)
            units.append({"p": p, "n": n, "m": m, "seed": sd, "N": N, "n_items": len(items),
                          "n_e_ge1": sum(lab), "diff_resid_cov": d, "pos_ctrl_spearman_size_cov": rho_size})
    obs = sum(diffs)/len(diffs)
    pooled = [sum(ps[i] for ps in perm_stats)/len(perm_stats) for i in range(nperm)]
    pval = (1+sum(1 for v in pooled if v >= obs))/(1+nperm)
    npos = sum(1 for d in diffs if d > 0); nunits = len(diffs)
    pos_ok = all(r > 0.3 for r in pos)
    aa_max = max(aa) if aa else 0.0
    if not pos_ok: verdict = "inconclusive"
    elif pval <= 0.01 and obs >= 0.01 and npos >= 0.75*nunits: verdict = "consistent_with_prediction"
    elif pval > 0.05 or obs <= 0: verdict = "falsified"
    else: verdict = "inconclusive"
    print(json.dumps({"lead": "LEAD-ICE-linearization-excess-fb-prior", "smoke": SMOKE,
        "statistic": "mean over units of (mean residual coverage of e>=1 FB subspaces minus size-matched e=0 baseline)",
        "observed": obs, "perm_p": pval, "n_perm": nperm, "units_positive": npos, "units": nunits,
        "positive_control_spearman_size_vs_cov": pos, "pos_control_ok": pos_ok,
        "aa_abs_diff_max": aa_max, "negative_control": "label permutation null",
        "thresholds": {"p": 0.01, "min_effect": 0.01, "frac_units_positive": 0.75},
        "seeds": seeds, "cells_pnm": cells, "n_random": nr, "n_struct": ns,
        "unit_detail": units, "wall_s": time.time()-T0, "verdict": verdict, "label": "[pcc:unverified]"}))
main()
