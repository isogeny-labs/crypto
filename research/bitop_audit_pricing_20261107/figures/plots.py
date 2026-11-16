#!/usr/bin/env python3
"""Figures for research/bitop_audit_pricing_20261107. Plots recorded values only.

Reads the Rust outputs beside this directory and computes no result:
`archive/breakdown-{before,after}.summary.json` (examples/ic_acct6_archive.rs),
`v2/holdout.summary.json` and `v2/holdout.pairs.jsonl`
(examples/bitop_v2_choices.rs), `sat/sat_rerun.jsonl` (ecbench exec).

    /tmp/report-venv/bin/python research/bitop_audit_pricing_20261107/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BLUE, ORANGE, GREEN, INK, MUTED = "#2a78d6", "#eb6834", "#2f9e6e", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 8.5, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Task B: the reasons the 2026-11-07 extension moved, before and after.
before = json.loads((ROOT / "archive/breakdown-before.summary.json").read_text())["by_reason"]
after = json.loads((ROOT / "archive/breakdown-after.summary.json").read_text())["by_reason"]
keys = sorted(k for k in set(before) | set(after) if before.get(k, 0) != after.get(k, 0))
keys += sorted(k for k in before if k.startswith("unpriced:solver_") or k == "uncounted:native_backend_exponentiations_field_adds_hashing_modr_elimination_and_cm_map")
labels = [k.replace("unpriced:", "unpriced: ").replace("uncounted:", "uncounted: ")
          .replace("field_sqrt has no bitop/v1 price over ", "field_sqrt over ") for k in keys]
fig, ax = plt.subplots(figsize=(8.6, 0.32 * len(keys) + 1.4))
y = range(len(keys))
ax.barh([i + 0.2 for i in y], [before.get(k, 0) for k in keys], 0.4, color=MUTED, label="before (origin/main)")
ax.barh([i - 0.2 for i in y], [after.get(k, 0) for k in keys], 0.4, color=BLUE, label="after (2026-11-07 extension)")
ax.set_yticks(list(y), labels, fontsize=7)
ax.invert_yaxis()
ax.set_xscale("symlog", linthresh=1)
ax.set_xlabel("records naming the reason (84,687 verified records)")
ax.set_title("Task B: reasons moved by the decisions; totals unchanged (58,277 complete,\n"
             "24,682 incomplete, 1,728 errors): each record touched stays incomplete for another reason",
             fontsize=8.5, loc="right")
ax.legend(frameon=False, loc="upper right", fontsize=7)
save(fig, "archive_reasons")

# 2. Task B: SAT word operations per conflict and per propagation, re-executed.
sat = [json.loads(l) for l in (ROOT / "sat/sat_rerun.jsonl").read_text().splitlines() if l.strip()]
fig, axs = plt.subplots(1, 2, figsize=(7.0, 2.6))
for ax, key, title in [(axs[0], "word_ops_per_conflict", "word operations per conflict"),
                       (axs[1], "word_ops_per_propagation", "word operations per propagation")]:
    for arm, c in [("sub6-da-sat", BLUE), ("sub8-da-sat", ORANGE)]:
        rs = [r for r in sat if r["arm"] == arm]
        ax.scatter([r["seq"] for r in rs], [r[key] for r in rs], color=c, s=18, label=arm)
    ax.set_title(title, fontsize=8.5)
    ax.set_xlabel("committed seq (ecbench_yield_sweep_20261004/koblitz)")
    ax.set_ylim(bottom=0)
axs[0].legend(frameon=False, fontsize=7, loc="lower right")
fig.suptitle("solver_conflicts stays unpriced: no constant per conflict (counted engine, 8 re-executions,\n"
             "conflicts and propagations reproduce the committed records exactly)", fontsize=8.5)
save(fig, "sat_per_conflict")

# 3. Task C: the eight choices' effects on the hold-out.
S = json.loads((ROOT / "v2/holdout.summary.json").read_text())
pairs = [json.loads(l) for l in (ROOT / "v2/holdout.pairs.jsonl").read_text().splitlines() if l.strip()]
fig, axs = plt.subplots(2, 2, figsize=(8.2, 6.0))
ax = axs[0][0]
items = [("C1 IC sum/max", S["c1_sum_over_max"]["ic.pipeline"]),
         ("C2 matrix dense/sparse", S["c2_dense_over_sparse_matrix"]),
         ("C2 IC total dense/sparse", S["c2_sum_dense_over_sum_sparse"])]
for i, (name, q) in enumerate(items):
    ax.plot([q["min"], q["max"]], [i, i], color=MUTED, linewidth=2)
    ax.scatter([q["median"]], [i], color=BLUE, zorder=3)
    hi = f"{q['max']:.0f}" if q["max"] >= 10 else f"{q['max']:.2f}"
    ax.annotate(f"{q['median']:.2f} [{q['min']:.2f}, {hi}]", (q["median"], i + 0.22), fontsize=7)
ax.set_yticks(range(len(items)), [n for n, _ in items], fontsize=7)
ax.set_xscale("log")
ax.set_xlabel("ratio (median, [min, max]) over 4,080 IC records")
ax.set_title("C1, C2: how much the declared memory moves", fontsize=8.5)
ax.invert_yaxis()

ax = axs[0][1]
walk = S["c7_walk_state_over_declared"]
names = [m for m in ["rho.negation", "kangaroo.vow", "ic.pipeline"] if walk.get(m)]
for i, m in enumerate(names):
    q = walk[m]
    ax.plot([q["min"], q["max"]], [i, i], color=MUTED, linewidth=2)
    ax.scatter([q["median"]], [i], color=ORANGE, zorder=3)
ax.axvline(1, color=INK, linewidth=0.8, linestyle="--")
ax.set_yticks(range(len(names)), names, fontsize=7)
ax.set_xscale("log")
ax.set_xlabel("walk state / declared memory")
ax.set_title("C7: the walk state left out (median, [min, max])", fontsize=8.5)
ax.invert_yaxis()

ax = axs[1][0]
rk = S["c5_ranking_by_method_pair"]
names = list(rk)
x = range(len(names))
for off, key, c, lab in [(-0.27, "ranked_sum", BLUE, "frozen (sum)"),
                         (0.0, "ranked_kangaroo_point_distance", GREEN, "C4 kangaroo 2κ+w+1"),
                         (0.27, "ranked_with_walk_state", ORANGE, "C7 walk state in")]:
    ax.bar([i + off for i in x], [rk[n][key] / rk[n]["pairs"] for n in names], 0.27, color=c, label=lab)
ax.set_xticks(list(x), [n.replace(".negation", "").replace(".pipeline", "").replace(".vow", "") for n in names],
              fontsize=6, rotation=35, ha="right")
ax.set_ylabel("share of pairs ranked")
ax.set_ylim(0, 1.05)
ax.legend(frameon=False, fontsize=6.5, loc="lower left")
ax.set_title("C5 (and C4, C7): which pairs the rule ranks", fontsize=8.5)

ax = axs[1][1]
ex = S["c6_exponents"]
names = list(ex)
x = range(len(names))
for off, key, c, lab in [(-0.27, "gates_slope", BLUE, "gates"), (0.0, "memory_sum_slope", ORANGE, "memory"),
                         (0.27, "at_sum_slope", GREEN, "at")]:
    ax.bar([i + off for i in x], [ex[n][key] for n in names], 0.27, color=c, label=lab)
ax.set_xticks(list(x), names, fontsize=7)
ax.set_ylabel("slope against log2 r (arm means, 25 sizes)")
ax.legend(frameon=False, fontsize=7)
ax.set_title("C6: gate, memory and at exponents", fontsize=8.5)
fig.suptitle("bitop/v2 open choices on the bitop_ladder hold-out (12,080 records, P16-P40). "
             "Recommendations pending owner sign-off", fontsize=8.5)
save(fig, "v2_choices")
