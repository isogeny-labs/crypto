"""Draw the figures from the CSVs written by `humbert summarize-lp .` (Rust).
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

def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)

def panel(inst, summ, series, title, ylabel, name, logy=False):
    fig, ax = plt.subplots(figsize=(6.4, 3.8))
    qs = [float(r["q"]) for r in summ]
    for i, (lab, col, icol, mcol) in enumerate(series):
        ax.scatter([float(r["q"]) * (1 + 0.03 * (i - 1)) for r in inst], [float(r[icol]) for r in inst], s=10, color=col, alpha=0.5)
        ax.plot(qs, [float(r[mcol]) for r in summ], color=col, marker="o", label=lab + " (median)")
    ax.axhline(1, color="black", lw=0.8, ls="--")
    ax.set_xscale("log")
    if logy:
        ax.set_yscale("log")
    ax.set_xlabel("q"); ax.set_ylabel(ylabel); ax.set_title(title); ax.legend(fontsize=7)
    save(fig, name)

i1, s1, i2, s2 = rd("instances_v1.csv"), rd("summary_v1.csv"), rd("instances_v2.csv"), rd("summary_v2.csv")
panel(i1, s1, [("A/A (iota rerun)", "#7a7a7a", "aa", "aa_med"), ("lp:0.5", "#2a6fb0", "lp50", "lp50_med"), ("lp:0.35", "#d9822b", "lp35", "lp35_med")],
      "v1: single large prime vs iota baseline (>1: arm faster), 5 seeds per q", "t_ic(iota) / t_ic(arm), same instance", "lp_speedup")
panel(i2, s2, [("A/A (pre rerun)", "#7a7a7a", "aa", "aa_med"), ("M1: precheck vs v1 retry", "#3a9a5b", "m1", "m1_med"), ("M2: lp:0.5 vs fixed baseline", "#2a6fb0", "m2", "m2_med")],
      "v2: LA precheck and large primes, 5 seeds per q", "time ratio, same instance (>1: second arm faster)", "v2_ratios", logy=True)
panel(i2, s2, [("iota, v1 retry", "#7a7a7a", "gap_v1", "gap_v1"), ("iota + precheck", "#3a9a5b", "gap_pre", "gap_pre"), ("lp:0.5 + precheck", "#2a6fb0", "gap_lp50pre", "gap_lp")],
      "Distance remaining to matched rho", "IC time / expected-steps rho", "v2_gap_to_rho", logy=True)
print("figures:", sorted(f for f in os.listdir(F) if f.endswith(".svg")))
