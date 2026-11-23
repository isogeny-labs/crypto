"""Plots recorded values from raw/summary.json only (no computation of results)."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

D = "research/blindspots_20261018/01-fp2-gaudry-k2/"
s = json.load(open(D + "raw/summary.json"))
A = s["arms"]
fig, ax = plt.subplots(1, 2, figsize=(11.5, 4.6))

style = {
    "torsion/ic-a": ("#8c564b", "ic-a (plain, torsion)", "o"),
    "torsion/ic-b": ("#1f77b4", "ic-b (+FGHR symmetry)", "s"),
    "torsion/ic-c1": ("#2ca02c", "ic-c1 (1 large prime)", "^"),
    "torsion/ic-c2": ("#9467bd", "ic-c2 (2 large primes)", "v"),
    "prime/ic-a": ("#e377c2", "ic-a (prime order)", "x"),
    "gls/ic-gls-psi": ("#ff7f0e", "ic-gls-psi (GLS)", "D"),
}
for k, (c, lab, m) in style.items():
    cells = A[k]["by_weight"]["w0"]["all_sizes"]["cells"]
    xs = [cells[z]["mean_log2_r"] for z in cells]
    ys = [cells[z]["mean_S"] for z in cells]
    ax[0].plot(xs, ys, marker=m, color=c, label=lab)
for k, c, lab, ls in (("torsion/rho-neg", "k", "rho-neg (torsion)", "-"), ("gls/rho-psi", "0.45", "rho-psi (GLS)", "--")):
    cells = A[k]["by_weight"]["w0"]["all_sizes"]["cells"]
    ax[0].plot([cells[z]["mean_log2_r"] for z in cells], [cells[z]["mean_S"] for z in cells], ls, color=c, marker=".", label=lab)
ax[0].set_yscale("log")
ax[0].set_xlabel("log2 r (mean over curves in the cell)")
ax[0].set_ylabel("S = cost / sqrt(r), GAE (w = 0), mean of curves")
ax[0].set_title("Cost against matched rho (measured, w=0)", fontsize=10)
ax[0].legend(fontsize=6.5, loc="center left", bbox_to_anchor=(0.02, 0.36), ncol=2)
ax[0].grid(alpha=0.3)

gk = {"torsion/ic-b-vs-rho-neg": ("#1f77b4", "ic-b / rho-neg"), "torsion/ic-c2-vs-rho-neg": ("#9467bd", "ic-c2 / rho-neg"),
      "torsion/ic-a-vs-rho-neg": ("#8c564b", "ic-a / rho-neg"), "gls/ic-gls-psi-vs-rho-psi": ("#ff7f0e", "ic-gls-psi / rho-psi")}
bits = ["24", "28", "32", "36", "40"]
for k, (c, lab) in gk.items():
    for w, ls, al in (("w0", "-", 0.9), ("w1", ":", 0.9)):
        g = s["gaps"][k][w]["per_size"]
        y = [g[b]["gap"] for b in bits]
        ax[1].plot([int(b) for b in bits], y, ls, color=c, alpha=al, marker="o", ms=3, label=lab + (" (w=0)" if w == "w0" else " (w=1)") if False else None)
        if w == "w0":
            ax[1].fill_between([int(b) for b in bits], [g[b]["gap_ci95"][0] for b in bits], [g[b]["gap_ci95"][1] for b in bits], color=c, alpha=0.12)
    ax[1].plot([], [], color=c, label=lab)
ax[1].plot([], [], "k-", label="solid: w=0, band: 95% CI")
ax[1].plot([], [], "k:", label="dotted: w=1")
ax[1].axhline(1, color="r", lw=1)
ax[1].text(24.2, 1.15, "parity with rho = 1", color="r", fontsize=7)
ax[1].set_yscale("log")
ax[1].set_ylim(0.8, 1000)
ax[1].set_xlabel("nominal log2 #E")
ax[1].set_ylabel("gap = IC cost / matched rho cost (distance remaining)")
ax[1].set_title("Gap per size, 95% cluster bootstrap over curves", fontsize=10)
ax[1].legend(fontsize=7, loc="upper left")
ax[1].grid(alpha=0.3)
fig.suptitle("Item 01: k=2 index calculus on E(F_p^2) vs matched rho; 945 rows, 32 files, summary.json (blindspot01.v1)", fontsize=9)
fig.tight_layout()
fig.savefig(D + "gap_vs_n.svg")
fig.savefig(D + "gap_vs_n.png", dpi=150)
