#!/usr/bin/env python3
"""Lay out recorded values from raw/*.json as SVG + PNG; plotting only, no analysis."""
import json, glob
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
LAD = HERE.parent / "ic_m2_linear_ladder_20261024" / "raw"
C = ["#2a78d6", "#eb6834", "#1baf7a"]
INK2, GRID = "#52514e", "#e4e3df"
plt.rcParams.update({"font.size": 10, "axes.edgecolor": INK2, "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.color": GRID,
                     "figure.facecolor": "#fcfcfb", "axes.facecolor": "#fcfcfb"})

def mean(xs): return sum(xs) / len(xs)
def rows(p): return json.loads(Path(p).read_text())["rows"]

# 1. trials per rung: frozen ladder vs cycle stop (a=0 rungs, same seed and targets)
rungs = [(13, "rung_a0_n13", "cyc_a0_n13"), (19, "rung_a0_n19", "cyc_a0_n19"),
         (23, "rung_a0_n23", "cyc_a0_n23"), (41, "rung_a0_n41", "cyc_a0_n41_t0")]
n = [r[0] for r in rungs]
lad = [mean([x["ic_trials"] for x in rows(LAD / f"{r[1]}.json")]) for r in rungs]
cyc = [mean([x["ic_trials"] for x in rows(HERE / "raw" / f"{r[2]}.json")]) for r in rungs]
fig, ax = plt.subplots(figsize=(6.4, 4))
ax.semilogy(n, lad, "o-", color=C[0], label="frozen ladder (90% coverage stop)")
ax.semilogy(n, cyc, "s-", color=C[1], label="cycle stop (first equation in t)")
ax.set_xlabel("n"); ax.set_ylabel("mean decomposition trials"); ax.legend(frameon=False)
fig.tight_layout(); fig.savefig(HERE / "trials_vs_n.svg"); fig.savefig(HERE / "trials_vs_n.png", dpi=150); plt.close(fig)

# 2. wall time against d at n = 31
ds = sorted(int(Path(p).stem.split("_d")[1]) for p in glob.glob(str(HERE / "raw" / "dsweep_n31_d*.json")))
tr = [mean([x["ic_trials"] for x in rows(HERE / "raw" / f"dsweep_n31_d{d}.json")]) for d in ds]
wl = [mean([x["ic_s"] for x in rows(HERE / "raw" / f"dsweep_n31_d{d}.json")]) for d in ds]
fig, ax = plt.subplots(figsize=(6.4, 4)); ax2 = ax.twinx()
ax.semilogy(ds, tr, "o-", color=C[0], label="trials"); ax2.semilogy(ds, wl, "s-", color=C[1], label="wall s")
ax2.grid(False); ax2.spines["right"].set_visible(True)
ax.set_xlabel("factor-base dimension d (n = 31)"); ax.set_ylabel("mean trials", color=C[0]); ax2.set_ylabel("mean wall seconds", color=C[1])
ax.axvline(32 / 3, color=INK2, ls=":", lw=1)
fig.tight_layout(); fig.savefig(HERE / "d_sweep_n31.svg"); fig.savefig(HERE / "d_sweep_n31.png", dpi=150); plt.close(fig)
