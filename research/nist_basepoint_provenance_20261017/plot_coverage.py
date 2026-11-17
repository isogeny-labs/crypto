"""Plotting only: lays out the recorded cells of out/coverage.json (transcribed
from the out/*.out records). Computes nothing."""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch, Rectangle

here = pathlib.Path(__file__).parent
cov = json.loads((here / "out/coverage.json").read_text())
rows, tests = cov["rows"], cov["tests"]
STYLE = {"ok": ("#2a6fdb", ""), "partial": ("#9cc0f0", "//"), "blind": ("#e0782f", "xx"), "absent": ("#f2f2f2", "")}
LABEL = {"ok": "searched: no hit, positive control found",
         "partial": "searched: some layouts blind (control missed or no specificity)",
         "blind": "detector cannot see this target",
         "absent": "not applicable"}

fig, ax = plt.subplots(figsize=(9.5, 6.4))
for j, t in enumerate(tests):
    for i, cell in enumerate(t["cells"]):
        color, hatch = STYLE[cell]
        ax.add_patch(Rectangle((j, i), 0.94, 0.9, facecolor=color, hatch=hatch,
                               edgecolor="white" if not hatch else "#555", lw=0.6))
ax.set_xlim(0, len(tests)); ax.set_ylim(len(rows), 0)
ax.set_xticks([j + 0.47 for j in range(len(tests))],
              [f"{t['id']}{'†' if t['impl'] != 'native' else ''}\n{t['label']}" for t in tests], rotation=62, ha="right", fontsize=8)
ax.set_yticks([i + 0.45 for i in range(len(rows))], rows, fontsize=9)
ax.tick_params(length=0)
for s in ax.spines.values(): s.set_visible(False)
ax.set_title("Base-point and B-163 seed provenance tests (2026-10-17): no target hit wherever the detector works",
             fontsize=10.5, loc="right")
ax.legend(handles=[Patch(facecolor=STYLE[k][0], hatch=STYLE[k][1], edgecolor="#555", label=LABEL[k]) for k in STYLE],
          loc="lower left", bbox_to_anchor=(0.01, 0.02), fontsize=8.5, frameon=True, facecolor="white", edgecolor="#ccc")
fig.text(0.01, 0.01, "† computed in Python in a standalone session; sources frozen in snapshot/ as evidence (no-Python rule), native port owed",
         fontsize=7.5, color="#555")
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(here / "coverage.svg")
fig.savefig(here / "coverage.png", dpi=150)
