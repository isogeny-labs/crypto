"""Layout only: reads summary.json (computed by the native `summarize` binary)
and draws the yield-ratio and cost-ratio panels. No research computation here."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = sys.argv[1]
s = json.load(open(f"{run}/summary.json"))
fig, ax = plt.subplots(1, 2, figsize=(11, 4.3))

# (a) yield ratio tw/std: observed against the exact-count prediction
a = ax[0]
colors = {"A1": "#1f77b4", "A2": "#ff7f0e", "B1": "#2ca02c", "B2": "#d62728", "B3": "#9467bd", "A0": "#7f7f7f"}
for r in s["yield_ratio"]:
    if r["curve"] == "A0":
        continue
    x = r["k"] + (0.12 if r["arm"].startswith("tw_b") else -0.12) + {"A1": -0.06, "A2": -0.02, "B1": 0.02, "B2": 0.06, "B3": 0.1}.get(r["curve"], 0)
    c = colors[r["curve"]]
    a.errorbar([x], [r["obs"]], yerr=[[r["obs"] - r["lo"]], [r["hi"] - r["obs"]]], fmt="o" if r["arm"].startswith("tw_a") else "s",
               color=c, ms=4, lw=1, alpha=0.9)
    a.plot([x - 0.04, x + 0.04], [r["pred"]] * 2, color="k", lw=1.5)
a.axhline(16, color="grey", ls=":", lw=0.8)
a.set_yscale("log")
a.set_xlabel("k  (base field F_Q, Q = 2^k; N = 4k)")
a.set_ylabel("yield ratio  twisted line / standard base")
a.set_title("(a) exact relation yield, m = 4\nmarkers: measured with 95% CI; black bar: exact-count prediction", fontsize=9)
a.set_xticks(sorted({r["k"] for r in s["yield_ratio"]}))
for n, c in colors.items():
    if n != "A0":
        a.plot([], [], "o", color=c, label=n)
a.legend(fontsize=7, ncol=3, loc="upper right")

# (b) cost ratio
b = ax[1]
b.axhspan(0.8, 1.25, color="#cfe8cf", alpha=0.6, label="preregistered tolerance [0.8, 1.25]")
for fam, c, off in (("A", "#1f77b4", -0.12), ("B", "#2ca02c", 0.0), ("all", "k", 0.12)):
    rows = [r for r in s["cost_by_k"] if r["family"] == fam]
    ks = [r["k"] + off for r in rows]
    g = [r["geo"] for r in rows]
    b.errorbar(ks, g, yerr=[[r["geo"] - r["lo"] for r in rows], [r["hi"] - r["geo"] for r in rows]], fmt="o-", color=c, ms=4, lw=1,
               label={"A": "family A (a,b in F_16): tw/std", "B": "family B (generic b): tw/std", "all": "all curves: tw/std"}[fam])
rows = [r for r in s["cost_by_k"] if r["family"] == "all" and r["aa_geo"] is not None]
b.errorbar([r["k"] + 0.24 for r in rows], [r["aa_geo"] for r in rows],
           yerr=[[r["aa_geo"] - r["aa_lo"] for r in rows], [r["aa_hi"] - r["aa_geo"] for r in rows]], fmt="^--", color="#888888", ms=4, lw=1,
           label="A/A: tw_a / tw_b (noise reference)")
b.axhline(1, color="k", lw=0.6)
b.set_xlabel("k  (base field F_Q, Q = 2^k; N = 4k)")
b.set_ylabel("counted F_Q mults, twisted / standard (D = 26)")
b.set_title("(b) Weil-descended system, weighted Macaulay elimination\ngeometric mean over targets, 95% bootstrap (closed systems only)", fontsize=9)
b.legend(fontsize=7, loc="best")
b.set_xticks(sorted({r["k"] for r in s["cost_by_k"]}))
fig.tight_layout()
fig.savefig(f"{run}/fig_c1_ladder.svg")
fig.savefig(f"{run}/fig_c1_ladder.png", dpi=150)
print("wrote", f"{run}/fig_c1_ladder.svg")
