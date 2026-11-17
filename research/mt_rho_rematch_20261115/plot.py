#!/usr/bin/env python3
"""summary.json -> fig_rematch.svg/png: IC-B / best rho per configuration (dots) and cell median (line)."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
c = json.load(open(os.path.join(D, "summary.json")))["cells"]
cells = [f"{pb}:{T}" for pb in (24, 28, 32) for T in (1024, 4096, 16384)]
lab = [f"2^{k.split(':')[0]}\nT={k.split(':')[1]}" for k in cells]
fig, ax = plt.subplots(figsize=(10, 4.4))
for key, name, col in (("icb_best", "IC-B / best rho (tuned or not)", "#15803d"),
                       ("icb_untuned", "IC-B / best untuned rho", "#9ca3af"),
                       ("tuned_untuned", "best tuned rho / best untuned rho", "#2563eb")):
    ax.plot(range(9), [c[k][key]["median"] for k in cells], "-", color=col, label=name)
    for i, k in enumerate(cells):
        ax.scatter([i] * len(c[k][key]["values"]), c[k][key]["values"], s=10, color=col, alpha=0.6)
ax.axhline(1, color="#374151", lw=0.8)
ax.set_xticks(range(9)); ax.set_xticklabels(lab, fontsize=7); ax.grid(alpha=0.3); ax.legend(fontsize=8)
ax.set_title("rematch: whole multi-target solve, ratios per instance (dots) and cell median (line)")
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(D, f"fig_rematch.{ext}"), dpi=150)
