#!/usr/bin/env python3
"""Figures for research/ic_acct6_archive_20261021. Plots recorded values only.

Reads breakdown-before.json, breakdown-after.json, breakdown-tranche.json and
plan.json (all written by examples/ic_acct6_archive.rs) and TRANCHE.md's table.
Computes no result: it reads recorded counts, and for the re-run plan it only
counts plan.json rows per recorded `rerun` status and sums their recorded
`incomplete` and `est_hours` fields, checked against PLAN.md's totals.

    /tmp/report-venv/bin/python research/ic_acct6_archive_20261021/figures/plots.py
"""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import Patch  # noqa: E402

HERE = Path(__file__).resolve().parent
D = HERE.parent
BEFORE = json.loads((D / "breakdown-before.json").read_text())
AFTER = json.loads((D / "breakdown-after.json").read_text())
TRANCHE = json.loads((D / "breakdown-tranche.json").read_text())
PLAN = json.loads((D / "plan.json").read_text())
for b in (BEFORE, AFTER, TRANCHE):
    assert b["schema"] == "ic-acct6.breakdown/v1"
BLUE, ORANGE, GREY, INK, MUTED = "#2a78d6", "#eb6834", "#b5b5b5", "#333333", "#6b6b6b"
plt.rcParams.update({"font.size": 9, "axes.spines.top": False, "axes.spines.right": False,
                     "axes.grid": True, "grid.color": "#e4e4e4", "grid.linewidth": 0.6,
                     "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg", bbox_inches="tight")
    fig.savefig(HERE / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


# 1a. Archive totals before/after phase A, by pricing status.
tb, ta = BEFORE["totals"], AFTER["totals"]
assert tb["records"] == ta["records"] == 73012
cats = [("complete", "complete"), ("incomplete", "incomplete (lower bound)"), ("errors", "unpriceable (error)")]
fig, ax = plt.subplots(figsize=(6.8, 2.6))
for i, (k, lab) in enumerate(cats):
    y = len(cats) - 1 - i
    ax.barh(y + 0.19, tb[k], height=0.34, color=GREY)
    ax.barh(y - 0.19, ta[k], height=0.34, color=BLUE)
    d = ta[k] - tb[k]
    ax.annotate(f"{tb[k]:,}", (tb[k], y + 0.19), xytext=(4, 0), textcoords="offset points",
                va="center", fontsize=7.5, color=MUTED)
    ax.annotate(f"{ta[k]:,}  ({d:+,})", (ta[k], y - 0.19), xytext=(4, 0), textcoords="offset points",
                va="center", fontsize=7.5, color=INK)
ax.set_yticks(range(len(cats)), [lab for _, lab in cats][::-1])
ax.set_xlim(0, 62000)
ax.grid(axis="y", visible=False)
ax.set_xlabel("verified non-warm-up archive records (73,012 in both)")
ax.set_title("Archive bitop/v1 pricing status, before and after the phase-A rules", fontsize=9)
ax.legend(handles=[Patch(color=GREY, label="before (rules at 4651c5d9f)"),
                   Patch(color=BLUE, label="after (rules at 42f23b5e4)")],
          fontsize=7, frameon=False, loc="lower right")
save(fig, "before_after_totals")

# 1b. What the new rules changed, by category, coloured by the records' status after.
bm, am = BEFORE["by_method_family"], AFTER["by_method_family"]
br, ar = BEFORE["by_reason"], AFTER["by_reason"]
rows = []
for key, lab in [("rho.negation|extension", "rho.negation, extension family: error → priced"),
                 ("ic.pipeline|extension", "ic.pipeline, extension family: error → priced")]:
    assert bm[key]["errors"] == bm[key]["records"]
    rows.append((lab, am[key]["complete"], am[key]["incomplete"]))
for reason, lab in [("unpriced:native_queries_uncharged", "CM-folded native: unpriced counters → priced"),
                    ("unpriced:projected_base_membership_checks_uncharged",
                     "projected base: unpriced counters → priced"),
                    ("unpriced:aux_supersingular", "isogeny tower: unpriced counter → priced")]:
    assert ar.get(reason, 0) == 0
    rows.append((lab, 0, br[reason]))  # README: these stay incomplete (never-counted IC work)
fig, ax = plt.subplots(figsize=(6.8, 3.1))
for i, (lab, c, inc) in enumerate(rows):
    y = len(rows) - 1 - i
    ax.barh(y, c, height=0.55, color=BLUE)
    ax.barh(y, inc, left=c, height=0.55, color=ORANGE)
    txt = f"{c + inc:,}: " + (f"{c} complete" if c else "") + (", " if c and inc else "") + \
          (f"{inc} still incomplete" if inc else "")
    ax.annotate(txt, (c + inc, y), xytext=(4, 0), textcoords="offset points", va="center",
                fontsize=7.5, color=INK)
ax.set_yticks(range(len(rows)), [r[0] for r in rows][::-1])
ax.set_xlim(0, 1000)
ax.grid(axis="y", visible=False)
ax.set_xlabel("records affected by the phase-A rules")
ax.set_title("What the new rules changed (status after phase A)", fontsize=9)
ax.legend(handles=[Patch(color=BLUE, label="now complete"),
                   Patch(color=ORANGE, label="still incomplete (never-counted work remains)")],
          fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.3, -0.28), ncol=2)
save(fig, "before_after_categories")

# 2. Re-run plan by status: incomplete records per disjoint group.
tranche_rows = [ln for ln in (D / "TRANCHE.md").read_text().splitlines()
                if ln.startswith("| `p") and "ok=true" in ln]
tranche_dirs = {ln.split("|")[2].strip().strip("`") for ln in tranche_rows}
assert len(tranche_dirs) == 2
groups = {}


def put(g, r):
    s = groups.setdefault(g, [0, 0, 0.0])
    s[0] += 1
    s[1] += r["incomplete"]
    s[2] += r["est_hours"]


for r in PLAN:
    if r["rerun"] == "ok":
        if r["dir"] in tranche_dirs:
            put("re-run in tranche 1", r)
        elif r["leaderboard"]:
            put("re-runnable, leaderboard source (15 of 17), to run", r)
        else:
            put("re-runnable, other, to run", r)
    elif "native backend" in r["rerun"]:
        put("not re-runnable: CM-folded native backend absent", r)
    elif r["rerun"].startswith("plan has"):
        put("not re-runnable: interrupted (fewer records than plan)", r)
    else:
        put("not re-runnable: no frozen spec.json", r)
assert len(PLAN) == 396 and sum(g[1] for g in groups.values()) == 22706
ok = [g for k, g in groups.items() if not k.startswith("not")]
assert sum(g[0] for g in ok) == 337 and sum(g[1] for g in ok) == 22013
assert round(sum(g[2] for g in groups.values()), 2) == 71.55
order = ["re-run in tranche 1", "re-runnable, leaderboard source (15 of 17), to run", "re-runnable, other, to run",
         "not re-runnable: CM-folded native backend absent",
         "not re-runnable: interrupted (fewer records than plan)", "not re-runnable: no frozen spec.json"]
fig, ax = plt.subplots(figsize=(7.6, 3.4))
for i, k in enumerate(order):
    n, inc, h = groups[k]
    y = len(order) - 1 - i
    col = BLUE if i == 0 else (ORANGE if not k.startswith("not") else GREY)
    ax.barh(y, inc, height=0.55, color=col)
    ax.annotate(f"{inc:,} records · {n} sessions · {h:.2f} h", (inc, y), xytext=(4, 0),
                textcoords="offset points", va="center", fontsize=7.5, color=INK)
ax.set_yticks(range(len(order)), order[::-1])
ax.set_xlim(0, 40000)
ax.grid(axis="y", visible=False)
ax.set_xlabel("incomplete records held by each group")
ax.set_title("Re-run plan: 396 sessions, 22,706 incomplete records, 71.55 h", fontsize=9)
ax.legend(handles=[Patch(color=BLUE, label="re-run (2 of 34 tranche sessions)"),
                   Patch(color=ORANGE, label="re-runnable unchanged, not yet run"),
                   Patch(color=GREY, label="not re-runnable unchanged")],
          fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.35, -0.24), ncol=3)
