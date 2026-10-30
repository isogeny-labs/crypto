#!/usr/bin/env python3
"""Figures for research/ic_acct3_hec_ecbench_20261021. Plots recorded values only.

Reads analysis/equivalence.md (written by examples/hec_ecbench_equivalence.rs from
sessions/g2-p41 and research/hec_bitop_20261020). Computes no result: it parses the
two tables and draws the recorded ecbench and study bitop/v1 counts side by side.

    /tmp/report-venv/bin/python research/ic_acct3_hec_ecbench_20261021/figures/plots.py
"""
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.lines import Line2D  # noqa: E402

HERE = Path(__file__).resolve().parent
SRC = (HERE.parent / "analysis" / "equivalence.md").read_text().splitlines()
BLUE, ORANGE, INK, MUTED = "#2a78d6", "#eb6834", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})
ARMS = {"hec.ic_fb_walk": ("IC  hec.ic_fb_walk", BLUE), "hec.rho_full_width": ("rho  hec.rho_full_width", ORANGE)}


def tables(lines):
    out, cur = [], None
    for ln in lines:
        if ln.startswith("|"):
            cells = [c.strip() for c in ln.strip("|").split("|")]
            if cur is None:
                cur = {"head": cells, "rows": []}
            elif not set("".join(cells)) <= set("-: "):
                cur["rows"].append(dict(zip(cur["head"], cells)))
        elif cur is not None:
            out.append(cur)
            cur = None
    if cur is not None:
        out.append(cur)
    return out


runs_t, phase_t = tables(SRC)
assert runs_t["head"][0] == "arm" and phase_t["head"][1] == "phase"
assert len(runs_t["rows"]) == 45 and all(r["equal"] == "yes" for r in runs_t["rows"])


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Per-phase sums, ecbench (filled) vs the original study (ring), per arm.
order = ["precompute", "walk", "oracle", "linear_algebra", "verification"]
rows = []
for m in ARMS:
    for ph in order:
        r = next(x for x in phase_t["rows"] if x["method"] == m and x["phase"] == ph)
        rows.append((m, ph, int(r["ecbench Σ bitop/v1"]), int(r["study Σ bitop/v1"])))
for m in ARMS:  # unattributed is recorded as 0 for both arms; stated in the caption
    u = next(x for x in phase_t["rows"] if x["method"] == m and x["phase"] == "unattributed")
    assert u["ecbench Σ bitop/v1"] == u["study Σ bitop/v1"] == "0"
fig, ax = plt.subplots(figsize=(7.2, 4.2))
n = len(rows)
for i, (m, ph, e, s) in enumerate(rows):
    y = n - 1 - i
    col = ARMS[m][1]
    ax.plot([0, e], [y, y], color=col, lw=1, alpha=0.35)
    ax.plot(s, y, "o", ms=11, mfc="none", mec=INK, mew=1)
    ax.plot(e, y, "o", ms=5.5, color=col)
    ax.annotate(f"{e:,}", (e, y), xytext=(9, 0), textcoords="offset points", va="center",
                fontsize=7, color=INK)
ax.set_xscale("log")
ax.set_xlim(1e5, 3e10)
ax.set_yticks(range(n), [("IC (5 runs) · " if m == "hec.ic_fb_walk" else "rho (40 runs) · ") + ph.replace("_", " ")
                          for m, ph, _, _ in rows][::-1])
ax.axhline(4.5, color=MUTED, lw=0.6)
ax.grid(axis="y", visible=False)
ax.set_xlabel("Σ bitop/v1 over the arm's runs (log scale)")
ax.set_title("hec g2 p41: per-phase bitop/v1, ecbench session vs original study", fontsize=9)
ax.legend(handles=[Line2D([], [], ls="", marker="o", ms=5.5, color=BLUE, label="ecbench, IC arm"),
                   Line2D([], [], ls="", marker="o", ms=5.5, color=ORANGE, label="ecbench, rho arm"),
                   Line2D([], [], ls="", marker="o", ms=11, mfc="none", mec=INK,
                          label="hec_bitop_20261020 (committed)")],
          fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.45, -0.13), ncol=3)
save(fig, "phases_ecbench_vs_study")

# 2. Per-run totals: ecbench (y) against the study (x), all 45 runs, with y = x.
fig, ax = plt.subplots(figsize=(5.2, 4.4))
for m, (label, col) in ARMS.items():
    rs = [r for r in runs_t["rows"] if r["method"] == m]
    xs = [int(r["study bitop/v1"]) for r in rs]
    ys = [int(r["ecbench bitop/v1"]) for r in rs]
    ax.plot(xs, ys, "o", ms=5, color=col, mec="white", mew=0.8, label=f"{label} ({len(rs)} runs)")
ax.plot([1e7, 2e8], [1e7, 2e8], color=INK, lw=0.8, ls="--", label="ecbench = study")
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlim(1e7, 2e8)
ax.set_ylim(1e7, 2e8)
ax.set_aspect("equal")
ax.set_xlabel("study bitop/v1 per run (hec_bitop_20261020)")
ax.set_ylabel("ecbench bitop/v1 per run (ECBS1he77b325d1865)")
ax.set_title("Per-run totals, 45 of 45 equal", fontsize=9)
ax.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "runs_ecbench_vs_study")
print("figures written")
