"""Lay out the Callgrind counts in data/ir.tsv (plotting only: the per-GAE
quotients are the only arithmetic)."""
import csv
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
(HERE / "figures").mkdir(exist_ok=True)
with (HERE / "data" / "ir.tsv").open() as f:
    rows = list(csv.DictReader(f, delimiter="\t"))
labels = [r["child"].split(" (")[0].replace(", oracle ", "\n") for r in rows]
m = [int(r["main_ir"]) / float(r["gae"]) for r in rows]
c = [int(r["candidate_ir"]) / float(r["gae"]) for r in rows]
fig, ax = plt.subplots(figsize=(11, 4.4))
y = range(len(rows))
ax.barh([i + 0.2 for i in y], m, 0.4, color="#777777", label="main c13f8397ff")
ax.barh([i - 0.2 for i in y], c, 0.4, color="#2f6db3", label="this change")
for i, (a, b) in enumerate(zip(m, c)):
    ax.text(b + 8, i - 0.2, f"{b:.0f} ({b / a - 1:+.1%})", va="center", fontsize=8)
    ax.text(a + 8, i + 0.2, f"{a:.0f}", va="center", fontsize=8, color="#555555")
ax.axvline(m[0] / 2, color="#c0392b", ls="--", lw=1)
ax.text(m[0] / 2 + 5, -0.6, "half of the default arm on main", color="#c0392b", fontsize=7)
ax.set_yticks(list(y))
ax.set_yticklabels(labels, fontsize=7)
ax.invert_yaxis()
ax.set_xlabel("Callgrind instructions per GAE (counted cost identical in every pair)", fontsize=8)
ax.legend(fontsize=8, loc="upper right")
ax.set_title("ecbench index calculus: x86-64 Euclid inversion and 32-bit product; records identical", fontsize=9)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(HERE / "figures" / f"ir_per_gae.{ext}", bbox_inches="tight", **kw)
