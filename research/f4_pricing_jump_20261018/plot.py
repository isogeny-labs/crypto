# Lays out recorded values only (bisect/sigs/*.json, written by bisect/sig.py
# from each step's records.jsonl); computes no result.
import json, statistics as st
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = Path(__file__).parent
S = {
    "2245b75\n(archived)": "000-2245b7596",
    "cd226d0ce\n(#1774, pricing)": "039-cd226d0ce",
    "f2c9202ca\n(#1862, harvest)": "096-f2c9202ca",
    "abee07ee7\n(main)": "303-abee07ee7",
}
sig = {k: json.load(open(H / "bisect/sigs" / f"{v}.json")) for k, v in S.items()}
arms = ["ic-f4-m2", "ic-f4-m3"]
colors = {"ic-f4-m2": "#2a6fb0", "ic-f4-m3": "#c8553d"}

fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.4))
x = range(len(S))
for i, arm in enumerate(arms):
    ys, lo, hi = [], [], []
    for k in S:
        v = [r["total_gae"] for r in sig[k] if r["arm"] == arm and not r["warmup"]]
        m = st.mean(v); ys.append(m); lo.append(m - min(v)); hi.append(max(v) - m)
    xs = [t + (i - 0.5) * 0.18 for t in x]
    a.errorbar(xs, ys, yerr=[lo, hi], fmt="o-", color=colors[arm], capsize=3,
               label=f"{arm} (n = {len(v)} rows)")
a.set_yscale("log")
a.set_xticks(list(x)); a.set_xticklabels(list(S), fontsize=8)
a.set_ylabel("total_gae per row (ecbench.gae), log scale")
a.set_title("(a) total_gae, mean and min–max, non-warm-up rows", fontsize=10)
a.legend(fontsize=8); a.grid(alpha=0.3, which="both")

pre, post = sig["cd226d0ce\n(#1774, pricing)"], sig["f2c9202ca\n(#1862, harvest)"]
for arm in arms:
    p = [(r["solver_ops"], q["solver_ops"]) for r, q in zip(pre, post) if r["arm"] == arm]
    b.scatter([u for u, _ in p], [w for _, w in p], s=14, color=colors[arm], label=f"{arm} ({len(p)} rows incl. warm-ups)")
lim = [min(r["solver_ops"] for r in pre) * 0.8, max(r["solver_ops"] for r in post) * 1.2]
b.plot(lim, lim, "k--", lw=0.8, label="y = x")
b.set_xscale("log"); b.set_yscale("log")
b.set_xlabel("solver word XORs before #1862 (cd226d0ce)")
b.set_ylabel("solver word XORs after #1862 (f2c9202ca)")
b.set_title("(b) counted solver work across the harvest merge", fontsize=10)
b.legend(fontsize=8); b.grid(alpha=0.3, which="both")

fig.suptitle("Inherited-F4 pricing jump, claim-koblitz-rootfind-n13 (F4 arms + rho reference), x86-64, L0", fontsize=11)
fig.tight_layout()
fig.savefig(H / "pricing_jump.svg")
fig.savefig(H / "pricing_jump.png", dpi=150)
