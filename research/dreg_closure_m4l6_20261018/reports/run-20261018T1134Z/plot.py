"""Lay out the recorded values of run 20261018T1134Z (host wfh2xovuq5aahv).
Every number below is copied from runs/*.jsonl and runs/*.log; nothing is
computed here beyond unit conversion (kB -> GB, s -> min)."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

runs = [  # label, closure_rank@4, columns@4, xl_rank@4, wall s, peak RSS kB, outcome
    ("A1 draw 42\n(unsat)", 1, 341326, 23757, 1467, 25846376, "closure refuted at 4"),
    ("A1 draw 214\n(unsat)", 1, 341056, 23757, 1417, 25716896, "closure refuted at 4"),
    ("A1 draw 0\n(sat control)", 214391, 342537, 23757, 2075, 21361800, "not refuted"),
]
c1 = ("C1 draw 0\n(m3-19-7, to 5)", None, None, None, 1048, 169306092, "allocation failed (SIGABRT)")

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(11, 4.8))
labels = [r[0] for r in runs]
ax1.bar(labels, [r[2] for r in runs], color="#d9d9d9", label="columns C(54, ≤4)")
ax1.bar(labels, [r[1] for r in runs], color="#1f77b4", label="closure rank at degree 4")
ax1.bar(labels, [r[3] for r in runs], color="none", edgecolor="#d62728", hatch="//", label="XL rank at degree 4")
ax1.set_yscale("log")
ax1.set_ylim(0.6, 1.5e6)
for i, r in enumerate(runs):
    ax1.annotate(r[6], (i, 6e5), ha="center", fontsize=8)
ax1.set_ylabel("rank or columns (log)")
ax1.set_title("(m,ℓ)=(4,6), n=15, S=−9: degree-4 closure per draw")
ax1.legend(fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=3, frameon=False)

names = labels + [c1[0]]
rss = [r[5] / 1e6 for r in runs] + [c1[5] / 1e6]
wall = [r[4] / 60 for r in runs] + [c1[4] / 60]
x = range(len(names))
ax2.bar([i - 0.2 for i in x], rss, width=0.4, color="#2ca02c", label="peak RSS (GB)")
ax2.axhline(44, color="#2ca02c", ls=":", lw=1); ax2.text(-0.45, 47, "A1 projection 44 GB", fontsize=7, color="#2ca02c")
ax2.axhline(184, color="k", ls="--", lw=1); ax2.text(2.55, 188, "prlimit cap 184 GB", fontsize=7)
ax2.axhline(217, color="#2ca02c", ls=":", lw=1); ax2.text(2.55, 221, "C1 projection 217 GB", fontsize=7, color="#2ca02c")
ax2.set_ylabel("GB"); ax2.set_ylim(0, 240)
ax3 = ax2.twinx()
ax3.bar([i + 0.2 for i in x], wall, width=0.4, color="#ff7f0e", label="wall (min)")
ax3.set_ylabel("minutes"); ax3.set_ylim(0, 45)
ax2.set_xticks(list(x)); ax2.set_xticklabels(names, fontsize=8)
ax2.set_title("resources per run (GNU time, 25 vCPU, cgroup 200 GB)")
h1, l1 = ax2.get_legend_handles_labels(); h2, l2 = ax3.get_legend_handles_labels()
ax2.legend(h1 + h2, l1 + l2, fontsize=8, loc="upper center", bbox_to_anchor=(0.5, -0.16), ncol=2, frameon=False)
fig.suptitle("dreg_closure_m4l6_20261018 · run 20261018T1134Z · host wfh2xovuq5aahv (commit 9d4541e2)", fontsize=10)
fig.tight_layout()
fig.savefig("closure-a1.svg")
fig.savefig("closure-a1.png", dpi=150)
