"""Graphs of the n2 real-model addition study, from the recorded files.
Layout only: values are parsed from the tools' own output; nothing is recomputed.

Usage: python plot.py <results-dir> <out-dir>
"""

import re
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

res, out = Path(sys.argv[1]), Path(sys.argv[2])
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})
OLD, NEW, RHO = "#b5651d", "#2a78d6", "#333333"


def run(prefix):
    return sorted(d for d in res.iterdir() if d.name.startswith(prefix))[-1]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# ---- per-operation cost, from glue2_checks.txt (bits 40 line) ----
chk = run("N1-check")
txt = (chk / "glue2_checks.txt").read_text()
num = r"([\d.]+)M\+([\d.]+)I"
m_add = re.search(rf"REAL bits 40 p=.*?old {num} new {num}", txt)
m_dbl = re.search(rf"REAL bits 40: double.*?old {num} new {num}", txt)
m_walk = re.search(rf"REAL bits 40: walk add.*?old {num} new {num}", txt)
m_b16 = re.search(rf"REAL bits 40: batched add, W=16: {num}", txt)
labels = ["addition\n(generic pair)", "walk addition\nD + R_j (mixed)", "doubling"]
old = [float(m_add.group(1)), float(m_walk.group(1)), float(m_dbl.group(1))]
new = [float(m_add.group(3)), float(m_walk.group(3)), float(m_dbl.group(3))]
fig, ax = plt.subplots(figsize=(6.2, 3.2))
xs = range(len(labels))
w = 0.36
ax.bar([x - w / 2 for x in xs], old, w, color=OLD, label="baseline radd_fast (+1 I)")
ax.bar([x + w / 2 for x in xs], new, w, color=NEW, label="radd_fast2 (+1 I)")
for x, o, n in zip(xs, old, new):
    ax.text(x - w / 2, o + 0.8, f"{o:.0f}", ha="center", fontsize=7)
    ax.text(x + w / 2, n + 0.8, f"{n:.0f}", ha="center", fontsize=7)
ax.axhline(22, color=RHO, ls="--", lw=0.9)
ax.text(1.5, 23.2, "round-4 parity estimate: 22 M (+ I)", ha="center", fontsize=7, color=RHO,
        bbox=dict(fc="white", ec="none", pad=1))
ax.set_xticks(list(xs))
ax.set_xticklabels(labels)
ax.set_ylabel("field multiplications per operation (counted)")
ax.set_ylim(0, 58)
ax.set_title(f"Genus-2 real-model Jacobian law, counted M per operation; run {chk.name}", fontsize=7.5)
ax.legend(fontsize=7, loc="upper left")
save(fig, "per_op_cost")

# ---- IC/rho at the gate's three points, from the gae_band output ----
g = run("N2-glue2")
band = (g / "gae_band.txt").read_text() + (run("N3-glue2") / "gae_band.txt").read_text()
pt = re.compile(r"GAE point bits (\d+) h (\d+) (Jglue2?) w ([\d.]+): ic [\d.e+-]+ GAE, rho [\d.e+-]+ GAE, ratio ([\d.]+)")
bd = re.compile(r"GAE band bits (\d+) h (\d+) (Jglue2?): \S+ in \[([\d.]+), ([\d.]+)\]")
pts, bands = {}, {}
for m in pt.finditer(band):
    pts[(int(m.group(1)), int(m.group(2)), m.group(3), float(m.group(4)))] = float(m.group(5))
for m in bd.finditer(band):
    bands[(int(m.group(1)), int(m.group(2)), m.group(3))] = (float(m.group(4)), float(m.group(5)))
fig, axs = plt.subplots(1, 2, figsize=(7.6, 4.3), sharey=True)
for ax, h in zip(axs, (1, 4)):
    sizes = sorted({k[0] for k in pts if k[1] == h})
    for arm, col, off, lab in (("Jglue", OLD, -0.35, "baseline Jglue"), ("Jglue2", NEW, 0.35, "Jglue2 (new formulas)")):
        for i, b in enumerate(sizes):
            lo, hi = bands[(b, h, arm)]
            ax.plot([b + off, b + off], [lo, hi], color=col, lw=3, alpha=0.25, solid_capstyle="butt",
                    label=f"{lab}: 12-cell band" if i == 0 else None)
        for w_, mk in ((0.0, "o"), (0.1, "s"), (1.0, "^")):
            ax.plot([b + off for b in sizes], [pts[(b, h, arm, w_)] for b in sizes], mk + "-", color=col, ms=3, lw=0.6,
                    label=f"{lab}, native {w_:g} GAE")
    ax.axhline(1, color=RHO, ls="--", lw=0.9)
    ax.text(sizes[0] - 0.8, 1.03, "rho", fontsize=7, color=RHO)
    ax.set_title(f"h = {h}", fontsize=8)
    ax.set_xlabel("log2 #E")
    ax.set_xticks(sizes)
axs[0].set_ylabel("IC cost / matched rho cost")
h_, l_ = axs[0].get_legend_handles_labels()
fig.legend(h_, l_, fontsize=6, ncol=4, loc="lower center")
fig.suptitle(f"Distance to rho: gate points (I = 3M, measured Jacobian op) and the 12-cell band; runs {g.name} and N3 (44 bits)", fontsize=7.5)
fig.tight_layout(rect=(0, 0.12, 1, 1))
fig.savefig(out / "ic_over_rho.svg")
fig.savefig(out / "ic_over_rho.png", dpi=150)
plt.close(fig)
