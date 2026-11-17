#!/usr/bin/env python3
"""Per-row outcome of the gate criteria R1..R4 (PROTOCOL.md) for Tables B and C. Plots recorded verdicts only.
usage: plot_outcome.py REPO  -> figures/row_outcome.svg and .png"""
import json, sys
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle

repo = Path(sys.argv[1]); here = repo / "research/ic_validation_gate_20261024"
Bs = json.load(open(here / "B/standing.json")); Cs = json.load(open(here / "C/standing.json"))
Bv = json.load(open(here / "B/validator-report.json")); Cv = json.load(open(here / "C/validator-report.json"))
Bc = json.load(open(here / "B/replay-certificate.json")); Cc = json.load(open(here / "C/replay-certificate.json"))
PASS, FAIL, NA = "#1b7f79", "#c0392b", "#9aa0a6"
cols = ["R1 replay", "R2 scalars\n[d]G = Q", "R3 recompute", "R4 rho reference\n+ 5a band"]
rows = []
for r in Bv["rows"]:
    rb = next(x for x in Bc["runs"] if x["slug"] == r["slug"])
    a = r["analysis"]
    rows.append(("B", r["slug"], [PASS if rb["replay_identical"] else FAIL, PASS if rb["scalars_archive_ok"] and rb["scalars_replay_ok"] else FAIL,
                 PASS if r["pass_recompute"] else FAIL, PASS if a["order_holds_across_band"] else FAIL],
                 f'{r["claimed"]["x_reference"]:.2f}x  (band {min(b["ratio"] for b in a["band"]):.2f}-{max(b["ratio"] for b in a["band"]):.2f})'))
for r in Cv["rows"]:
    rc = next(x for x in Cc["runs"] if x["slug"] == r["slug"])
    rows.append(("C", r["slug"], [PASS if rc["identical"] == rc["sampled_runs"] else FAIL, NA,
                 PASS if r["pass_recompute"] else FAIL, FAIL], f'{r["claimed"]["x_reference"]:.2f}x  (no band)'))
fig, ax = plt.subplots(figsize=(9.6, 8.4))
for i, (t, slug, cs, txt) in enumerate(rows):
    y = len(rows) - 1 - i
    for j, c in enumerate(cs):
        ax.add_patch(Rectangle((j, y), 0.92, 0.82, color=c))
        ax.text(j + 0.46, y + 0.41, {PASS: "pass", FAIL: "fail", NA: "cannot check"}[c], ha="center", va="center", color="white", fontsize=7.5)
    ax.text(-0.1, y + 0.41, f"{t}  {slug}", ha="right", va="center", fontsize=8)
    ax.text(len(cols) + 0.1, y + 0.41, txt, ha="left", va="center", fontsize=8)
ax.set_xlim(0, len(cols) + 2.6); ax.set_ylim(-0.2, len(rows) + 0.2)
ax.set_xticks([j + 0.46 for j in range(len(cols))]); ax.set_xticklabels(cols, fontsize=8.5); ax.xaxis.tick_top()
ax.set_yticks([]); [s.set_visible(False) for s in ax.spines.values()]
ax.text(len(cols) + 0.1, len(rows) + 0.35, "x reference (board)\n(IC/rho across 5a band w=0..1)", fontsize=8, va="bottom")
ax.set_title("Validation gate outcome per leaderboard row, Tables B and C (2026-10-24)\n"
             "B: 13 of 13 pass.  C: replay and recompute hold; rho reference fails (different study and process, no band).\n"
             "All x reference values are above 1: the gap to matched rho is the distance remaining.", fontsize=9, pad=40)
fig.tight_layout()
fig.savefig(here / "figures/row_outcome.svg"); fig.savefig(here / "figures/row_outcome.png", dpi=150)
print("ok", len(rows))
