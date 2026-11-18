"""Draw the figures from the CSVs written by `humbert summarize-filter` (Rust).
Plotting only: every plotted number is read from those files."""
import csv, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(HERE, "results")
F = os.path.join(HERE, "figures"); os.makedirs(F, exist_ok=True)
plt.rcParams.update({"font.size": 9, "axes.grid": True, "grid.alpha": 0.3})
rd = lambda name: list(csv.DictReader(open(os.path.join(R, name))))
inst, summ, ph = rd("instances_v4.csv"), rd("summary_v4.csv"), rd("phases_v4.csv")
qs = [float(r["q"]) for r in summ]
COL = {"pre": "#7a7a7a", "lp50": "#d9822b", "dlp50": "#2a6fb0", "dlpm50": "#8e5ab5", "dlpf50": "#3a9a5b"}


def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)


def jit(q, i, n):
    return float(q) * (1 + 0.025 * (i - (n - 1) / 2))


fig, ax = plt.subplots(figsize=(6.4, 3.8))
series = [("dlpf / dlp (full elimination)", "mm_f_over_dlp", COL["dlp50"]),
          ("dlpf / dlpm (no merges)", "mm_f_over_m", COL["dlpm50"]),
          ("dlpf / lp:0.5", "mm_f_over_lp", COL["lp50"])]
for i, (lab, c, col) in enumerate(series):
    ax.scatter([jit(r["q"], i, 3) for r in inst], [float(r[c]) for r in inst], s=12, color=col, alpha=0.7, label=lab)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("LA mulmods ratio, same instance")
ax.set_title("F1/F2 (counted): Wiedemann work of the filtered matrix (<1: dlpf cheaper)")
ax.legend(fontsize=7)
save(fig, "filter_mulmods")

fig, ax = plt.subplots(figsize=(6.4, 3.8))
series = [("lp:0.5 / dlpf", "t_lp_over_f", COL["lp50"]), ("pre / dlpf", "t_pre_over_f", COL["pre"]),
          ("dlp / dlpf", "t_dlp_over_f", COL["dlp50"]), ("A/A dlpf / dlpf_aa", "aa", "black")]
for i, (lab, c, col) in enumerate(series):
    ax.scatter([jit(r["q"], i, 4) for r in inst], [float(r[c]) for r in inst], s=12, color=col, alpha=0.7, label=lab)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("t_ic(other) / t_ic(dlpf), same instance")
ax.set_title("F3 (wall time, L0): paired ratios (>1: dlpf faster), 5 seeds per q")
ax.legend(fontsize=7)
save(fig, "filter_ratios")

fig, ax = plt.subplots(figsize=(6.4, 3.6))
for tag, c in [("pre", "gap_pre"), ("lp50", "gap_lp50"), ("dlp50", "gap_dlp50"), ("dlpm50", "gap_dlpm50"), ("dlpf50", "gap_dlpf50")]:
    ax.plot(qs, [float(r[c]) for r in summ], color=COL[tag], marker="o", label=tag)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("IC time / expected-steps rho (median)")
ax.set_title("F4: distance remaining to matched rho")
ax.legend(fontsize=7)
save(fig, "filter_gap_to_rho")

fig, axs = plt.subplots(1, 3, figsize=(9.6, 3.3))
for tag in ["pre", "lp50", "dlp50", "dlpm50", "dlpf50"]:
    rows = [r for r in ph if r["tag"] == tag]
    x = [float(r["q"]) for r in rows]
    for a, k in zip(axs, ["la_cols", "la_nnz", "la_mulmods"]):
        a.plot(x, [float(r[k]) for r in rows], marker="o", color=COL[tag], label=tag)
for a, yl in zip(axs, ["matrix columns", "matrix non-zeros", "LA mulmods"]):
    a.set_xscale("log"); a.set_yscale("log"); a.set_xlabel("q"); a.set_ylabel(yl + " (median)")
axs[0].legend(fontsize=7)
fig.suptitle("Matrix shape per arm: dimension, weight and Wiedemann work", fontsize=9)
save(fig, "filter_matrix")
print("figures:", sorted(f for f in os.listdir(F) if f.endswith(".svg")))
