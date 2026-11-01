"""Render metrics computed by the native quartic_twist24_summary example."""
import json
import pathlib
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

root = pathlib.Path(__file__).resolve().parent
rows = json.loads(pathlib.Path(sys.argv[1]).read_text())["curves"]
plt.rcParams["svg.hashsalt"] = "quartic-twist24-20261022"
plt.rcParams["svg.fonttype"] = "none"

fig, (a, b) = plt.subplots(1, 2, figsize=(11.5, 4.2), layout="constrained")
x = [r["log2_r"] for r in rows]
for key, label, color, marker in [
    ("residual_ratio", "Residual queries", "tab:blue", "o"),
    ("la_ratio", "Modular row work", "tab:purple", "s"),
    ("m_ratio", "Counted Fp multiplies", "tab:green", "^"),
]:
    y = [r[key + "_median"] for r in rows]
    lo = [r[key + "_median"] - r[key + "_min"] for r in rows]
    hi = [r[key + "_max"] - r[key + "_median"] for r in rows]
    a.errorbar(x, y, yerr=[lo, hi], label=label, color=color,
               marker=marker, lw=1.6, capsize=2.5, ms=5)
a.axhline(1, color="0.5", lw=1, ls="--")
a.set(xlabel="Prime subgroup size (log₂ r)", ylabel="Cubic-only / twist24 count",
      title="Same physical factor base")
a.set_ylim(bottom=0)
a.grid(alpha=.2)
a.legend(fontsize=8, loc="upper left")

y = [r["twist24_to_rho_median"] for r in rows]
lo = [r["twist24_to_rho_median"] - r["gap_min"] for r in rows]
hi = [r["gap_max"] - r["twist24_to_rho_median"] for r in rows]
b.errorbar(x, y, yerr=[lo, hi], color="tab:red", marker="D",
           lw=1.6, capsize=2.5, ms=5)
b.axhline(1, color="0.5", lw=1, ls="--")
b.set_yscale("log")
b.set(xlabel="Prime subgroup size (log₂ r)",
      ylabel="Twist24 IC / matched twist24 rho (Fp multiplies)",
      title="One-target counted-multiplication gap to rho")
b.grid(alpha=.2, which="both")
for xx, yy, r in zip(x, y, rows):
    b.annotate(str(r["p"]), (xx, yy), xytext=(0, 7),
               textcoords="offset points", ha="center", fontsize=7)

svg = root / "quartic_twist24_cost_v2.svg"
fig.savefig(svg)
fig.savefig(root / "quartic_twist24_cost_v2.png", dpi=150)
plt.close(fig)
svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
