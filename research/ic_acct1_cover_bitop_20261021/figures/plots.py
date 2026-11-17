#!/usr/bin/env python3
"""Figures for research/ic_acct1_cover_bitop_20261021. Plots recorded values only.

Reads analysis.json (examples/jv_cover_bitop_analyse.rs). Computes no result:
it only orders rows, takes the min/max of recorded band bounds for the drawn
envelope, and sums recorded phase means into the labelled groups.

    /tmp/report-venv/bin/python research/ic_acct1_cover_bitop_20261021/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402
from matplotlib.patches import Patch  # noqa: E402

HERE = Path(__file__).resolve().parent
A = json.loads((HERE.parent / "analysis.json").read_text())
assert A["schema"] == "ic_acct1_cover_bitop_analysis/v1"
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. IC/rho per row, bitop/v1 ratio with 95% CI and the section 5a band, against 1.
order = [(1511, 8, "R3"), (1511, 8, "R0"), (1511, 64, "R3"), (1511, 64, "R0"),
         (1009, 8, "R3"), (1009, 8, "R0"), (1009, 64, "R3"), (1009, 64, "R0")]
rows = {(r["p"], r["moves"], r["rho"]): r for r in A["rows"]}
assert set(rows) == set(order)
fig, ax = plt.subplots(figsize=(6.8, 4.6))
shapes = {0.0: "o", 0.1: "s", 1.0: "D"}
for i, key in enumerate(order):
    r = rows[key]
    y = len(order) - 1 - i
    lo = min(b["ci95"][0] for b in r["band"])
    hi = max(b["ci95"][1] for b in r["band"])
    ax.barh(y - 0.18, hi - lo, left=lo, height=0.22, color=ORANGE, alpha=0.28, lw=0)
    for b in r["band"]:
        ax.plot(b["ratio"], y - 0.18, shapes[b["w_gae"]], ms=4.5, mfc="white", mec=ORANGE, mew=1.2)
    ax.plot(r["ci95"], [y + 0.14] * 2, color=BLUE, lw=2, solid_capstyle="round")
    ax.plot(r["ratio"], y + 0.14, "o", ms=6, color=BLUE, mec="white", mew=1)
    if i == 0:
        ax.annotate(f"{r['ratio']:.4f} [{r['ci95'][0]:.4f}, {r['ci95'][1]:.4f}]",
                    (r["ci95"][1], y + 0.14), xytext=(6, 0), textcoords="offset points",
                    va="center", fontsize=7.5, color=INK)
        ax.annotate(f"band max U = {hi:.4f}", (hi, y - 0.18), xytext=(6, 0),
                    textcoords="offset points", va="center", fontsize=7.5, color=INK)
labels = [f"p={p}, moves={m}, {rho}" + ("  (primary)" if i == 0 else "")
          for i, (p, m, rho) in enumerate(order)]
ax.set_yticks(range(len(order)), labels[::-1])
ax.get_yticklabels()[-1].set_fontweight("bold")
ax.set_xscale("log")
ax.set_xlim(4e-3, 1.6)
ax.axvline(1, color=INK, lw=1, ls="--")
ax.text(0.93, len(order) - 1.5, "IC = rho", ha="right", va="center", fontsize=7.5, color=INK)
ax.grid(axis="y", visible=False)
ax.set_xlabel("IC / rho, ratio of mean bitop/v1 per solve (log scale)")
ax.set_title("Genus-3 cover IC / rho, both executed on the same challenge curves", fontsize=9)
handles = [Line2D([], [], color=BLUE, lw=2, marker="o", ms=5, mec="white",
                  label="bitop/v1 ratio, 95% bootstrap CI")]
handles += [Patch(color=ORANGE, alpha=0.28, label="§5a band: span of 95% CIs over w")]
handles += [Line2D([], [], ls="", marker=shapes[w], ms=4.5, mfc="white", mec=ORANGE, mew=1.2,
                   label=f"§5a ratio at w = {w:g} GAE") for w in (0.0, 0.1, 1.0)]
ax.legend(handles=handles, fontsize=7, frameon=False, loc="upper center",
          bbox_to_anchor=(0.4, -0.16), ncol=2)
save(fig, "ratio_vs_rho")

# 2. IC phase breakdown, primary row (p = 1511, moves = 8; identical for R3 and R0).
ph = rows[(1511, 8, "R3")]["ic_phase_bitops_mean"]
assert ph == rows[(1511, 8, "R0")]["ic_phase_bitops_mean"]
groups = [("relations (sieve)", ["relations"]),
          ("descent", ["descent"]),
          ("linear algebra (mod ℓ)", ["linear_algebra"]),
          ("walk + transport + model", ["walk", "transport", "model"]),
          ("factor base + verification", ["factor_base", "verification"])]
assert sorted(k for _, ks in groups for k in ks) == sorted(k for k in ph if k != "unattributed")
assert ph["unattributed"] == 0
vals = [sum(ph[k] for k in ks) for _, ks in groups]
total = rows[(1511, 8, "R3")]["ic_bitops_mean"]
fig, ax = plt.subplots(figsize=(6.6, 2.7))
ys = range(len(groups))[::-1]
ax.barh(list(ys), vals, height=0.6, color=BLUE)
for y, v in zip(ys, vals):
    share = 100 * v / total
    s = f"{share:.0f}%" if share >= 1 else (f"{share:.1f}%" if share >= 0.1 else "<0.1%")
    ax.annotate(f"{v:.2e}  ({s})", (v, y), xytext=(5, 0), textcoords="offset points",
                va="center", fontsize=7.5, color=INK)
ax.set_yticks(list(ys), [g for g, _ in groups])
ax.set_xlim(0, max(vals) * 1.35)
ax.grid(axis="y", visible=False)
ax.set_xlabel("mean bitop/v1 per solve (4 instances)")
ax.set_title(f"IC arm by phase, p = 1511, moves = 8 (total {total:.2e} bitops)", fontsize=9)
save(fig, "ic_phases_primary")
print("figures written")
