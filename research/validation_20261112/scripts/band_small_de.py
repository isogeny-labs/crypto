#!/usr/bin/env python3
"""Target 4, AGENTS.md 5a band from the replayed raw data (validator code):
single-instance IC total and best matched rho per arm-cell at w = 0, 0.1, 1;
order per arm-cell; and, for the rho per-step work the instrument does not emit
(partition hash, DP test, DP-table query/insert), the number of extra native
operations per rho walk operation that would be needed to reverse the order at
w = 0.1 or 1.  Usage: band_small_de.py small_de.json sd_rho.jsonl OUT.json"""
import json, sys
raw = json.load(open(sys.argv[1])); W = (0.0, 0.1, 1.0)
rho_runs = {}
for l in open(sys.argv[2]):
    d = json.loads(l); rho_runs.setdefault((d["bits"], d["m"], d["d"], d["kind"]), []).append(d)
def nat(n): return n["muls"] + n["invs"] + n["tests"] + n["la_ops"]
rows = []
for c in raw["cells"]:
    key = (c["bits"], c["m"], c["d"])
    rk = {}
    for r in c["rho"]:
        runs = r["runs"]; n = len(runs)
        walk = sum(x["counters"]["walk_operations"] for x in rho_runs[key + (r["kind"],)]) / n
        rk[r["kind"]] = {"A": r["automorphisms"], **{str(w): sum(x["gae"] + w * x["native"] for x in runs) / n for w in W}, "walk_ops": walk}
    best = {str(w): min(rk, key=lambda k: rk[k][str(w)]) for w in W}
    for a in c["arms"]:
        rel = {w: sum(r["gae"] + a["build_gae"] + w * (nat(r["native"]) + nat(a["build_native"])) for r in a["reps"]) / len(a["reps"]) for w in W}
        tgt = {w: sum(t["gae"] + w * nat(t["native"]) for t in a["targets"]) / len(a["targets"]) for w in W}
        ic = {str(w): rel[w] + tgt[w] for w in W}
        rb = {str(w): rk[best[str(w)]][str(w)] for w in W}
        ratio = {k: ic[k] / rb[k] for k in ic}
        order = "IC above" if all(v > 1 for v in ratio.values()) else ("IC below" if all(v < 1 for v in ratio.values()) else "unresolved")
        flip = min((ic[str(w)] - rb[str(w)]) / w / rk[best[str(w)]]["walk_ops"] for w in (0.1, 1.0))
        rows.append({"bits": c["bits"], "m": c["m"], "d": c["d"], "arm": a["arm"], "ic": ic, "rho_best": rb, "rho_best_kind": best,
                     "rho_best_A": {k: rk[best[k]]["A"] for k in best}, "ratio": ratio, "order": order,
                     "extra_rho_native_per_walk_op_to_flip": flip, "certified": sum(1 for t in a["targets"] if t["certified"])})
out = {"arm_cells": len(rows), "order_counts": {o: sum(1 for r in rows if r["order"] == o) for o in ("IC above", "IC below", "unresolved")},
       "min_ratio": {str(w): min(r["ratio"][str(w)] for r in rows) for w in W},
       "min_extra_rho_native_per_walk_op_to_flip": min(r["extra_rho_native_per_walk_op_to_flip"] for r in rows),
       "argmin_flip": min(rows, key=lambda r: r["extra_rho_native_per_walk_op_to_flip"]),
       "rows": rows}
json.dump(out, open(sys.argv[3], "w"), indent=1)
print(json.dumps({k: v for k, v in out.items() if k != "rows"}, indent=1))
hl = [r for r in rows if (r["bits"], r["m"], r["d"], r["arm"]) == (28, 2, -11, "A2")][0]
print("headline cell 28/2/-11/A2:", json.dumps({k: hl[k] for k in ("ic", "rho_best", "rho_best_kind", "rho_best_A", "ratio", "order")}))
