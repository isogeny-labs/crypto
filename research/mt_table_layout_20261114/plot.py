#!/usr/bin/env python3
"""summary.json -> fig_layout.svg/png: target-phase ratios to A by arm, and whole-arm ratios to the best rho."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D = os.path.dirname(os.path.abspath(__file__))
c = json.load(open(os.path.join(D, "summary.json")))["cells"]
cells = [f"{pb}:{T}" for pb in (24, 28, 32) for T in (1024, 4096, 16384)]
lab = [f"2^{k.split(':')[0]}\nT={k.split(':')[1]}" for k in cells]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(12, 4.4))
for key, name, col in (("Bs_A_t", "batch 32, std table", "#6b7280"), ("Bf1_A_t", "flat table, no batch", "#d97706"),
                       ("Bf8_A_t", "flat + batch 8 + prefetch", "#2563eb"), ("B_A_t", "flat + batch 32 + prefetch (B)", "#15803d")):
    a1.plot(range(9), [c[k][key]["median"] for k in cells], "-o", ms=4, color=col, label=name)
a1.axhline(1, color="#374151", lw=0.8); a1.axhline(0.8, color="#15803d", ls="--", lw=0.8, label="primary gate 0.8")
a1.set_title("target-phase wall time / baseline A (median of 4 instances)")
for key, name, col in (("A_rho", "baseline IC (A) / best rho", "#9ca3af"), ("B_rho", "IC with B / best rho", "#15803d")):
    a2.plot(range(9), [c[k][key]["median"] for k in cells], "-o", ms=4, color=col, label=name)
a2.axhline(1, color="#374151", lw=0.8)
a2.set_title("whole multi-target solve / best multi-target rho")
for a in (a1, a2):
    a.set_xticks(range(9)); a.set_xticklabels(lab, fontsize=7); a.grid(alpha=0.3); a.legend(fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(D, f"fig_layout.{ext}"), dpi=150)
