#!/usr/bin/env python3
"""C6: compare every number in analysis.json, tune.json (grid and choice) and curves.json (at COMMIT) with the
validator's out/recompute.json and out/select_check.json.  Every leaf of analysis.json must be matched
(an unmatched leaf is a failure, not a skip).  Writes out/compare_claims.json."""
import math, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from common import *

rc = json.load(open(os.path.join(OUT, "recompute.json")))
sc = json.load(open(os.path.join(OUT, "select_check.json")))
an = json.loads(blob(f"{D}/analysis.json"))
tu = json.loads(blob(f"{D}/tune.json"))
cells, unmatched = [], []


def cmp(name, claimed, mine):
    if isinstance(claimed, bool) or isinstance(mine, bool) or isinstance(claimed, str):
        ok = claimed == mine
        rel = 0.0 if ok else None
    else:
        rel = abs(claimed - mine) / max(abs(mine), 1e-300)
        ok = rel <= 1e-12
    cells.append({"name": name, "claimed": claimed, "recomputed": mine, "rel_diff": rel, "ok": ok})


def leaves(o, pre=""):
    if isinstance(o, dict):
        for k, v in o.items():
            yield from leaves(v, f"{pre}/{k}")
    elif isinstance(o, list):
        for i, v in enumerate(o):
            yield from leaves(v, f"{pre}[{i}]")
    else:
        yield pre, o


def mine_for(path):
    """Map an analysis.json leaf path to the validator's number."""
    parts = path.strip("/").split("/")
    if parts[0] == "gates":
        return rc["gates"][parts[1]]
    if parts[0] == "q_check":
        return {"runs": rc["checks"]["Q"]["runs"], "mean_over_sqrt_pi_r_2": rc["checks"]["Q"]["mean_over_sqrt_pi_r_2"]}[parts[1]]
    if parts[0] == "tune":
        c = rc["partA"]["choice"]
        g = rc["partA"]["grid"][f"{c['cc']}|{c['w_max']},{c['fill_max']:g}"]
        return {"cc": c["cc"], "w_max": c["w_max"], "fill_max": c["fill_max"], "runs": g["runs"], "eligible": g["eligible"],
                "mean_cost_w1": c["mean_cost_w1"]}[parts[1]]
    if parts[0] == "xstar_is_x":
        c = rc["partA"]["choice"]
        return c["cc"] == "2.0" and c["w_max"] == 4 and c["fill_max"] == 1.5
    if parts[0] == "primes":
        p = rc["primes"][parts[1].split("[")[0]]
        if parts[2] == "keys":
            return p["keys"]
        if parts[2].startswith("curves"):
            return p["curves"][int(parts[2].split("[")[1][:-1])]
        if parts[2] == "xstar_over_x_w1":
            return p["xstar_over_x_w1"][parts[3]]
        if parts[2] == "arms":
            a = p["arms"][parts[3]]
            if parts[4] == "per_curve_neg_w1":
                return a["per_curve_neg_w1"][parts[5]]
            v = a[parts[4]]
            return v[parts[5]] if isinstance(v, dict) else v
    if parts[0] == "slope":
        s = rc["slope"][parts[1]]
        if parts[2] == "ic_cost_w1_in_p":
            return s["ic_cost_w1_in_p"]
        if parts[3] == "note":
            return None
        return s["ratio_vs_neg_w1"][parts[3]]
    return None


for path, v in leaves(an):
    if path.startswith("/gate_failures") or path == "/claims" or path.endswith("/note"):
        continue
    m = mine_for(path)
    if m is None:
        unmatched.append(path)
        continue
    cmp("analysis.json" + path, v, m)
for k, g in tu["grid"].items():
    mine = rc["partA"]["grid"][k]
    for f in ("runs", "eligible", "mean_cost_w1"):
        cmp(f"tune.json/grid/{k}/{f}", g[f], mine[f])
for f in ("cc", "w_max", "fill_max", "mean_cost_w1"):
    cmp(f"tune.json/choice/{f}", tu["choice"][f], rc["partA"]["choice"][f])
cmp("curves.json (admitted, chosen, orders) equal the validator's selection", True, all(sc["curves_json_matches"].values()))
ok = all(c["ok"] for c in cells) and not unmatched
mx = max((c["rel_diff"] for c in cells if c["rel_diff"] is not None), default=0)
res = {"cells": len(cells), "exact": sum(c["rel_diff"] == 0 for c in cells), "max_rel_diff": mx, "unmatched_leaves": unmatched,
       "ok": ok, "rows": cells}
dump("compare_claims.json", res)
print({k: v for k, v in res.items() if k != "rows"})
for c in cells:
    if not c["ok"]:
        print("MISMATCH", c)
