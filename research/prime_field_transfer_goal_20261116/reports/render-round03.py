"""Document layout only: plot recorded native bit lengths, never crypto arithmetic."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

base = Path(__file__).resolve().parent
summary = json.loads((base / "../receipts/round-03/native-summary-01.json").read_text())
rows = [item["fixture"] for item in summary["fixtures"]]
plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 11,
                     "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(10, 3.8), layout="constrained")
fig.patch.set_facecolor("#f5f8f7")
ax.set_facecolor("#f5f8f7")
for offset, key, label, color in [(-.25, "r_bits", "Target field r", "#306b59"),
                                 (0, "p_bits", "Source field p", "#82ac9b"),
                                 (.25, "B_bits", "Shared integer B", "#bd8a42")]:
    positions = [i + offset for i in range(len(rows))]
    bars = ax.bar(positions, [row[key] for row in rows], width=.24,
                  label=label, color=color)
    ax.bar_label(bars, padding=3, fontsize=9)
ax.set_xticks(range(len(rows)), ["m=" + str(row["m"]) for row in rows])
ax.set_ylabel("Recorded bit length (bits)")
ax.set_ylim(0, 600)
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", color="#d8e2dc", zorder=0)
ax.set_axisbelow(True)
ax.legend(loc="upper left", ncols=3, frameon=False, fontsize=10)
ax.set_title("Six fixed CM-target fixtures · round03 qualifier01", loc="left", pad=15)
fig.savefig(base / "round-03-bits.svg")
plt.close(fig)
