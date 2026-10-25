"""ICC_curve with 95 % CI for the three 32 curves x 32 V arms (n = 19 m = 2 random V,
n = 31 and n = 23 m = 3 geometric V), and the recorded simulated verdict rates at 32 V.
Recorded values only; computes no result. A missing input file is skipped."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

root = Path(__file__).resolve().parents[4]
here = Path(__file__).parent
E = root / "experiments/koblitz_presentation"
# (file, key, label, colour)
ARMS = [
    ("icc_n19_m2_rand_s1-32.json", "n=19 a2=1 l=9 m=2", "n = 19, m = 2 Gröbner, random V", "C3"),
    ("icc_n31_m3_geom_s1-32.json", "n=31 a2=0 l=10 m=3", "n = 31, m = 3 PinX3, geometric V", "C0"),
    ("icc_n23_m3_geom_s1-32.json", "n=23 a2=1 l=8 m=3", "n = 23, m = 3 PinX3, geometric V", "C2"),
]
METRICS = [("work_per_call", "work per call"), ("yield_per_Fm", "yield per |F|^m"),
           ("log_us_per_call", "ln µs per call"), ("log_projected_solve", "ln projected solve")]

arms = []
for fn, key, lab, col in ARMS:
    path = E / fn
    if not path.exists():
        print(f"skip (missing): {fn}")
        continue
    arms.append((fn, lab, col, json.load(open(path))[key]))

# (a) ICC_curve with CI, four metrics, three arms
fig, ax = plt.subplots(figsize=(8.0, 4.0))
n = len(arms)
for j, (fn, lab, col, d) in enumerate(arms):
    for i, (m, _) in enumerate(METRICS):
        p, (lo, hi) = d[m]["icc_curve"], d[m]["ci95"]
        x = i + (j - (n - 1) / 2) * 0.22
        ax.errorbar([x], [p], yerr=[[p - lo], [hi - p]], fmt="o", capsize=3, color=col,
                    label=lab if i == 0 else None)
ax.axhline(0.05, ls="--", color="green", lw=1, label="H0 bound 0.05")
ax.axhline(0.10, ls=":", color="red", lw=1, label="H1 threshold 0.10")
ax.set_xticks(range(len(METRICS)))
ax.set_xticklabels([t for _, t in METRICS], fontsize=8)
ax.set_xlim(-0.5, len(METRICS) - 0.5)
ax.set_ylim(-0.01, 0.12)
ax.set_ylabel("ICC_curve (95 % CI, 2000 bootstrap over curves)")
ax.set_title("32 curves x 32 V per arm: ICC_curve by metric", fontsize=9)
ax.grid(alpha=0.3); ax.legend(fontsize=7, loc="upper right")
fig.tight_layout(); fig.savefig(here / "icc_three_arms.svg"); plt.close(fig)

# (b) recorded simulated verdict rates at 32 V, work_per_call, from each file's `power` rows
fig, axes = plt.subplots(1, 2, figsize=(8.0, 3.4), sharex=True, sharey=True)
sims = set()
for fn, lab, col, d in arms:
    rows = [r for r in d.get("power", {}).get("work_per_call", []) if r["subspaces"] == 32]
    rows.sort(key=lambda r: r["icc_true"])
    sims |= {r["sims"] for r in rows}
    for ax, k in zip(axes, ["p_h0", "p_h1"]):
        ax.plot([r["icc_true"] for r in rows], [r[k] for r in rows], "o-", color=col, label=lab)
for ax, t in zip(axes, ["P(H0)", "P(H1)"]):
    ax.set_title(t, fontsize=9); ax.set_xlabel("planted true ICC_curve")
    ax.grid(alpha=0.3); ax.set_ylim(-0.02, 1.02)
axes[0].set_ylabel(f"rate ({', '.join(str(s) for s in sorted(sims))} sims per point)")
axes[0].legend(fontsize=7)
fig.suptitle("work per call, 32 curves x 32 V: recorded simulated verdict rates", fontsize=9)
fig.tight_layout(); fig.savefig(here / "power_32v.svg"); plt.close(fig)
