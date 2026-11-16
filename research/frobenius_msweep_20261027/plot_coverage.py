"""Layout only: draws coverage.json (recorded audit values). Computes nothing.

Usage: python plot_coverage.py   (writes coverage.svg / coverage.png beside it)
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import Patch, Rectangle  # noqa: E402

here = Path(__file__).parent
cov = json.loads((here / "coverage.json").read_text())
MS = list(range(2, 21))
INK, MUTED, GRID = "#1f2328", "#6e7781", "#d0d7de"
PROOF, MEAS, PLAN = "#ddf4ff", "#0969da", "#bf8700"
plt.rcParams.update({"font.size": 8.5, "svg.fonttype": "none"})

rows = [(c["id"], c["text"], c) for c in cov["claims"]]
rows += [(g["id"], g["question"], g) for g in cov["prepared_cells"]]
fig, ax = plt.subplots(figsize=(10.5, 4.8))
for r, (rid, text, d) in enumerate(rows):
    y = len(rows) - 1 - r
    if rid.startswith("C") and d["proof_scope"] == "all m":
        ax.add_patch(Rectangle((-0.5, y - 0.42), len(MS), 0.84, color=PROOF, lw=0))
    ms = d.get("measured_m", d.get("m", []))
    col = MEAS if rid.startswith("C") else PLAN
    for m in ms:
        ax.add_patch(Rectangle((MS.index(m) - 0.42, y - 0.36), 0.84, 0.72, color=col, lw=0))
    label = f"{rid}  {text}"
    if rid.startswith("C"):
        label += f"  [{d['verdict']}]"
    ax.text(-0.8, y, label if len(label) < 100 else label[:97] + "…", ha="right", va="center", color=INK)
ax.axhline(len(cov["prepared_cells"]) - 0.5, color=MUTED, lw=0.8, ls="--")
ax.set_xlim(-0.5, len(MS) - 0.5)
ax.set_ylim(-0.6, len(rows) - 0.4)
ax.set_xticks(range(len(MS)), [str(m) for m in MS])
ax.set_yticks([])
ax.set_xlabel("m (factor-base points per decomposition)")
for x in range(len(MS)):
    ax.axvline(x - 0.5, color=GRID, lw=0.4)
for s in ("top", "right", "left"):
    ax.spines[s].set_visible(False)
ax.legend(handles=[Patch(color=PROOF, label="proved for all m (argument)"),
                   Patch(color=MEAS, label="measured (prior work)"),
                   Patch(color=PLAN, label="prepared cell, not yet run")],
          frameon=False, loc="upper center", bbox_to_anchor=(0.5, 1.12), ncol=3)
fig.suptitle("LEAD-ICB-frobenius-search-symmetry: proved, measured and prepared coverage by m\n"
             "(audit wf_3231c71e-f6b, 2026-10-26; cells G1-G6 prepared 2026-10-27, not run)", fontsize=9, y=1.08)
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(here / f"coverage.{ext}", bbox_inches="tight", **kw)
print("wrote coverage.svg/png")
