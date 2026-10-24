#!/usr/bin/env python3
"""Plot retained per-process values without estimating a new result."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
rows = json.loads((HERE / "../../ic1_rows_38978215742.json").read_text())["rows"]
ks = [int(row["arm"].split("_")[0][1:]) for row in rows]
colors = ["#b85b20" if "_" in row["arm"] else "#176b87" for row in rows]
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
fig, axes = plt.subplots(2, 1, figsize=(9.5, 5.6), layout="constrained", sharex=True)
for ax, key, scale, label in [
    (axes[0], "cold_process_ms", 1000, "Complete fresh-process IC interval (s)"),
    (axes[1], "one_target_online_ms", 1, "Recorded target-dependent stage (ms)"),
]:
    ax.scatter(ks, [row[key] / scale for row in rows], c=colors, s=36, zorder=3)
    ax.set_ylabel(label, fontsize=10)
    ax.grid(color="#d9e0e5", linewidth=0.6)
    ax.spines[["top", "right"]].set_visible(False)
axes[0].annotate("K1107: 116.222 s", (1107, 116.222192149), xytext=(1108, 117),
                 arrowprops={"arrowstyle": "-", "color": "#176b87"}, fontsize=10)
axes[0].set_ylim(115.5, 123.5)
axes[1].set_ylim(0, 540)
axes[1].set_xticks(range(1102, 1121, 2))
axes[1].set_xlabel("K (folded columns); every integer from 1102 through 1120 was measured")
fig.suptitle("N83 exposed-point adjacent-K screen · run 38978215742", fontsize=13)
fig.savefig(HERE / "observations.svg")
fig.savefig(HERE / "observations.png", dpi=150)

