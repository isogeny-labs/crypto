"""Original vs replication ICC at n = 31, m = 3 (recorded values only; computes no result)."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
root = Path(__file__).resolve().parents[4]
here = Path(__file__).parent
E = root / "experiments/koblitz_presentation"
K = "n=31 a2=0 l=10 m=3"
orig = json.load(open(E / "icc_n31_m3_geom_diag.json"))[K]
rep = json.load(open(E / "icc_n31_m3_geom_rep_s5-8.json"))[K]
frep = json.load(open(E / "icc_31_0_l10_F_rep.json"))[K]
names = ["yield_per_Fm", "log_us_per_call", "work_per_call", "log_projected_solve", "diag_factor_base_points"]
fig, ax = plt.subplots(figsize=(8.0, 3.8))
for j, (lab, d, c) in enumerate([("seeds 1-4 (original)", orig, "C0"), ("seeds 5-8 (replication)", rep, "C1"), ("32 fresh V, |F| only", frep, "C2")]):
    for i, k in enumerate(names):
        if k not in d:
            continue
        p, (lo, hi) = d[k]["icc_curve"], d[k]["ci95"]
        ax.errorbar([i + (j - 1) * 0.22], [p], yerr=[[p - lo], [hi - p]], fmt="o", capsize=3, color=c, label=lab if i == len(names) - 1 else None)
ax.axhline(0.05, ls="--", color="green", lw=1)
ax.axhline(0.10, ls=":", color="red", lw=1)
ax.set_xticks(range(len(names))); ax.set_xticklabels([n.replace("diag_", "") for n in names], rotation=15, fontsize=8)
ax.set_ylim(-0.02, 0.5); ax.set_ylabel("ICC_curve (95 % CI)")
ax.set_title("n = 31, m = 3, PinX3: the work/|F| curve effect does not replicate", fontsize=9)
ax.legend(fontsize=8); ax.grid(alpha=0.3)
fig.tight_layout(); fig.savefig(here / "icc_replication.svg")
