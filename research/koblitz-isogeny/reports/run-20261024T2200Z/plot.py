"""ICC_curve vs number of subspaces V at n = 31, m = 3, the 32-V results at n = 31
and n = 23, the 32-V diagnostics, and the recorded simulated verdict rates
(recorded values only; computes no result). A missing input file is skipped."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parents[4]
here = Path(__file__).parent
E = root / "experiments/koblitz_presentation"
K31 = "n=31 a2=0 l=10 m=3"
K23 = "n=23 a2=1 l=8 m=3"
# (file, key, label, colour, panel)
FILES = [
    ("icc_n31_m3_geom_diag.json", K31, "4 V (seeds 1–4)", "C0", 0),
    ("icc_n31_m3_geom_s1-8_power.json", K31, "8 V (seeds 1–8)", "C1", 0),
    ("icc_n31_m3_geom_s1-32.json", K31, "32 V (seeds 1–32)", "C2", 0),
    ("icc_n23_m3_geom_s1-32.json", K23, "32 V (seeds 1–32)", "C3", 1),
]
METRICS = ["yield_per_Fm", "log_us_per_call", "work_per_call", "log_projected_solve"]
DIAG = ["diag_factor_base_points", "diag_pins_per_call", "diag_roots_per_pin"]
TITLES = ["n = 31, a2 = 0, l = 10 (32 curves)", "n = 23, a2 = 1, l = 8 (32 curves)"]

data = []
for fn, key, lab, col, panel in FILES:
    path = E / fn
    if not path.exists():
        print(f"skip (missing): {fn}")
        continue
    data.append((fn, lab, col, panel, json.load(open(path))[key]))


def icc_panel(ax, series, names, panel):
    s = [x for x in series if x[3] == panel]
    n = len(s)
    for j, (fn, lab, col, _, d) in enumerate(s):
        for i, m in enumerate(names):
            if m not in d:
                continue
            p, (lo, hi) = d[m]["icc_curve"], d[m]["ci95"]
            x = i + (j - (n - 1) / 2) * 0.22
            ax.errorbar([x], [p], yerr=[[p - lo], [hi - p]], fmt="o", capsize=3, color=col,
                        label=lab if i == 0 else None)
    ax.axhline(0.05, ls="--", color="green", lw=1, label="H0 bound 0.05")
    ax.axhline(0.10, ls=":", color="red", lw=1, label="H1 threshold 0.10")
    ax.set_xticks(range(len(names)))
    ax.set_xticklabels([m.replace("diag_", "") for m in names], rotation=15, fontsize=8)
    ax.set_xlim(-0.5, len(names) - 0.5)
    ax.set_ylim(-0.02, 0.5); ax.set_title(TITLES[panel], fontsize=9)
    ax.grid(alpha=0.3); ax.legend(fontsize=7)


# (a) ICC with CI per metric; left n = 31 at 4, 8, 32 V, right n = 23 at 32 V
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.8), sharey=True)
for p, ax in enumerate(axes):
    icc_panel(ax, data, METRICS, p)
axes[0].set_ylabel("ICC_curve (95 % CI)")
fig.suptitle("m = 3 PinX3, geometric V: ICC_curve by metric and number of subspaces V", fontsize=9)
fig.tight_layout(); fig.savefig(here / "icc_vs_subspaces.svg"); plt.close(fig)

# (b) diagnostics at 32 V
fig, axes = plt.subplots(1, 2, figsize=(9.0, 3.4), sharey=True)
for p, ax in enumerate(axes):
    icc_panel(ax, [x for x in data if x[1].startswith("32 V")], DIAG, p)
axes[0].set_ylabel("ICC_curve (95 % CI)")
fig.suptitle("32 V diagnostics: factor-base size, pins per call, roots per pin", fontsize=9)
fig.tight_layout(); fig.savefig(here / "diagnostics.svg"); plt.close(fig)

# (c) recorded simulated verdict rates for work_per_call, from the 32-V files
fig, axes = plt.subplots(2, 2, figsize=(8.0, 6.0), sharex=True, sharey=True)
for r_i, (fn, lab, col, panel, d) in enumerate([x for x in data if x[1].startswith("32 V")]):
    rows = d.get("power", {}).get("work_per_call")
    if not rows:
        continue
    sims = sorted({r.get("sims") for r in rows})
    for V, c in zip(sorted({r["subspaces"] for r in rows}), ["C0", "C1", "C2"]):
        rs = sorted((r for r in rows if r["subspaces"] == V), key=lambda r: r["icc_true"])
        for ax, key in zip(axes[r_i], ["p_h0", "p_h1"]):
            ax.plot([r["icc_true"] for r in rs], [r[key] for r in rs], "o-", color=c, label=f"{V} V")
    axes[r_i][0].set_ylabel(f"{TITLES[panel].split(' (')[0]}\nrate ({sims[0]} sims per point)", fontsize=8)
for ax, t in zip(axes[0], ["P(H0)", "P(H1)"]):
    ax.set_title(t, fontsize=9)
for ax in axes[1]:
    ax.set_xlabel("planted true ICC_curve")
for ax in axes.flat:
    ax.grid(alpha=0.3); ax.set_ylim(-0.02, 1.02)
axes[0][0].legend(fontsize=8)
fig.suptitle("work_per_call, 32 curves: recorded simulated verdict rates (fitted additive model)", fontsize=9)
fig.tight_layout(); fig.savefig(here / "power.svg"); plt.close(fig)
