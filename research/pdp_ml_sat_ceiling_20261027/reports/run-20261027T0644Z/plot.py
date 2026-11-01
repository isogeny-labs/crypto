"""Lay out recorded values from run1/ as SVG + 150 dpi PNG. Computes nothing new.

Usage: python plot.py   (from this directory; needs matplotlib)
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
RUN = HERE.parent.parent / "run1"
A = RUN / "analysis"
FAM = ["n15l5", "n17l6"]
TRIPLES = {"n15l5": 32 * 33 * 34 // 6, "n17l6": 64 * 65 * 66 // 6}
GROUPS = ["F-x", "F-xx", "F-tel64", "F-tel512"]
C = {"n15l5": "#2a6fb0", "n17l6": "#d0702a", "ctrl": "#6b6b6b", "ceil": "#3a9a5b"}
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


summ = {f: json.loads((A / f"summary_{f}.json").read_text()) for f in FAM}
l3 = {f: json.loads((A / f"l3_{f}.json").read_text()) for f in FAM}

# 1. L1 test AUC with bootstrap 95% CI, both splits, plus controls.
fig, axes = plt.subplots(1, 2, figsize=(8.2, 3.4), sharey=True)
for ax, split in zip(axes, ["even_odd", "aa_mod4"]):
    labels = GROUPS + ["positive\ncontrol"]
    for j, f in enumerate(FAM):
        d = summ[f]["L1_L2"][split]
        xs, ys, lo, hi = [], [], [], []
        for i, g in enumerate(GROUPS):
            m = d[g]["model"]
            xs.append(i + (j - 0.5) * 0.3)
            ys.append(m["test_auc"])
            lo.append(m["test_auc"] - m["test_auc_ci95"][0])
            hi.append(m["test_auc_ci95"][1] - m["test_auc"])
        pc = d["positive_control"]
        xs.append(len(GROUPS) + (j - 0.5) * 0.3)
        ys.append(pc["test_auc"])
        lo.append(pc["test_auc"] - pc["test_auc_ci95"][0])
        hi.append(pc["test_auc_ci95"][1] - pc["test_auc"])
        ax.errorbar(xs, ys, yerr=[lo, hi], fmt="o", color=C[f], capsize=3, label=f)
        neg = [d[g]["negative_control_auc"] for g in GROUPS]
        ax.scatter([i + (j - 0.5) * 0.3 for i in range(len(GROUPS))], neg, marker="x", color=C["ctrl"], s=18,
                   label="permuted-label control" if j == 0 else None)
    ax.axhline(0.5, color="k", lw=0.7, ls=":")
    ax.axhline(0.6, color="#b33", lw=0.7, ls="--", label="close threshold 0.6" if split == "even_odd" else None)
    ax.set_xticks(range(len(labels)), labels)
    ax.set_title(f"split {split}")
    ax.set_ylim(0.35, 0.85)
axes[0].set_ylabel("held-out AUC (bootstrap 95% CI)")
axes[0].legend(fontsize=7, loc="upper left")
fig.suptitle("L1 triage: can a learner tell decomposable targets apart? (run1, seed 20261027)", fontsize=9)
save(fig, "l1_auc")

# 2. Held-out gain per lever (relations / conflict vs solve-all) against exact-label ceiling.
fig, ax = plt.subplots(figsize=(8.2, 3.4))
levers = GROUPS + ["L2_cutoff"]
for j, f in enumerate(FAM):
    d = summ[f]["L1_L2"]["even_odd"]
    for i, g in enumerate(levers):
        m = d[g] if g == "L2_cutoff" else d[g]["model"]
        x = i + (j - 0.5) * 0.3
        ax.errorbar([x], [m["test_gain"]], yerr=[[m["test_gain"] - m["test_gain_ci95"][0]], [m["test_gain_ci95"][1] - m["test_gain"]]],
                    fmt="o", color=C[f], capsize=3, label=f if i == 0 else None)
        if g != "L2_cutoff":
            ax.scatter([x], [d[g]["ceiling_gain_exact_labels"]], marker="_", s=200, color=C["ceil"],
                       label="ceiling: exact labels" if (i == 0 and j == 0) else None)
ax.axhline(1.0, color="k", lw=0.7, ls=":")
ax.axhline(1.5, color="#b33", lw=0.7, ls="--", label="fire threshold 1.5x")
ax.set_yscale("log")
ax.set_xticks(range(len(levers)), ["F-x", "F-xx", "F-tel64", "F-tel512", "L2 fixed\ncutoff"])
ax.set_ylabel("held-out gain in relations/conflict\n(bootstrap 95% CI, log scale)")
ax.legend(fontsize=7, loc="upper right", ncol=2)
ax.set_title("L1/L2 held-out gain vs the perfect-classifier ceiling (split even_odd, run1)", fontsize=9)
save(fig, "gain_vs_ceiling")

# 3. Conflict distributions in units of sorted triples.
fig, axes = plt.subplots(1, 2, figsize=(8.2, 3.0))
for ax, f in zip(axes, FAM):
    rows = [json.loads(l) for l in (RUN / f"rows_{f}.jsonl").read_text().splitlines()]
    t = TRIPLES[f]
    sat = [r["conflicts"] / t for r in rows if r["sat"]]
    uns = [r["conflicts"] / t for r in rows if not r["sat"]]
    bins = [i / 50 for i in range(0, 56)]
    ax.hist(uns, bins=bins, color="#999", label=f"UNSAT (n={len(uns)})")
    ax.hist(sat, bins=bins, color=C[f], alpha=0.75, label=f"SAT, time to witness (n={len(sat)})")
    ax.set_xlabel("solver conflicts / sorted factor-base triples")
    ax.set_title(f"{f}: {t} sorted triples")
    ax.legend(fontsize=7)
axes[0].set_ylabel("targets")
fig.suptitle("Native CDCL+XOR on symmetrised S4: refutation = one conflict per triple (run1 rows)", fontsize=9)
save(fig, "conflict_distribution")

# 4. L3 branching arms.
fig, ax = plt.subplots(figsize=(8.2, 3.0))
arms = ["a0_default", "a1_x_all", "a2_x1x2", "a3_e_vars", "a4_learned_activity"]
for j, f in enumerate(FAM):
    vals = [l3[f][a]["ratio_to_a0"] for a in arms]
    xs = [i + (j - 0.5) * 0.3 for i in range(len(arms))]
    ax.bar(xs, vals, width=0.3, color=C[f], label=f)
    for x, a in zip(xs, arms):
        if l3[f][a]["capped"]:
            ax.text(x, l3[f][a]["ratio_to_a0"] * 1.08, f"≥ (all {l3[f][a]['capped']} capped)", ha="center", fontsize=6, rotation=90, va="bottom")
ax.set_yscale("log")
ax.set_ylim(0.5, 60)
ax.axhline(1.0, color="k", lw=0.7, ls=":")
ax.set_xticks(range(len(arms)), ["a0 default", "a1 x-bits first", "a2 X1,X2 first", "a3 e-vars first", "a4 learned\n(VSIDS activity)"])
ax.set_ylabel("summed refutation conflicts / a0")
ax.legend(fontsize=7)
ax.set_title("L3 branching order on held-out UNSAT targets (60 n15l5, 30 n17l6; cap 10x a0 median)", fontsize=9)
save(fig, "l3_branching")
print("ok")
