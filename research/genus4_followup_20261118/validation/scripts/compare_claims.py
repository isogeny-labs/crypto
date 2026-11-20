#!/usr/bin/env python3
"""Compare the validator's recompute (out/recompute.json) with the producer's analysis.json and
sensitivity.json (read as git blobs at COMMIT). Prints every compared cell and its relative difference.
Usage: compare_claims.py REPO RECOMPUTE_JSON OUT_JSON [COMMIT]"""
import json, subprocess, sys
REPO, REC, OUT = sys.argv[1:4]
COMMIT = sys.argv[4] if len(sys.argv) > 4 else "6213ecbc0da0ed4917ab75805258201d6629b477"
D = "research/genus4_followup_20261118"
show = lambda p: json.loads(subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{D}/{p}"], check=True, capture_output=True).stdout)
an, se, rc = show("analysis.json"), show("sensitivity.json"), json.load(open(REC))
N = rc["numbers"]
rows = []
def cmp(name, claimed, mine):
    rel = abs(claimed - mine) / max(abs(claimed), abs(mine), 1e-300)
    rows.append({"name": name, "claimed": claimed, "recomputed": mine, "rel_diff": rel})
for cc, v in an["select"].items():
    cmp(f"partA cc={cc} mean cost w=1", v["mean_cost_q3_w1"], N["partA"][cc]["mean_cost_w1"])
for a in ("w", "x"):
    pa, ma = an["arms"][a], N["arms"][a]
    for k in [k for k in pa if isinstance(pa[k], dict)]:
        for e in ("ratio", "lo", "hi"):
            cmp(f"{a} {k} {e}", pa[k][e], ma[k][e])
    for k in [k for k in pa if isinstance(pa[k], float) and k.startswith("vs_")]:
        cmp(f"{a} {k}", pa[k], ma[k]["ratio"])
    for k in ("share_la", "mean_cost_w1"):
        cmp(f"{a} {k}", pa[k], ma[k])
    ps = se[a]
    for k in [k for k in ps if k.startswith("vs_")]:
        for e in ("ratio", "lo", "hi"):
            cmp(f"{a} sens {k} {e}", ps[k][e], ma[k][e])
    h = ps["q3_w1_hi_over_50_seeds"]
    m = ma["q3_w1_hi_over_50_seeds"]
    cmp(f"{a} 50-seed hi min", h["min"], m["min"])
    cmp(f"{a} 50-seed hi median (sorted[25])", h["median"], m["median_upper"])
    cmp(f"{a} 50-seed hi max", h["max"], m["max"])
    cmp(f"{a} 50-seed frac below 1", h["frac_below_1"], m["frac_below_1"])
cmp("q_mean_over_theory", an["q_mean_over_theory"], N["q_mean_over_plain_theory"])
cmp("n_keys", an["n_keys"], N["n_keys"])
cmp("n_targets", an["n_targets"], N["n_targets"])
worst = max(rows, key=lambda r: r["rel_diff"])
out = {"cells": len(rows), "max_rel_diff": worst["rel_diff"], "worst": worst,
       "n_exact": sum(r["rel_diff"] == 0 for r in rows), "n_le_1e-12": sum(r["rel_diff"] <= 1e-12 for r in rows),
       "cc_star_claimed": an["cc_star"], "cc_star_recomputed": N["cc_star"],
       "gates_claimed": an["gates"], "rows": rows}
json.dump(out, open(OUT, "w"), indent=1)
for r in rows:
    print(f"{r['name']:<45} claimed {r['claimed']:<22.17g} recomputed {r['recomputed']:<22.17g} rel {r['rel_diff']:.2e}")
print({k: v for k, v in out.items() if k != "rows"})