save(fig, "rerun_plan")

# 3. Tranche 1 outcome per re-run session: complete vs incomplete counted records.
ss = TRANCHE["sessions"]
fig, ax = plt.subplots(figsize=(6.8, 2.5))
for i, s in enumerate(ss):
    y = len(ss) - 1 - i
    c, inc = s["counted"]["complete"], s["counted"]["incomplete"]
    ax.barh(y, c, height=0.5, color=BLUE)
    ax.barh(y, inc, left=c, height=0.5, color=ORANGE)
    ax.annotate(f"{c} complete (rho) + {inc} incomplete (ic.shared_rank)", (c + inc, y), xytext=(4, 0),
                textcoords="offset points", va="center", fontsize=7.5, color=INK)
ax.set_yticks(range(len(ss)), [s["dir"].rsplit("/", 1)[1] for s in ss][::-1])
ax.set_xlim(0, 820)
ax.grid(axis="y", visible=False)
ax.set_xlabel("counted records of the re-run session (both audits ok, 2 replays each)")
ax.set_title("Tranche 1: the two sessions that ran", fontsize=9)
ax.legend(handles=[Patch(color=BLUE, label="complete"), Patch(color=ORANGE, label="incomplete")],
          fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.4, -0.6), ncol=2)
save(fig, "tranche1")
print("figures written")
