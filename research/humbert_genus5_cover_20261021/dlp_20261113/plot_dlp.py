"""Draw the figures from the CSVs written by `humbert summarize-dlp` (Rust).
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
inst, summ = rd("instances_v3.csv"), rd("summary_v3.csv")
qs = [float(r["q"]) for r in summ]

def save(fig, name):
    fig.tight_layout(); fig.savefig(os.path.join(F, name + ".svg")); fig.savefig(os.path.join(F, name + ".png"), dpi=150); plt.close(fig)

fig, ax = plt.subplots(figsize=(6.4, 3.8))
for i, (lab, col, icol, mcol) in enumerate([("A/A (pre rerun)", "#7a7a7a", "aa", "aa_med"),
                                           ("dlp:0.5 vs pre (iota + precheck)", "#2a6fb0", "pre_over_dlp50", "m_med")]):
    ax.scatter([float(r["q"]) * (1 + 0.03 * (i - 0.5)) for r in inst], [float(r[icol]) for r in inst], s=10, color=col, alpha=0.5)
    ax.plot(qs, [float(r[mcol]) for r in summ], color=col, marker="o", label=lab + " (median)")
ax.scatter([float(r["q"]) * 1.03 for r in inst], [float(r["lp50_over_dlp50"]) for r in inst], s=10, color="#d9822b", alpha=0.5)
ax.plot(qs, [float(r["lp_over_dlp_med"]) for r in summ], color="#d9822b", marker="o", label="dlp:0.5 vs lp:0.5 (median)")
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("t_ic(other) / t_ic(dlp:0.5), same instance")
ax.set_title("Double large primes: paired wall-time ratios (>1: dlp:0.5 faster), 5 seeds per q")
ax.legend(fontsize=7)
save(fig, "dlp_ratios")

fig, ax = plt.subplots(figsize=(6.4, 3.6))
for lab, col, c in [("iota + precheck", "#7a7a7a", "gap_pre"), ("lp:0.5", "#d9822b", "gap_lp50"), ("dlp:0.5", "#2a6fb0", "gap_dlp50"), ("dlp:0.35", "#3a9a5b", "gap_dlp35")]:
    ax.plot(qs, [float(r[c]) for r in summ], color=col, marker="o", label=lab)
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("IC time / expected-steps rho (median)")
ax.set_title("Distance remaining to matched rho")
ax.legend(fontsize=7)
save(fig, "dlp_gap_to_rho")

fig, ax = plt.subplots(figsize=(6.4, 3.4))
ax.scatter([float(r["q"]) for r in inst], [float(r["cand_dlp50_over_lp50"]) for r in inst], s=12, color="#2a6fb0")
ax.axhline(1, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlabel("q"); ax.set_ylabel("relation candidates dlp:0.5 / lp:0.5")
ax.set_title("R1 (counted): candidates needed, per instance (<1: dlp:0.5 needs fewer)")
save(fig, "dlp_candidates")
print("figures:", sorted(f for f in os.listdir(F) if f.endswith(".svg")))
