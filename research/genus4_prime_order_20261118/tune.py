#!/usr/bin/env python3
"""PROTOCOL section 3: pick (cc*, sge*) from part A. Writes tune.json; prints 'cc W,F'."""
import glob, gzip, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
CCS = ["1.4", "2.0", "2.8"]
SGES = [(4, 1.5), (8, 2), (16, 3), (32, 4)]
BASE = ("2.0", (4, 1.5))
grid = {}
for cc in CCS:
    rows = []
    for f in glob.glob(os.path.join(HERE, f"a_cc{cc}.jsonl*")):
        op = gzip.open if f.endswith(".gz") else open
        rows += [json.loads(l) for l in op(f, "rt") if l.strip()]
    for w, fm in SGES:
        lbl = f"+sge({w},{fm})"
        runs = {(r["curve_seed"], r["target_seed"], r["seed"]): r for r in rows
                if r.get("kind") == "run" and r["arm"].endswith(lbl) and "ferrari" in r["arm"]}
        ok = len(runs) == 12 and all(r.get("certified") and r.get("ok") for r in runs.values())
        cost = lambda r: r["group_ops"] + r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
        m = sum(map(cost, runs.values())) / len(runs) if runs else float("inf")
        grid[f"{cc}|{w},{fm}"] = {"cc": cc, "w_max": w, "fill_max": fm, "runs": len(runs), "eligible": ok, "mean_cost_w1": m}
def dist(v):
    return abs(float(v["cc"]) - 2.0) + abs(v["w_max"] - 4) / 4 + abs(v["fill_max"] - 1.5)
elig = [v for v in grid.values() if v["eligible"]]
best = min(elig, key=lambda v: (v["mean_cost_w1"], dist(v)))
json.dump({"grid": grid, "choice": best, "baseline": {"cc": BASE[0], "w_max": 4, "fill_max": 1.5}}, open(os.path.join(HERE, "tune.json"), "w"), indent=1)
print(f"{best['cc']} {best['w_max']},{best['fill_max']}")
