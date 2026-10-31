"""Render native summary values; all arithmetic is in the Rust summarizer."""
import json
import math
import pathlib
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = pathlib.Path(__file__).resolve().parent
rows = json.loads(pathlib.Path(sys.argv[1]).read_text())["curves"]
plt.rcParams["svg.hashsalt"] = "quartic-mixed-lines-20261022"
plt.rcParams["svg.fonttype"] = "none"

x = [math.log2(r["r"]) for r in rows]
fig, (a, b) = plt.subplots(1, 2, figsize=(11.8, 4.1), layout="constrained")
a.plot(x, [r["shared_yield_ratio_median"] for r in rows], "o-", color="tab:blue", label="Hit yield, same targets")
a.plot(x, [r["query_ratio_median"] for r in rows], "s-", color="tab:orange", label="Queries to twist24 solve")
a.axhline(1, color="0.45", ls="--", lw=1)
a.set(xlabel="Prime subgroup size (log₂ r)", ylabel="L0∪L2 / L0 yield; L0 / L0∪L2 queries",
      title="Relation yield and collection stop")
a.grid(alpha=.2)
a.legend(fontsize=8)

y = [r["m_ratio_median"] for r in rows]
lo = [r["m_ratio_median"]-r["m_ratio_min"] for r in rows]
hi = [r["m_ratio_max"]-r["m_ratio_median"] for r in rows]
b.errorbar(x, y, yerr=[lo, hi], fmt="D-", color="tab:green", capsize=3,
           label="Counted Fp multiplies")
b.plot(x, [r["la_ratio_median"] for r in rows], "^-", color="tab:purple",
       label="Modular row operations")
b.axhline(1, color="0.45", ls="--", lw=1)
b.set(xlabel="Prime subgroup size (log₂ r)", ylabel="L0 / L0∪L2 count",
      title="Separate cost units; 3 seeds per curve")
b.grid(alpha=.2)
b.legend(fontsize=8)
for xx, yy, r in zip(x,y,rows):
    b.annotate(str(r["p"]), (xx, yy), xytext=(0, 7), textcoords="offset points",
               ha="center", fontsize=7)

svg = root / "quartic_mixed_lines_v1.svg"
fig.savefig(svg)
fig.savefig(root / "quartic_mixed_lines_v1.png", dpi=150)
plt.close(fig)
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
