#!/usr/bin/env python3
"""PROTOCOL section 3: pick cc* from part A (p = 8209 only). Prints cc* and writes select.json."""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
GRID = ["0.5", "0.7", "1.0", "1.4", "2.0"]
out = {}
for cc in GRID:
    rows = [json.loads(l) for f in glob.glob(os.path.join(HERE, f"a_cc{cc}.jsonl*"))
            for l in (__import__("gzip").open(f, "rt") if f.endswith(".gz") else open(f)) if l.strip()]
    runs = {(r["curve_seed"], r["target_seed"], r["seed"]): r for r in rows if r.get("kind") == "run" and r.get("g") == 4}
    ok = len(runs) == 36 and all(r.get("certified") and r.get("ok") for r in runs.values())
    cost = lambda r: r["group_ops"] + r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
    m = sum(map(cost, runs.values())) / len(runs) if runs else float("inf")
    out[cc] = {"runs": len(runs), "eligible": ok, "mean_cost_q3_w1": m}
elig = [c for c in GRID if out[c]["eligible"]]
best = min(elig, key=lambda c: (out[c]["mean_cost_q3_w1"], -float(c)))
json.dump({"grid": out, "cc_star": best}, open(os.path.join(HERE, "select.json"), "w"), indent=1)
print(best)
