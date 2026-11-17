#!/usr/bin/env python3
"""Layout only: render recorded native fields; no research arithmetic or solver."""
import json
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

root = Path(__file__).resolve().parent
record = json.loads((root / "runs/round22-native-summary.json").read_text())
fig = plt.figure(figsize=(11, 8.7), constrained_layout=True)
grid = fig.add_gridspec(2, 2, height_ratios=[1, 1.5])
ax = fig.add_subplot(grid[0, 0])
names = ["Regular cycles\nR21 theorem", "Norm-zero factors\nR22 exact lattice"]
values = [record["recovery"]["regular_target_R21_least_scalar"],
          record["recovery"]["augmentation_target_least_scalar"]]
ax.bar(names, values, color=["#667a91", "#167c65"], width=0.55)
ax.set_ylabel("Least positive recovery scalar (integer)")
ax.set_ylim(0, 6)
for i, value in enumerate(values):
    ax.text(i, value + .12, str(value), ha="center", fontsize=16, weight="bold")
ax.set_title("Different target objects; no cost comparison", fontsize=12)
ax.spines[["right", "top"]].set_visible(False)

ax = fig.add_subplot(grid[0, 1])
ax.axis("off")
r = record["recovery"]
ax.text(0, .98, "Complete integral calculation", va="top", fontsize=15, weight="bold")
lines = ["Fixed CM order: w² + w + 2 = 0",
         f"Integer system: {r['integer_system_rows']} × {r['integer_system_columns']}",
         "Smith invariants: " + ", ".join(map(str, r["Smith_invariants"])),
         f"Integral kernel rank: {r['kernel_rank']}",
         f"Both-sign order-11 fixtures: {r['both_sign_order11_fixtures']}",
         "Small certificate: B₁ M₁ + B₃ (7I) = I"]
ax.text(0, .78, "\n".join(lines), va="top", fontsize=12, linespacing=1.65)

ax = fig.add_subplot(grid[1, :])
ax.set_xlim(0, 10)
ax.set_ylim(0, 6)
ax.axis("off")
def box(x, y, text, color, width=4.2, height=1.15):
    ax.add_patch(FancyBboxPatch((x, y), width, height,
                 boxstyle="round,pad=0.12", linewidth=1.2,
                 edgecolor=color, facecolor=color + "13"))
    ax.text(x + width/2, y + height/2, text, ha="center", va="center", fontsize=11)
box(.2, 4.5, "CM augmentation recovery\nExact d = 1; complete Smith certificate", "#167c65")
box(5.5, 4.5, "Final polarized quotient\nKernel actions / quotient sections open", "#9a4e13")
box(.2, 2.65, "Tate / QR function field\nBase divisors / tower; original SS5 fiber checked", "#167c65")
box(5.5, 2.65, "Proper level-77 construction\nLifted divisors / complete RR / map open", "#9a4e13")
box(.2, .8, "Node187 coupled valuations\nExcludes four old models under written bridges", "#166784")
box(5.5, .8, "Polynomial certificate / reverse evaluator\nhᵏ f ∈ I and arbitrary divisors still open", "#9a4e13")
for y in [5.075, 3.225, 1.375]:
    ax.annotate("", xy=(5.25, y), xytext=(4.65, y),
                arrowprops={"arrowstyle": "->", "linestyle": "--", "color": "#9a4e13"})
ax.text(5, .1, "Dashed arrows mark outstanding construction obligations, not established maps.",
        ha="center", fontsize=10)
fig.suptitle("Koblitz cover transfers · Round22 · exact evidence and remaining obligations",
             fontsize=15, weight="bold")
fig.supxlabel("Source: runs/round22-native-summary.json · exact integers, no sampling uncertainty · no IC cost or rank result",
              fontsize=9)
fig.savefig(root / "round22.svg")
svg = root / "round22.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(root / "round22.png", dpi=150)

if __name__ == "__main__":
    import typst
    typst.compile(str(root / "report.typ"), output=str(root / "report.pdf"), root=str(root))
