#!/usr/bin/env python3
"""Target 3: independent re-execution (validator's vlib Cantor) of the J(C) IC arm
(A1) of cover_pushforward for the primary-run seeds, from the instance dump
(val_cp_dump) and the collector spec of measure.rs (xorshift64 stream, full
degree-1 base, stop at nfb + 8, double-and-add charging, +1 per trial, dense
elimination counting).  Compares group ops, trials, relations, LA counter and
the log with pushforward.json.  Usage: recompute_cp_icjc.py cp_dump.jsonl pushforward.json OUT.json [maxp]"""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
M64 = (1 << 64) - 1
dump = {(d["p"], d["k"]): d for d in map(json.loads, open(sys.argv[1]))}
doc = json.load(open(sys.argv[2])); maxp = int(sys.argv[4]) if len(sys.argv) > 4 else 10 ** 9
def cost(n): return 0 if n == 0 else (n.bit_length() - 1) + (bin(n).count("1") - 1)
def is_sq(a, p): a %= p; return a == 0 or pow(a, (p - 1) // 2, p) == 1
out = []
for row in doc["rows"]:
    p = row["p"]
    if p > maxp: continue
    for k, s in enumerate(row["seeds"]):
        d = dump[(p, k)]; r = d["r"]; seed = s["seed"]
        J = Jac2(p, d["jac_f"]); G = J.norm((d["g"]["u"], d["g"]["v"])); T = J.norm((d["t"]["u"], d["t"]["v"]))
        fb = [x for x in range(p) if is_sq(peval(J.f, x, p), p)]; idx = {x: i for i, x in enumerate(fb)}; nfb = len(fb)
        st = [(seed * 0x9E3779B97F4A7C15 & M64) | 1]
        def nxt():
            v = st[0]; v ^= (v << 13) & M64; v ^= v >> 7; v ^= (v << 17) & M64; st[0] = v; return v
        mG, mT = {}, {}
        def mulc(c, D, n):
            if n not in c: c[n] = J.mul(n, D)
            return c[n]
        want = nfb + 8; trials = ops = 0; rel = []
        while len(rel) < want:
            trials += 1; a = nxt() % r; b = nxt() % r; ops += cost(a) + cost(b) + 1
            u, v = J.add(mulc(mG, G, a), mulc(mT, T, b))
            if len(u) - 1 != 2: continue
            disc = (u[1] * u[1] - 4 * u[0]) % p
            if disc == 0 or not is_sq(disc, p): continue
            sq = tonelli(disc, p); i2 = pow(2, -1, p)
            vec = {}
            for x in (((-u[1] + sq) * i2) % p, ((-u[1] - sq) * i2) % p):
                y = peval(v, x, p); y0 = min(y, (p - y) % p)
                vec[idx[x]] = vec.get(idx[x], 0) + (1 if y == y0 else -1)
            rel.append((vec, a, b))
        # elimination with measure.rs counting; rows kept sparse-to-dense lazily
        m = []
        for vec, a, b in rel:
            rw = [0] * (nfb + 2)
            for i, e in vec.items(): rw[i] = e % r
            rw[nfb] = a; rw[nfb + 1] = b; m.append(rw)
        ncols = nfb + 2; la = 0; prow = 0
        for col in range(nfb):
            sel = next((i for i in range(prow, len(m)) if m[i][col] != 0), None)
            if sel is None: continue
            m[prow], m[sel] = m[sel], m[prow]
            inv = pow(m[prow][col], r - 2, r); pr = [x * inv % r for x in m[prow]]; m[prow] = pr; la += ncols
            for i in range(len(m)):
                if i != prow and m[i][col] != 0:
                    f = m[i][col]; m[i] = [(x - f * y) % r for x, y in zip(m[i], pr)]; la += ncols
            prow += 1
            if prow >= len(m): break
        xs = None
        for rw in m:
            if rw[nfb + 1] != 0 and not any(rw[:nfb]):
                xs = (-rw[nfb]) % r * pow(rw[nfb + 1], r - 2, r) % r; la += 1; break
        mine = {"group_ops": ops, "trials": trials, "relations": len(rel), "la_mulmods": la, "recovered": xs}
        claimed = {q: s["ic_jc"][q] for q in mine}
        o = {"p": p, "k": k, "seed": seed, "mine": mine, "claimed": claimed, "match": mine == claimed,
             "log_certified": xs is not None and J.norm(J.mul(xs, G)) == T}
        out.append(o); print(p, k, mine, "match" if o["match"] else ("MISMATCH", claimed), "cert", o["log_certified"], flush=True)
json.dump(out, open(sys.argv[3], "w"), indent=1)
print("cells", len(out), "all match", all(o["match"] for o in out), "all certified", all(o["log_certified"] for o in out))
