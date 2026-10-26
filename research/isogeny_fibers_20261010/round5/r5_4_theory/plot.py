"""Plot recorded values only (conductor.json; cost factors copied from the
recorded tables cited in RESULTS.md). No computation of results here."""
import json, pathlib
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
cond = json.loads((here / "conductor.json").read_text())

fig, ax = plt.subplots(figsize=(7, 4.3))
colors = plt.cm.viridis([i / 6 for i in range(6)])
for c, row in zip(colors, cond["rows"]):
    eps = [float(e) for e in row["eps"]][1:]
    fr = [float(x) for x in row["frac_lmax_ge_p_eps"]][1:]
    he = [float(x) for x in row["heuristic_union_bound"]][1:]
    pts = [(e, f) for e, f in zip(eps, fr) if f > 0]
    ax.semilogy([e for e, _ in pts], [f for _, f in pts], "o-", color=c,
                label=f"p ≈ 2^{row['bits']} ({row['ordinary_traces']} traces)")
    ax.semilogy(eps, he, "--", color=c, lw=0.8)
ax.set_xlabel("ε  (gap prime ℓ ≥ p^ε)")
ax.set_ylabel("fraction of ordinary classes")
ax.set_title("Classes whose conductor index f_π has a prime factor ℓ ≥ p^ε\n"
             "(measured: markers, exact over all traces; dashed: Σ(1+(p/ℓ))/ℓ² heuristic)",
             fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.tight_layout()
fig.savefig(here / "conductor_gap.svg")

# cost factors: law vs recorded (ranges as error bars)
rows = [
    ("§15.2 census T-closed\nΩ=F, μ=3", 3.0, 2.59, 2.85, 2.85),
    ("16.4 sweep over F\nΩ=F, μ=3 (derived)", 3.0, 2.67, 3.62, 3.0),
    ("16.4 quotient sweep\nΩ=F/M, μ=3", 1.0, 0.93, 1.07, 1.0),
    ("A1-R u/x per relation\nΩ=F, μ=2", 3.02, 2.95, 2.95, 2.95),
    ("A1-R u-proj/tz-proj\nper trial (F vs F/M)", 2.0, 2.01, 2.01, 2.01),
    ("R5.4 char-2 census\nhits F / hits Ω (30 bases)", 2.0, 2.0, 2.0, 2.0),
]
fig, ax = plt.subplots(figsize=(8, 4.3))
xs = range(len(rows))
for x, (lab, law, lo, hi, mid) in zip(xs, rows):
    ax.plot([x - 0.3, x + 0.3], [law, law], color="#c0392b", lw=2,
            label="law (Prop. 15.2′)" if x == 0 else None)
    ax.errorbar([x], [mid], yerr=[[mid - lo], [hi - mid]], fmt="o", color="#2c3e50",
                capsize=4, label="recorded (range)" if x == 0 else None)
ax.set_xticks(list(xs))
ax.set_xticklabels([r[0] for r in rows], fontsize=7)
ax.set_ylabel("cost or hit ratio")
ax.set_title("Saturation cost factor by enumeration domain: law vs recorded values", fontsize=9)
ax.legend(fontsize=8)
ax.grid(alpha=0.3, axis="y")
fig.tight_layout()
fig.savefig(here / "cost_factors.svg")
