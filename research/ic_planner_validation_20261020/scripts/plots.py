"""Lay out analysis/summary.json as SVG + 150 dpi PNG. Plotting only: every
number comes from summary.json (scripts/summary.sh, jq over the ecbench output).
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
rows = json.loads((here / "analysis" / "summary.json").read_text())
out = here / "figures"
out.mkdir(exist_ok=True)
primary = [r for r in rows if r["curve_tag"].startswith("r")]
label = {r["curve_tag"]: f"{r['curve_tag']} (log2 r {r['log2_r']:.1f})" for r in rows}
ARMS = ["half-base", "double-base", "full-rank", "random-targets", "rootfind"]
COL = {"r24": "#4f81bd", "r26": "#9bbb59", "r28": "#c0504d", "c24": "#a6c0e0", "c28": "#cddcae", "c30": "#e0a8a7"}
NOTE = ("ecbench GAE; 24 measured runs per arm and curve. r24/r26/r28: registered curves (amendment 1, primary);\n"
        "c24/c28/c30: prime_search curves of the original protocol (supplementary, hatched).")


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Every one-change alternative against the plan, in counted S.
fig, ax = plt.subplots(figsize=(7.6, 4.4))
w = 0.13
for k, r in enumerate(rows):
    alts = {a["arm"]: a for a in r["alternatives"]}
    xs = [i + (k - 2.5) * w for i in range(len(ARMS))]
    ys = [alts[a]["ops_ratio"] for a in ARMS]
    lo = [alts[a]["ops_ratio"] - alts[a]["ci95"][0] for a in ARMS]
    hi = [alts[a]["ci95"][1] - alts[a]["ops_ratio"] for a in ARMS]
    ax.bar(xs, ys, w, yerr=[lo, hi], capsize=1.5, color=COL[r["curve_tag"]], label=label[r["curve_tag"]],
           hatch=None if r["curve_tag"].startswith("r") else "///", edgecolor="white", linewidth=0.3)
ax.axhline(1, color="black", lw=0.8)
ax.set_yscale("log")
ax.set_xticks(range(len(ARMS)))
ax.set_xticklabels(ARMS, fontsize=8)
ax.set_ylabel("S(alternative) / S(plan), paired, 95% interval")
ax.set_title("The planner's choice against each one-change alternative (w = 0)", fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax.grid(alpha=0.3, axis="y", which="both")
fig.text(0.01, 0.005, NOTE + " Above 1: the plan is cheaper.", fontsize=5.5)
fig.subplots_adjust(bottom=0.16)
save(fig, "alternatives_vs_plan")

# 2. Base size across the pricing band.
fig, ax = plt.subplots(figsize=(6.4, 4.0))
for r in rows:
    alts = {a["arm"]: a for a in r["alternatives"]}
    lw = 1.6 if r["curve_tag"].startswith("r") else 0.8
    for arm, style in (("half-base", "o--"), ("double-base", "s-")):
        ax.plot(r["weights"], alts[arm]["band_ratio"], style, color=COL[r["curve_tag"]], lw=lw, ms=4,
                label=f"{arm}, {r['curve_tag']}")
ax.axhline(1, color="black", lw=0.8)
ax.set_xscale("symlog", linthresh=0.1)
ax.set_xticks(rows[0]["weights"])
ax.set_xticklabels([str(x) for x in rows[0]["weights"]])
ax.set_xlabel("w: GAE charged per unpriced native operation (AGENTS.md §5a)")
ax.set_ylabel("mean S(alternative) / mean S(plan)")
ax.set_title("Base size: the plan's optimum holds at w = 0 and 0.1, not at w = 1", fontsize=9)
ax.legend(fontsize=5, ncol=3)
ax.grid(alpha=0.3)
fig.text(0.01, 0.005, NOTE, fontsize=5)
save(fig, "base_size_band")

# 3. The planner's predicted gap to rho against the measured one.
fig, ax = plt.subplots(figsize=(5.8, 4.0))
srt = sorted(rows, key=lambda r: r["log2_r"])
ax.plot([r["log2_r"] for r in srt], [r["predicted_plan_over_rho"] for r in srt], "o--", color="#7f7f7f",
        label="planner prediction (ic-planner/1)")
for subset, mk, lab in ((primary, "s", "measured, registered curves"),
                        ([r for r in rows if r not in primary], "D", "measured, supplementary curves")):
    xs = [r["log2_r"] for r in subset]
    m = [r["plan_over_rho"]["ratio"] for r in subset]
    lo = [r["plan_over_rho"]["ratio"] - r["plan_over_rho"]["ci95"][0] for r in subset]
    hi = [r["plan_over_rho"]["ci95"][1] - r["plan_over_rho"]["ratio"] for r in subset]
    ax.errorbar(xs, m, yerr=[lo, hi], fmt=mk, color="#4f81bd" if subset is primary else "#c0504d",
                capsize=3, label=lab + " (S(plan)/S(rho.negation))")
ax.set_xlabel("log2 r")
ax.set_ylabel("S(IC) / S(rho)")
ax.set_title("Distance remaining to rho, predicted and measured", fontsize=9)
ax.legend(fontsize=6)
ax.grid(alpha=0.3)
fig.text(0.01, 0.005, NOTE, fontsize=4.5)
save(fig, "rho_gap_predicted_vs_measured")
print("wrote", sorted(p.name for p in out.iterdir()))
