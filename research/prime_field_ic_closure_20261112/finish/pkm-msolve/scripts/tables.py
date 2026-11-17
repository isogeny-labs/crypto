#!/usr/bin/env python3
"""Markdown tables from analysis.json + closure/certs.jsonl.  Usage: tables.py OUTDIR"""
import json
import os
import sys

OUT = sys.argv[1]
A = json.load(open(os.path.join(OUT, "analysis.json")))
certs = {}
cf = os.path.join(OUT, "closure", "certs.jsonl")
if os.path.exists(cf):
    for ln in open(cf):
        r = json.loads(ln)
        certs.setdefault(r["system"][:-3], {})[r["c"]] = r["one_in_closure"]

print("| arm | p | m | t | N | kind | D (draws) | max cols W | unit | grid zeros | wall s | peak RSS MB | checks | PC closure (c: 1 in V_c) |")
print("|---|---|--:|--:|--:|---|---|---|---|---|---|---|---|---|")
for c in sorted(A["cells"], key=lambda c: (c["arm"], c["p"], c["m"], c["t"], str(c["kind"]))):
    names = []
    pcs = []
    # closure certs by name pattern
    for nm, d in certs.items():
        parts = nm.split("_")
        if parts[0] == c["arm"] and parts[1] == f"p{c['p']}" and parts[2] == f"m{c['m']}" and parts[3] == f"t{c['t']}":
            kind = "planted" if parts[4].startswith("pl") else "random"
            if kind == c["kind"]:
                pcs.append(f"{parts[4]}: " + ", ".join(f"{k}:{'yes' if v else ('no' if v is False else '?')}" for k, v in sorted(d.items())))
    print(f"| {c['arm']} | {c['p']} | {c['m']} | {c['t']} | {c['m']*c['t']} | {c['kind']} | {' / '.join(map(str, c['D_draws']))} | "
          f"{' / '.join(map(str, c['W_draws']))} | {' / '.join(str(u) for u in c['unit'])} | {' / '.join(str(z) for z in c['grid_zeros'])} | "
          f"{' / '.join(str(round(w)) for w in c['wall_s'])} | {' / '.join(map(str, c['rss_MB']))} | {' / '.join(c['checks'])} | {'; '.join(pcs)} |")
print()
print("| arm/m | t_lo->t_hi | D_lo -> D_hi | sigma = dD/dN | alpha*(w=1) | alpha*(w=2) | verdict | flag | LS slope (all t) | s_W (log2 W ratio, last step) | budget m-3 |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for k, v in A["verdict"].items():
    sig = "-" if v["sigma"] is None else f"{v['sigma']:.3f}"
    ls = "-" if v["ls_slope_all_t"] is None else f"{v['ls_slope_all_t']:.3f}"
    sw = "-" if v["s_W"] is None else f"{v['s_W']:.2f}"
    print(f"| {k} | {v['t_lo']}->{v['t_hi']} | {v['D_lo']} -> {v['D_hi']} | {sig} | {v['alpha1']} | {v['alpha2']} | {v['verdict']} | {v['flag']} | {ls} | {sw} | {v['budget_m_minus_3']} |")
print()
print("ITEM VERDICT:", A["item_verdict"])
