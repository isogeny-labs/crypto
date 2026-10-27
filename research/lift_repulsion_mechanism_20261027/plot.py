"""Layout only: plots values recorded in rows.jsonl / summary.json (research-visuals skill).
Both files are written by the native Rust crate; this script computes nothing
(it only selects, orders and draws recorded fields).

Usage: python3 plot.py reports/run-<UTC>/
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

run = Path(sys.argv[1])
rows = [json.loads(line) for line in (run / "rows.jsonl").read_text().splitlines() if line]

INK, MUTED, GRID = "#1f2328", "#6e7781", "#d0d7de"
CURVE, NULL, PLANT = "#0969da", "#bf5700", "#6e7781"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": INK, "ytick.color": INK, "svg.fonttype": "none"})


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(run / f"{name}.{ext}", bbox_inches="tight", **kw)
    plt.close(fig)


def style(ax):
    ax.grid(True, color=GRID, lw=0.6)
    ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)


def sel(n, pred):
    return [r for r in rows if r["n"] == n and pred(r)]


is_curve = lambda r: r["arm"].startswith("curve")
is_null = lambda r: r["arm"].startswith("null")
NS = [9, 11, 13, 12]

# Fig 1: ratio_4 vs Walsh 4th moment (kurtosis), curves vs nulls
fig, axs = plt.subplots(1, 4, figsize=(13, 3.4))
for ax, n in zip(axs, NS):
    nu, cu = sel(n, is_null), sel(n, is_curve)
    ax.scatter([r["kurtosis"] for r in nu], [r["ratio_A2_A3_A4"][2] for r in nu], s=14, c=NULL, alpha=0.6, label="density-paired / extra nulls")
    ax.scatter([r["kurtosis"] for r in cu], [r["ratio_A2_A3_A4"][2] for r in cu], s=22, c=CURVE, marker="s", label="curves")
    ax.axhline(1.0, color=MUTED, lw=0.8, ls="--")
    ax.set_xlabel("Walsh 4th moment  ΣW⁴/(q (q−1)²)")
    ax.set_title(f"N = {n}" + (" (composite)" if n == 12 else ""))
    style(ax)
axs[0].set_ylabel("A₄ / E₄")
axs[0].legend(frameon=False, fontsize=7, loc="upper left")
fig.suptitle("All-lift 4-subspace count against the Walsh 4th moment: curves sit at 2 (Sato–Tate), nulls at 3", y=1.03)
save(fig, "fig1_ratio4_vs_kurtosis")

# Fig 2: positive control (planted rank-one Walsh bias), log-log
fig, axs = plt.subplots(1, 3, figsize=(10.5, 3.3))
for ax, n in zip(axs, [9, 11, 13]):
    nu, cu = sel(n, is_null), sel(n, is_curve)
    pl = sel(n, lambda r: r["arm"].startswith("bias_"))
    ax.scatter([r["kurtosis"] for r in nu], [r["ratio_A2_A3_A4"][2] for r in nu], s=10, c=NULL, alpha=0.5, label="nulls")
    ax.scatter([r["kurtosis"] for r in cu], [r["ratio_A2_A3_A4"][2] for r in cu], s=14, c=CURVE, marker="s", label="curves")
    ax.scatter([r["kurtosis"] for r in pl], [r["ratio_A2_A3_A4"][2] for r in pl], s=26, edgecolors=INK, marker="D", facecolors="none", label="planted Walsh bias")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("Walsh 4th moment")
    ax.set_title(f"N = {n}")
    style(ax)
axs[0].set_ylabel("A₄ / E₄")
axs[0].legend(frameon=False, fontsize=7, loc="upper left")
fig.suptitle("Positive control: planting a large 4th moment raises the count", y=1.03)
save(fig, "fig2_positive_control")

# Fig 3: first-order prediction against exact ratio
fig, axs = plt.subplots(1, 4, figsize=(13, 3.4))
for ax, n in zip(axs, NS):
    nu, cu = sel(n, is_null), sel(n, is_curve)
    ax.scatter([r["pred_ratio_k2_k3_k4"][2] for r in nu], [r["ratio_A2_A3_A4"][2] for r in nu], s=12, c=NULL, alpha=0.6, label="nulls")
    ax.scatter([r["pred_ratio_k2_k3_k4"][2] for r in cu], [r["ratio_A2_A3_A4"][2] for r in cu], s=22, c=CURVE, marker="s", label="curves")
    lo = min(r["ratio_A2_A3_A4"][2] for r in nu + cu)
    hi = max(r["ratio_A2_A3_A4"][2] for r in nu + cu)
    ax.plot([lo, hi], [lo, hi], color=MUTED, lw=0.8, ls="--")
    ax.set_xlabel("first-order prediction  1 + Δ̂₄/E₄")
    ax.set_title(f"N = {n}" + (" (composite)" if n == 12 else ""))
    style(ax)
axs[0].set_ylabel("exact A₄ / E₄")
axs[0].legend(frameon=False, fontsize=7, loc="upper left")
fig.suptitle("Line and plane correlations predict the exact count (dashed: y = x)", y=1.03)
save(fig, "fig3_first_order_vs_exact")

# Fig 4: standardised correlations, curves vs nulls
fig, axs = plt.subplots(2, 3, figsize=(10.5, 5.2), sharex="col")
for j, n in enumerate([9, 11, 13]):
    nu, cu = sel(n, is_null), sel(n, is_curve)
    for i, (key, lab) in enumerate([("z_plane", "plane (4-point) z"), ("z_line", "line (3-point) z")]):
        ax = axs[i][j]
        ax.scatter([r[key] for r in nu], [0.0] * len(nu), s=10, c=NULL, alpha=0.5)
        ax.scatter([r[key] for r in cu], [1.0] * len(cu), s=14, c=CURVE, marker="s")
        ax.set_yticks([0, 1])
        ax.set_yticklabels(["nulls", "curves"])
        ax.set_ylim(-0.6, 1.6)
        ax.axvline(0.0, color=MUTED, lw=0.8, ls="--")
        ax.set_xlabel(lab)
        if i == 0:
            ax.set_title(f"N = {n}")
        style(ax)
fig.suptitle("Curves are pinned: the 4-point correlation is far below random, the 3-point one is two-valued", y=1.0)
save(fig, "fig4_standardised_correlations")
print("figures written to", run)
