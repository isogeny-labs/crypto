#!/usr/bin/env python3
"""Plot recorded complete-query counts and incomplete chart prefixes."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parent
new = json.loads((root / "evidence/factorwise_extract_v1/summary.json").read_text())
old = json.loads((root / "evidence/full_cutoff_extract_v1/summary.json").read_text())
fixtures = [(127, 4), (8191, 4), (8191, 5)]
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10, 3.5), constrained_layout=True)
for offset, data, mode, label, color in [
    (-.24, old, "FullCutoff", "Historical pushforward", "#9b9b9b"),
    (0, new, "FactorwiseCutoff", "Factorwise", "#237f6b"),
    (.24, new, "ProductionCofactorMsolve", "Cofactor + msolve", "#6887ad"),
]:
    counts = [sum(r["status"] == "exact" for r in data if (r["prime"], r["depth"]) == f and r["mode"] == mode) for f in fixtures]
    positions = [i + offset for i in range(3)]
    ax.bar(positions, [4, 4, 5], width=.22, color="#ebebeb")
    ax.bar(positions, counts, width=.22, color=color, label=label)
    for x, count, total in zip(positions, counts, [4, 4, 5]):
        ax.text(x, count + .12, f"{count}/{total}", ha="center", fontsize=8)
ax.set_xticks(range(3), ["F127\ndegree 16", "F8191\ndegree 16", "F8191\ndegree 32"])
ax.set_ylim(0, 6.6)
ax.set_ylabel("Complete queries / planned queries")
ax.set_title("Whole answers under fixed limits")
ax.legend(frameon=False, fontsize=8, loc="upper left")
partial = [r for r in new if r["mode"] == "FactorwiseCutoff" and r["depth"] == 5]
counts = [r["charts_completed"] for r in partial]
bx.bar(range(5), counts, color="#b56c43")
bx.axhline(119, color="#555555", ls="--", lw=.8)
bx.text(2, 122, "119 charts required per answer", ha="center", fontsize=8)
for x, count in enumerate(counts): bx.text(x, count + 2, str(count), ha="center")
bx.set_ylim(0, 132)
bx.set_xticks(range(5), ["17G", "43G", "89G", "152G", "Planted"])
bx.set_ylabel("Completed factorwise charts")
bx.set_title("Degree 32: every answer remains incomplete")
for a in (ax, bx): a.spines[["top", "right"]].set_visible(False)
fig.savefig(root / "factorwise-extract-counts.svg")
svg = root / "factorwise-extract-counts.svg"
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
fig.savefig(root / "factorwise-extract-counts.pdf")
