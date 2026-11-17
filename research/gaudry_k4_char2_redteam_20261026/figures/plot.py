"""Lay out recorded S values (no computation beyond reading records).

    python research/gaudry_k4_char2_redteam_20261026/figures/plot.py
"""
import json
import pathlib

import matplotlib

matplotlib.use("svg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
plt.rcParams["svg.hashsalt"] = "k4-char2-redteam"
recs = json.loads((HERE.parent / "replication" / "inventory.json").read_text())["records"]
fig, ax = plt.subplots(figsize=(7.5, 4.2))
style = {"rho-neg": ("#d1495b", "s"), "ic-wied": ("#2a9d4b", "o"), "ic-dense": ("#7f7f7f", "o")}
for arm, (colour, marker) in style.items():
    clean = [r for r in recs if r["arm"] == arm and not r["killed"] and not r["warmup"] and r["verified"]]
    ax.scatter([r["m"] for r in clean], [r["S"] for r in clean], color=colour, marker=marker,
               label=f"{arm} (clean, n={len(clean)})", zorder=3)
    killed = [r for r in recs if r["arm"] == arm and r["killed"] and r["verified"]]
    if killed:
        ax.scatter([r["m"] for r in killed], [r["S"] for r in killed], facecolors="none",
                   edgecolors=colour, marker=marker, s=70,
                   label=f"{arm} (killed run, NOT evidence, n={len(killed)})", zorder=3)
ax.axhline(0.8862, color="#d1495b", ls=":", lw=1, label="rho theory S = 0.886 (negation)")
ax.set_yscale("log")
ax.set_xlabel("m  (curve over F_{2^{4m}}, subgroup about 2^{4m-1})")
ax.set_ylabel("recorded S = GAE / sqrt(r)  (log scale)")
ax.set_title("k = 4 IC vs rho in characteristic 2: every recorded target\n"
             "study research/gaudry_k4_char2_crossover_20261015 (branch claude/k4-char2-crossover-dev)", fontsize=9)
ax.grid(alpha=0.3)
ax.legend(fontsize=7)
fig.tight_layout()
fig.savefig(HERE / "s-by-m.svg")
fig.savefig(HERE / "s-by-m.png", dpi=150)
