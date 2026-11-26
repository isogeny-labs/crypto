#!/usr/bin/env python3
"""Exact replay trajectories (projected rank vs relations) and the paired query-count panel."""
import json, sys, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
R = sys.argv[1]
rep = json.load(open(f"{R}/evidence/exact-replay/replay.json"))
fig, (a1, a2) = plt.subplots(1, 2, figsize=(12.5, 4.4))
for rule, col, lab in (("fb2", "#868e96", ">= 2 factor-base terms (#5097 rule)"), ("all", "#4c6ef5", "all arities admitted")):
    t = rep["rules"][rule]["trajectory"]
    a1.plot([r[0] for r in t], [r[3] for r in t], color=col, label=f"{lab}: point projected rank")
    a1.plot([r[0] for r in t], [r[2] for r in t], color=col, ls=":", label=f"{lab}: LP rank")
    ft = rep["rules"][rule]["first_target_row"]
    a1.axvline(ft, color=col, ls="--", lw=0.8)
    a1.text(ft, 30 if rule == "fb2" else 80, f" target at row {ft}", color=col, fontsize=7)
for q in rep["rules"]["all"]["by_query"]:
    a1.axvline(q["rows"], color="#dee2e6", lw=0.5, zorder=0)
a1.set_xlabel("relations collected (frozen #5097 stream order; grey: query boundaries)")
a1.set_ylabel("rank"); a1.set_title("Exact replay on the frozen 11-query stream", fontsize=10); a1.legend(fontsize=7)
pan = json.load(open(f"{R}/evidence/panel-02/summary.json"))
import glob
runs = {r["file"]: r for r in map(json.loads, open(f"{R}/evidence/panel-02/runs.jsonl"))}
pairs = [(r["queries"], runs[f.replace("-fb2", "-all")]["queries"]) for f, r in runs.items() if f.endswith("-fb2.json")]
xs = [p[0] for p in pairs]; ys = [p[1] for p in pairs]
a2.scatter(xs, ys, s=14, color="#4c6ef5")
lo, hi = min(xs + ys) - 2, max(xs + ys) + 2
a2.plot([lo, hi], [lo, hi], color="#adb5bd", lw=0.8)
m = pan["metrics"]["queries"]
a2.set_xlabel("queries to verified recovery, #5097 rule"); a2.set_ylabel("queries, all arities admitted")
a2.set_title(f"Pair table, {pan['salts']} paired salts: {100*(1-1/m['fb2_over_all'][0]):.1f}% fewer queries "
             f"(all lower in {m['all_lower']}/{pan['salts']})", fontsize=10)
for a in (a1, a2): a.grid(alpha=.3)
fig.suptitle("Higher large-prime arity on frozen B677 M31: rank replay and collection work", fontsize=10)
fig.tight_layout()
for ext in ("svg", "pdf", "png"):
    fig.savefig(f"{R}/figures/arity.{ext}", dpi=150)
