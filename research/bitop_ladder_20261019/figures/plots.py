#!/usr/bin/env python3
"""Figures for research/bitop_ladder_20261019. Plots recorded values only.

Reads summary.json (copied from ecbench bound fits and ecbench work),
params.json and select/selection.json (select.py over ecbench work), and the
2026-10-15 campaign's params.json for comparison. Computes nothing.

    /tmp/report-venv/bin/python research/bitop_ladder_20261019/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent
ROOT = STUDY.parent.parent
S = json.loads((STUDY / "summary.json").read_text())
SEL = json.loads((STUDY / "select" / "selection.json").read_text())
CAMPAIGN = json.loads((ROOT / "research/ic_frontier_campaign_20261015/params.json").read_text())

COLOR = {
    "rho-neg": "#2a78d6",
    "bsgs-neg": "#1baf7a",
    "kangaroo": "#eda100",
    "ic-pair-rule": "#eb6834",
    "ic-pair-b32": "#6250d6",
    "ic-rootfind-m3-rule": "#e87ba4",
}
LABEL = {
    "rho-neg": "rho, negation (reference)",
    "bsgs-neg": "BSGS, negation",
    "kangaroo": "kangaroo",
    "ic-pair-rule": "IC pair table, per-size base",
    "ic-pair-b32": "IC pair table, fixed base 32",
    "ic-rootfind-m3-rule": "IC root-finding m=3, per-size base",
}
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Exponents in additions with 95% intervals, per arm and fit range.
rows = []
for arm in ["rho-neg", "bsgs-neg", "kangaroo", "ic-rootfind-m3-rule", "ic-pair-b32", "ic-pair-rule"]:
    for rng in ["16-26", "16-28", "16-32", "33-40"]:
        f = S["fits"].get(f"{arm}-{rng}")
        if f:
            rows.append((arm, rng, f))
fig, ax = plt.subplots(figsize=(7.2, 5.4))
for i, (arm, rng, f) in enumerate(rows):
    lo, hi = f["alpha_additions_ci95"]
    ax.plot([lo, hi], [i, i], color=COLOR[arm], lw=2, solid_capstyle="round")
    ax.plot(f["alpha_additions"], i, "o", color=COLOR[arm], ms=5)
ax.set_yticks(range(len(rows)), [f"{LABEL[a]}  {r} bits ({f['runs']} runs)" for a, r, f in rows], fontsize=7.5)
ax.axvline(0.5, color="#555", lw=1, ls="--")
ax.text(0.5, len(rows) - 0.2, " generic ½", fontsize=7.5, color="#555", va="top")
ax.axvline(0.766, color=COLOR["ic-pair-b32"], lw=1, ls=":")
ax.text(0.766, len(rows) - 0.2, " bitop/v1 study,\n base 32: 0.766", fontsize=7.5, color=COLOR["ic-pair-b32"], va="top")
ax.set_xlabel("exponent α in additions (cost ∝ r^α), with 95% interval")
ax.set_ylim(len(rows) + 0.6, -0.6)
ax.set_title("Fitted exponents, hold-out ladder (ecbench bound fit --unit bitop/v1)", fontsize=9.5)
ax.grid(axis="y", visible=False)
save(fig, "exponents")

# 2. Cost relative to rho per size (mean bitop/v1 over rho-neg's).
gap = S["gap_to_rho"]
arms = gap["arms"]
fig, ax = plt.subplots(figsize=(7.2, 4.2))
for j, arm in enumerate(arms):
    if arm == "rho-neg":
        continue
    pts = [(r[0], r[j + 1]) for r in gap["rows"] if r[j + 1] is not None]
    ax.plot([p[0] for p in pts], [p[1] for p in pts], "-o", ms=3, lw=1.6, color=COLOR[arm], label=LABEL[arm])
ax.axhline(1, color=COLOR["rho-neg"], lw=1.2, ls="--", label=LABEL["rho-neg"] + " = 1")
ax.set_yscale("log")
ax.set_xlabel("curve size (bits of p; one a = −3 prime-order curve per size)")
ax.set_ylabel("mean bitop/v1 ÷ rho's, same 80 targets")
ax.set_title("Distance to rho per curve size (80 hold-out targets each, every run verified)", fontsize=9.5)
ax.legend(fontsize=7.5, frameon=False, loc="upper left")
save(fig, "gap_to_rho")

# 3. Selected base per size, against the campaign's GAE selection.
P = S["params"]
fig, ax = plt.subplots(figsize=(7.2, 3.8))
b = sorted(int(k) for k in P["pair"])
ax.plot(b, [P["pair"][str(k)] for k in b], "-o", ms=3, color=COLOR["ic-pair-rule"], label="pair table, chosen in bitop/v1 (this study)")
rb = sorted(int(k) for k in P["rf3"])
ax.plot(rb, [P["rf3"][str(k)] for k in rb], "-o", ms=3, color=COLOR["ic-rootfind-m3-rule"], label="root-finding m=3, chosen in bitop/v1")
cp = CAMPAIGN["prime"]
ax.plot([r["bits"] for r in cp], [r["pair_size"] for r in cp], "s", ms=5, mfc="none", color="#555",
        label="pair table, chosen in GAE at w=0.1 (2026-10-15 campaign)")
ax.axhline(32, color=COLOR["ic-pair-b32"], lw=1, ls=":", label="fixed base 32")
ax.set_yscale("log", base=2)
ax.set_xlabel("curve size (bits)")
ax.set_ylabel("factor-base size (abscissae)")
ax.set_title("Base size chosen per curve size on the train panel", fontsize=9.5)
ax.legend(fontsize=7.5, frameon=False)
save(fig, "base_selection")

# 4. Train-panel cost curves for the pair table at four sizes.
fig, ax = plt.subplots(figsize=(7.2, 3.8))
for bits, col in [(20, "#86b6ef"), (28, "#3987e5"), (34, "#1c5cab"), (40, "#0d366b")]:
    pts = sorted((r[2], r[5]) for r in SEL["rows"] if r[0] == bits and r[1] == "pair" and r[5] is not None)
    ax.plot([p[0] for p in pts], [p[1] for p in pts], "-o", ms=3, color=col, label=f"{bits} bits")
    sel = P["pair"][str(bits)]
    ax.plot([sel], [dict(pts)[sel]], "*", ms=10, color=col)
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("factor-base size (abscissae)")
ax.set_ylabel("mean bitop/v1 per solve (train targets)")
ax.set_title("Train panel: pair-table cost against base size (★ selected)", fontsize=9.5)
ax.legend(fontsize=7.5, frameon=False)
save(fig, "train_curves")
print("figures written")
