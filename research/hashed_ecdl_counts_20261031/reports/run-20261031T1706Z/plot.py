"""Plots recorded values only: results/decision.json (written by the frozen analysis/decide.py).
No new computation beyond reading ratio, ci95_linearised_t and completeness flags."""
import json, sys, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
dec = json.load(open(sys.argv[1])); out = sys.argv[2]
W = ["0", "0.1", "1"]; COL = ["#1b6ca8", "#d98b00", "#8a2be2"]; CURVES = ["fp10", "fp12", "fp14", "fp16"]
def cells(cmp): return [c for c in dec["cells"] if c["comparison"] == cmp]
# Graph 1: Track C ratios with recorded linearised-t 95% intervals
fig, axs = plt.subplots(1, 2, figsize=(10, 4.2), sharey=False)
for ax, cmp, title in zip(axs, ["C-rho", "C-ic"], ["tensor-m2 S / rho-neg S", "tensor-m2 S / ic-mitm S"]):
    cs = sorted(cells(cmp), key=lambda c: c["log2_r"])
    for i, w in enumerate(W):
        xs = [c["log2_r"] + (i - 1) * 0.08 for c in cs]; ys = [c["ratio"][i] for c in cs]
        lo = [c["ratio"][i] - c["ci95_linearised_t"][i][0] for c in cs]; hi = [c["ci95_linearised_t"][i][1] - c["ratio"][i] for c in cs]
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o-", color=COL[i], capsize=3, label=f"{w} GAE / unpriced op")
    ax.set_yscale("log"); ax.set_xticks([c["log2_r"] for c in cs]); ax.set_xticklabels(CURVES[:len(cs)])
    ax.set_xlabel("curve (prime group order about 2^10 to 2^16)"); ax.set_ylabel("ratio of counted S (>1: tensor costs more)"); ax.set_title(title); ax.grid(alpha=.3, which="both")
axs[0].legend(fontsize=8)
fig.suptitle("Track C: counted S, whole-point tensor IC (ic.tensor_contraction_v1) vs references; 95% linearised-t intervals as recorded\nsessions ECBS1hb5ca4da84455 (10-14 bit), ECBS1hab6ef45acb59 (16 bit); shared per-cell seeds, 1 round", fontsize=8)
fig.tight_layout(); fig.savefig(out + "/graph1-trackC-ratios.svg"); fig.savefig(out + "/graph1-trackC-ratios.png", dpi=150); plt.close(fig)
# Graph 2: Track B completeness grid + w=0 ratio for complete cells
cmps = [("B-rho-selectors", "native-selectors"), ("B-rho-partial", "native-partial"), ("B-rho-source", "native-source"), ("B-rho-width", "native-width"), ("B-rho-ic", "ic-mitm")]
fig, ax = plt.subplots(figsize=(8, 3.8))
for yi, (cmp, name) in enumerate(cmps):
    byslug = {c["slug"].split("-")[1]: c for c in cells(cmp)}
    for xi, cv in enumerate(CURVES):
        c = byslug[cv]; done = c["complete"]
        ax.add_patch(plt.Rectangle((xi - .45, yi - .45), .9, .9, color="#9bd3a4" if done else "#d9d9d9", ec="k", lw=.6))
        ax.text(xi, yi, (f"{c['ratio'][0]:.3g}x\n(w=0)" if done else f"incomplete\n{c.get('verified_pairs', 0)} verified pairs"), ha="center", va="center", fontsize=7)
ax.set_xlim(-.6, 3.6); ax.set_ylim(-.6, len(cmps) - .4); ax.set_xticks(range(4)); ax.set_xticklabels(CURVES)
ax.set_yticks(range(len(cmps))); ax.set_yticklabels([n for _, n in cmps]); ax.invert_yaxis(); ax.set_aspect("auto")
ax.set_title("Track B (ECBS1he2397c126e47): arm S / rho-neg S where the cell is complete\ngreen = all pairs verified (recorded ratio at 0 GAE); grey = exhausted executions, left unknown (not ranked)", fontsize=8)
fig.tight_layout(); fig.savefig(out + "/graph2-trackB-cells.svg"); fig.savefig(out + "/graph2-trackB-cells.png", dpi=150); plt.close(fig)
