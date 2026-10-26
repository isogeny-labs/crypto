"""Layout only: plots values recorded in summary.json / rows.jsonl.

Computes nothing new (research-visuals skill). Usage:
    python plot.py reports/run-<id>
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

run = Path(sys.argv[1])
rid = run.name
s = json.loads((run / "summary.json").read_text())
fams = s["families"]
rows = [json.loads(line) for line in (run / "rows.jsonl").read_text().splitlines()]

INK, MUTED, GRID = "#1f2328", "#6e7781", "#d0d7de"
OBS, HEUR, PRED = "#0969da", "#bf8700", "#8250df"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": MUTED, "axes.labelcolor": INK,
                     "xtick.color": INK, "ytick.color": INK, "svg.fonttype": "none"})


def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(run / f"{name}.{ext}", bbox_inches="tight", **kw)
    plt.close(fig)


# Figure 1: observed off-parity fraction vs the independent-lift heuristic.
a = sorted((f for f in fams if f["arm"] == "A_random"), key=lambda f: (f["m"], f["n"]))
labels = [f"m={f['m']}\nn={f['n']}" for f in a]
obs = [f["observed_off_fraction"] for f in a]
lo = [f["observed_off_fraction"] - f["observed_off_cp95"][0] for f in a]
hi = [f["observed_off_cp95"][1] - f["observed_off_fraction"] for f in a]
heu = [f["heuristic_off_fraction"] for f in a]
x = range(len(a))
fig, ax = plt.subplots(figsize=(7.2, 3.4))
w = 0.38
ax.bar([i - w / 2 for i in x], heu, w, color=HEUR, label="heuristic: 1 − p_E^m (independent lifts)")
ax.bar([i + w / 2 for i in x], obs, w, color=OBS, yerr=[lo, hi], capsize=3, ecolor=INK,
       label="observed, 95% Clopper–Pearson")
for i, f in enumerate(a):
    ax.text(i + w / 2, f["observed_off_cp95"][1] + 0.02, f"{f['observed_off_fraction']:.3f}\nN={int(f['curve.solutions'])}",
            ha="center", va="bottom", fontsize=7, color=INK)
ax.set_xticks(list(x), labels)
ax.set_ylabel("fraction of solutions with an\noff-parity (twist-lifting) coordinate")
ax.set_ylim(0, 1.05)
ax.grid(axis="y", color=GRID, lw=0.6)
ax.set_axisbelow(True)
ax.spines[["top", "right"]].set_visible(False)
ax.legend(frameon=False, loc="upper center", ncol=2, bbox_to_anchor=(0.5, 1.13), fontsize=8)
ax.set_title(f"Arm A (random rational targets, 5 curves per n) — {rid}", fontsize=9, pad=24)
save(fig, "fig1_off_parity_fraction")

# Figure 2: predicted vs observed off-parity solution counts, per target.
fig, ax = plt.subplots(figsize=(4.6, 4.2))
style = {"A_random": (OBS, "o", "A random"), "C1_planted_xR_in_V": (PRED, "s", "C1 planted x_R ∈ V"),
         "C2_planted_T_pairs": (HEUR, "^", "C2 planted T-pairs")}
for arm, (col, mk, lab) in style.items():
    pts = [(r["curve_model"]["pred_off"], r["curve_model"]["obs_off"]) for r in rows
           if r.get("family") == "decomp" and r["arm"] == arm and r["m"] == 3]
    nz = [(p + 0.5, o + 0.5) for p, o in pts]
    n0 = sum(1 for p, o in pts if p == 0 and o == 0)
    ax.scatter([p for p, _ in nz], [o for _, o in nz], s=14, marker=mk, color=col, alpha=0.6,
               label=f"{lab}: {len(pts)} targets ({n0} at 0,0)")
lim = [0.4, 600]
ax.plot(lim, lim, color=MUTED, lw=0.8, ls="--", label="observed = predicted")
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlim(lim)
ax.set_ylim(lim)
ax.set_xlabel("predicted off-parity solutions + 0.5 (built from target and V only)")
ax.set_ylabel("observed off-parity solutions + 0.5")
ax.grid(color=GRID, lw=0.5)
ax.legend(frameon=False, fontsize=7, loc="upper left")
ax.set_title(f"m = 3, curve model, n ∈ {{11,13,17,19}} — {rid}", fontsize=9)
save(fig, "fig2_predicted_vs_observed")

# Figure 3: diagram of the mechanism.
fig, ax = plt.subplots(figsize=(7.2, 2.6))
ax.axis("off")
box = dict(boxstyle="round,pad=0.4", fc="#f6f8fa", ec=MUTED)
ax.text(0.5, 0.92, "E(F_{q²}) ⊃ x-rational points  =  E(F_q)  ∪  φ(E′(F_q)),   intersection = E[2](F_q) = {O, T}",
        ha="center", fontsize=9, bbox=box)
ax.text(0.2, 0.55, "E(F_q): σ = +1", ha="center", fontsize=9, color=OBS, bbox=box)
ax.text(0.8, 0.55, "φ(E′(F_q)): σ = −1", ha="center", fontsize=9, color=HEUR, bbox=box)
ax.text(0.5, 0.55, "R = A + B\nσ ⇒ R = A − B\n⇒ 2B = 0", ha="center", fontsize=9, bbox=box)
ax.annotate("", (0.38, 0.55), (0.29, 0.55), arrowprops=dict(arrowstyle="->", color=MUTED))
ax.annotate("", (0.62, 0.55), (0.69, 0.55), arrowprops=dict(arrowstyle="->", color=MUTED))
ax.text(0.5, 0.13,
        "Summation polynomial S_{m+1}(x_1..x_m, x_R) = 0 with x_i ∈ V:  off-parity sub-sum ∈ {O, T}.\n"
        "m = 2: no mixed solution.  m = 3: mixed solutions are exactly (x_R, x, x) and (x(R+T), x, x(P_x+T)).",
        ha="center", fontsize=8, color=INK)
save(fig, "fig3_mechanism")
print("figures written to", run)
