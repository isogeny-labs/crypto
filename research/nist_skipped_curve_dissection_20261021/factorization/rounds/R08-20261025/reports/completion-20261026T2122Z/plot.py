"""Display recorded counters only; no research computation or inference."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

here = Path(__file__).resolve().parent
data = json.loads((here / "counts.json").read_text())
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10, "svg.fonttype": "none"})
fig, (ax, flow) = plt.subplots(2, 1, figsize=(10, 5), gridspec_kw={"height_ratios": [2, 1.4]})
rows = data["raw_checkpoints"]
bars = ax.barh([r["id"] for r in rows], [r["raw_lines"] for r in rows], color=["#94a3b8", "#64748b", "#2563eb"])
ax.bar_label(bars, labels=[f'{r["raw_lines"]:,}' for r in rows], padding=5, fontsize=10)
ax.set_xlim(0, 12500000)
ax.xaxis.set_major_formatter(FuncFormatter(lambda v, _: f"{v/1e6:g}M"))
ax.set_xlabel("Raw relation rows in the saved collection file")
ax.set_title("Completed checkpoints: more raw data; no new factor", loc="left", weight="bold")
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="x", alpha=.18)
ax.set_axisbelow(True)
flow.axis("off")
flow.set_xlim(0, 1); flow.set_ylim(0, 1)
flow.text(0, .97, "R07 filtered the older R06 file — not the R08 file", weight="bold", va="top")
for x, label in [(.15, "4,592,399\nraw input"), (.50, "4,518,304\nunique incl. free rows"), (.85, "92\nsingleton survivors")]:
    flow.text(x, .48, label, ha="center", va="center", bbox={"boxstyle": "round,pad=.55", "fc": "#eff6ff", "ec": "#94a3b8"})
flow.annotate("", xy=(.35,.48), xytext=(.27,.48), arrowprops={"arrowstyle":"->", "color":"#64748b"})
flow.annotate("", xy=(.76,.48), xytext=(.65,.48), arrowprops={"arrowstyle":"->", "color":"#64748b"})
flow.text(.5, .04, "R08 filtering: NOT RUN • raw counts do not establish matrix rank or percentage complete", ha="center", fontsize=9)
fig.tight_layout(pad=1.1)
fig.savefig(here / "counts.svg", bbox_inches="tight")
