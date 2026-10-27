"""Lay out analysis/summary.json as SVG + 150 dpi PNG. Plotting only: every
number comes from summary.json (examples/ic_planner_exhaustive_summary.rs over ecbench output)."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
d = json.loads((here / "analysis" / "summary.json").read_text())
T, P = d["tables"], d["predictions"]
out = here / "figures"
out.mkdir(exist_ok=True)
NOTE = "ecbench paired compare, cluster 95% intervals; S = GAE/sqrt(r); every run verified (README)."


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


def err(c):
    return [[c["ratio"] - c["ci95"][0]], [c["ci95"][1] - c["ratio"]]]


# 1. The residual ladder: res4/plan at w = 0, 0.1, 1 over ten sizes.
fig, ax = plt.subplots(figsize=(6.6, 4.2))
lad = sorted(T["res4_ladder"], key=lambda r: r["log2_r"])
for w, c, lab in (("0", "#c0504d", "w = 0 (counted)"), ("0.1", "#e46c0a", "w = 0.1"), ("1", "#4f81bd", "w = 1")):
    xs = [r["log2_r"] for r in lad]
    ys = [r["res4_over_plan"][w]["ratio"] for r in lad]
    lo = [r["res4_over_plan"][w]["ratio"] - r["res4_over_plan"][w]["ci95"][0] for r in lad]
    hi = [r["res4_over_plan"][w]["ci95"][1] - r["res4_over_plan"][w]["ratio"] for r in lad]
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o-", color=c, ms=4, capsize=2, label=lab)
for r in lad:
    if r["source"] == "ic_planner_exhaustive_20261026":
        ax.axvspan(r["log2_r"] - 0.15, r["log2_r"] + 0.15, color="#eeeeee", zorder=0)
ax.axhline(1, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xlabel("log2 r (grey: new sizes in this study)")
ax.set_ylabel("S_w(res4) / S_w(plan)")
ax.set_title("Residual collector (B = 4) against the plan across the §5a band, ten sizes", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "residual_ladder")

# 2. Held-out weight rules: alt/plan at w = 0 and w = 1 per curve.
fig, axes = plt.subplots(1, 3, figsize=(9.6, 3.6), sharey=False)
groups = [
    ("W1 prime: 1.41·N₀ / N₀", [(s, T["R"][s]["over_plan"]["m141"]) for s in T["R"]]),
    ("W2 binary: (d₀+1) / d₀", [(s, T["B"][s]["over_plan"]["dp1"]) for s in T["B"]]),
    ("W3 Koblitz: 0.71·c₀ / c₀", [(s, T["K"][s]["p71_over_pipeline"]) for s in T["K"]]),
]
for ax, (title, rows) in zip(axes, groups):
    for i, (s, cw) in enumerate(rows):
        for w, c, dx in (("0", "#c0504d", -0.12), ("1", "#4f81bd", 0.12)):
            ax.errorbar([i + dx], [cw[w]["ratio"]], yerr=err(cw[w]), fmt="o", color=c, ms=4, capsize=2,
                        label=f"w = {w}" if i == 0 else None)
    ax.axhline(1, color="black", lw=0.8)
    ax.set_xticks(range(len(rows)))
    ax.set_xticklabels([s.split("-")[1] for s, _ in rows], fontsize=7)
    ax.set_title(title, fontsize=8.5)
    ax.grid(alpha=0.3)
axes[0].set_ylabel("paired S ratio (alt / plan)")
axes[0].legend(fontsize=7)
fig.text(0.01, 0.005, NOTE + " Below 1: the alternative is cheaper.", fontsize=5.5)
save(fig, "weight_rules_heldout")

# 3. Koblitz routing: descent/pipeline and both against rho, by size.
fig, ax = plt.subplots(figsize=(6.6, 4.0))
ks = sorted(T["K"], key=lambda s: T["K"][s]["log2_r"])
x = [T["K"][s]["log2_r"] for s in ks]
for key, c, lab in (("descent_over_pipeline", "#7f7f7f", "descent / pipeline"),
                    ("pipeline_over_rho", "#4f81bd", "pipeline / rho (w = 0)"),
                    ("descent_over_rho", "#c0504d", "descent / rho (w = 0)")):
    cs = [T["K"][s][key]["0"] if key == "pipeline_over_rho" else T["K"][s][key] for s in ks]
    ax.errorbar(x, [c_["ratio"] for c_ in cs],
                yerr=[[c_["ratio"] - c_["ci95"][0] for c_ in cs], [c_["ci95"][1] - c_["ratio"] for c_ in cs]],
                fmt="o-", color=c, ms=4, capsize=2, label=lab)
ax.axhline(1, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xlabel("log2 r")
ax.set_ylabel("paired S ratio, counted")
ax.set_title("Koblitz: §23 descent, the ic plan pipeline and rho.signed_frobenius_strong", fontsize=9)
ax.legend(fontsize=7)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, NOTE, fontsize=5.5)
save(fig, "koblitz_routing")

# 4. Curve-free null: 16 ratios with bootstrap intervals.
fig, ax = plt.subplots(figsize=(6.6, 3.6))
cases = sorted(P["N1"]["cases"], key=lambda c: (c["r"], c["base"]))
for i, c in enumerate(cases):
    col = "#c0504d" if c["excludes_1"] else "#4f81bd"
    ax.errorbar([i], [c["ratio"]], yerr=err(c), fmt="o", color=col, ms=4, capsize=2)
ax.axhline(1, color="black", lw=0.8)
ax.set_xticks(range(len(cases)))
ax.set_xticklabels([c["case"].replace("null-", "") for c in cases], rotation=60, fontsize=6)
ax.set_ylabel("mean group ops, curve-free / curve")
ax.set_title(f"Curve-free null: {P['N1']['excluding_1']} of {P['N1']['n']} intervals exclude 1 (red)", fontsize=9)
ax.grid(alpha=0.3)
fig.text(0.01, 0.005, "128 solves per carrier and case, seed 20261107; bootstrap 95% (4000 resamples).", fontsize=5.5)
save(fig, "curve_free_null")
print("figures:", sorted(p.name for p in out.iterdir()))
