"""n = 83 F6 x-only query and build: x-only baseline vs ARM64 PMULL candidate.
Reads only data-bin-pmull.json; writes fig-bin-pmull.{pdf,svg}.
Medians and ratios of medians are derived here from the per-run values."""
import json, os, statistics
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-pmull.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False})
C = {"baseline": "#0072B2", "candidate": "#D55E00"}
LAB = {"baseline": "x-only baseline", "candidate": "packed PMULL candidate"}
MK = {"baseline": "o", "candidate": "s"}

rows = {(r["arm"], r["points"]): r for r in D["rows"]}
pts = sorted({r["points"] for r in D["rows"]})
fig, axes = plt.subplots(1, 2, figsize=(7.0, 2.7))

ax = axes[0]
for arm in ("baseline", "candidate"):
    med = []
    for p in pts:
        q = [v / 1e9 for v in rows[(arm, p)]["query_ns"]]
        ax.plot([p] * len(q), q, marker=MK[arm], color=C[arm], ls="none", ms=3.2, alpha=0.45)
        med.append(statistics.median(q))
    ax.plot(pts, med, marker=MK[arm], color=C[arm], ms=5, lw=1.1, label=LAB[arm] + " (median)")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xticks(pts); ax.set_xticklabels([f"{p:,}" for p in pts])
ax.minorticks_off()
ax.set_xlabel("usable base points")
ax.set_ylabel("exact four-sum query time (s)")
ax.legend(frameon=False, fontsize=6.8, loc="upper center", bbox_to_anchor=(0.5, -0.2), ncol=1)
ax.text(0.98, 0.04, "(a)", transform=ax.transAxes, ha="right")

ax = axes[1]
for metric, mk, lab in (("query_ns", "o", "query"), ("build_ns", "^", "index build")):
    r = [statistics.median(rows[("baseline", p)][metric]) / statistics.median(rows[("candidate", p)][metric])
         for p in pts]
    ax.plot(pts, r, marker=mk, color="#009E73" if metric == "query_ns" else "#CC79A7", ms=5, lw=1.1,
            label=f"{lab}, ratio of medians")
# Full base: adjacent interleaved pairs (baseline, candidate, candidate, baseline): pairs 1-2 and 3-4.
fb, fc = rows[("baseline", pts[-1])], rows[("candidate", pts[-1])]
order = sorted([(o, "b", q) for o, q in zip(fb["order"], fb["query_ns"])] +
               [(o, "c", q) for o, q in zip(fc["order"], fc["query_ns"])])
pairs = [order[0][2] / order[1][2], order[3][2] / order[2][2]]
ax.plot([pts[-1]] * 2, pairs, marker="x", color="#009E73", ls="none", ms=5,
        label="query, paired runs at 4,054")
ax.axhline(1, color="#444444", ls="--", lw=0.9, label="ratio 1 (no change)")
ax.set_xscale("log")
ax.set_xticks(pts); ax.set_xticklabels([f"{p:,}" for p in pts])
ax.minorticks_off()
ax.set_ylim(0, 7.6)
ax.set_xlabel("usable base points")
ax.set_ylabel("baseline / candidate time")
ax.legend(frameon=False, fontsize=6.8, loc="upper center", bbox_to_anchor=(0.5, -0.2), ncol=2, columnspacing=1.0)
ax.text(0.02, 0.04, "(b)", transform=ax.transAxes, ha="left")

fig.tight_layout()
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(H, f"fig-bin-pmull.{ext}"), bbox_inches="tight")
print("pairs", [round(x, 3) for x in pairs])
