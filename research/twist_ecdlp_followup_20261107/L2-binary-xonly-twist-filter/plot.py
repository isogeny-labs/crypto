# Plots read recorded values from results.json only (no recomputation of experimental numbers).
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

D = json.load(open("results.json"))
os.makedirs("figs", exist_ok=True)
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False, "figure.dpi": 130})
C = {"U": "#1f77b4", "F": "#d62728", "S": "#2ca02c", "rho": "#444444", "twist": "#d62728"}
cells = D["binary_cells"]
labels = [f"m={c['m']}\nk={c['k']}" for c in cells]
figs = []

# (a) violation upper bounds vs coin-flip
fig, ax = plt.subplots(figsize=(7.5, 3.6))
xs = range(len(cells))
ub = [max(c["violation_upper95"], 1e-12) for c in cells]
ax.scatter(xs, ub, color=C["F"], zorder=3, label="95% upper bound on non-degenerate twist-coordinate rate (observed violations in bar label)")
for i, c in enumerate(cells):
    ax.annotate(f"{int(c['violations'])} of {int(c['unique_solutions'])}", (i, ub[i]), textcoords="offset points", xytext=(0, 7), ha="center", fontsize=7)
ax.hlines([0.75, 0.875], -0.5, len(cells) - 0.5, colors=["#999999", "#555555"], linestyles="--")
ax.text(len(cells) - 0.55, 0.78, "coin-flip m=2: 0.75", ha="right", fontsize=7)
ax.text(len(cells) - 0.55, 0.92, "coin-flip m=3: 0.875", ha="right", fontsize=7)
ax.set_yscale("log"); ax.set_ylim(1e-5, 2)
ax.set_xticks(list(xs)); ax.set_xticklabels(labels)
ax.set_ylabel("rate (log scale)"); ax.set_title("H1: non-liftable solutions per cell vs the independent-lift model")
ax.legend(fontsize=7, loc="center left")
fig.tight_layout(); fig.savefig("figs/fig_a_violations.png"); fig.savefig("figs/fig_a_violations.svg"); figs.append(fig)

# (b) z histogram
zs = [z for c in cells for z in c["z_arm_list"]]
fig, ax = plt.subplots(figsize=(6, 3.4))
ax.hist(zs, bins=24, density=True, color="#8fb3d9", edgecolor="white")
g = [i / 50 for i in range(-200, 201)]
ax.plot(g, [math.exp(-x * x / 2) / math.sqrt(2 * math.pi) for x in g], color=C["rho"], lw=1.2, label="N(0,1) (predicted)")
ax.set_xlabel("z = (f_twist - (1 - mu_E)) / sd, one per curve and cell (arm factor base)")
ax.set_ylabel("density"); ax.set_title(f"H2: twist fraction of factor-base columns, {len(zs)} curve-cells")
ax.legend(fontsize=8)
fig.tight_layout(); fig.savefig("figs/fig_b_zhist.png"); fig.savefig("figs/fig_b_zhist.svg"); figs.append(fig)

# (c) call ratio and cost ratio vs registered values
fig, axs = plt.subplots(1, 2, figsize=(9, 3.6))
ax = axs[0]
ax.scatter(xs, [c["call_ratio_mean"] for c in cells], color=C["U"], zorder=3, label="measured mean U/F stage-1 calls")
ax.scatter(xs, [c["call_ratio_ref"] for c in cells], marker="_", s=300, color=C["rho"], label="registered 2.0 / 4.0")
ax.set_xticks(list(xs)); ax.set_xticklabels(labels, fontsize=7); ax.set_ylabel("call ratio"); ax.set_title("solver-call ratio"); ax.legend(fontsize=7)
ax = axs[1]
reg = [c["R_registered"] for c in cells]
ax.fill_between([-0.5, len(cells) - 0.5], [0.9] * 2, [1.1] * 2, color="#eeeeee")  # placeholder band replaced by per-cell below
ax.cla()
for i, c in enumerate(cells):
    ax.vlines(i, c["R_registered"] * 0.9, c["R_registered"] * 1.1, color="#bbbbbb", lw=8)
ax.scatter(xs, reg, marker="_", s=200, color=C["rho"], zorder=3, label="registered R (band +-10%)")
ax.scatter(xs, [c["R_hat_native"] for c in cells], color=C["U"], zorder=4, label="measured R_hat (native ops, U/F)")
lo = [c["R_hat_cost_c0.1"][0] - c["R_hat_cost_c0.1"][1] for c in cells]
hi = [c["R_hat_cost_c0.1"][2] - c["R_hat_cost_c0.1"][0] for c in cells]
ax.errorbar([i + 0.15 for i in xs], [c["R_hat_cost_c0.1"][0] for c in cells], yerr=[lo, hi], fmt="s", ms=3, color=C["F"], label="cost ratio at c=0.1, 95% cluster CI")
ax.axhline(1.0, color="#999999", lw=0.8)
ax.set_xticks(list(xs)); ax.set_xticklabels(labels, fontsize=7); ax.set_ylabel("cost ratio U/F"); ax.set_title("H3: cost ratio vs registered table"); ax.legend(fontsize=7)
fig.tight_layout(); fig.savefig("figs/fig_c_ratios.png"); fig.savefig("figs/fig_c_ratios.svg"); figs.append(fig)

# (d) IC vs rho across the GAE band
rho = {r["k"]: r for r in D["rho"]}
rows = D["ic_vs_rho"]
def pick(m, k, arm):
    cand = [r for r in rows if r["m"] == m and r["k"] == k and r["arm"] == arm]
    for pref in ("measured_d_determined", "extrapolated, N_need"):
        for r in cand:
            if r["source"].startswith(pref):
                return r, pref.startswith("measured")
    return None, False
fig, axs = plt.subplots(1, 2, figsize=(10, 3.9), sharey=False)
for ax, m in zip(axs, (2, 3)):
    ks = sorted({r["k"] for r in rows if r["m"] == m})
    for arm, mk in (("U", "o"), ("F", "^")):
        for k in ks:
            r, meas = pick(m, k, arm)
            if r is None or k not in rho:
                continue
            rr = rho[k]
            ratios = r["gap"]
            ax.plot([0, 1, 2], ratios, marker=mk, color=C[arm], alpha=0.35 + 0.65 * (ks.index(k) + 1) / len(ks), ls="-" if meas else ":", lw=1)
            ax.annotate(f"k={k}", (2, ratios[2]), textcoords="offset points", xytext=(4, 0), fontsize=6)
    ax.axhline(1.0, color=C["rho"], lw=1)
    ax.set_yscale("log"); ax.set_xticks([0, 1, 2]); ax.set_xticklabels(["c=0", "c=0.1", "c=1"])
    ax.set_title(f"m={m}: IC cost / matched rho cost (gap; >1 = rho cheaper)")
    ax.set_ylabel("gap ratio (log)")
axs[0].plot([], [], color=C["U"], marker="o", label="arm U"); axs[0].plot([], [], color=C["F"], marker="^", label="arm F")
axs[0].plot([], [], color="k", ls="-", label="measured (d-determined)"); axs[0].plot([], [], color="k", ls=":", label="extrapolated (registered N_need)")
axs[0].legend(fontsize=7)
fig.tight_layout(); fig.savefig("figs/fig_d_cost_band.png"); fig.savefig("figs/fig_d_cost_band.svg"); figs.append(fig)

with PdfPages("figs/RESULTS_figures.pdf") as pdf:
    for f in figs:
        pdf.savefig(f)
print("plots written", len(figs))
