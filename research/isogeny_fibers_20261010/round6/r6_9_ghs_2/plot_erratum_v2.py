"""R7.5: corrected 'off from 1/g! by more than 10x' figure for R6.9.  Plots measured_rate and
model_heuristic_inv_gfact from relrate.json (ratio = their quotient); no new measurement."""
import json
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
matplotlib.rcParams["svg.fonttype"] = "none"
rel = json.load(open("relrate.json"))
col = {4: "#2f6f9f", 8: "#d08a1f", 16: "#3d8a4f", 32: "#7b4f9d"}
mk = {1: "o", 2: "s"}
fig, ax = plt.subplots(figsize=(8.2, 4.8))
n_over = 0
for r in rel:
    x = r["measured_rate"] / r["model_heuristic_inv_gfact"]; over = x > 10; n_over += over
    ax.scatter(r["genus"] + (r["seed"] - 1.5) * 0.12 + {4: -0.12, 8: -0.04, 16: 0.04, 32: 0.12}[r["q"]] * 0.5, x, marker=mk[r["seed"]], s=46,
               facecolors=col[r["q"]] if over else "none", edgecolors=col[r["q"]], linewidths=1.4)
    if over and r["q"] == 8:
        ax.annotate(f"q=8, g=6: {x:.1f}x\n(7th run, missing in text)", (r["genus"], x), xytext=(r["genus"] - 1.5, x * 4), fontsize=8,
                    arrowprops=dict(arrowstyle="->", lw=0.8))
for q, c in col.items():
    ax.scatter([], [], marker="o", facecolors="none", edgecolors=c, label=f"q = {q}")
ax.axhline(10, color="#c0392b", lw=1.6, ls="--", label="10x")
ax.axhline(1, color="k", lw=0.8)
ax.set_yscale("log"); ax.set_xlabel("genus g"); ax.set_ylabel("measured smooth rate / (1/g!)")
ax.set_title(f"R6.9 erratum v2: {n_over} of {len(rel)} runs exceed 10x Gaudry's 1/g!  (filled = exceeds 10x; circle seed 1, square seed 2)\n"
             "q=4: g=6,7,8 (6 runs); q=8: g=6 seed 1 (1 run).  Original text said 6.", fontsize=8.5)
ax.grid(axis="y", which="both", alpha=0.3); ax.legend(fontsize=8, loc="upper left")
fig.tight_layout(); fig.savefig("erratum_v2_ratio_vs_gfact.svg")
print(n_over)
