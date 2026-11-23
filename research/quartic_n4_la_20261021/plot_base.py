# Lays out recorded values only: reads results/summary.json (written by `n4 summarize`).
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
out = sys.argv[2]
sz = [r for r in rows if "q" in r and "ic_meq" in r]
fits = {r["fit"]: r for r in rows if "fit" in r}
B = sorted([r for r in sz if r["arm"] == "B0"], key=lambda r: r["q"])
A = sorted([r for r in sz if r["arm"] == "A0"], key=lambda r: r["q"])
plt.rcParams.update({"font.size": 9})
# 1. cost vs q
fig, ax = plt.subplots(figsize=(6.2, 4.2))
q = [r["q"] for r in B]
ax.loglog(q, [r["rho_meq"][0] for r in B], "o-", color="#c0392b", label="matched rho (E0/K), mean of runs")
ax.loglog(q, [r["ic_meq"][0] for r in B], "s-", color="#1f6f8b", label="IC-B0 (reduced FB + large prime), cold")
if A:
    ax.loglog([r["q"] for r in A], [r["ic_meq"][0] for r in A], "^--", color="#7f8c8d", label="IC-A0 (all points), cold")
ax.loglog(q, [r["ic_meq"][2] for r in B], "s:", color="#1f6f8b", alpha=.5, label="IC-B0, w = 1 GAE")
ax.loglog(q, [r["rho_meq"][2] for r in B], "o:", color="#c0392b", alpha=.5, label="rho, w = 1 GAE")
ax.set_xlabel("q  (E0 over F_{q^3}, ell ~ q^3/4)")
ax.set_ylabel("cost, F_q-multiplication equivalents")
ax.grid(True, which="both", alpha=.3)
ax.legend(fontsize=7)
ax.set_title("N4: cold single-target cost vs q")
fig.tight_layout(); fig.savefig(out + "/cost_vs_q.svg"); fig.savefig(out + "/cost_vs_q.png", dpi=150); plt.close(fig)
# 2. ratio
fig, ax = plt.subplots(figsize=(6.2, 3.8))
for b, col, lab in [(0, "#1f6f8b", "w = 0"), (2, "#e67e22", "w = 1 GAE")]:
    y = [r["ratio"][b] for r in B]
    lo = [r["ratio_lo"][b] for r in B]; hi = [r["ratio_hi"][b] for r in B]
    ax.errorbar(q, y, yerr=[[a - c for a, c in zip(y, lo)], [d - a for a, d in zip(y, hi)]], fmt="o-", color=col, capsize=2, label="IC-B0 / rho, " + lab)
ax.axhline(1, color="k", lw=.8)
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q"); ax.set_ylabel("IC cost / rho cost  (95% bootstrap)")
ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=8)
ax.set_title("N4: distance to matched rho")
fig.tight_layout(); fig.savefig(out + "/ratio_vs_q.svg"); fig.savefig(out + "/ratio_vs_q.png", dpi=150); plt.close(fig)
# 3. phases
names = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
fig, ax = plt.subplots(figsize=(6.2, 3.8))
bottom = [0.0] * len(B)
cols = ["#95a5a6", "#2980b9", "#bdc3c7", "#c0392b", "#8e44ad", "#27ae60", "#f1c40f"]
for i, n in enumerate(names):
    v = [r["phase_meq"][i] / sum(r["phase_meq"]) for r in B]
    if max(v) < 0.005: continue
    ax.bar(range(len(B)), v, bottom=bottom, color=cols[i], label=n)
    bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(B))); ax.set_xticklabels([str(r["q"]) for r in B], rotation=45)
ax.set_ylabel("share of IC-B0 cost"); ax.set_xlabel("q"); ax.legend(fontsize=7, ncol=2)
ax.set_title("N4: IC-B0 phase shares")
fig.tight_layout(); fig.savefig(out + "/phases.svg"); fig.savefig(out + "/phases.png", dpi=150); plt.close(fig)
print("ok")
