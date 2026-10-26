# Layout only: plots the per-prime p-values recorded in ../runs/census.jsonl.
# Computes nothing beyond the Bonferroni lines stated in PROTOCOL.md section 4.
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(here, "..", "runs", "census.jsonl"))]
roles = [("target", "#c0392b", "Solinas-shape (29)"),
         ("residue-matched", "#2c6fbb", "residue-matched random (58)"),
         ("generic", "#7f8c8d", "generic random (58)")]
n = len(rows)
thr = 0.01 / (3 * n)
thr2 = 0.01 / n

fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.4), gridspec_kw={"width_ratios": [3, 1.3]})
ax = axes[0]
x = 0
ticks, labels = [], []
for fkey, fname in [("p_f1", "F1: v2(#E) vs own-residue model"),
                    ("p_f3", "F3: 3 | #E"), ("p_f4", "F4: 5 | #E")]:
    for i, (role, col, lab) in enumerate(roles):
        ps = [r[fkey] for r in rows if r["role"] == role]
        xs = [x + 0.12 * ((j * 7919) % 17 - 8) / 8 for j in range(len(ps))]
        ax.scatter(xs, ps, s=12, color=col, alpha=0.75, label=lab if fkey == "p_f1" else None)
        x += 1
    ticks.append(x - 2)
    labels.append(fname)
    x += 1
ax.axhline(thr, color="#333", ls="--", lw=1)
ax.text(0, thr * 1.6, f"Bonferroni threshold {thr:.1e}", fontsize=7.5)
ax.set_yscale("log")
ax.set_ylim(1e-6, 1.5)
ax.set_xticks(ticks)
ax.set_xticklabels(labels, fontsize=8)
ax.set_ylabel("p-value (model for the prime's own residue)")
ax.set_title("Fit to the matrix model: no test below threshold", fontsize=10)
ax.legend(fontsize=7.5, loc="lower left", frameon=False)
ax.grid(True, which="both", alpha=0.2)

ax = axes[1]
for i, (role, col, lab) in enumerate(roles):
    ps = [r["p_f2"] for r in rows if r["role"] == role]
    xs = [i + 0.12 * ((j * 7919) % 17 - 8) / 8 for j in range(len(ps))]
    ax.scatter(xs, ps, s=12, color=col, alpha=0.75)
ax.axhline(thr2, color="#333", ls="--", lw=1)
ax.set_yscale("log")
ax.set_ylim(1e-14, 1)
ax.set_xticks([0, 1, 2])
ax.set_xticklabels(["Solinas", "matched", "generic"], fontsize=8)
ax.set_title("F2 positive control:\nwrong-class model rejected", fontsize=10)
ax.grid(True, which="both", alpha=0.2)
fig.text(0.01, 0.01,
         "N = 4000 random curves per prime, 16-22 bit primes, seed 20261023. Source: runs/census.jsonl "
         "(PROTOCOL.md frozen in commit 588bdf5ac before the run).",
         fontsize=6.8, color="#444")
fig.tight_layout(rect=(0, 0.05, 1, 1))
fig.savefig(os.path.join(here, "census_pvalues.svg"))
fig.savefig(os.path.join(here, "census_pvalues.png"), dpi=150)
