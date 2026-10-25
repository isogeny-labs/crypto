"""Render recorded native counters; no factorization or research computation."""
import json
from datetime import datetime
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.dates as dates
import matplotlib.pyplot as plt

here = Path(__file__).resolve().parent
rows = json.loads((here / "relations.json").read_text())
plt.rcParams["svg.fonttype"] = "none"
fig, ax = plt.subplots(figsize=(8.5, 3), layout="constrained")
ax.plot([datetime.fromisoformat(r["utc_date"] + "T" + r["utc_time"]) for r in rows],
        [r["raw_relations"] for r in rows], color="#2457a6")
ax.xaxis.set_major_formatter(dates.DateFormatter("%H:%M"))
ax.ticklabel_format(axis="y", style="plain")
ax.set(xlabel="2026-10-23 UTC (native log)", ylabel="Per-allocation raw relation count",
       title="R06: 1,427,642 new raw relations before clean bounded stop")
ax.grid(alpha=.2)
fig.savefig(here / "relations.svg")
p = here / "relations.svg"
p.write_text("\n".join(s.rstrip() for s in p.read_text().splitlines()) + "\n")
