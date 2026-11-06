"""Lay out recorded and derived values from the cited files; computes nothing new.

Run from the figures/ directory:  python3 -I plot.py

Figure 1 (degree_vs_ell): refutation degree against factor-base dimension ell.
  Repository instruments (chained S3 system, random F2-subspace bases, b = 1):
    research/notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md Results 4-14
      plain-XL refutation degree and MutantXL fall-closure degree
    research/ic_next10_20261014/06-equivariant-f4-m4/README.md
      msolve F4 maximum step degree
    research/dreg_closure_m4l6_20261018/RESULTS.md  (m = 4, ell = 6 closure)
  Literature (Magma F4 maximum step degree, the same quantity as the closure):
    Kosters-Yeo arXiv:1503.08001 and Kousidis-Wiemers arXiv:1906.05594 (m = 2)
    Galbraith-Gebregiyorgis ePrint 2014/806 Tables 2-3 (m = 3, 4)
    Huang-Petit-Shinohara-Takagi ePrint 2015/358 Table 4 (m = 5, 6, linear S3 chain)
  Values are the typical cell reading; ranges in the README tables.

Figure 2 (arity_curve): cost against arity m at n = 131 (ECC2K-130), all derived.
    research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md section 5.3
      free-oracle floor and per-target oracle budget (log2 group operations)
    README section 1.3 of this study: the what-if rows (degree-4 closure held,
      w = 2.2, 8-bit column-touch to rho-step conversion), derived, not measured.
"""
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

# ---------------------------------------------------------------- Figure 1
# (ell, degree); None marks a lower bound (plotted hollow with a '>=' label).
repo_xl = {
    3: [(2, 5), (3, 6), (4, 6), (5, 7)],
    4: [(1, 5), (2, 6), (3, 7)],           # ell = 3 is a lower bound (>= 7)
}
repo_xl_bound = {4: [3]}
repo_closure = {
    3: [(2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (7, 5)],   # ell = 7: > 4, plotted at 5
    4: [(1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4)],
}
repo_closure_bound = {3: [7]}
repo_f4 = {
    3: [(3, 4), (4, 4), (5, 4)],
    4: [(1, 4), (2, 4), (3, 4), (4, 4)],
}
lit_f4 = {
    "m = 2, Kosters-Yeo / Kousidis-Wiemers": [(8, 3), (9, 4), (18, 4), (19, 5), (24, 5)],
    "m = 3, Galbraith-Gebregiyorgis 2014": [(5, 4), (6, 4), (7, 4), (8, 4)],
    "m = 4, Galbraith-Gebregiyorgis 2014": [(3, 4), (4, 4)],
    "m = 5, HPST 2015 (n = 13-29)": [(2, 4.2), (3, 4.5)],
    "m = 6, HPST 2015 (n = 13-29)": [(2, 4.7), (3, 5.0)],
}

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10.5, 4.0))

for m, pts in repo_xl.items():
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    ax1.plot(xs, ys, marker="s", color="#de2d26" if m == 3 else "#a50f15",
             label=f"plain XL, m = {m}")
    for ell in repo_xl_bound.get(m, []):
        y = dict(pts)[ell]
        ax1.annotate(">= 7", (ell, y), textcoords="offset points", xytext=(6, -3), fontsize=7)
for m, pts in repo_closure.items():
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    ax1.plot(xs, ys, marker="o", color="#3182bd" if m == 3 else "#08519c",
             label=f"MutantXL closure, m = {m}")
    for ell in repo_closure_bound.get(m, []):
        y = dict(pts)[ell]
        ax1.annotate("> 4", (ell, y), textcoords="offset points", xytext=(5, -9), fontsize=7)
for m, pts in repo_f4.items():
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    ax1.plot(xs, ys, marker="^", ls=":", color="#31a354" if m == 3 else "#006d2c",
             label=f"msolve F4 step degree, m = {m}")
