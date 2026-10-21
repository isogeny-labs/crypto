# Lays out recorded values only: reads results/summary.json (written by `v2 summarize`).
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
out = sys.argv[2]
cap = sys.argv[3] if len(sys.argv) > 3 else "N4 v2 grid"
os.makedirs(out, exist_ok=True)
plt.rcParams.update({"font.size": 8.5})
ICC = {"B0": "#7f8c8d", "S": "#2980b9", "L0": "#85c1e9", "L1": "#5dade2", "L": "#1a5276", "D": "#27ae60", "DL1": "#82e0aa", "DL": "#145a32"}
RHC = {"plain": "#c0392b", "neg": "#e67e22"}
ics = [r for r in rows if r.get("kind") == "ic_size"]
rhs = [r for r in rows if r.get("kind") == "rho_size"]
def ser(arm, rs=ics):
    s = sorted([r for r in rs if r["arm"] == arm], key=lambda r: r["q"])
    return s
def save(fig, name):
    fig.tight_layout(); fig.savefig(f"{out}/{name}.svg"); fig.savefig(f"{out}/{name}.png", dpi=150); plt.close(fig)

# 1. cost vs q
fig, ax = plt.subplots(figsize=(6.6, 4.4))
for a in ["B0", "S", "L", "D", "DL"]:
    s = ser(a)
    if s: ax.loglog([r["q"] for r in s], [r["meq"][0] for r in s], "s-", ms=3, color=ICC[a], label=f"IC-{a}")
for a, lab in [("plain", "rho (plain)"), ("neg", "rho (negation map)")]:
    s = ser(a, rhs)
    if s: ax.loglog([r["q"] for r in s], [r["meq"][0] for r in s], "o--", ms=3, color=RHC[a], label=lab)
ax.set_xlabel("q   (E0 over F_{q^3}, ell ~ q^3/4)"); ax.set_ylabel("cold single-target cost, M-eq (w = 0)")
ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7, ncol=2)
ax.set_title("Cost vs q, mean over units / runs\n" + cap, fontsize=8.5)
save(fig, "cost_vs_q")

# 2. ratio to rho-neg with CI
rat = [r for r in rows if r.get("kind") == "ratio"]
fig, axs = plt.subplots(1, 2, figsize=(8.4, 3.8), sharey=True)
for ax, rho in zip(axs, ["neg", "plain"]):
    for a in ["B0", "L", "D", "DL"]:
        s = sorted([r for r in rat if r["ic"] == a and r["rho"] == rho], key=lambda r: r["q"])
        if not s: continue
        q = [r["q"] for r in s]; y = [r["ratio"][0] for r in s]
        ax.errorbar(q, y, yerr=[[a_ - b for a_, b in zip(y, [r["lo"][0] for r in s])], [b - a_ for a_, b in zip(y, [r["hi"][0] for r in s])]], fmt="o-", ms=3, capsize=2, color=ICC[a], label=f"{a}, w = 0")
        ax.plot(q, [r["ratio"][2] for r in s], ":", color=ICC[a], alpha=.7)
    ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log")
    ax.set_xlabel("q"); ax.set_title(f"IC / rho-{rho} (95% bootstrap; dotted: w = 1 GAE)", fontsize=8)
    ax.grid(True, which="both", alpha=.3)
axs[0].set_ylabel("cost ratio"); axs[0].legend(fontsize=7)
fig.suptitle(cap, fontsize=8)
save(fig, "ratio_vs_q")

# 3. wall time
fig, ax = plt.subplots(figsize=(6.6, 4.2))
for a in ["B0", "S", "L", "D", "DL"]:
    s = ser(a)
    if s: ax.loglog([r["q"] for r in s], [r["wall_s"] for r in s], "s-", ms=3, color=ICC[a], label=f"IC-{a}")
for a in ["plain", "neg"]:
    s = ser(a, rhs)
    if s: ax.loglog([r["q"] for r in s], [r["wall_s"] for r in s], "o--", ms=3, color=RHC[a], label=f"rho-{a}")
