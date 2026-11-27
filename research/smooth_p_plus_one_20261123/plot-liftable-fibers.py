#!/usr/bin/env python3
"""Plot recorded query statuses and chart counts; no research computation."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
rows = json.loads((root / "evidence/liftable_fibers_v1/summary.json").read_text())
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10, 3.5), constrained_layout=True)
fixtures = [(127, 4, 4), (8191, 4, 4), (8191, 5, 5)]
statuses = [("exact", "Exact", "#237f6b"), ("undetermined", "Incomplete", "#be8b56"),
            ("mismatch", "Mismatch", "#b94242"), ("missing", "Not attempted", "#d2d2d2")]
for arm, offset, mode in [("L", -.18, "LiftableCutoff"), ("C", .18, "ProductionCofactorMsolve")]:
    bottom = [0, 0, 0]
    for status, label, color in statuses:
        counts = []
        for p, d, planned in fixtures:
            selected = [r for r in rows if r["prime"] == p and r["depth"] == d and r["mode"] == mode]
            counts.append(planned - len(selected) if status == "missing" else sum(r["status"] == status for r in selected))
        pos = [x + offset for x in range(3)]
        ax.bar(pos, counts, bottom=bottom, width=.32, color=color, label=label if arm == "L" else None)
        for x, y, n in zip(pos, bottom, counts):
            if n: ax.text(x, y + n/2, str(n), ha="center", va="center", fontsize=8, color="white" if status in ("exact", "mismatch") else "black")
        bottom = [a + b for a, b in zip(bottom, counts)]
    for x in range(3): ax.text(x + offset, -.28, arm, ha="center", va="top")
ax.set_xticks(range(3), ["F127 / degree 16", "F8191 / degree 16", "F8191 / degree 32"])
ax.tick_params(axis="x", pad=15, labelsize=8)
ax.set_ylim(0, 6.7)
ax.set_ylabel("Planned queries, by outcome")
ax.set_title("L: liftable fibers   C: cofactor")
ax.legend(frameon=False, ncol=2, fontsize=8, loc="upper left")
large = [r for r in rows if r["mode"] == "LiftableCutoff" and r["depth"] == 5]
counts = [r["charts_completed"] for r in large]
bx.bar(range(5), counts, color=["#237f6b" if r["status"] == "exact" else "#be8b56" for r in large])
bx.axhline(119, color="#555555", ls="--", lw=.8)
for x, n in enumerate(counts): bx.text(x, n+2, str(n), ha="center")
bx.set_xticks(range(5), ["17G", "43G", "89G", "152G", "Planted"])
bx.set_ylim(0, 138)
bx.set_ylabel("Completed charts / 119 required")
bx.set_title("Degree 32: four complete answers, one timeout")
for a in (ax, bx): a.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "liftable-fiber-outputs.pdf")
fig.savefig(root / "liftable-fiber-outputs.svg")
svg = root / "liftable-fiber-outputs.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
