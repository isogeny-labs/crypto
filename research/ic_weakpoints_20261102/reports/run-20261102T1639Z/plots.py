"""Layout only: plots values recorded in ../data/summary*.jsonl (computed by the Rust tool).
No statistic is computed here."""
import json, math, sys
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).parent
D = HERE.parent.parent / "data"
S = [json.loads(l) for l in (D / "summary.jsonl").read_text().splitlines()]
S2 = [json.loads(l) for l in (D / "summary_y2.jsonl").read_text().splitlines()]
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False})
C = {"n2": "#2a6fb0", "n3": "#c0392b", "full": "#c0392b", "lp1": "#2a6fb0", "g": "#555"}
RUN = "run 20261102T1639Z, tool 1a572f0607+"


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# fig1: solving degree and Macaulay cost vs p
alg = [s for s in S if s["summary"] == "algebra"]
fig, ax = plt.subplots(1, 2, figsize=(7.2, 2.8))
for n in (2, 3):
    a = [s for s in alg if s["n"] == n]
    x = [math.log2(s["p"]) for s in a]
    ax[0].plot(x, [s["sd_median"] for s in a], "o-", color=C[f"n{n}"], label=f"n={n}: solving degree")
    ax[0].fill_between(x, [s["sd_min"] for s in a], [s["sd_max"] for s in a], color=C[f"n{n}"], alpha=.2)
    ax[0].plot(x, [s["qdim_max"] for s in a], "s--", color=C[f"n{n}"], ms=3, alpha=.6, label=f"n={n}: quotient dim (max)")
    ax[1].plot(x, [s["c_mults_mean"] for s in a], "o-", color=C[f"n{n}"], label=f"n={n}")
ax[0].set_yscale("log", base=2)
ax[0].set_xlabel("log2 p")
ax[0].set_ylabel("degree / dimension")
ax[0].legend(fontsize=7, frameon=False)
ax[0].set_title("Row 1/5: Macaulay solving degree (8 targets per p)", fontsize=9)
ax[1].set_yscale("log")
ax[1].set_xlabel("log2 p")
ax[1].set_ylabel("F_p multiplications at solving degree")
ax[1].set_title("c(n): dense Macaulay elimination cost", fontsize=9)
ax[1].legend(fontsize=7, frameon=False)
fig.text(0.01, 0.01, RUN, fontsize=6, color="#888")
save(fig, "fig1_solving_degree")

# fig2: dispersion index per curve with CI, Y and Y2
yl = [s for s in S if s["summary"] == "yield"] + [dict(s, y2=True) for s in S2 if s["summary"] == "yield"]
fig, ax = plt.subplots(1, 2, figsize=(7.2, 2.8))
for i, s in enumerate(yl):
    col = "#2a6fb0" if s.get("y2") else "#c0392b"
    ax[0].errorbar(i, s["dispersion"], yerr=[[s["dispersion"] - s["dispersion_ci"][0]], [s["dispersion_ci"][1] - s["dispersion"]]], fmt="o", color=col, ms=3)
    ax[1].errorbar(i, s["mean_over_lambda"], yerr=[[s["mean_over_lambda"] - s["mean_ci"][0] / s["lambda_pred"]], [s["mean_ci"][1] / s["lambda_pred"] - s["mean_over_lambda"]]], fmt="o", color=col, ms=3)
for a in ax:
    a.axhline(1, color="#888", lw=.8)
    a.set_xticks(range(len(yl)))
    a.set_xticklabels([f"{s['p']}/{s['curve']}" for s in yl], rotation=90, fontsize=6)
    a.set_xlabel("p / curve (red: frozen arm Y, blue: follow-up Y2)")
ax[0].set_yscale("log")
ax[0].set_ylabel("var / mean of decompositions per target")
ax[0].set_title("Row 3: dispersion index, 95% bootstrap CI", fontsize=9)
ax[1].set_ylabel("mean / lambda_pred")
ax[1].set_title("Row 3: mean yield vs F^3/(6N)", fontsize=9)
fig.text(0.01, 0.01, RUN + "; 2000 (Y) / 6000 (Y2) targets per curve", fontsize=6, color="#888")
save(fig, "fig2_yield_dispersion")

# fig3: factor-base fraction
fig, ax = plt.subplots(figsize=(4.2, 2.8))
for arm, src, mk in (("Y", S, "o"), ("Y2", S2, "s")):
    ph = [s for s in src if s["summary"] == "phi_pooled"]
    for kind in ("full", "lp1"):
        x = [s["phi"] * (1.04 if arm == "Y2" else 1) for s in ph]
        y = [s[f"{kind}_ratio"] for s in ph]
        lo = [s[f"{kind}_ratio"] - s[f"{kind}_ci"][0] for s in ph]
        hi = [s[f"{kind}_ci"][1] - s[f"{kind}_ratio"] for s in ph]
        ax.errorbar(x, y, yerr=[lo, hi], fmt=mk + "-", color=C[kind], ms=4, alpha=1 if arm == "Y2" else .5,
                    label=f"{arm} {'full relation' if kind == 'full' else 'one large prime'}")
ax.axhline(1, color="#888", lw=.8)
ax.set_xscale("log", base=2)
ax.set_xlabel("factor-base fraction phi")
ax.set_ylabel("observed / exact hypergeometric expectation")
ax.set_title("Rows 2/7: yield vs factor-base size (p >= 251, pooled)", fontsize=9)
ax.legend(fontsize=6, frameon=False)
save(fig, "fig3_phi")

