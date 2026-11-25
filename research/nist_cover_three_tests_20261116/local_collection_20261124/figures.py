"""Render recorded native results and clocks only; no cryptographic arithmetic."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
d = json.loads((root / "RESULTS.json").read_text())
native = json.loads((root / "evidence/instrumented-v1/result.json").read_text())
plt.rcParams.update({"svg.fonttype": "none", "font.size": 11})
fig, ax = plt.subplots(figsize=(7.3, 2.7), constrained_layout=True)
labels = ["Original\nrank", "Expanded\nfull rank", "New-column\nrank", "Closed\nold-base rank"]
values = [d["rank"]["baseline_rank"], d["rank"]["full_rank"], d["rank"]["new_rank"], d["rank"]["closed_rank"]]
bars = ax.bar(labels, values, color=["#8596A7", "#4C78A8", "#59A14F", "#E6B85C"], width=0.57)
for bar, value in zip(bars, values):
    ax.text(bar.get_x()+bar.get_width()/2, value, str(value), ha="center", va="bottom")
ax.set_ylim(0, 124)
ax.set_ylabel("Rank over the subgroup field")
ax.set_title("Actual local P-192 collection: no added closed rank")
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "ranks.svg")
fig.savefig(root / "ranks.pdf")
plt.close(fig)

phases = d["instrumented_phase_us"]
names = list(phases)
values = [phases[n]/1000000 for n in names]  # Presentation-only unit conversion.
fig, ax = plt.subplots(figsize=(7.5, 3.8), constrained_layout=True)
ax.barh([n.removesuffix("_us").replace("_", " ") for n in names], values, color="#4C78A8")
ax.set_xscale("log")
ax.set_xlabel("Recorded wall seconds (log scale)")
ax.set_title("Instrumented collector subphases", fontsize=11)
ax.invert_yaxis()
ax.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "phases.svg")
fig.savefig(root / "phases.pdf")
plt.close(fig)

lines = ['digraph incidence {', 'graph [rankdir=LR,bgcolor="white",fontname="Helvetica",label="Actual private columns: first two added rows shown",labelloc=t];', 'node [shape=box,style="rounded,filled",fillcolor="#EDF2F7",fontname="Helvetica"];']
shown = native["private_columns"][:4]
for owner, col, coefficient in shown:
    lines.append(f'r{owner} [label="Added row {owner}"];')
    lines.append(f'c{col} [label="New column {col}",fillcolor="#DDEFE5"];')
    lines.append(f'r{owner} -> c{col} [label="{coefficient:+d}"];')
lines += ['note [label="Complete certificate: every new column has one owner",fillcolor="#FFF0CD"];', '}']
(root / "incidence.dot").write_text("\n".join(lines)+"\n")
