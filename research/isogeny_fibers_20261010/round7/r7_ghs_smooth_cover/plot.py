"""Plots recorded values only: relrate_descended.json (this study) and ../../round6/r6_9_ghs_2/relrate.json
(R6.9 stand-in curves) -> fig1_*.svg/png, fig2_*.svg/png, figure_data.tsv.  No result is computed here."""
import json, os, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = os.path.dirname(os.path.abspath(__file__))
mine = json.load(open(os.path.join(D, "relrate_descended.json")))
r69 = json.load(open(os.path.join(D, "..", "..", "round6", "r6_9_ghs_2", "relrate.json")))
rows = [c for run in mine["runs"] for c in run["curves"] if "error" not in c]
tsv = ["# descended: l\tn\tN\tbeta\tgenus\tjacobian_order\tJ0_size\texact_split_in_J0\tmeasured_A\tse_A\tH_order\texact_split_in_H\tmeasured_B\tse_B\tinv_gfact"]
for c in rows:
    A, B, J = c["rate_A_walk_on_J0"], c["rate_B_subgroup_H"], c["subgroup_generated_by_points_and_images"]
    tsv.append("\t".join(str(x) for x in [c["l"], c["n"], c["N"], hex(c["beta"]), c["genus"], c["jacobian_order"], J["size"],
        J["exact_split_fraction"], A["measured_rate"], A["measured_rate_se"], B["H_order"], B["exact_split_fraction_in_H"],
        B["measured_rate"], B["measured_rate_se"], c["model_heuristic_inv_gfact"]]))
tsv.append("# R6.9 stand-in (random imaginary curves, q=4, g=8): seed\tjacobian_order\tmeasured\tse\texact_split\tinv_gfact")
for r in r69:
    if r["l"] == 2 and r["genus"] == 8:
        tsv.append("\t".join(str(x) for x in [r["seed"], r["jacobian_order_exact"], r["measured_rate"], r["measured_rate_se"], r["model_exact_split_fraction"], r["model_heuristic_inv_gfact"]]))
open(os.path.join(D, "figure_data.tsv"), "w").write("\n".join(tsv) + "\n")

def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(D, name + ".svg"))
    fig.savefig(os.path.join(D, name + ".png"), dpi=150)
    plt.close(fig)

# fig1: measured vs exact split fraction, real descended Jacobian vs R6.9 stand-ins (q=4, g=8 only)
fig, ax = plt.subplots(figsize=(6.6, 4.8))
lo, hi = 3e-4, 5e-2
g = [lo, hi]
ax.plot(g, g, "k-", lw=1, label="measured = exact")
ax.plot(g, [4 * v for v in g], "k:", lw=0.8, label="4x band"); ax.plot(g, [v / 4 for v in g], "k:", lw=0.8)
st = [r for r in r69 if r["l"] == 2 and r["genus"] == 8]
for i, r in enumerate(st):
    ax.errorbar(r["model_exact_split_fraction"], r["measured_rate"], yerr=r["measured_rate_se"], fmt="s", mfc="none", color="C1", ms=7,
                label="R6.9 stand-in curves (random, q=4, g=8; N=12 row)" if i == 0 else None)
sel = [c for c in rows if c["l"] == 2 and c["genus"] == 8]
style = {6: ("C0", "descended, N=12 (l,n)=(2,6), walk in J0"), 7: ("C4", "descended, N=14 (l,n)=(2,7), walk in J0")}
seen = set()
for c in sel:
    col, lab = style[c["n"]]
    A = c["rate_A_walk_on_J0"]
    ax.errorbar(c["subgroup_generated_by_points_and_images"]["exact_split_fraction"], A["measured_rate"], yerr=A["measured_rate_se"], fmt="o", color=col, ms=6,
                label=lab if c["n"] not in seen else None)
    seen.add(c["n"])
for i, c in enumerate(sel):
    B = c["rate_B_subgroup_H"]
    ax.errorbar(B["exact_split_fraction_in_H"], B["measured_rate"], yerr=B["measured_rate_se"], fmt="^", color="C2", ms=5, alpha=0.8,
                label="descended, walk in H=<Phi(E(K))> (both N)" if i == 0 else None)
h = sel[0]["model_heuristic_inv_gfact"]
ax.axvline(h, color="C3", ls="--", lw=1); ax.text(h * 1.08, 1.0e-3, "1/g!\n(g=8)", color="C3", fontsize=7)
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlim(2e-5, hi); ax.set_ylim(lo, hi)
ax.set_xlabel("exact split-divisor fraction (enumerated, same group as the walk)"); ax.set_ylabel("measured fully-split rate along the walk")
ax.set_title("R7.1 relation rate at q=4, g=8: descended Jacobians vs R6.9 stand-ins\n(error bars: binomial s.e. of the walk; 100000 steps per walk)", fontsize=9)
ax.legend(fontsize=6.3, loc="upper left"); save(fig, "fig1_relrate_descended_vs_standin")

# fig2: ratio to 1/g!, per curve, grouped
fig, ax = plt.subplots(figsize=(6.6, 4.2))
groups = [("R6.9 stand-ins\n(q=4, g=8)", [r["measured_rate"] / r["model_heuristic_inv_gfact"] for r in st], "s", "C1", True),
          ("descended N=12\n(2,6), g=8", [c["rate_A_walk_on_J0"]["measured_rate"] / c["model_heuristic_inv_gfact"] for c in sel if c["n"] == 6], "o", "C0", False),
          ("descended N=14\n(2,7), g=8", [c["rate_A_walk_on_J0"]["measured_rate"] / c["model_heuristic_inv_gfact"] for c in sel if c["n"] == 7], "o", "C4", False)]
for gi, (lab, vals, mk, col, hollow) in enumerate(groups):
    for j, v in enumerate(vals):
        ax.scatter(gi + (j - (len(vals) - 1) / 2) * 0.08, v, marker=mk, facecolors="none" if hollow else col, edgecolors=col, s=36)
ax.axhline(10, color="k", ls="--", lw=0.8); ax.text(2.45, 11.5, "10x (R6.9 falsifier)", fontsize=7, ha="right")
ax.axhline(1, color="k", lw=0.8)
ax.set_yscale("log"); ax.set_xlim(-0.6, 2.6); ax.set_xticks(range(3)); ax.set_xticklabels([x[0] for x in groups], fontsize=8)
ax.set_ylabel("measured rate / (1/g!)   [one point per curve]"); ax.set_title("Measured fully-split rate over Gaudry's 1/g! heuristic (g=8)", fontsize=9)
save(fig, "fig2_ratio_to_inverse_gfactorial")
print("ok", len(rows))