# fig4: cost band
pl = [s for s in S if s["summary"] == "pipeline"]
md = [s for s in S if s["summary"] == "model"]
fig, ax = plt.subplots(1, 2, figsize=(7.2, 2.8))
for wi, (w, col) in enumerate(((0, "#2a9d55"), (0.1, "#e08e0b"), (1, "#c0392b"))):
    ax[0].scatter([math.log2(s["ell"]) for s in pl], [s["ratio_negmap_w"][wi] for s in pl], color=col, s=12, label=f"w = {w}")
ax[0].axhline(1, color="#888", lw=.8)
ax[0].set_yscale("log")
ax[0].set_xlabel("log2 ell (prime subgroup)")
ax[0].set_ylabel("IC / rho (negation-map reference)")
ax[0].set_title("Row 6: complete toy solves, 15 curves (measured)", fontsize=9)
ax[0].legend(fontsize=7, frameon=False)
for m, col in zip(md, ("#e08e0b", "#c0392b")):
    g = m["grid"]
    ax[1].plot([r["log2_group"] for r in g], [r["ratio"] for r in g], "-", color=col, label=f"w = {m['w']}")
ax[1].axhline(1, color="#888", lw=.8)
ax[1].set_yscale("log")
ax[1].set_xlabel("log2 group order (p^3)")
ax[1].set_ylabel("IC / rho, best phi, no large primes")
ax[1].set_title("DERIVED model from measured tau, c(3), LA", fontsize=9)
ax[1].legend(fontsize=7, frameon=False)
fig.text(0.01, 0.01, RUN, fontsize=6, color="#888")
save(fig, "fig4_cost_band")
print("ok")

# fig5: large-prime variation (arm L, Amendment 2) -- only if its summary exists
SD = D / "summary_dlp.jsonl"
if SD.exists():
    L = [json.loads(l) for l in SD.read_text().splitlines()]
    cells = [s for s in L if s["summary"] == "dlp_cell"]
    fits = {(s["r"], s["lp"]): s for s in L if s["summary"] == "dlp_fit"}
    models = [s for s in L if s["summary"] == "dlp_model"]
    fig, ax = plt.subplots(1, 3, figsize=(10.5, 3.0))
    names = {0: "0 large primes", 1: "1 large prime", 2: "2 large primes"}
    cols = {0: "#888888", 1: "#2a6fb0", 2: "#c0392b"}
    for lp in (0, 1, 2):
        c = [s for s in cells if abs(s["r"] - 2 / 3) < 1e-3 and s["lp"] == lp]
        x = [math.log2(s["p"]) for s in c]
        y = [s["T_geomean"] for s in c]
        mk = ["o" if s["all_reached"] else "x" for s in c]
        for xi, yi, m in zip(x, y, mk):
            ax[0].scatter(xi, yi, color=cols[lp], marker=m, s=18)
        ax[0].plot(x, y, color=cols[lp], lw=.8, label=names[lp] + (f"  slope {fits[(c[0]['r'], lp)]['alpha']:.2f}" if c and (c[0]['r'], lp) in fits else "  (censored)"))
    ax[0].set_yscale("log")
    ax[0].set_xlabel("log2 p")
    ax[0].set_ylabel("trials to |FB|+21 full rows")
    ax[0].set_title("|FB| ~ p^(2/3)/2: trials (x = censored)", fontsize=9)
    ax[0].legend(fontsize=6, frameon=False)
    rs = sorted({s["r"] for s in fits.values()}, reverse=True)
    for lp in (0, 1, 2):
        pts = [(r, fits[(r, lp)]["alpha"], fits[(r, lp)]["alpha_se"]) for r in rs if (r, lp) in fits]
        if pts:
            ax[1].errorbar([q[0] for q in pts], [q[1] for q in pts], yerr=[2 * q[2] for q in pts], fmt="o-", color=cols[lp], ms=4, label=names[lp])
    rr = [0.5, 1.0]
    ax[1].plot(rr, [3 - 2 * r for r in rr], ":", color="#888", label="0LP law 3-2r")
    ax[1].axhline(1.5, color="#2a9d55", lw=.8, ls="--", label="rho exponent 1.5")
    ax[1].axhline(4 / 3, color="#c0392b", lw=.6, ls="--", label="GTTD 4/3")
    ax[1].set_xlabel("factor-base scaling r (|FB| ~ p^r / 2)")
    ax[1].set_ylabel("fitted exponent of trials in p (+-2 se)")
    ax[1].set_title("Trial exponents by variant", fontsize=9)
    ax[1].legend(fontsize=6, frameon=False)
    for m, ls in zip(models, ("-", "--")):
        for lp in (0, 1, 2):
            ax[2].plot([g["log2_group"] for g in m["grid"]], [g["variants"][lp]["ratio"] for g in m["grid"]], ls, color=cols[lp], lw=1,
                       label=f"{names[lp]}, w={m['w']}")
    ax[2].axhline(1, color="#888", lw=.8)
    ax[2].set_yscale("log")
    ax[2].set_xlabel("log2 group order (p^3)")
    ax[2].set_ylabel("IC / rho (negation map), best r")
    ax[2].set_title("DERIVED: fitted laws + measured c(3)", fontsize=9)
    ax[2].legend(fontsize=5.5, frameon=False, ncol=2)
    fig.text(0.01, 0.01, "arm L (Amendment 2), seed 20261104, 2 curves per p", fontsize=6, color="#888")
    save(fig, "fig5_large_primes")
    print("fig5 ok")
