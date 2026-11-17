"""Layout only: plot recorded PARI bit lengths and recorded summary counts."""
import json
from pathlib import Path
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

here = Path(__file__).resolve().parent
study = here.parent.parent
rows = json.loads((study / "results.json").read_text())
summary = json.loads((study / "summary.json").read_text())
labels = {
    "p256-4p-minus1": "P-256, shared: 4p−1",
    "published_224-twist2": "P-224, published: twist₂",
    "published_256-6n-plus1": "P-256, published: 6n+1",
    "published_256-7n-minus1": "P-256, published: 7n−1",
    "published_256-twist2": "P-256, published: twist₂",
    "skipped_224_1485-conductor4": "P-224, block 1485: conductor₄ term",
    "skipped_224_5736-twist2": "P-224, block 5736: twist₂",
    "skipped_256_876-4n-minus1": "P-256, block 876: 4n−1",
    "skipped_256_876-twist2": "P-256, block 876: twist₂",
}
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(11, 8.3))
ys = list(range(len(rows)))
series = [
    ("target_bits", -0.23, "Target integer", "#c9d2db"),
    ("largest_proven_prime_bits", 0, "Largest proven prime factor", "#176e9c"),
    ("largest_residual_bits", 0.23, "Remaining composite cofactor", "#d8732e"),
]
for key, shift, label, color in series:
    values = [r["verification"][key] for r in rows]
    bars = ax.barh([y + shift for y in ys], values, height=0.21, label=label, color=color)
    for bar, value in zip(bars, values):
        if value:
            ax.text(value + 4, bar.get_y() + bar.get_height() / 2, str(value), va="center", fontsize=8)
ax.set_yticks(ys, [labels[r["id"]] for r in rows])
ax.invert_yaxis()
ax.set_xlim(0, 570)
ax.set_xlabel("Integer size (bits)")
ax.set_title("Six complete factorizations; three verified composite residuals", loc="left", pad=35)
ax.legend(loc="lower left", bbox_to_anchor=(0, 1.015), ncol=3, frameon=False, fontsize=8)
ax.grid(axis="x", alpha=0.2)
ax.set_axisbelow(True)
ax.spines[["top", "right", "left"]].set_visible(False)
fig.text(0.02, 0.015, "Run nist-factor-continuation-20261021T1424Z · nine fixed integers, one partition each.\n"
         "Exact bit lengths from results.json; no sampling uncertainty. A composite cofactor is not a prime factor.", fontsize=8)
fig.subplots_adjust(left=0.36, right=0.96, bottom=0.1, top=0.87)
fig.savefig(here / "factor-sizes.svg", metadata={"Date": None})
fig.savefig(here / "factor-sizes.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(11, 2.7))
ax.set_xlim(-0.15, 11.1)
ax.set_ylim(0, 2.7)
ax.axis("off")
boxes = [
    (0.1, "Original DiSSECT\n438 cases\n10 timeout rows / 14 fields"),
    (2.9, "Exact hex reconstruction\n9 distinct integers\nNative PARI + GMP-ECM"),
    (5.7, "Independent replay\n9 exact products\nPrime certificates + Lean"),
    (8.5, f"{summary['complete_targets']} complete / {summary['partial_targets']} partial\n"
          f"{summary['resolved_fields']} fields resolved\n{summary['unresolved_fields']} fields still partial"),
]
for x, label in boxes:
    ax.add_patch(FancyBboxPatch((x, 0.85), 2.35, 1.2, boxstyle="round,pad=0.12", facecolor="#edf4f8", edgecolor="#176e9c"))
    ax.text(x + 1.175, 1.45, label, ha="center", va="center", fontsize=9)
for x in [2.5, 5.3, 8.1]:
    ax.annotate("", xy=(x + 0.3, 1.45), xytext=(x, 1.45), arrowprops={"arrowstyle": "->", "color": "#176e9c"})
ax.text(0.05, 0.15, "Different denominators: cases → timeout rows/fields → distinct arithmetic targets.\n"
        "This supplement preserves original timeouts; it does not establish a historical skip reason.", fontsize=9)
fig.tight_layout()
fig.savefig(here / "evidence-flow.svg", metadata={"Date": None})
fig.savefig(here / "evidence-flow.png", dpi=150)
plt.close(fig)
