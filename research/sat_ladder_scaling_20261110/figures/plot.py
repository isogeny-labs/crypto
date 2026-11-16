# F4 figures. This script only draws from results/run.json (recorded values: summary, fits, curves, rows);
# it computes no result. Usage: python plot.py <path-to-results/run.json> <out-dir>
# Run F4-RUN-02, study sat_ladder_scaling_20261110. matplotlib only.
import json, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = json.load(open(sys.argv[1])); out = sys.argv[2]
S = run["summary"]["per_n"]; F = run["fits"]["FRESH"]; budget = run["header"]["conflict_budget"]
curves = {c["n"]: c for c in run["curves"]}; rows = run["rows"]
BLUE, ORANGE, GREY = "#1f5fa8", "#c45a00", "#666666"
L2 = math.log2

# ---------- Figure 1: log2 median conflicts vs n ----------
fig, ax = plt.subplots(figsize=(7.4, 5.0))
ns = [s["n"] for s in S]
for key, cen, col, mk, lab, dx in (("fresh_median_conflicts", "fresh_censored", BLUE, "o", "FRESH (new solver per target)", -0.04),
                                   ("reuse_median_conflicts", "reuse_censored", ORANGE, "s", "REUSE (one incremental solver)", 0.04)):
    for s in S:
        n, m, c = s["n"], s[key], s[cen]
        if m is None:   # median is censored: draw open marker at the budget with an up arrow
            ax.plot(n + dx, L2(budget), mk, mfc="white", mec=col, ms=9, mew=1.6, zorder=3)
            ax.annotate("", xy=(n + dx, L2(budget) + 0.9), xytext=(n + dx, L2(budget)),
                        arrowprops=dict(arrowstyle="->", color=col, lw=1.4))
            ax.annotate(f"{c}/10 cens.", (n + dx, L2(budget)), textcoords="offset points",
                        xytext=(8 if dx > 0 else -8, -16), fontsize=8, color=col, ha="left" if dx > 0 else "right")
        else:
            ax.plot(n + dx, L2(m), mk, color=col, ms=8, zorder=3)
            if c:
                ax.annotate(f"{c}/10 cens.", (n + dx, L2(m)), textcoords="offset points", xytext=(8, -3), fontsize=8, color=col)
    xs = [s["n"] + dx for s in S if s[key] is not None]
    ys = [L2(s[key]) for s in S if s[key] is not None]
    ax.plot(xs, ys, "-", color=col, lw=1, alpha=0.5, label=lab)
xs = [min(F["uncensored_n"]), max(F["uncensored_n"])]
ax.plot(xs, [F["alpha"] * x + F["intercept"] for x in xs], "--", color=BLUE, lw=2,
        label=f"FRESH fit n=4..7: alpha = {F['alpha']:.3f}  [95% CI {F['ci95_lo']:.3f}, {F['ci95_hi']:.3f}]")
n0 = S[0]["n"]; y0 = L2(S[0]["fresh_median_conflicts"])
xr = [n0, ns[-1]]
ax.plot(xr, [y0 + 0.5 * (x - n0) for x in xr], ":", color=GREY, lw=1.8,
        label="reference slope 0.5 (rho exponent) through FRESH n=4; scale only, not a cost comparison")
ax.axhline(L2(budget), color=GREY, lw=0.8, alpha=0.6)
ax.text(ns[0] - 0.2, L2(budget) + 0.25, "budget 4e6 conflicts", fontsize=8, color=GREY)
ax.set_xlabel("n = bits of p (one prime-order curve per n)"); ax.set_ylabel("log2 of median conflicts over 10 targets")
ax.set_xticks(ns); ax.set_ylim(-5.5, 24.5); ax.grid(alpha=0.25)
ax.legend(fontsize=7.5, loc="lower right", frameon=True)
ax.set_title("F4-RUN-02: CDCL conflicts on the x-only ladder (CaDiCaL 3.0.1)\nopen marker + arrow: median censored (lower bound)", fontsize=10)
fig.text(0.01, 0.005, "Source: results/run.json (summary, fits). 10 targets per n and arm; budget 4e6; local shared host. [pcc:unverified]", fontsize=7)
fig.tight_layout(rect=(0, 0.02, 1, 1))
for ext in ("svg", "png"):
    fig.savefig(f"{out}/conflicts_vs_n.{ext}", dpi=160)
plt.close(fig)

# ---------- Figure 2: REUSE conflicts by solve order ----------
floor = 0.5
fig, axes = plt.subplots(2, 3, figsize=(10, 6), sharex=True)
for ax, s in zip(axes.flat, S):
    n = s["n"]
    rr = sorted([r for r in rows if r["n"] == n and r["arm"] == "REUSE"], key=lambda r: r["solve_order"])
    xs = [r["solve_order"] for r in rr]
    ys = [max(r["conflicts"], floor) for r in rr]
    ax.plot(xs, ys, "-", color=ORANGE, lw=0.8, alpha=0.6)
    for r, y in zip(rr, ys):
        z = r["conflicts"] == 0
        ax.plot(r["solve_order"], y, "v" if z else "s", ms=7, color=ORANGE,
                mfc="white" if (r["censored"] or z) else ORANGE, mec=ORANGE, mew=1.4)
    fm = s["fresh_median_conflicts"]
    if fm is not None:
        ax.axhline(fm, color=BLUE, ls="--", lw=1.4)
        ax.text(9.2, fm, "FRESH median", color=BLUE, fontsize=7, ha="right", va="bottom")
    else:
        ax.axhline(budget, color=BLUE, ls="--", lw=1.4)
        ax.text(9.2, budget*1.15, f"FRESH median censored ({s['fresh_censored']}/10)", color=BLUE, fontsize=7, ha="right", va="bottom")
    ax.set_yscale("log"); ax.set_ylim(floor * 0.6, 3e7); ax.grid(alpha=0.25, which="major")
    ax.set_title(f"n = {n}  (r = {curves[n]['r']})", fontsize=9)
    if n >= 7: ax.set_xlabel("solve_order (0 = first solve)")
    if n in (4, 7): ax.set_ylabel("REUSE conflicts (log)")
    ax.set_xticks(range(10))
fig.suptitle("F4-RUN-02: REUSE conflicts per solve, by solve order. Filled square: solved; open square: censored at 4e6; "
             "open triangle at the floor: 0 conflicts (plotted at 0.5)", fontsize=9)
fig.text(0.01, 0.005, "Source: results/run.json (rows). 10 targets per n; FRESH median from summary. [pcc:unverified]", fontsize=7)
fig.tight_layout(rect=(0, 0.02, 1, 0.95))
for ext in ("svg", "png"):
    fig.savefig(f"{out}/reuse_by_order.{ext}", dpi=160)
