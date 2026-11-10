#!/usr/bin/env python3
"""Print the README tables from data/analysis.json and data/census.json, and the
exploratory (not preregistered) fits restricted to one chain length L
(L = number of prime factors of d with multiplicity). Output: data/tables.md,
data/exploratory_fits.json."""
import json, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
A = json.load(open(os.path.join(D, "data", "analysis.json")))["groups"]
C = {(c["k"], c["d_or_t"], c["gidx"]): c for c in json.load(open(os.path.join(D, "data", "census.json")))}
from analyse import fit  # same OLS + t-interval as the preregistered estimator

def L(d):
    n, q = 0, 2
    while d > 1:
        while d % q == 0:
            d //= q; n += 1
        q += 1
    return n

out, ex = [], {}
out.append("| group | d = |V| | L | columns | base points | covered m=2 | covered m=3 | T (field ops) | solving degree | max cols | status |")
out.append("|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|")
for key, g in A.items():
    k = int(key.split("_")[0][1:]); gi = int(key.split("_g")[-1])
    for r in g["rows"]:
        c = C.get((k, r["d"], gi), {})
        cov = c.get("covered_points", {})
        T = f"{r['T']:.4g}" if r["T"] else "-"
        st = "complete"
        if r["capped"]:
            cf = json.load(open(os.path.join(D, "cells", f"{key.rsplit('_g', 1)[0]}_d{r['d']}_g{gi}.json")))
            st = "censored (OOM at 3.5 GB)" if cf.get("failure") else "censored (1200 s cap)"
        out.append(f"| {key} | {r['d']} | {L(r['d'])} | {c.get('columns','-')} | {c.get('base_points','-')} | {cov.get('m2','-')} | {cov.get('m3','-')} | {T} | {r['sd_max'] or '-'} | {r['max_cols'] or '-'} | {st} |")
    ok = [r for r in g["rows"] if not r["capped"] and r["T"]]
    for l in sorted({L(r["d"]) for r in ok}):
        sel = [r for r in ok if L(r["d"]) == l]
        if len(sel) >= 3 and sel[-1]["d"] / sel[0]["d"] >= 3:
            s, lo, hi, se = fit([math.log(r["v_size"]) for r in sel], [math.log(r["T"]) for r in sel])
            ex[f"{key}_L{l}"] = {"n": len(sel), "d": [r["d"] for r in sel], "slope": s, "lo95": lo, "hi95": hi}
out.append("")
out.append("| exploratory fit (one chain length) | d values | n | slope | 95% interval |")
out.append("|:--|:--|--:|--:|:--|")
for k, v in ex.items():
    iv = f"[{v['lo95']:.2f}, {v['hi95']:.2f}]" if v["lo95"] is not None else "-"
    out.append(f"| {k} | {', '.join(map(str, v['d']))} | {v['n']} | {v['slope']:.2f} | {iv} |")
open(os.path.join(D, "data", "tables.md"), "w").write("\n".join(out) + "\n")
json.dump(ex, open(os.path.join(D, "data", "exploratory_fits.json"), "w"), indent=1)
print("\n".join(out[-len(ex) - 2:]))
