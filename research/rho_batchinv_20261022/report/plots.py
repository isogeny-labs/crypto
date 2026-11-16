"""Lay out recorded values from ../summary.json and the Callgrind function
summaries as SVG (and 150 dpi PNG) graphs.  Plotting only: every number is
read from a committed record; the only arithmetic is recorded instruction
counts divided by recorded walk-step counts."""
import json
import pathlib
import re

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = pathlib.Path(__file__).resolve().parent
study = here.parent
summary = json.loads((study / "summary.json").read_text())
BASE, CAND = "#4c72b0", "#55a868"


def save(fig, name):
    fig.savefig(here / f"{name}.svg")
    fig.savefig(here / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Instructions per walk step by arm, n = 41 and 53.
cells = summary["instructions"]
fig, ax = plt.subplots(figsize=(7, 3.4))
for i, cell in enumerate(cells):
    for j, arm in enumerate(["baseline", "candidate"]):
        a = next(x for x in cell["arms"] if x["arm"] == arm)
        ax.bar(i * 3 + j, a["ir_per_step"], color=[BASE, CAND][j],
               label=["main (412b60289)", "fixed-width chunks"][j] if i == 0 else None)
        ax.text(i * 3 + j, a["ir_per_step"] + 8, f"{a['ir_per_step']:.1f}", ha="center", fontsize=8)
ax.set_xticks([i * 3 + 0.5 for i in range(len(cells))])
ax.set_xticklabels([f"n = {c['n']} (outputs identical: {c['identical']})" for c in cells], fontsize=8)
ax.set_ylim(0, 820)
ax.set_ylabel("instructions per walk step")
ax.set_title("Strong rho, 4 fixtures: Callgrind Ir / walk_steps", fontsize=10)
ax.legend(fontsize=8, loc="lower right")
fig.tight_layout()
save(fig, "instructions-per-step")

# 2. Where the step goes at n = 53, baseline vs candidate.
groups = [
    ("batch inversion: chains and loops", r"batch_inv_clmul"),
    ("batch inversion: the one inverse", r"::inv \[|inv_clmul"),
    ("walk body (canonicalize, add, table)", r"solve"),
]
rows = {}
n53 = next(c for c in cells if c["n"] == 53)
for arm in ["baseline", "candidate"]:
    text = (study / f"runs/callgrind/fixture-n53-4-{arm}.annotate.txt").read_text()
    total = int(re.search(r"([\d,]+) \(100\.0%\)\s+PROGRAM TOTALS", text).group(1).replace(",", ""))
    steps = next(a["walk_steps"] for a in n53["arms"] if a["arm"] == arm)
    parts = {g: 0 for g, _ in groups}
    for line in text.splitlines():
        m = re.match(r"\s*([\d,]+) \(\s*[\d.]+%\)\s+(\S.*)", line)
        if not m or "PROGRAM TOTALS" in line:
            continue
        for g, pat in groups:
            if re.search(pat, m.group(2).split(":<", 1)[-1]):
                parts[g] += int(m.group(1).replace(",", ""))
                break
    parts["other"] = total - sum(parts.values())
    rows[arm] = {k: v / steps for k, v in parts.items()}
fig, ax = plt.subplots(figsize=(7, 3.4))
names = [g for g, _ in groups] + ["other"]
colors = ["#55a868", "#ccb974", "#4c72b0", "#bbbbbb"]
for i, arm in enumerate(["baseline", "candidate"]):
    left = 0
    for name, color in zip(names, colors):
        ax.barh(i, rows[arm][name], left=left, color=color, label=name if i == 0 else None)
        if rows[arm][name] > 20:
            ax.text(left + rows[arm][name] / 2, i, f"{rows[arm][name]:.0f}", ha="center", va="center", fontsize=7)
        left += rows[arm][name]
    ax.text(left + 8, i, f"{left:.0f}", va="center", fontsize=8)
ax.set_yticks([0, 1])
ax.set_yticklabels(["main", "fixed-width chunks"])
ax.invert_yaxis()
ax.set_xlim(0, 800)
ax.set_xlabel("instructions per walk step (self cost by function, inlined code attributed to its caller)")
ax.set_title("Where a step's instructions go (n = 53, 4 fixtures)", fontsize=10)
ax.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.28), ncol=2)
fig.tight_layout()
save(fig, "instructions-by-function")

# 3. Native time per step.
t = summary["native_time"]
fig, ax = plt.subplots(figsize=(7, 3.6))
for i, cell in enumerate(t["cells"]):
    for j, (arm, color) in enumerate([("baseline_ns", BASE), ("candidate_ns", CAND)]):
        s = cell[arm]
        ax.bar(i * 3 + j, s["median"], color=color,
               label=("main" if j == 0 else "fixed-width chunks") if i == 0 else None)
        ax.errorbar(i * 3 + j, s["median"], yerr=[[s["median"] - s["min"]], [s["max"] - s["median"]]],
                    color="black", capsize=4)
    ax.text(i * 3 + 0.5, max(cell["baseline_ns"]["max"], cell["candidate_ns"]["max"]) * 1.03,
            f"x{cell['ratio']:.3f} (A/A {100 * cell['aa_spread']:.1f}%)", ha="center", fontsize=8)
ax.set_ylim(0, max(c["baseline_ns"]["max"] for c in t["cells"]) * 1.18)
ax.set_xticks([i * 3 + 0.5 for i in range(len(t["cells"]))])
ax.set_xticklabels([c["slug"] for c in t["cells"]], fontsize=7)
ax.set_ylabel("ns per walk step (median, range; n = 5 each)")
ax.set_title(f"Native time, 5 A/B rounds, isolated_bench ({t['isolation']['runs']} runs, "
             f"{len(t['isolation']['contended'])} contended)", fontsize=10)
ax.legend(fontsize=8, loc="lower right")
fig.tight_layout()
save(fig, "native-time")
for arm, r in rows.items():
    print(arm, {k: round(v, 1) for k, v in r.items()})