ax1.set_xlabel("factor-base dimension ell")
ax1.set_ylabel("refutation degree")
ax1.set_title("Repository instruments (chained S3, n <= 15; 4 draws per cell)", fontsize=9)
ax1.set_xlim(0.5, 11.5)
ax1.set_xticks(range(1, 9))
ax1.set_ylim(2.5, 8)
ax1.grid(alpha=0.3)
ax1.legend(fontsize=7, loc="center right")

colors = ["#636363", "#31a354", "#006d2c", "#fd8d3c", "#e6550d"]
for (label, pts), c in zip(lit_f4.items(), colors):
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    ax2.plot(xs, ys, marker="o", color=c, label=label)
ax2.axvspan(0.5, 8, color="#deebf7", alpha=0.5, lw=0)
ax2.text(1.0, 2.7, "largest Delta = n - m*ell\nin these papers: 13-76\n(repository cells: Delta ~ 0)",
         fontsize=7, color="#08519c")
ax2.set_xlabel("factor-base dimension ell")
ax2.set_ylabel("Magma F4 maximum step degree")
ax2.set_title("Published F4 step degrees (= closure degree), by arity", fontsize=9)
ax2.set_xlim(0.5, 25)
ax2.set_ylim(2.5, 8)
ax2.grid(alpha=0.3)
ax2.legend(fontsize=7, loc="upper right")
fig.suptitle("Solving degree against factor-base dimension: repository (left) and literature (right)",
             fontsize=10)
fig.tight_layout()
fig.savefig("degree_vs_ell.svg")
fig.savefig("degree_vs_ell.png", dpi=150)

# ---------------------------------------------------------------- Figure 2
rho = 60.81
ms = [2, 3, 4, 5, 6, 8]
floor = [89.25, 68.58, 56.40, 48.44, 42.85, 35.61]        # free-oracle floor, derived
budget = [-27.86, -6.78, 5.71, 13.94, 19.77, 27.35]        # per-target oracle budget, derived
whatif_chain = {4: rho + 47, 5: rho + 35, 6: rho + 26, 8: rho + 16}   # derived what-if (README 1.3)
whatif_fghr = {4: rho + 31, 5: rho + 15, 6: rho + 4, 8: rho - 10}

fig, ax = plt.subplots(figsize=(7.4, 4.2))
ax.axhline(rho, color="#555555", ls="--", lw=0.9, label="Pollard rho with <-1> x <pi>: 2^60.81 (measured constant)")
ax.plot(ms, floor, marker="o", color="#000000", label="free-oracle floor (derived)")
ax.plot(ms, budget, marker="s", color="#756bb1", label="per-target oracle budget (derived)")
ax.plot(list(whatif_chain), list(whatif_chain.values()), marker="D", ls="-.", color="#de2d26",
        label="what-if: chained S3, degree-4 closure held (derived, unmeasured)")
ax.plot(list(whatif_fghr), list(whatif_fghr.values()), marker="D", ls="-.", color="#3182bd",
        label="what-if: FGHR direct, degree-4 held (derived; illustrative at m >= 5)")
ax.axvspan(4.5, 8.5, color="#fee8c8", alpha=0.6, lw=0)
ax.text(4.6, 95, "no algebraic degree measured\nat m >= 5 in any repository;\nliterature: ell <= 3 only",
        fontsize=7, color="#7f2704")
ax.set_xlabel("arity m (summands per relation)")
ax.set_ylabel("log2 cost at n = 131 (group operations)")
ax.set_title("ECC2K-130: cost against arity; every value derived, none measured", fontsize=10)
ax.set_xticks(ms)
ax.set_ylim(-35, 115)
ax.grid(alpha=0.3)
ax.legend(fontsize=6.5, loc="center left", bbox_to_anchor=(0.0, 0.40))
fig.tight_layout()
fig.savefig("arity_curve.svg")
fig.savefig("arity_curve.png", dpi=150)
print("wrote degree_vs_ell.{svg,png} and arity_curve.{svg,png}")
