"""Lay out the recorded rows.jsonl (output of examples/k233_admissible.rs).
No computation: values are plotted as recorded."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = [json.loads(l) for l in open("rows.jsonl")]
series = [("d_chained_w1", "chained N, w=1", "#1f77b4", "o"),
          ("d_chained_w2_2", "chained N, w=2.2", "#d62728", "s"),
          ("d_elim_w1", "N=m·ℓ, w=1", "#2ca02c", "^"),
          ("d_elim_w2_2", "N=m·ℓ, w=2.2", "#9467bd", "v")]
fig, axs = plt.subplots(1, 4, figsize=(13, 3.6), sharey=True)
for ax, m in zip(axs, (3, 4, 5, 6)):
    rs = sorted((r for r in rows if r["m"] == m), key=lambda r: r["ell"])
    for i, (k, lab, c, mk) in enumerate(series):
        off = (i - 1.5) * 0.9
        xs = [j + off / 4 for j in range(len(rs))]
        ax.plot([x for x, r in zip(xs, rs) if r[k] is not None], [r[k] for r in rs if r[k] is not None], mk, color=c, label=lab, ms=6)
        nx = [x for x, r in zip(xs, rs) if r[k] is None]
        ax.plot(nx, [1] * len(nx), mk, color=c, mfc="none", ms=6)
    ax.axhline(4, color="k", ls="--", lw=1)
    ax.set_title(f"m = {m}")
    ax.set_xlabel("stable ℓ")
    ax.set_xticks(range(len(rs)), [r["ell"] for r in rs])
    ax.set_xlim(-0.6, len(rs) - 0.4)
    ax.set_ylim(0.5, 11.5)
    ax.grid(alpha=0.3)
axs[0].set_ylabel("admissible D (hollow at 1 = none, D < 2)")
axs[0].text(0.02, 4.15, "D = 4", transform=axs[0].get_yaxis_transform(), fontsize=8)
axs[0].legend(fontsize=7, loc="upper left")
fig.suptitle("K-233 derived row: admissible Macaulay degree vs Frobenius-stable ℓ (derived, no measurement)", fontsize=10)
fig.tight_layout()
fig.savefig("admissible_vs_ell.svg")
fig.savefig("admissible_vs_ell.png", dpi=150)
