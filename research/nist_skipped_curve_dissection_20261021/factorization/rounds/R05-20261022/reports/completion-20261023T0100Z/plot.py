"""Plot recorded native counters only; no cryptographic arithmetic or orchestration."""
import json
from datetime import datetime
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.dates as mdates
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
rows = json.loads((here / "sieving-progress.json").read_text())
times = [datetime.fromisoformat(r["utc_date"] + "T" + r["utc_time"]) for r in rows]
counts = [r["raw_relations"] for r in rows]
fig, ax = plt.subplots(figsize=(8.5, 3.5), layout="constrained")
ax.plot(times, counts, color="#2457a6", linewidth=1.8)
ax.scatter([times[-1]], [counts[-1]], color="#a33131", zorder=3)
ax.annotate("3,164,757 main-file raw relations\nExit 139; no new factor",
            (times[-1], counts[-1]), xytext=(-12, -40),
            textcoords="offset points", ha="right", fontsize=9)
ax.xaxis.set_major_formatter(mdates.DateFormatter("%H:%M"))
ax.ticklabel_format(axis="y", style="plain")
ax.set(xlabel="2026-10-22 UTC (native log timestamps)",
       ylabel="Reported main-file raw relations (count)",
       title="R05: retained relation collection before native failure")
ax.grid(alpha=0.22)
fig.suptitle("One allocation; no duplicate/rank check or replicate interval", fontsize=9)
fig.savefig(here / "sieving-progress.svg")
svg = here / "sieving-progress.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(here / "sieving-progress.png", dpi=150)
