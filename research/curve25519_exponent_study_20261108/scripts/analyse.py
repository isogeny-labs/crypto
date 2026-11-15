#!/usr/bin/env python3
"""Fit ln T vs ln |V| per (k, m, form) as fixed in PROTOCOL.md section 5."""
import glob, json, math, os, sys
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
T975 = {1: 12.706, 2: 4.303, 3: 3.182, 4: 2.776, 5: 2.571, 6: 2.447, 7: 2.365, 8: 2.306, 9: 2.262,
        10: 2.228, 11: 2.201, 12: 2.179, 13: 2.160, 14: 2.145}

def fit(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    s = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    c = my - s * mx
    if n > 2:
        rss = sum((y - c - s * x) ** 2 for x, y in zip(xs, ys))
        se = math.sqrt(rss / (n - 2) / sxx)
        t = T975.get(n - 2, 1.96)
        return s, s - t * se, s + t * se, se
    return s, None, None, None

cells = {}
for f in sorted(glob.glob(os.path.join(D, "cells", "*.json"))):
    j = json.load(open(f))
    key = (j["k"], j["m"], j["form"], j["gidx"])
    cells.setdefault(key, []).append(j)
out = {"groups": {}}
for key, cs in sorted(cells.items()):
    cs.sort(key=lambda j: j["d_or_t"])
    rows = []
    for j in cs:
        rows.append({"d": j["d_or_t"], "v_size": j.get("v_size"), "capped": j["capped"],
                     "T": j.get("mean_field_ops"), "done": j.get("targets_done"),
                     "sd_max": max([r["solving_degree_max"] for r in j.get("per_target", [])] or [None]) if j.get("per_target") else None,
                     "max_cols": max([r["max_cols"] for r in j.get("per_target", [])] or [0]),
                     "n_vars": j.get("n_vars"), "input_terms": j.get("input_terms"),
                     "sym_terms": j.get("sym_terms"),
                     "consistent": sum(1 for r in j.get("per_target", []) if not r["inconsistent"])})
    ok = [r for r in rows if not r["capped"] and r["T"]]
    g = {"rows": rows}
    for name, lo in (("fit_d_ge_9", 9), ("fit_all", 0)):
        sel = [r for r in ok if r["d"] >= lo]
        if len(sel) >= 4 and sel[-1]["d"] / sel[0]["d"] >= 4:
            s, l, u, se = fit([math.log(r["v_size"]) for r in sel], [math.log(r["T"]) for r in sel])
            g[name] = {"n": len(sel), "slope": s, "lo95": l, "hi95": u, "se": se}
        else:
            g[name] = None
    loc = []
    for a, b in zip(ok, ok[1:]):
        loc.append({"d": [a["d"], b["d"]], "slope": math.log(b["T"] / a["T"]) / math.log(b["v_size"] / a["v_size"])})
    g["local_slopes"] = loc
    out["groups"][f"k{key[0]}_m{key[1]}_{key[2]}_g{key[3]}"] = g
json.dump(out, open(os.path.join(D, "data", "analysis.json"), "w"), indent=1)
for k, g in out["groups"].items():
    print(k, g["fit_d_ge_9"], "n_cells", len(g["rows"]), "capped", sum(r["capped"] for r in g["rows"]))
