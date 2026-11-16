"""Draw the figures from the CSVs written by `humbert summarize-solver` (Rust).
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
inst, summ, ph = rd("instances_v5.csv"), rd("summary_v5.csv"), rd("phases_v5.csv")
qs = [float(r["q"]) for r in summ]
ARMS = ["pre_l1", "lp50_w2", "dlpf50_w2", "dlpf50_w1", "dlpf50_l1", "lp50_l1", "dlpx50_l1"]
COL = {"pre_l1": "#7a7a7a", "lp50_w2": "#f0b27a", "dlpf50_w2": "#9fd3ae", "dlpf50_w1": "#5fb57a",
       "dlpf50_l1": "#1f7a3e", "lp50_l1": "#d9822b", "dlpx50_l1": "#8e5ab5"}


def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)


def jit(q, i, n):
    return float(q) * (1 + 0.02 * (i - (n - 1) / 2))


# S1/S2/X1: counted ratios per instance
fig, ax = plt.subplots(figsize=(6.4, 3.8))
series = [("dlpf w1 / w2 (S1, band 0.45-0.55)", "mm_w1_over_w2", COL["dlpf50_w1"]),
          ("dlpf l1 / w1 (S2, <= 0.50)", "mm_l1_over_w1", COL["dlpf50_l1"]),
          ("lp l1 / w2 (S2, <= 0.25)", "mm_lpl1_over_lpw2", COL["lp50_l1"]),
          ("dlpx / dlpf, both l1 (X1, < 1)", "mm_x_over_f", COL["dlpx50_l1"])]
for i, (lab, c, col) in enumerate(series):
    ax.scatter([jit(r["q"], i, 4) for r in inst], [float(r[c]) for r in inst], s=12, color=col, alpha=0.8, label=lab)
ax.axhspan(0.45, 0.55, color=COL["dlpf50_w1"], alpha=0.08)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("LA mulmods ratio, same instance")
ax.set_title("Counted linear-algebra work of the solver variants")
ax.legend(fontsize=7)
save(fig, "solver_mulmods")

# wall-time paired ratios
fig, ax = plt.subplots(figsize=(6.4, 3.8))
series = [("t_LA dlpf l1 / w2 (S4)", "tla_l1_over_w2", COL["dlpf50_l1"]),
          ("t_ic lp l1 / dlpf l1 (G1, >1: dlpf faster)", "t_lpl1_over_fl1", COL["lp50_l1"]),
          ("t_ic dlpf / dlpx, l1 (X2, >1: dlpx faster)", "t_f_over_x", COL["dlpx50_l1"]),
          ("t_ic dlpf w2 / dlpf l1 (old best / new)", "t_fw2_over_fl1", COL["dlpf50_w2"]),
          ("A/A dlpx / dlpx_aa", "aa_x", "black")]
for i, (lab, c, col) in enumerate(series):
    ax.scatter([jit(r["q"], i, 5) for r in inst], [float(r[c]) for r in inst], s=12, color=col, alpha=0.8, label=lab)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("paired wall-time ratio (L0)")
ax.set_title("Paired wall-time ratios, same instance (exploratory, L0)")
ax.legend(fontsize=7)
save(fig, "solver_ratios")

# gap to rho
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for a in ARMS:
    ax.plot(qs, [float(r["gap_" + a]) for r in summ], "o-", color=COL[a], label=a, ms=4)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q")
ax.set_ylabel("median t_ic / expected-steps rho")
ax.set_title("Distance remaining to rho (same instance; below 1: IC faster)")
ax.legend(fontsize=7, ncol=2)
save(fig, "solver_gap_to_rho")

# phase breakdown (stacked) at each q
fig, axs = plt.subplots(1, len(qs), figsize=(8.5, 3.6), sharey=False)
for ax, q in zip(axs if len(qs) > 1 else [axs], qs):
    rows = [r for r in ph if float(r["q"]) == q]
    tags = [r["tag"] for r in rows]
    rel = [float(r["t_relations"]) for r in rows]
    la = [float(r["t_linear_algebra"]) for r in rows]
    other = [float(r["t_ic"]) - x - y for r, x, y in zip(rows, rel, la)]
    x = range(len(rows))
    ax.bar(x, rel, color="#6fa8dc", label="relations")
    ax.bar(x, la, bottom=rel, color="#e06666", label="linear algebra")
    ax.bar(x, other, bottom=[a + b for a, b in zip(rel, la)], color="#bbbbbb", label="other")
    ax.set_xticks(list(x)); ax.set_xticklabels(tags, rotation=60, ha="right", fontsize=7)
    ax.set_title(f"q = {int(q)}"); ax.set_ylabel("median seconds (L0)")
axs[0].legend(fontsize=7)
save(fig, "solver_phases")
