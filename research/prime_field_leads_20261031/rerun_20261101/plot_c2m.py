"""Plots values already recorded in C2_M/msolve_trajectories.csv. No computation of results.
Left: step degree per F4 round; right: matrix cells (rows x cols) per round; decision cells
(models 0-3) solid, models 4-7 thin; the dashed line marks round 95, the common prefix of the
original 420 s run."""
import csv, os, sys
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
matplotlib.rcParams["svg.fonttype"] = "none"
H = os.path.dirname(os.path.abspath(__file__))
cells = {}
for r in csv.DictReader(open(f"{H}/C2_M/msolve_trajectories.csv")):
    cells.setdefault(r["cell"], []).append((int(r["round"]), int(r["deg"]), int(r["rows"]) * int(r["cols"])))
col = {"crater": "#1f6f8b", "floor": "#c4562a"}
fig, ax = plt.subplots(1, 2, figsize=(11, 4.4))
seen = set()
for cell, v in sorted(cells.items()):
    side, model = cell.split("_")[0], int(cell.split("_")[1])
    dec = model < 4
    lab = f"{side} (models 0-3)" if dec and side not in seen else None
    if lab: seen.add(side)
    kw = dict(color=col[side], lw=1.1 if dec else 0.5, alpha=0.8 if dec else 0.35, label=lab)
    ax[0].plot([x for x, _, _ in v], [d for _, d, _ in v], **kw)
    ax[1].plot([x for x, _, _ in v], [c for _, _, c in v], **kw)
for a in ax: a.axvline(95, ls="--", color="grey", lw=0.8); a.set_xlabel("F4 round"); a.grid(alpha=0.3)
ax[0].set_ylabel("step degree"); ax[1].set_ylabel("matrix cells (rows x cols)"); ax[1].set_yscale("log")
ax[0].set_title("Step degree per round (all 32 cells; dashed: original 95-round prefix)", fontsize=8)
ax[1].set_title("Matrix size per round", fontsize=9); ax[0].legend(fontsize=8)
fig.suptitle("C2 rerun, M instance (p = 932413, B = 47): msolve -l 1, cap 3600 s, every cell completed", fontsize=10)
fig.tight_layout(); fig.savefig(f"{H}/C2_M/graph_msolve_trajectory.svg")
