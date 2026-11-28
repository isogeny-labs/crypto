#!/usr/bin/env python3
"""Figures and REPORT.pdf from summary.json (recorded ecbench compare/band output and per-arm phase means only)."""
import json, os, textwrap
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); S = json.load(open(os.path.join(HERE, "summary.json")))
os.makedirs(os.path.join(HERE, "figures"), exist_ok=True)
W = ["0", "0.1", "1"]
ARMS = ["gen-c1", "lv-lean", "lv-sib", "lv-tau", "lv-all", "rho-v5", "rho-strong"]
PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "verify", "setup", "search", "internal_verification"]
cols = {"factor_base": "#8c564b", "oracle_setup": "#1f77b4", "relations": "#ff7f0e", "linear_algebra": "#2ca02c", "verify": "#7f7f7f",
        "setup": "#9467bd", "search": "#d62728", "internal_verification": "#bcbd22"}
fig, ax = plt.subplots(1, 2, figsize=(13, 4.6))
arms = [a for a in ARMS if a in S["phases"] and a != "rho-strong"]
bottom = [0] * len(arms)
for p in PH:
    v = [S["phases"][a].get(p, 0) for a in arms]
    if not any(v): continue
    ax[0].bar(arms, v, bottom=bottom, color=cols[p], label=p); bottom = [b + x for b, x in zip(bottom, v)]
ax[0].set_title("K1 n29: mean charged GAE per solve by phase (w = 0; rho-strong ~932 omitted)", fontsize=9)
ax[0].legend(fontsize=7); ax[0].tick_params(axis="x", rotation=30, labelsize=8); ax[0].set_ylabel("GAE")
x = range(len(W))
for a, c, m in (("gen-c1", "#7f7f7f", "o--"), ("lv-all", "#1f77b4", "s-"), ("rho-v5", "#c0392b", "^-"), ("rho-strong", "#9467bd", "v-")):
    ax[1].plot(list(x), [S["band_S"][a][w] for w in W], m, color=c, label=a)
ax[1].set_xticks(list(x)); ax[1].set_xticklabels([f"w = {w}" for w in W]); ax[1].set_yscale("log"); ax[1].set_ylabel("band S (mean priced GAE / sqrt r)")
ax[1].set_title("Band S across the pricing band", fontsize=9); ax[1].legend(fontsize=7); plt.tight_layout()
for e in ("png", "svg"): fig.savefig(os.path.join(HERE, f"figures/fig1-phases-band.{e}"), dpi=130)
fig, ax = plt.subplots(figsize=(9, 5.2))
labels, vals, lo, hi = [], [], [], []
for pair in ("lv-all/gen-c1", "lv-lean/gen-c1", "lv-sib/gen-c1", "lv-tau/gen-c1", "gen-c1/rho-v5", "lv-all/rho-v5", "gen-c1/rho-strong", "lv-all/rho-strong"):
    for w in W:
        p = S["ratios"][pair][w]; labels.append(f"{pair}  w={w}"); vals.append(p["ratio"]); lo.append(p["ratio"] - p["lo"]); hi.append(p["hi"] - p["ratio"])
ax.errorbar(vals, range(len(vals)), xerr=[lo, hi], fmt="o", ms=3, capsize=2)
ax.set_yticks(range(len(vals))); ax.set_yticklabels(labels, fontsize=6.5); ax.axvline(1, color="k", lw=0.6); ax.set_xscale("log"); ax.invert_yaxis()
ax.set_title("Paired cost ratios with 95% cluster-bootstrap intervals (K1 n29)", fontsize=9); plt.tight_layout()
for e in ("png", "svg"): fig.savefig(os.path.join(HERE, f"figures/fig2-ratios.{e}"), dpi=130)
txt = open(os.path.join(HERE, "README.md")).read() if os.path.exists(os.path.join(HERE, "README.md")) else ""
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    lines = []
    for para in txt.splitlines():
        lines += textwrap.wrap(para, 150) or [""]
    for i in range(0, len(lines), 70):
        f = plt.figure(figsize=(11, 8.5)); f.text(0.03, 0.97, "\n".join(lines[i:i + 70]), va="top", family="monospace", fontsize=6.2); pdf.savefig(f); plt.close(f)
    for name in ("fig1-phases-band.png", "fig2-ratios.png"):
        img = plt.imread(os.path.join(HERE, "figures", name)); f = plt.figure(figsize=(11, 8.5)); plt.imshow(img); plt.axis("off"); pdf.savefig(f); plt.close(f)
print("ok")
