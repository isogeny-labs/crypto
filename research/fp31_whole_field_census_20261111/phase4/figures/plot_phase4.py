#!/usr/bin/env python3
"""Lays out recorded values of phase4/analysis.json and the diag_*.json files as SVG.
Usage: plot_phase4.py analysis.json diag_mechanism.json diag_order.json diag_consistency.json outdir"""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
A, M, O, C = (json.load(open(f)) for f in sys.argv[1:5]); out = sys.argv[5]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, axs = plt.subplots(1, 3, figsize=(7.2, 2.8))
ps = [c["friedman"]["p"] for c in A["per_class"]]
axs[0].hist(ps, bins=10, range=(0, 1), color="#888", edgecolor="w"); axs[0].axhline(len(ps) / 10, color="r", lw=0.8)
axs[0].set_xlabel("within-class Friedman p (45 classes)"); axs[0].set_ylabel("classes")
axs[0].set_title(f"T1: KS p = {A['primary']['p']:.1e}", fontsize=8)
pc = C["per_class"]
axs[1].scatter([c["spearman_plain_vs_aa"] for c in pc], [c["spearman_plain_vs_negation"] for c in pc], s=10, color="#444")
axs[1].axhline(0, color="grey", lw=0.5); axs[1].axvline(0, color="grey", lw=0.5)
axs[1].set_xlabel("rank corr. plain vs A/A (same walks)"); axs[1].set_ylabel("rank corr. plain vs negation")
axs[1].set_title("per-curve speed is persistent", fontsize=8)
prof = O["session_profile_by_decile"]
axs[2].plot([p["session_decile"] for p in prof], [100 * p["mean_log_ns_per_gae_minus_class_mean"] for p in prof], "o-", color="#444")
axs[2].axhline(0, color="grey", lw=0.5)
axs[2].set_xlabel("position in session (decile)"); axs[2].set_ylabel("% ns/GAE vs class mean")
iso, oth = M["iso_vs_base_abs_log_diff"]["median"], M["other_pairs_abs_log_diff"]["median"]
axs[2].set_title(f"iso/base {100*iso:.1f}% vs other {100*oth:.1f}%", fontsize=8)
fig.tight_layout(); fig.savefig(f"{out}/phase4_diagnostics.svg"); plt.close(fig)
print("ok")
