#!/usr/bin/env python3
"""Figures (SVG in figures/, PNG renders in the scratchpad) and REPORT.pdf from summary.json and README.md.
Plots recorded ecbench compare/band outputs and per-arm phase means only. Written after data (PROTOCOL.md section 5); adapted from koblitz_k1_sibwalk_20261118/make_report.py to n37 only."""
import json, os, textwrap
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
plt.rcParams["svg.fonttype"] = "none"; plt.rcParams["svg.hashsalt"] = "n37k8"
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); S = json.load(open(os.path.join(HERE, "summary.json")))
PNG = os.environ.get("PNG_DIR", "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/sw8-png")
os.makedirs(os.path.join(HERE, "figures"), exist_ok=True); os.makedirs(PNG, exist_ok=True)
W = ["0", "0.1", "1"]; N = ["37"]
PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "verify", "setup", "search", "internal_verification"]
cols = {"factor_base": "#8c564b", "oracle_setup": "#1f77b4", "relations": "#ff7f0e", "linear_algebra": "#2ca02c", "verify": "#7f7f7f",
        "setup": "#9467bd", "search": "#d62728", "internal_verification": "#bcbd22"}
FIGS = []
def save(fig, name):
    FIGS.append(fig); fig.savefig(os.path.join(HERE, "figures", name + ".svg")); fig.savefig(os.path.join(PNG, name + ".png"), dpi=80)
fig, ax = plt.subplots(2, 1, figsize=(7, 8)); ax = [[ax[0]], [ax[1]]]
for i, n in enumerate(N):
    arms = ["ls", "sw", "rho-v5"]; bottom = [0] * 3
    for p in PH:
        v = [S["phases"][n][a].get(p, 0) for a in arms]
        if not any(v): continue
        ax[0][i].bar(arms, v, bottom=bottom, color=cols[p], label=p); bottom = [b + x for b, x in zip(bottom, v)]
    ax[0][i].set_title(f"K1 n{n}: mean charged GAE per solve (w = 0)", fontsize=9); ax[0][i].legend(fontsize=6)
    b = S["band_S"][f"n{n}-pooled"]
    for a, c, m in (("ls", "#7f7f7f", "o--"), ("sw", "#1f77b4", "s-"), ("rho-v5", "#c0392b", "^-"), ("rho-strong", "#9467bd", "v-")):
        ax[1][i].plot(range(3), [b[a][w] for w in W], m, color=c, label=a)
    ax[1][i].set_xticks(range(3)); ax[1][i].set_xticklabels([f"w = {w}" for w in W]); ax[1][i].set_yscale("log")
    ax[1][i].set_title(f"n{n}: pooled band S (3 sessions)", fontsize=9); ax[1][i].legend(fontsize=7)
plt.tight_layout(); save(fig, "fig1-phases-band")
fig, ax = plt.subplots(figsize=(9, 6)); labels, vals, lo, hi = [], [], [], []
for n in N:
    for scope, pair in [("pooled", "sw/ls"), ("r1", "sw/ls"), ("r2", "sw/ls"), ("r3", "sw/ls"),
                        ("pooled", "ls/rho-v5"), ("pooled", "sw/rho-v5"), ("pooled", "ls/rho-strong"), ("pooled", "sw/rho-strong")]:
        for w in W:
            p = S["ratios"][n][scope][pair][w]; labels.append(f"n{n} {scope} {pair} w={w}"); vals.append(p["ratio"])
            lo.append(p["ratio"] - p["lo"]); hi.append(p["hi"] - p["ratio"])
ax.errorbar(vals, range(len(vals)), xerr=[lo, hi], fmt="o", ms=2.5, capsize=1.5)
ax.set_yticks(range(len(vals))); ax.set_yticklabels(labels, fontsize=6.5); ax.axvline(1, color="k", lw=0.6); ax.set_xscale("log"); ax.invert_yaxis()
ax.set_title("Paired cost ratios, 95% intervals (wider of linearised-t and cluster bootstrap)", fontsize=9); plt.tight_layout(); save(fig, "fig2-ratios")
txt = open(os.path.join(HERE, "README.md")).read()
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    lines = []
    for para in txt.splitlines(): lines += textwrap.wrap(para, 150) or [""]
    for i in range(0, len(lines), 70):
        f = plt.figure(figsize=(11, 8.5)); f.text(0.03, 0.97, "\n".join(lines[i:i + 70]), va="top", family="monospace", fontsize=6.2); pdf.savefig(f); plt.close(f)
    for f in FIGS: pdf.savefig(f); plt.close(f)  # vector pages (smaller than embedded renders)
print("ok")
