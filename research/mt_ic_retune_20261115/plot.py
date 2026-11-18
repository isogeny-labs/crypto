#!/usr/bin/env python3
"""fig_retune.svg/.png/.pdf from results/tune_table.json and summary.json."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
tune = json.load(open(os.path.join(HERE, "results", "tune_table.json")))
summ = json.load(open(os.path.join(HERE, "summary.json")))
ch = summ["choice"]
COL = {3: "#2a6fdb", 4: "#d1495b", 5: "#3a9d5d"}
fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.2))
for k in (3, 4, 5):
    pts = sorted((float(key.split(":")[2]), v["score"]) for key, v in tune.items() if key.split(":")[1] == str(k))
    a.plot([p[0] for p in pts], [p[1] for p in pts], "o-", color=COL[k], label=f"k = {k}")
a.axhline(1, color="#888", lw=0.8, ls="--")
a.plot([ch["mu"]], [ch["score"]], marker="*", ms=16, color="black", ls="none", label=f"chosen k = {ch['k']}, μ = {ch['mu']}")
a.set_yscale("log"); a.set_xlabel("μ (table size ∝ μ²)"); a.set_ylabel("time / default (k = 4, μ = 1.0), geometric mean")
a.set_title("Tune set W32-{0..3}: 12 configurations"); a.legend(frameon=False, fontsize=8)
Ts = sorted(summ["cells"], key=int)
series = [("chosen_over_default", "IC chosen / IC default", "#d1495b"),
          ("chosen_over_best_rho", "IC chosen / best rho", "#222222"),
          ("default_over_best_rho", "IC default / best rho", "#999999")]
for j, (key, lab, c) in enumerate(series):
    for x, T in enumerate(Ts):
        d = summ["cells"][T][key]
        xs = [x + (j - 1) * 0.22] * len(d["per_instance"])
        b.plot(xs, d["per_instance"], "o", color=c, alpha=0.45, ms=5)
        b.plot([x + (j - 1) * 0.22], [d["median"]], "_", color=c, ms=18, mew=2.5, label=lab if x == 0 else None)
b.axhline(1, color="#888", lw=0.8, ls="--")
b.set_xticks(range(len(Ts))); b.set_xticklabels([f"T = {T}" for T in Ts])
b.set_ylabel("whole-solve time ratio (median bar, instances dots)")
b.set_title("Holdout W32-{4..7}, 2^32"); b.legend(frameon=False, fontsize=8)
fig.suptitle("Multi-target IC retune at 2^32 [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(HERE, f"fig_retune.{ext}"), dpi=150 if ext == "png" else None)
