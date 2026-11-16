"""Render values already computed by the frozen Rust analyzer; no statistics."""
import argparse
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

parser = argparse.ArgumentParser()
parser.add_argument("--counts-png-only", action="store_true", help="Render the compact preview without changing SVG/PDF artifacts")
args = parser.parse_args()

ROOT = Path(__file__).resolve().parent
groups = json.loads((ROOT / "run/summary.json").read_text())["groups"]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})

def save(fig, name):
    if not args.counts_png_only:
        fig.savefig(ROOT / (name + ".svg"), bbox_inches="tight")
    if not args.counts_png_only or name == "counts":
        # PNG is a preview; full-resolution vector evidence remains in SVG/PDF.
        fig.savefig(ROOT / (name + ".png"), dpi=60 if name == "counts" else 180, bbox_inches="tight")
    plt.close(fig)

fig, axes = plt.subplots(2, 2, figsize=(10, 7.2))
for row, pop in enumerate(("conditioned-sum", "uniform-x-rational")):
    for col, field in enumerate(("word62", "p224")):
        ax = axes[row, col]
        selected = [g for g in groups if g["group"].startswith("poly-main:" + field + ":") and (":" + pop + ":finite") in g["group"]]
        selected.sort(key=lambda g: int(g["group"].split(":")[2][1:]))
        x = [int(g["group"].split(":")[2][1:]) for g in selected]
        for arm, label, color in (("schoolbook", "Schoolbook Vieta", "#c84d21"), ("newton", "Karatsuba / Newton identical", "#197b53"), ("lookup", "Batched lookup", "#226aa5")):
            ax.plot(x, [g["arms"][arm]["muls"]["median"] for g in selected], "o-", label=label, color=color, markersize=4)
        if field == "p224":
            extra = next(g for g in groups if g["group"] == "poly-supplemental:p224:m8192:" + pop + ":finite")
            ax.plot([8192], [extra["arms"]["lookup"]["muls"]["median"]], marker="D", fillstyle="none", color="#226aa5", linestyle="none")
            ax.annotate("All 4 polynomial arms timed out\n30 s each; supplemental n=1", (8192, 7e7), ha="right", va="top", color="#a84527", fontsize=8)
        ax.set(xscale="log", yscale="log", xlabel="Coset size m", ylabel="Primitive field multiplications", title=field + " · " + pop)
        if field == "p224":
            ax.set_ylim(1e3, 1.2e8)
        ax.grid(alpha=.25)
        if row == 0 and col == 0:
            ax.legend(fontsize=7)
fig.suptitle("Matched native component counts; medians over 3 retained targets/population\nSupplemental P224/m8192 kept separate; all clocks below L2", fontsize=11)
fig.tight_layout(rect=(0, 0, 1, .93))
save(fig, "counts")

fig, ax = plt.subplots(figsize=(8, 3.1))
keys = ["poly-main:word62:m8192:conditioned-sum:finite", "poly-main:word62:m16384:conditioned-sum:finite", "poly-main:p224:m512:conditioned-sum:finite", "poly-main:p224:m2048:conditioned-sum:finite"]
selected = [next(g for g in groups if g["group"] == k) for k in keys]
r = [g["paired_multiplication_ratios"]["schoolbook/newton"] for g in selected]
x = list(range(len(r)))
ax.errorbar(x, [v["median"] for v in r], yerr=[[v["median"]-v["min"] for v in r], [v["max"]-v["median"] for v in r]], fmt="o", capsize=5, color="#197b53", label="Schoolbook / Karatsuba; observed range")
for i, g in enumerate(selected):
    ax.plot(i, g["paired_multiplication_ratios"]["newton/newton-aa"]["median"], "s", color="#8253a0", markersize=5)
    ax.plot(i, g["paired_multiplication_ratios"]["lookup/lookup-fast"]["median"], "+", color="#226aa5", markersize=9)
ax.plot([], [], "s", color="#8253a0", label="A/A count ratio")
ax.plot([], [], "+", color="#226aa5", label="Unaffected lookup control ratio")
ax.axhline(1, color="gray", linestyle="--", linewidth=1)
ax.set(xticks=x, xticklabels=["word62 / 8192", "word62 / 16384", "P224 / 512", "P224 / 2048"], ylim=(.87, 2.38), ylabel="Paired multiplication-count ratio", title="Conditioned inputs: 3 matched retained targets/configuration")
ax.grid(axis="y", alpha=.25)
ax.legend(fontsize=8, loc="upper right")
fig.tight_layout()
save(fig, "ratios")

cert = json.loads((ROOT / "split-tower-certificate.json").read_text())
fig, ax = plt.subplots(figsize=(8, 2.6))
positions = {4: (.5, .9), 11: (.25, .53), 14: (.75, .53), 10: (.13, .15), 12: (.37, .15), 2: (.63, .15), 9: (.87, .15)}
for leaf in cert["leaves"]:
    a, b = leaf["path"]
    ax.annotate("", xy=positions[b], xytext=positions[a], arrowprops={"arrowstyle": "->", "color": "#66809a"})
for b in (11, 14):
    ax.annotate("", xy=positions[4], xytext=positions[b], arrowprops={"arrowstyle": "->", "color": "#66809a"})
for value, (xx, yy) in positions.items():
    ax.text(xx, yy, str(value), ha="center", va="center", bbox={"boxstyle": "round,pad=.4", "fc": "#e7f0f7", "ec": "#66809a"})
ax.text(.5, .66, "(X²+1)/(2X)", ha="center", fontsize=10)
ax.text(.5, -.04, "F₁₇ exact example: 4 leaves, degree-2 projectors, degree-3 coordinate certificates", ha="center", fontsize=9)
ax.set(xlim=(0, 1), ylim=(-.12, 1.08), title="A full split rational-map fiber tree (not an elliptic-isogeny assertion)")
ax.axis("off")
fig.tight_layout()
save(fig, "split-tower")
