"""Lay out recorded values from results.jsonl (no computation of results)."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = sys.argv[1] if len(sys.argv) > 1 else "."
rows = [json.loads(l) for l in open(f"{here}/results.jsonl") if l.strip().startswith("{")]
bm = {(r["f"], r["n"]): r["eps_bm_equal_bounds"] for r in rows if r["mode"] == "bm"}
opt = [r for r in rows if r["mode"] == "optimize" and r["f"] == "s3"]
best = {n: max([r["reach_at_8x_points"] for r in opt if r["n"] == n], default=None) for n in (2, 3)}

labels, vals, colors = [], [], []
items = [
    ("JM box, S3 n=3\n(2/(d(n+1)), CORRECTION.md)", 0.25, "#9aa5b1"),
    ("JM extended, S3 n=3\n(CORR-20261024-b2a799)", 0.2803, "#9aa5b1"),
    ("Any lattice, equal bounds\nS3 n=3 (Brunn–Minkowski)", bm.get(("s3", 3)), "#3b6ea8"),
    ("Any lattice, equal bounds\nS3 n=2 (Brunn–Minkowski)", bm.get(("s3", 2)), "#3b6ea8"),
    ("Best region found, any bounds\nS3 n=3 (optimiser, 8x points)", best[3], "#d08c2c"),
    ("Best region found, any bounds\nS3 n=2 (optimiser, 8x points)", best[2], "#d08c2c"),
]
for l, v, c in items:
    if v is not None:
        labels.append(l); vals.append(v); colors.append(c)
fig, ax = plt.subplots(figsize=(9, 4.8))
y = range(len(vals))
ax.barh(list(y), vals, color=colors)
for i, v in enumerate(vals):
    ax.text(v + 0.01, i, f"{v:.3f}", va="center", fontsize=9)
ax.axvline(8 / 9, color="#b33", ls="--"); ax.text(8 / 9 + 0.005, len(vals) - 0.6, "demand 8/9\n(m,w)=(4,2)", color="#b33", fontsize=9)
ax.axvline(0.5, color="#d08c2c", ls=":"); ax.text(0.505, -0.45, "univariate\ndegree-2 limit 1/2", color="#d08c2c", fontsize=8)
ax.axvline(1.0, color="#555", ls=":"); ax.text(1.005, -0.4, "information\nbound 1", color="#555", fontsize=8)
ax.set_yticks(list(y)); ax.set_yticklabels(labels, fontsize=8)
ax.set_xlim(0, 1.12); ax.set_xlabel("reach ε = log_p(∏ bounds) per equation (Howgrave-Graham convention)")
ax.set_title("Coppersmith reach of a windowed S3 block vs index-calculus demand\nresults.jsonl, examples/smallroot_optimal_reach.rs, 2026-10-26", fontsize=10)
fig.tight_layout(); fig.savefig(f"{here}/reach_vs_demand.svg"); fig.savefig(f"{here}/reach_vs_demand.png", dpi=150)
print("wrote", here)
