#!/usr/bin/env python3
"""Figure for EXP-PKM-DENSE-DEC-001 Amendments 1-3 (larger S) from results.json and results_large.json."""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
F = json.load(open(os.path.join(HERE, "results.json")))["cells"]
L = json.load(open(os.path.join(HERE, "results_large.json")))
cells = F + L["cells"]
fig, axes = plt.subplots(1, 2, figsize=(10, 4))
for ax, m in zip(axes, (2, 3)):
    cs = [c for c in cells if c["m"] == m]

    def ser(key, vk, p=None):
        pts = {}
        for c in cs:
            if c["vkind"] == vk and c.get(key) and (p is None or c["p"] == p):
                pts.setdefault(c["S"], c[key])
        return sorted(pts), [pts[s] for s in sorted(pts)]

    for key, vk, lab, sty, col in (("tower_planted_ops_median", "coset", "tower oracle", "o-", "#3b6ea8"),
                                   ("dense_planted_ops_median", "coset", "dense oracle, coset V", "^:", "#7a9e4b"),
                                   ("dense_planted_ops_median", "random", "dense oracle, random V", "s--", "#c8553d"),
                                   ("enum_ops", "coset", "enumeration over V", "d-", "#555555")):
        x, y = ser(key, vk)
        ax.plot(x, y, sty, color=col, label=lab, ms=4)
    if m == 2:
        lb = [v for v in L["censored_lower_bounds"].values()]
        ax.plot([v["S"] for v in lb], [v["ops_at_cutoff_median"] for v in lb], "^", color="#7a9e4b", mfc="none",
                ms=9, label="dense, censored at 1800 s (lower bound)")
    else:
        ax.axvline(128, color="#999", ls=":", lw=1)
        ax.text(0.97, 0.06, "S = 128: OOM-killed\nat 108.5 GiB after 11.3 h", transform=ax.transAxes,
                ha="right", fontsize=7, color="#555")
    hi = 2 ** 14.5 if m == 2 else 2 ** 7.5
    ax.axvspan(256 * 1.2 if m == 2 else 32 * 1.2, hi, color="#f2efe6", zorder=-1)
    ax.set_xlim(2 ** 3.6 if m == 2 else 2 ** 1.6, hi)
    ax.set_xscale("log", base=2)
    ax.set_yscale("log")
    ax.set_xlabel("|V| = S  (shaded: RunPod extension)")
    ax.set_ylabel("field_ops to find all relations (planted, median)")
    ax.set_title(f"m = {m}")
    ax.grid(alpha=0.3, which="both")
axes[0].legend(fontsize=7, loc="lower right")
fig.suptitle("Larger S: the dense form keeps beating the tower; enumeration keeps beating both", fontsize=9)
for ext in ("svg", "png"):
    fig.savefig(os.path.join(HERE, "figures", f"find_cost_large.{ext}"), dpi=160, bbox_inches="tight")
print("ok")
