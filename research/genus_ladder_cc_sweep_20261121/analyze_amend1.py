#!/usr/bin/env python3
"""Amendment 1: la_mul/rel_mul per cc and best cc against the LA price kappa. Usage: analyze_amend1.py WORKDIR OUT.json"""
import json, sys, os
CCS = ["1.0", "1.5", "2.0", "3.0", "4.0"]
KAP = [1, 2, 3, 4, 6, 9]
wd, out = sys.argv[1:3]
res = []
for fam in ("g3", "g4"):
    by = {}
    for cc in CCS:
        for l in open(os.path.join(wd, f"{fam}_cc{cc}.jsonl")):
            r = json.loads(l)
            if r.get("kind") == "run":
                by.setdefault((r["p"], cc, r["arm"]), []).append(r)
    for p in sorted({k[0] for k in by}):
        cell = {"family": fam, "p": p, "la_over_rel": {}, "best_cc_by_kappa": {}}
        for arm in ("C_double_lp+pre+sge(8,3)", "C_double_lp+pre"):
            for cc in CCS:
                rs = by[(p, cc, arm)]
                cell["la_over_rel"][f"{arm}|{cc}"] = sum(r["la_mul"] for r in rs) / sum(r["rel_mul"] for r in rs)
            for k in KAP:
                m = {}
                for cc in CCS:
                    rs = by[(p, cc, arm)]
                    m[cc] = sum(r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + k * r["la_mul"] for r in rs) / len(rs)
                b = min(m, key=m.get)
                cell["best_cc_by_kappa"][f"{arm}|{k}"] = {"best": b, "gain_vs_2.0": 1 - m[b] / m["2.0"]}
        res.append(cell)
json.dump(res, open(out, "w"), indent=1, sort_keys=True)
