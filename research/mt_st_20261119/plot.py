#!/usr/bin/env python3
"""fig_st.svg/.png/.pdf from summary.json. Optional args: summary path, output dir."""
import json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
summ = json.load(open(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "summary.json")))
out = sys.argv[2] if len(sys.argv) > 2 else HERE
sizes = sorted(summ["cells"], key=int)
fig, (a, b) = plt.subplots(1, 2, figsize=(12, 4.4))
for x, pb in enumerate(sizes):
    d = summ["cells"][pb]["ic_over_rho"]
    a.plot([x] * len(d["per_instance"]), d["per_instance"], "o", color="#222222", alpha=0.4, ms=5)
    a.plot([x], [d["median"]], "_", color="#d1495b", ms=22, mew=3)
a.axhline(1, color="#888", lw=0.8, ls="--")
a.set_xticks(range(len(sizes))); a.set_xticklabels([f"2^{s}" for s in sizes])
a.set_ylabel("best IC / best batched rho (mean cost per target)"); a.set_title("Single target: IC / rho")
for key, lab, col in [("best_ic_mean_ms", "best IC", "#d1495b"), ("best_rho_mean_ms", "best batched rho (st lanes)", "#2a6fdb")]:
    xs, ys = [], []
    for x, pb in enumerate(sizes):
        v = summ["cells"][pb][key]
        b.plot([x] * len(v), list(v.values()), "o", color=col, alpha=0.4, ms=5)
        xs.append(x); ys.append(sorted(v.values())[len(v) // 2])
    b.plot(xs, ys, "-", color=col, label=lab)
b.set_yscale("log"); b.set_xticks(range(len(sizes))); b.set_xticklabels([f"2^{s}" for s in sizes])
b.set_ylabel("mean ms per target (log)"); b.set_title("Single-target cost by group size"); b.legend(frameon=False, fontsize=8)
fig.suptitle("Single-target IC vs batched rho, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(out, f"fig_st.{ext}"), dpi=150 if ext == "png" else None)
