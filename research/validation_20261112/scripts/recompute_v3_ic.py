#!/usr/bin/env python3
"""Target 1, independent re-execution of the v3 IC arm in the validator's own
arithmetic (vlib.py Cantor; no producer code).  From the certificate of each
`ic_jc` row (curve f, G, T, r, seed) it regenerates the collector's alpha/beta
stream (xorshift64 as specified in measure.rs collect_relations), recomputes
each R = [alpha]G + [beta]T with vlib, decides smoothness over the full
degree-1 factor base, recounts the charged Cantor operations (double-and-add
as specified in measure.rs scalar_counted, +1 per trial), and repeats the
elimination's counting convention (measure.rs solve_linear_inner) on the
validator-built rows.  It also checks every kept relation independently:
[alpha]G + [beta]T == sum e_i (P_i - inf) in J(C).
Usage: recompute_v3_ic.py certs_v3.jsonl results_v3.json OUT.json"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
M64 = (1 << 64) - 1
certs = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
rows = {r["p"]: r for r in json.load(open(sys.argv[2]))["rows"]}
def cost(n):
    return 0 if n == 0 else (n.bit_length() - 1) + (bin(n).count("1") - 1)
def is_sq(a, p):
    a %= p
    return a == 0 or pow(a, (p - 1) // 2, p) == 1
out = []
for c in certs:
    if c["arm"] != "ic_jc": continue
    p, r, seed = c["p"], c["r"], c["seed"]; row = rows[p]
    J = Jac2(p, c["group"]["f"])
    G = J.norm((c["g"]["u"], c["g"]["v"])); T = J.norm((c["t"]["u"], c["t"]["v"]))
    fb = [x for x in range(p) if is_sq(peval(J.f, x, p), p)]
    idx = {x: i for i, x in enumerate(fb)}; nfb = len(fb)
    st = (seed * 0x9E3779B97F4A7C15 & M64) | 1
    def nxt():
        global st
        st ^= (st << 13) & M64; st ^= st >> 7; st ^= (st << 17) & M64
        return st
    want = nfb + 8; trials = ops = 0; rel = []; relcheck_fail = 0
    # cache multiples of G and T (values only; costs are charged by formula)
    mG = {}; mT = {}
    def mulc(cache, D, k):
        if k not in cache: cache[k] = J.mul(k, D)
        return cache[k]
    while len(rel) < want and trials < 50_000_000:
        trials += 1
        a = nxt() % r; b = nxt() % r
        ops += cost(a) + cost(b) + 1
        R = J.add(mulc(mG, G, a), mulc(mT, T, b))
        u, v = R
        if len(u) - 1 != 2: continue
        c0, c1 = u[0], u[1]
        disc = (c1 * c1 - 4 * c0) % p
        if not is_sq(disc, p) or disc == 0: continue
        s = tonelli(disc, p); i2 = pow(2, -1, p)
        roots = [((-c1 + s) * i2) % p, ((-c1 - s) * i2) % p]
        ent = []
        for x in roots:
            y = peval(v, x, p)
            if x not in idx: ent = None; break
            y0 = min(y, (p - y) % p)
            ent.append((idx[x], 1 if y == y0 else -1, x, y))
        if ent is None: continue
        vec = [0] * nfb
        for (i, e, _, _) in ent: vec[i] += e
        # independent relation check: R == sum e_i (P_i - inf), P_i canonical (x, y0)
        S = Jac2.ID
        for (i, e, x, y) in ent:
            y0 = min(y, (p - y) % p)
            P = ([(-x) % p, 1], ptrim([y0]))
            S = J.add(S, P if e == 1 else J.neg(P))
        if J.norm(S) != J.norm(R): relcheck_fail += 1
        rel.append((vec, a, b))
    # elimination with measure.rs counting convention (validator re-implementation)
    m = [[e % r for e in vec] + [a, b] for (vec, a, b) in rel]
    ncols = nfb + 2; la = 0; prow = 0
    for col in range(nfb):
        sel = next((i for i in range(prow, len(m)) if m[i][col] != 0), None)
        if sel is None: continue
        m[prow], m[sel] = m[sel], m[prow]
        inv = pow(m[prow][col], r - 2, r)
        m[prow] = [x * inv % r for x in m[prow]]; la += ncols
        for i in range(len(m)):
            if i != prow and m[i][col] != 0:
                f = m[i][col]
                m[i] = [(x - f * y) % r for x, y in zip(m[i], m[prow])]; la += ncols
        prow += 1
        if prow >= len(m): break
    xsol = None
    for rw in m:
        if all(e == 0 for e in rw[:nfb]) and rw[nfb + 1] != 0:
            xsol = (-rw[nfb]) % r * pow(rw[nfb + 1], r - 2, r) % r; la += 1; break
    o = {"p": p, "r": r, "seed": seed, "nfb": nfb, "relations": len(rel), "trials": trials, "group_ops": ops,
         "la_mulmods": la, "x": xsol, "relations_failing_independent_check": relcheck_fail,
         "claimed": {"nfb": row["ic_jac_fb"], "relations": row["ic_jac_relations"], "trials": row["ic_jac_trials"],
                     "group_ops": row["ic_jac_group_ops"], "la_mulmods": row["ic_jac_la_mulmods"], "x": row["ic_claimed_log"],
                     "cert_solver_ops": c["solver_ops"], "cert_k": c["claimed_k"]}}
    o["xsol_times_G_eq_T"] = xsol is not None and J.norm(J.mul(xsol, G)) == T
    o["match"] = all(o[k] == o["claimed"][k] for k in ("nfb", "relations", "trials", "group_ops", "la_mulmods", "x"))
    print(p, {k: o[k] for k in ("nfb", "relations", "trials", "group_ops", "la_mulmods", "x", "relations_failing_independent_check", "xsol_times_G_eq_T", "match")}, flush=True)
    out.append(o)
json.dump(out, open(sys.argv[3], "w"), indent=1)
print("all match:", all(o["match"] for o in out))
