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
NOTE = "ecbench GAE; 24 verified runs per arm and curve; w = GAE per unpriced native operation (AGENTS.md §5a)."
cols = ["#4f81bd", "#9bbb59", "#f79646", "#8064a2", "#c0504d"]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


fig, axes = plt.subplots(1, 2, figsize=(10, 4.0))
for ax, fam, title in ((axes[0], "binary", "binary-subspace dimension (m = 2^k)"),
                       (axes[1], "koblitz", "Koblitz orbit columns (m · c0)")):
    for c, g in zip(cols, d["grid"][fam]):
        ax.plot([p["m"] for p in g["grid"]], [p["G"] for p in g["grid"]], "o-", ms=3.5, color=c,
                label=f"w = {g['w']} (m* = {g['best']['m']})")
    ax.axhline(1, color="black", lw=0.8)
    ax.set_xscale("log", base=2)
    ax.set_yscale("log")
    ax.set_xlabel("base multiplier m")
    ax.set_ylabel("G(m, w)")
    ax.set_title(f"Phase A, {title}", fontsize=9)
    ax.legend(fontsize=6.5)
    ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE + " G = geometric mean over three curves of S_w(m)/S_w(plan).", fontsize=5.5)
save(fig, "derivation_grid")

fig, ax = plt.subplots(figsize=(6.2, 3.8))
rows = d["koblitz_holdout"]
for k, (w, c) in enumerate((("0", "#7f7f7f"), ("0.3", "#f79646"), ("1", "#c0504d"))):
    xs = [i + (k - 1) * 0.22 for i in range(len(rows))]
    ys = [r["alt_over_plan"][w]["ratio"] for r in rows]
    lo = [y - r["alt_over_plan"][w]["ci95"][0] for y, r in zip(ys, rows)]
    hi = [r["alt_over_plan"][w]["ci95"][1] - y for y, r in zip(ys, rows)]
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o", color=c, capsize=3, label=f"w = {w}")
ax.axhline(1, color="black", lw=0.8)
ax.set_xticks(range(len(rows)))
ax.set_xticklabels([f"{r['tag']} ({r['alternative']})\n{r['slug']}" for r in rows], fontsize=6)
ax.set_ylabel("S_w(0.71·c0 columns) / S_w(plan), paired, 95% interval")
ax.set_title("Koblitz held out: the smaller base wins once native work is charged", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, axis="y")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "koblitz_holdout")
print("wrote", sorted(p.name for p in out.iterdir()))
