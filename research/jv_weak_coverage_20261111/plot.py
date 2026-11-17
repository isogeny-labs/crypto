#!/usr/bin/env python3
"""Coverage of the JV weak class by prime: admitted share of uniform curves (U) and of full-2-torsion curves (F),
the implemented walk's reach on admitted classes (W), the census truth at small p, and operational coverage."""
import json
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
a = json.load(open("analysis.json"))
ps = [p for p in ("37", "53", "1009", "1511", "1823")]
x = range(len(ps))
fig, ax = plt.subplots(figsize=(8, 4.4))
def series(key, sub, ci, color, label, dx, marker):
    pts = [(i, a[p][key][sub], a[p][key][ci]) for i, p in enumerate(ps) if key in a[p] and a[p][key].get("n", a[p][key].get("walked", 0))]
    for i, v, (lo, hi) in pts:
        ax.errorbar(i + dx, v, yerr=[[v - lo], [hi - v]], fmt=marker, color=color, capsize=3,
                    label=label if i == pts[0][0] else None)
series("F", "admitted", "admitted_ci", "#7f8c8d", "full-2-torsion curves: admitted", -0.15, "s")
series("U", "admitted", "admitted_ci", "#2471a3", "uniform curves: admitted (upper bound on coverage)", -0.05, "o")
series("W", "reached_frac", "reached_ci", "#c0392b", "admitted classes: implemented walk reaches a weak curve", 0.05, "v")
ax.plot([0, 1], [1 - 290 / 24642, 1 - 494 / 73034], "*", color="#27ae60", ms=11, label="admitted classes holding a weak curve (exact census, ledger §18.6)")
ax.set_xticks(list(x)); ax.set_xticklabels([f"p = {p}" for p in ps]); ax.set_ylim(-0.03, 1.4)
ax.set_ylabel("fraction (Wilson 95 %)"); ax.grid(alpha=0.3); ax.legend(fontsize=7.5, loc="upper center", ncol=2)
ax.set_title("How often a curve over F_{p^6} lands in a Joux–Vitse weak class", fontsize=10)
fig.tight_layout(); fig.savefig("coverage.svg"); fig.savefig("coverage.png", dpi=200)
