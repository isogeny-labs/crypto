"""Plots values already recorded in <inst>/work_per_target.csv and <inst>/msolve_trajectories.csv. No computation of results."""
import csv, sys, os, random, statistics
os.environ.setdefault("MPLCONFIGDIR", os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", ".tmp", "mpl"))
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
matplotlib.rcParams["svg.fonttype"] = "none"
C, Fl = "#1f6f8b", "#c4562a"
def load(path):
    with open(path) as f: return list(csv.DictReader(f))
for inst in sys.argv[1:]:
    rows = load(f"{inst}/work_per_target.csv")
    fig, axes = plt.subplots(1, 2, figsize=(10, 4.4))
    for ax, col, title in zip(axes, ["work_total", "work_variable_gcd_roots"], ["total counted field multiplications per target", "variable part (gcd and root extraction) per target"]):
        random.seed(1)
        for xi, kind, colr in [(0, "crater", C), (1, "floor", Fl)]:
            sel = [r for r in rows if r["kind"] == kind]
            ys = [int(r[col]) for r in sel]; xs = [xi + random.uniform(-0.28, 0.28) for _ in ys]
            ax.scatter(xs, ys, s=7, alpha=0.55, color=colr, linewidths=0)
            med = statistics.median(ys); ax.hlines(med, xi - 0.4, xi + 0.4, color="black", lw=1.6)
            ax.text(xi + 0.42, med, f"median {med:,.0f}", va="center", fontsize=8)
        ax.set_xticks([0, 1]); ax.set_xticklabels(["crater (8 models x 32 targets)", "floor (8 models x 32 targets)"], fontsize=8)
        ax.set_xlim(-0.6, 1.95); ax.set_title(title, fontsize=9); ax.set_ylabel("field multiplications (counted)")
    fig.suptitle(f"C2 instance {inst}: same r-subgroup, same target scalars, B-matched interval bases", fontsize=10)
    fig.tight_layout(); fig.savefig(f"{inst}/graph_work_per_target.svg"); plt.close(fig)
    tp = f"{inst}/msolve_trajectories.csv"
    if os.path.exists(tp):
        tr = load(tp); cells = {}
        for r in tr: cells.setdefault(r["cell"], []).append((int(r["round"]), int(r["deg"]), int(r["rows"]) * int(r["cols"])))
        fig, axes = plt.subplots(1, 2, figsize=(10, 4.4))
        for cell, v in cells.items():
            colr = C if cell.startswith("crater") else Fl
            axes[0].plot([x[0] for x in v], [x[1] for x in v], color=colr, lw=0.8, alpha=0.7)
            axes[1].plot([x[0] for x in v], [x[2] for x in v], color=colr, lw=0.8, alpha=0.7)
        axes[0].set_title("F4 round degree (msolve -v 2)", fontsize=9); axes[1].set_title("matrix rows x columns per round", fontsize=9); axes[1].set_yscale("log")
        for ax in axes: ax.set_xlabel("F4 round index (deterministic)")
        axes[0].set_ylabel("degree of selected pairs"); axes[1].set_ylabel("rows x cols")
        axes[0].plot([], [], color=C, label="crater cells"); axes[0].plot([], [], color=Fl, label="floor cells"); axes[0].legend(fontsize=8)
        fig.suptitle(f"C2 instance {inst}: constrained S_4 system, one line per msolve cell", fontsize=10)
        fig.tight_layout(); fig.savefig(f"{inst}/graph_msolve_trajectory.svg"); plt.close(fig)
