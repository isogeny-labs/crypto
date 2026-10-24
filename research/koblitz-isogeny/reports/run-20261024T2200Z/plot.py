"""ICC_curve vs number of subspaces V, and recorded power rows, at n = 31, m = 3
(recorded values only; computes no result). A missing input file is skipped."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parents[4]
here = Path(__file__).parent
E = root / "experiments/koblitz_presentation"
K = "n=31 a2=0 l=10 m=3"
FILES = [
    ("icc_n31_m3_geom_diag.json", "4 V (seeds 1–4)", "C0"),
    ("icc_n31_m3_geom_s1-8_power.json", "8 V (seeds 1–8)", "C1"),
    ("icc_n31_m3_geom_s1-32.json", "32 V (seeds 1–32)", "C2"),
]
METRICS = ["yield_per_Fm", "log_us_per_call", "work_per_call", "log_projected_solve"]

data = []
for fn, lab, col in FILES:
    path = E / fn
    if not path.exists():
        print(f"skip (missing): {fn}")
        continue
    data.append((fn, lab, col, json.load(open(path))[K]))

# (a) ICC with CI per metric, one colour per file
fig, ax = plt.subplots(figsize=(8.0, 3.8))
n = len(data)
for j, (fn, lab, col, d) in enumerate(data):
    for i, m in enumerate(METRICS):
        if m not in d:
            continue
        p, (lo, hi) = d[m]["icc_curve"], d[m]["ci95"]
        x = i + (j - (n - 1) / 2) * 0.22
        ax.errorbar([x], [p], yerr=[[p - lo], [hi - p]], fmt="o", capsize=3, color=col,
                    label=lab if i == 0 else None)
ax.axhline(0.05, ls="--", color="green", lw=1, label="H0 bound 0.05")
ax.axhline(0.10, ls=":", color="red", lw=1, label="H1 threshold 0.10")
ax.set_xticks(range(len(METRICS))); ax.set_xticklabels(METRICS, rotation=15, fontsize=8)
ax.set_ylim(-0.02, 0.5); ax.set_ylabel("ICC_curve (95 % CI)")
ax.set_title("n = 31, m = 3: ICC_curve by metric and number of subspaces V (32 curves)", fontsize=9)
ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(here / "icc_vs_subspaces.svg"); plt.close(fig)

# (b) recorded power rows for work_per_call
rows = None
for fn, lab, col, d in data:
    if "power" in d and "work_per_call" in d["power"]:
        rows = d["power"]["work_per_call"]
        break
fig, axes = plt.subplots(1, 2, figsize=(8.0, 3.4), sharex=True, sharey=True)
if rows:
    sims = sorted({r.get("sims") for r in rows if r.get("sims") is not None})
    for V, c in zip(sorted({r["subspaces"] for r in rows}), ["C0", "C1", "C2", "C3"]):
        rs = sorted((r for r in rows if r["subspaces"] == V), key=lambda r: r["icc_true"])
        for ax, key in zip(axes, ["p_h0", "p_h1"]):
            ax.plot([r["icc_true"] for r in rs], [r[key] for r in rs], "o-", color=c, label=f"{V} V")
    note = f"; {sims[0]} simulations per point" if len(sims) == 1 else ""
    fig.suptitle(f"work_per_call, 32 curves: recorded simulated verdict rates{note}", fontsize=9)
for ax, t in zip(axes, ["P(H0)", "P(H1)"]):
    ax.set_xlabel("true ICC_curve"); ax.set_ylabel(t); ax.set_title(t, fontsize=9)
    ax.grid(alpha=0.3); ax.set_ylim(-0.02, 1.02)
axes[0].legend(fontsize=8)
fig.tight_layout(); fig.savefig(here / "power.svg"); plt.close(fig)
