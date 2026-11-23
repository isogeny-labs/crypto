#!/usr/bin/env python3
"""Validator counterfactual (model on measured phases): ic5 with a known-log factor base (F_i = r_i G, r_i random),
i.e. the same k-term table and target trials with the relation search and the mod-n linear algebra replaced by
m double-and-add scalar multiplications, each charged (bits - 1) doublings + bits/2 additions = 1.5 bits GAE.
Reports ic5_generic / ic5 and the IC-specific (relation + LA) share of ic5, per cell (median), cI = 10, h = 0.1.
usage: generic_ic5.py <rows.jsonl> <out.json>"""
import json, sys, statistics, collections
rows = [json.loads(l) for l in open(sys.argv[1])]
def g(v, ci=10, h=0.1): return (ci * v[0] + v[1] + v[2]) / (ci + 3) + h * (v[3] + v[4])
res = collections.defaultdict(list)
for r in rows:
    if r["arm"] != "ic5": continue
    ph = r["phases"]; tot = g([r["I"], r["M"], r["S"], r["H"], r["L"]])
    rel = g(ph["relations"]) - g(ph["table"])
    bits = r["n"].bit_length(); repl = r["m"] * 1.5 * bits
    res[(r["pb"], r["T"])].append(((tot - rel + repl) / tot, rel / tot))
out = {f"{pb}/{T}": dict(generic_over_ic5=round(statistics.median(x[0] for x in v), 4), ic_specific_share=round(statistics.median(x[1] for x in v), 4)) for (pb, T), v in sorted(res.items())}
json.dump(out, open(sys.argv[2], "w"), indent=1); print(out)
