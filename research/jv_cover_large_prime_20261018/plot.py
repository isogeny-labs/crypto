"""Draws the recorded (yields.json, curves.json) and derived (cost.json) numbers.
It computes nothing: every value plotted is read from those files, produced by
examples/jv_lp_model.rs.   plot.py <raw dir> <out dir>"""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

raw, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
cost = json.load(open(f"{raw}/cost.json"))
curves = json.load(open(f"{raw}/curves.json"))
yields = json.load(open(f"{raw}/yields.json"))
C = ["#1f77b4", "#d95f02", "#1b9e77", "#7570b3", "#e7298a", "#666666"]
plt.rcParams.update({"font.size": 9, "axes.grid": True, "grid.alpha": 0.25})

def save(fig, name):
    fig.savefig(f"{out}/{name}.svg"); fig.savefig(f"{out}/{name}.png", dpi=150); plt.close(fig)

# 1. S(alpha)/S_full
fig, ax = plt.subplots(1, 2, figsize=(10, 3.8))
groups = [([101, 251, 1009, 4096, 12000], "p <= 12k (the crossing range): derived"),
          ([1e6, 1e7, 1e8, 1e9], "p >= 1e6: derived, f < 0.05 extrapolated")]
for a, (ps, title) in zip(ax, groups):
    for i, p in enumerate(ps):
        row = next(r for r in cost["table"] if r["p"] == p)
        for sch, ls in (("single", "-"), ("double", "--")):
            sw = row["schemes"][sch]["sweep"]
            a.plot([s["alpha"] for s in sw], [s["S_over_Sfull"] for s in sw], ls, color=C[i],
                   label=f"p={p:g} {sch}" if sch == "single" else None, lw=1.4)
    a.axhline(1.0, color="k", lw=0.8); a.axhline(0.9, color="r", lw=0.8, ls=":")
    a.set_xlabel("alpha = log|F'| / log|F|"); a.set_ylabel("S(alpha) / S_full")
    a.set_title(title, fontsize=9)
    a.set_ylim(0.0 if ps[0] > 1e5 else 0.995, 1.3 if ps[0] > 1e5 else 1.15)
    a.set_xlim(0.45, 1.02) if ps[0] > 1e5 else a.set_xlim(0.85, 1.005)
    a.legend(fontsize=7)
fig.suptitle("Cost with a sub-base F' (solid: 1 large prime, dashed: 2); red dotted = the 10 % gate", fontsize=9)
fig.tight_layout(); save(fig, "fig_cost_vs_alpha")

# 2. measured vs modelled yields
fig, ax = plt.subplots(1, len(yields), figsize=(5.2 * len(yields), 3.8), squeeze=False)
for a, y in zip(ax[0], yields):
    fs = [r["f"] for r in y["yields"]]
    for k, (lab, c) in enumerate((("0 outside (full)", C[0]), ("1 outside (1-partial)", C[1]), ("2 outside (2-partial)", C[2]))):
        m = [r["cells"][k]["measured_prefix"] for r in y["yields"]]
        lo = [r["cells"][k]["wilson_lo"] for r in y["yields"]]
        hi = [r["cells"][k]["wilson_hi"] for r in y["yields"]]
        h = [r["cells"][k]["model_hypergeometric"] for r in y["yields"]]
        a.plot(fs, h, "-", color=c, lw=1.2)
        a.errorbar(fs, m, yerr=[[max(0, x - l) for x, l in zip(m, lo)], [max(0, u - x) for x, u in zip(m, hi)]], fmt="o", ms=3.5, color=c, label=lab, capsize=2)
    n = y["yields"][0]["n_decompositions"]
    a.set_title(f"p={y['p']}: {int(n)} real decompositions, {y['seeds']} seeds", fontsize=9)
    a.set_xlabel("f = |F'|/|F|  (F' = first f|F| base elements)"); a.set_ylabel("share of decompositions")
    a.legend(fontsize=7)
fig.suptitle("Measured (points, Wilson 95 %) vs modelled (lines, hypergeometric) yield of the six points", fontsize=9)
fig.tight_layout(); save(fig, "fig_yields")

# 3. solutions needed X/N
c = curves[-1]
fig, a = plt.subplots(figsize=(5.6, 3.8))
for sch, col in (("single", C[0]), ("double", C[1]), ("generic", C[2])):
    pts = [(r["f"], r[sch]["mean"]) for r in c["rows"] if r[sch]["mean"] is not None]
    a.plot(*zip(*pts), "o-", ms=3, color=col, label={"single": "1 large prime", "double": "2 large primes", "generic": "any elimination (bound)"}[sch])
a.axhline(1.0, color="k", lw=0.8, label="plain: N+1")
a.set_yscale("log"); a.set_xlabel("f = |F'|/|F|"); a.set_ylabel("solutions needed / N")
a.set_title(f"Monte Carlo, N={c['N']} (uniform 6-subsets); points: mean of {c['reps']} trials", fontsize=9)
a.legend(fontsize=7); fig.tight_layout(); save(fig, "fig_solutions_needed")

# 4. LA share
fig, a = plt.subplots(figsize=(5.6, 3.6))
ps = [r["p"] for r in cost["table"]]; sh = [r["LA_share_plain"] for r in cost["table"]]
a.loglog(ps, sh, "o-", color=C[0]); a.axvspan(53, 12000, color="#bbbbbb", alpha=0.35, label="p <= 12k")
for k, v in cost["p_at_LA_share"].items():
    a.plot([v], [float(k.split("_")[-1])], "s", color=C[1])
    a.annotate(f"{float(k.split('_')[-1]):.0%} at p={v:.2g}", (v, float(k.split('_')[-1])), fontsize=7, xytext=(4, -10), textcoords="offset points")
a.set_xlabel("p  (N = p/2)"); a.set_ylabel("linear-algebra share of S (plain)")
a.set_title("Where the LA starts to matter (derived)", fontsize=9); a.legend(fontsize=7)
fig.tight_layout(); save(fig, "fig_la_share")
print("figures in", out)
