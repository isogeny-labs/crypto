# Report rendering of already recorded counters; no cryptographic computation.
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
root = Path(__file__).parent
record = json.loads((root / "counts.json").read_text())
plt.rcParams.update({"font.family": "DejaVu Sans", "svg.fonttype": "none"})
fig, ax = plt.subplots(figsize=(7.0, 3.6), layout="constrained")
x = list(range(len(record["degree"])))
for shift, key, label, color in [(-.19,"input_rows","Input multiples","#365f7c"),(.19,"recorded_pivots","Recorded pivots","#4d8e76")]:
    bars = ax.bar([i+shift for i in x],record[key],width=.36,label=label,color=color)
    ax.bar_label(bars,padding=3,fontsize=9)
ax.set_xticks(x,[str(i) for i in record["degree"]])
ax.set_xlabel("Degree cap D")
ax.set_ylabel("Recorded count")
ax.set_title("Fixed F17 norm input: exact counters, no timing claim")
ax.spines[["top","right"]].set_visible(False)
ax.set_ylim(0,7800)
ax.legend(frameon=False,loc="upper left")
fig.savefig(root / "counts.svg")
svg = root / "counts.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
