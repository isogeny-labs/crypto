"""Document layout only: plot retained fixture counts and render the report.

No field arithmetic, experiment orchestration, or result recomputation occurs
here. The Rust fixtures and development receipt are the sources of the data.
Requires matplotlib and typst. Run after DEVELOPMENT_RECEIPT.json is written.
"""
from pathlib import Path
import csv

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

HERE = Path(__file__).resolve().parent
FIXTURES = HERE.parent / "fixtures"
with (FIXTURES / "kernel_correctness_fixtures.csv").open() as source:
    rows = {row["arm"]: row for row in csv.DictReader(source)}
order = ["Aplus_cyclic", "Aminus_H0", "Aminus_H1", "Aminus_H2"]
labels = ["A+ cyclic", "A− · H0", "A− · H1", "A− · H2"]
plt.rcParams.update({
    "font.family": "DejaVu Sans", "font.size": 10,
    "svg.fonttype": "none", "axes.spines.top": False,
    "axes.spines.right": False, "axes.spines.left": False,
    "axes.edgecolor": "#CCD5DE", "text.color": "#142C42",
    "axes.labelcolor": "#142C42", "xtick.color": "#142C42",
    "ytick.color": "#53687B",
})
fig, ax = plt.subplots(figsize=(7.1, 3.0))
for offset, field, color, label in [
    (-0.19, "accepted_E_x", "#214D70", "Original target E"),
    (0.19, "accepted_E1_x", "#D08C3F", "Native base on E₁"),
]:
    positions = [i + offset for i in range(len(order))]
    values = [int(rows[key][field]) for key in order]
    bars = ax.bar(positions, values, width=0.34, color=color, label=label)
    ax.bar_label(bars, padding=3, fontsize=9)
ax.set_xticks(range(len(order)), labels)
ax.set_ylim(0, 600)
ax.set_yticks([0, 200, 400, 600])
ax.set_ylabel("Usable factor-base columns")
ax.grid(axis="y", color="#E3E9EE", linewidth=0.7)
ax.set_axisbelow(True)
ax.tick_params(axis="both", length=0)
ax.legend(loc="upper center", bbox_to_anchor=(0.5, 1.22), ncol=2, frameon=False)
fig.subplots_adjust(left=0.11, right=0.99, bottom=0.15, top=0.83)
fig.savefig(HERE / "fixed_columns.svg", bbox_inches="tight")
svg_path = HERE / "fixed_columns.svg"
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
fig.savefig(HERE / "fixed_columns.png", dpi=150, bbox_inches="tight")
plt.close(fig)
typst.compile(str(HERE / "REPORT.typ"), output=str(HERE / "Prime31_Isogeny_Comparison.pdf"), root=str(HERE.parent))
