"""Lay out analysis/summary.json as SVG + 150 dpi PNG. Plotting only: every
number comes from summary.json (scripts/summary.sh, jq over ecbench output)."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
d = json.loads((here / "analysis" / "summary.json").read_text())
out = here / "figures"
out.mkdir(exist_ok=True)
rows = d["rows"]
x = [r["log2_r"] for r in rows]
NOTE = "ecbench GAE, S = GAE/sqrt(r); 24 verified runs per arm and curve; registered prime curves (README)."


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


fig, ax = plt.subplots(figsize=(6.6, 4.2))
styles = {"plan": ("#4f81bd", "s-", "plan (pair table, N0 abscissae)"), "resN0": ("#7f7f7f", "s--", "residual, N0"),
          "res64": ("#e6b8b7", "o-", "residual, 64"), "res16": ("#da9694", "o-", "residual, 16"),
          "res4": ("#c0504d", "o-", "residual, 4"), "rho": ("black", "^-", "rho.negation")}
for arm, (c, st, lab) in styles.items():
    ax.plot(x, [r["mean_s"][arm][0] for r in rows], st, color=c, ms=4, label=lab)
ax.set_yscale("log")
ax.set_xlabel("log2 r")
ax.set_ylabel("mean S at w = 0 (counted)")
ax.set_title("Residual collector against the plan and rho, counted unit", fontsize=9)
ax.legend(fontsize=6.5)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "s_by_size")

fig, ax = plt.subplots(figsize=(6.6, 4.0))
for k, (w, c) in enumerate((("0", "#4f81bd"), ("0.1", "#9bbb59"), ("1", "#c0504d"))):
    xs = [v + (k - 1) * 0.15 for v in x]
    ys = [r["res4_over_plan"][w]["ratio"] for r in rows]
    lo = [y - r["res4_over_plan"][w]["ci95"][0] for y, r in zip(ys, rows)]
    hi = [r["res4_over_plan"][w]["ci95"][1] - y for y, r in zip(ys, rows)]
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o-", color=c, capsize=2, ms=4, label=f"w = {w}")
ax.axhline(1, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xlabel("log2 r")
ax.set_ylabel("S_w(residual, 4) / S_w(plan), paired, 95% interval")
ax.set_title("The order reverses inside the AGENTS.md §5a band at every size", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE + " w = GAE per unpriced native operation.", fontsize=5)
save(fig, "band_reversal")

fig, ax = plt.subplots(figsize=(6.0, 3.6))
nl = d["null"]
ax.errorbar(range(len(nl)), [n["ratio"] for n in nl],
            yerr=[[n["ratio"] - n["ci95"][0] for n in nl], [n["ci95"][1] - n["ratio"] for n in nl]],
            fmt="o", color="#4f81bd", capsize=3)
ax.axhline(1, color="black", lw=0.8)
ax.set_xticks(range(len(nl)))
ax.set_xticklabels([n["case"].replace("null-", "") for n in nl], fontsize=6.5, rotation=30)
ax.set_ylabel("curve-free / curve group operations")
ax.set_title("Curve-free null: the collector costs the same on Z/r", fontsize=9)
ax.grid(alpha=0.3, axis="y")
fig.text(0.01, 0.005, "64 solves per case on each carrier, same seeds; bootstrap 95% interval of the ratio of means.", fontsize=5.5)
save(fig, "curve_free_null")
print("wrote", sorted(p.name for p in out.iterdir()))
