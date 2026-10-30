# Lays out recorded values only: results/summary_all.json (written by `n4 summarize`).
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
out = sys.argv[2]
sz = [r for r in rows if "q" in r and "ic_meq" in r]
cmp_ = [r for r in rows if "cmp" in r]
def arm(a): return sorted([r for r in sz if r["arm"] == a], key=lambda r: r["q"])
B0, B1, B2, B0n = arm("B0"), arm("B1"), arm("B2"), arm("B0n")
plt.rcParams.update({"font.size": 9})
fig, ax = plt.subplots(figsize=(6.4, 4.4))
ax.loglog([r["q"] for r in B2], [r["rho_meq"][0] for r in B2], "o-", color="#c0392b", label="matched rho (E0/K), mean of runs")
ax.loglog([r["q"] for r in B0], [r["ic_meq"][0] for r in B0], "s--", color="#7f8c8d", label="B0 baseline (#2283)")
ax.loglog([r["q"] for r in B1], [r["ic_meq"][0] for r in B1], "d:", color="#e67e22", label="B1 structured elimination")
ax.loglog([r["q"] for r in B2], [r["ic_meq"][0] for r in B2], "s-", color="#1f6f8b", label="B2 double large primes")
ax.set_xlabel("q  (E0 over F_{q^3}, ell ~ q^3/4)"); ax.set_ylabel("cold single-target cost, F_q-multiplication equivalents")
ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7); ax.set_title("N4 iteration 2: cost vs q")
fig.tight_layout(); fig.savefig(out + "/cost_vs_q.svg"); fig.savefig(out + "/cost_vs_q.png", dpi=150); plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.9))
for a, col in [("B2/B0", "#1f6f8b"), ("B1/B0", "#e67e22"), ("B0n/B0", "#7f8c8d")]:
    c = sorted([r for r in cmp_ if r["cmp"] == a], key=lambda r: r["q"])
    if not c: continue
    q = [r["q"] for r in c]; y = [r["ratio"] for r in c]
    ax.errorbar(q, y, yerr=[[a_ - r["lo"] for a_, r in zip(y, c)], [r["hi"] - a_ for a_, r in zip(y, c)]], fmt="o-", color=col, capsize=2, label="cost " + a + " (paired by size)")
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q"); ax.set_ylabel("cost ratio (95% bootstrap)"); ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7)
ax.set_title("Improvement over the recorded B0 baseline")
fig.tight_layout(); fig.savefig(out + "/ratio_vs_b0.svg"); fig.savefig(out + "/ratio_vs_b0.png", dpi=150); plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.9))
q = [r["q"] for r in B2]
ax.errorbar(q, [r["ratio"][0] for r in B2], yerr=[[r["ratio"][0] - r["ratio_lo"][0] for r in B2], [r["ratio_hi"][0] - r["ratio"][0] for r in B2]], fmt="s-", color="#1f6f8b", capsize=2, label="B2 / rho, w = 0")
ax.errorbar(q, [r["ratio"][2] for r in B2], yerr=[[r["ratio"][2] - r["ratio_lo"][2] for r in B2], [r["ratio_hi"][2] - r["ratio"][2] for r in B2]], fmt="o-", color="#e67e22", capsize=2, label="B2 / rho, w = 1 GAE")
ax.plot([r["q"] for r in B0], [r["ratio"][0] for r in B0], "s--", color="#7f8c8d", label="B0 / rho, w = 0 (baseline)")
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q"); ax.set_ylabel("IC cost / rho cost"); ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7)
ax.set_title("Distance to matched rho")
fig.tight_layout(); fig.savefig(out + "/ratio_vs_rho.svg"); fig.savefig(out + "/ratio_vs_rho.png", dpi=150); plt.close(fig)
names = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
cols = ["#95a5a6", "#2980b9", "#bdc3c7", "#c0392b", "#8e44ad", "#27ae60", "#f1c40f"]
fig, ax = plt.subplots(figsize=(6.4, 3.9)); bottom = [0.0] * len(B2)
for i, n in enumerate(names):
    v = [r["phase_meq"][i] / sum(r["phase_meq"]) for r in B2]
    if max(v) < 0.005: continue
    ax.bar(range(len(B2)), v, bottom=bottom, color=cols[i], label=n); bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(B2))); ax.set_xticklabels([str(r["q"]) for r in B2], rotation=45); ax.set_ylabel("share of B2 cost"); ax.set_xlabel("q"); ax.legend(fontsize=7, ncol=2)
ax.set_title("B2 phase shares")
fig.tight_layout(); fig.savefig(out + "/phases_b2.svg"); fig.savefig(out + "/phases_b2.png", dpi=150); plt.close(fig)
print("ok")
