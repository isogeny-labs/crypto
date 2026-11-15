"""Graph for the K1 pair-table dimension benchmarks entry.

Plots recorded values only, from the frozen summary the IC leaderboard reads
(research/ml_cryptanalysis_pt_lgrow_k1_20261114/summary.json): gap to matched
strong signed-Frobenius rho in bits, dimension 7 (before) and the n-sized
dimension (after), at w = 0, 0.1, 1 GAE per unpriced native operation, with
the 95% intervals recorded there. No value is computed here.

    python3 -I benchmarks/20261114-k1-pair-table-dimension/plot.py
"""
import json
import pathlib

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SUMMARY = ROOT / "research/ml_cryptanalysis_pt_lgrow_k1_20261114/summary.json"
S = json.loads(SUMMARY.read_text())
WS = ("0", "0.1", "1")
plt.rcParams["svg.hashsalt"] = "k1-pair-table-dimension"

fig, axes = plt.subplots(1, len(S["rows"]), figsize=(11.0, 4.3))
for ax, row in zip(axes, S["rows"]):
    xs = range(len(WS))
    for key, lab, col, mk, off in (
        ("before", "dimension 7 (before)", "#8c8c8c", "s", -0.12),
        ("after", f"dimension {row['pair_table_dimension']} (after)", "#08519c", "o", 0.12),
    ):
        ys = [row["band"][w][f"gap_{key}_bits"] for w in WS]
        ci = [row["band"][w][f"gap_{key}_ci95"] for w in WS]
        ax.errorbar([x + off for x in xs], ys,
                    yerr=[[y - c[0] for y, c in zip(ys, ci)], [c[1] - y for y, c in zip(ys, ci)]],
                    fmt=mk, color=col, capsize=3, label=lab)
        for x, y in zip(xs, ys):
            ax.annotate(f"{y:.2f}", (x + off, y), xytext=(6 if off > 0 else -30, -3),
                        textcoords="offset points", fontsize=8, color=col)
    ax.set_xticks(list(xs))
    ax.set_xticklabels([f"w = {w}" for w in WS])
    ax.set_xlim(-0.6, len(WS) - 0.4)
    ax.set_title(f"K1 n = {row['n']}, r = 2^{row['log2_r']:.2f}\n{row['slug']}", fontsize=9)
    ax.set_ylabel("gap to matched strong rho [bits] (lower is closer)")
    ax.grid(alpha=0.3)
    ax.legend(fontsize=8, loc="upper left")
fig.suptitle("K1 Koblitz, one target, cold: pair-table dimension sized for n vs fixed dimension 7 "
             "(ecbench counted GAE, 40 verified solves per arm, 95% CI)", fontsize=9.5)
fig.text(0.5, 0.005, "Source: research/ml_cryptanalysis_pt_lgrow_k1_20261114/summary.json "
         "(validation VAL-20261114-d9190c); rho is cheaper than every arm at every w.",
         ha="center", fontsize=7.5, color="#444444")
fig.tight_layout(rect=(0, 0.03, 1, 1))
fig.savefig(HERE / "gap.svg", metadata={"Date": None})
fig.savefig(HERE / "gap.png", dpi=150)
print("wrote gap.svg, gap.png")
