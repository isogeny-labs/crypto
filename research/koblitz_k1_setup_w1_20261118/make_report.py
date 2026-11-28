#!/usr/bin/env python3
"""Figures (SVG in figures/, PNG renders in the scratchpad) and REPORT.pdf from summary.json and README.md.
Plots recorded ecbench compare/band outputs and per-arm phase means only. Written after data (PROTOCOL.md section 6);
adapted from koblitz_n37_sibwalk_k8_20261118/make_report.py to three curves and a w = 1 phase split."""
import json, os, textwrap
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
plt.rcParams["svg.fonttype"] = "none"; plt.rcParams["svg.hashsalt"] = "k1w1"
from matplotlib.backends.backend_pdf import PdfPages
HERE = os.path.dirname(os.path.abspath(__file__)); S = json.load(open(os.path.join(HERE, "summary.json")))
PNG = os.environ.get("PNG_DIR", "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/w1-png")
os.makedirs(os.path.join(HERE, "figures"), exist_ok=True); os.makedirs(PNG, exist_ok=True)
W = ["0", "0.1", "1"]; N = ["23", "29", "37"]
PH = ["factor_base", "oracle_setup", "relations", "linear_algebra", "verify"]
cols = {"factor_base": "#8c564b", "oracle_setup": "#1f77b4", "relations": "#ff7f0e", "linear_algebra": "#2ca02c", "verify": "#7f7f7f"}
CAL = ("lookups", "row_ops", "frobenius_maps", "canonicalisations", "target_guard_probes", "as_solves")
def priced_w1(n, arm, ph):
    """Mean group ops + native ops priced at w = 1 in one phase (the band's default pricing, as model.py's cost())."""
    nat = S["native"][n][arm]; g = S["phases"][n][arm].get(ph + ".group_ops", 0); u = 0
    for k, v in nat.items():
        if not k.startswith(ph + "."): continue
        c = k[len(ph) + 1:]
        if c in CAL or (c.endswith("_uncharged") and c != "factor_base_vec_bytes_floor_uncharged"): u += v
        if c in ("wn.table_insert_attempts", "wn.sibling_second_points"): u += v
    return g + u
FIGS = []
def save(fig, name):
    FIGS.append(fig); fig.savefig(os.path.join(HERE, "figures", name + ".svg")); fig.savefig(os.path.join(PNG, name + ".png"), dpi=80)
fig, ax = plt.subplots(2, 3, figsize=(11, 7.5))
for i, n in enumerate(N):
    arms = ["inc", "w1-neg", "w1-ids", "w1"]; bottom = [0] * len(arms)
    for p in PH:
        v = [priced_w1(n, a, p) for a in arms]
        ax[0][i].bar(arms, v, bottom=bottom, color=cols[p], label=p); bottom = [b + x for b, x in zip(bottom, v)]
    ax[0][i].set_title(f"K1 n{n}: mean ops per solve priced at w = 1", fontsize=8); ax[0][i].tick_params(labelsize=7)
    if i == 0: ax[0][i].legend(fontsize=6)
    b = S["band_S"][f"n{n}-pooled"]
    for a, c, m in (("inc", "#7f7f7f", "o--"), ("w1", "#1f77b4", "s-"), ("rho-v5", "#c0392b", "^-"), ("rho-strong", "#9467bd", "v-")):
        ax[1][i].plot(range(3), [b[a][w] for w in W], m, color=c, label=a)
    ax[1][i].set_xticks(range(3)); ax[1][i].set_xticklabels([f"w = {w}" for w in W], fontsize=7); ax[1][i].set_yscale("log")
    ax[1][i].set_title(f"n{n}: pooled band S (3 sessions)", fontsize=8)
    if i == 0: ax[1][i].legend(fontsize=6)
plt.tight_layout(); save(fig, "fig1-phases-band")
fig, ax = plt.subplots(figsize=(9, 10)); labels, vals, lo, hi = [], [], [], []
for n in N:
    for scope, pair in [("pooled", "w1/inc"), ("r1", "w1/inc"), ("r2", "w1/inc"), ("r3", "w1/inc"), ("pooled", "w1-neg/inc"), ("pooled", "w1-ids/inc")]:
        for w in W:
            p = S["ratios"][n][scope][pair][w]; labels.append(f"n{n} {scope} {pair} w={w}"); vals.append(p["ratio"])
            lo.append(p["ratio"] - p["lo"]); hi.append(p["hi"] - p["ratio"])
ax.errorbar(vals, range(len(vals)), xerr=[lo, hi], fmt="o", ms=2.5, capsize=1.5)
ax.set_yticks(range(len(vals))); ax.set_yticklabels(labels, fontsize=5.5); ax.axvline(1, color="k", lw=0.6); ax.invert_yaxis()
ax.set_title("w1 / inc and single levers: paired cost ratios, 95% (wider of linearised-t and cluster bootstrap)", fontsize=8)
plt.tight_layout(); save(fig, "fig2-ratios")
fig, ax = plt.subplots(figsize=(8, 4.5)); x = 0; ticks = []
for n in N:
    for j, w in enumerate(W):
        for k, (pair, c) in enumerate((("inc/rho-v5", "#7f7f7f"), ("w1/rho-v5", "#1f77b4"))):
            p = S["ratios"][n]["pooled"][pair][w]
            ax.errorbar([x + 0.3 * k], [p["ratio"]], yerr=[[p["ratio"] - p["lo"]], [p["hi"] - p["ratio"]]], fmt="o", color=c, ms=3, capsize=2,
                        label=pair if (n, w) == ("23", "0") else None)
        ticks.append((x + 0.15, f"n{n}\nw={w}")); x += 1
ax.axhline(1, color="k", lw=0.6); ax.set_yscale("log"); ax.set_xticks([t for t, _ in ticks]); ax.set_xticklabels([l for _, l in ticks], fontsize=6)
ax.set_title("Distance to rho-v5 (pooled, paired, 95%); not a gate", fontsize=9); ax.legend(fontsize=7); plt.tight_layout(); save(fig, "fig3-gap-v5")
txt = open(os.path.join(HERE, "README.md")).read()
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    lines = []
    for para in txt.splitlines(): lines += textwrap.wrap(para, 150) or [""]
    for i in range(0, len(lines), 70):
        f = plt.figure(figsize=(11, 8.5)); f.text(0.03, 0.97, "\n".join(lines[i:i + 70]), va="top", family="monospace", fontsize=6.2); pdf.savefig(f); plt.close(f)
    for f in FIGS: pdf.savefig(f); plt.close(f)
print("ok")