ax.set_xlabel("q"); ax.set_ylabel("wall seconds per cold solve (single thread, shared host, L0)")
ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7, ncol=2)
ax.set_title("Wall time (practicality note; host load average 70-150)\n" + cap, fontsize=8.5)
save(fig, "wall_vs_q")

# 4. phase shares
names = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
cols = ["#95a5a6", "#2980b9", "#bdc3c7", "#c0392b", "#8e44ad", "#27ae60", "#f1c40f"]
arms = [a for a in ["B0", "S", "L", "D", "DL"] if ser(a)]
fig, axs = plt.subplots(1, len(arms), figsize=(2.2 * len(arms) + 1, 3.6), sharey=True)
if len(arms) == 1: axs = [axs]
for ax, a in zip(axs, arms):
    s = ser(a); bottom = [0.0] * len(s)
    for i, n in enumerate(names):
        v = [r["phase_meq"][i] / sum(r["phase_meq"]) for r in s]
        if max(v) < 0.005: continue
        ax.bar(range(len(s)), v, bottom=bottom, color=cols[i], label=n)
        bottom = [x + y for x, y in zip(bottom, v)]
    ax.set_xticks(range(len(s))); ax.set_xticklabels([str(r["k"]) for r in s], fontsize=6)
    ax.set_title(f"IC-{a}", fontsize=8); ax.set_xlabel("k (q < 2^k)")
axs[0].set_ylabel("share of cold M-eq"); axs[-1].legend(fontsize=6, loc="lower right")
fig.suptitle("Phase shares, " + cap, fontsize=8)
save(fig, "phases")

# 5. exponents by range
fits = [r for r in rows if r.get("kind") == "fit"]
fig, ax = plt.subplots(figsize=(7.2, 3.8))
arms5 = ["B0", "S", "L", "D", "DL", "plain", "neg"]
for j, rng in enumerate(["low", "high", "full"]):
    for i, a in enumerate(arms5):
        f = [r for r in fits if r["arm"] == a and r["range"] == rng]
        if not f: continue
        f = f[0]; x = i + (j - 1) * 0.25
        ax.errorbar([x], [f["e"][0]], yerr=[[f["e"][0] - f["lo"][0]], [f["hi"][0] - f["e"][0]]], fmt=["v", "^", "o"][j], color=ICC.get(a, RHC.get(a)), capsize=2, ms=4,
                    label={"low": "low k 12..16", "high": "high k 17..22", "full": "full k 12..22"}[rng] if i == 0 else None)
ax.axhline(1.0, color="k", lw=.6, ls=":"); ax.axhline(1.2, color="k", lw=.6, ls="--"); ax.axhline(1.5, color="#c0392b", lw=.6, ls=":")
ax.set_xticks(range(len(arms5))); ax.set_xticklabels(["IC-" + a if a not in RHC else "rho-" + a for a in arms5])
ax.set_ylabel("fitted exponent e (w = 0), 95% bootstrap"); ax.legend(fontsize=7)
ax.set_title("Cost exponent by q range (lines: 1, 6/5, 3/2)\n" + cap, fontsize=8.5); ax.grid(True, axis="y", alpha=.3)
save(fig, "exponents")

# 6. paired improvement vs B0
pr = [r for r in rows if r.get("kind") == "paired" and r["ref"] == "B0"]
fig, ax = plt.subplots(figsize=(6.6, 3.8))
for a in ["S", "L", "D", "DL"]:
    s = sorted([r for r in pr if r["arm"] == a], key=lambda r: r["q"])
    if not s: continue
    q = [r["q"] for r in s]; y = [r["ratio"][0] for r in s]
    ax.errorbar(q, y, yerr=[[a_ - b for a_, b in zip(y, [r["lo"][0] for r in s])], [b - a_ for a_, b in zip(y, [r["hi"][0] for r in s])]], fmt="o-", ms=3, capsize=2, color=ICC[a], label=f"{a} / B0")
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q"); ax.set_ylabel("cost ratio to frozen B0, same instances (w = 0)")
ax.grid(True, which="both", alpha=.3); ax.legend(fontsize=7)
ax.set_title("Each change against the frozen B0 (paired bootstrap over seeds)\n" + cap, fontsize=8.5)
save(fig, "vs_b0")
print("ok")
