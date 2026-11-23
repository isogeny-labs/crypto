#!/usr/bin/env python3
"""Recompute the Prop. B count on the E1 'subfield' draws from exp/e1_subfield.jsonl.
ASSUMPTION (not verified against Sage): Sage's modulus='minimal_weight' = irreducible of least weight,
ties broken by smallest integer value. Consistency of the results with the recorded deg_le1_dim is the only check."""
import json, sys
import gf2n
from gf2n import GF, irreducible, poly_str
from propB_check import fixed_field_basis, descend, count_deg_le1, predicted

SRC = "/home/user/crypto/research/fall_degree_bounds_20261008/exp/e1_subfield.jsonl"
def pol(s):
    v = 0
    for t in s.split(" + "):
        t = t.strip(); v |= 1 if t == "1" else 2 if t == "z" else 1 << int(t.split("^")[1])
    return v
def minimal_weight(n):
    best = None
    for m in range(1 << n, 1 << (n + 1)):
        if m & 1 and irreducible(m, n):
            w = bin(m).count("1")
            if best is None or w < best[0]: best = (w, m)
            if w == 3 and best[0] == 3: break
    # full scan for min weight needed only if no trinomial found first
    if best[0] > 3:
        for m in range(1 << n, 1 << (n + 1)):
            if m & 1 and bin(m).count("1") == best[0] and irreducible(m, n): return m
    return best[1]
recs = [json.loads(l) for l in open(SRC) if l.strip()]
recs = [r for r in recs if "x3" in r and r.get("family") == "subfield"]
res = []; cache = {}
for r in recs:
    n = r["n"]; npr = r["nprime"]
    if n not in cache:
        m = minimal_weight(n); gf2n.IRRED[n] = m
        F = GF(n); cache[n] = (F, *fixed_field_basis(F, npr)); cache[n] += (m,)
    F, V, Vs, Vb, m = cache[n]
    x3, b = pol(r["x3"]), pol(r["b"])
    cnt, one, rk = count_deg_le1(descend(F, Vb, x3, b))
    pc, pone = predicted(F, Vs, npr, x3, b)
    res.append(dict(n=n, seed=r["seed"], modulus=poly_str(m), x3_in_V=x3 in Vs, b_in_V=b in Vs,
                    recorded=r["deg_le1_dim"], recomputed=cnt, formula=pc, one_in_span=one,
                    descended_rank=rk, recorded_eq_recomputed=r["deg_le1_dim"] == cnt, formula_eq_recomputed=pc == cnt))
    print(res[-1], flush=True)
json.dump(res, open("crosscheck_e1_results.json", "w"), indent=1)
print("draws", len(res), "recorded==recomputed", sum(x["recorded_eq_recomputed"] for x in res),
      "formula==recomputed", sum(x["formula_eq_recomputed"] for x in res))
