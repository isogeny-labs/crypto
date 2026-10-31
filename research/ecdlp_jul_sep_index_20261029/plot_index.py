#!/usr/bin/env python3
"""Figures for the Jul-Sep ECDLP decision index (reads INDEX.json only)."""
import collections, json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__))
IDX = json.load(open(os.path.join(H, "INDEX.json")))
rows = IDX["rows"]; FAM = set(IDX["scope_families"])
rows = [r for r in rows if r["month"] in ("2026-07", "2026-08", "2026-09")]
OUT = ("support", "support_scoped", "reject_scoped", "weaken", "inconclusive", "supersede", "pause")
def cls(v):
    if v in OUT: return "outcome"
    if v.startswith("approve"): return "approve"
    if v.startswith("revise") or v in ("amend", "refine"): return "revise"
    if v == "expand": return "expand"
    return "other"
weeks = collections.Counter(); wcls = collections.defaultdict(collections.Counter)
import datetime as dt
for r in rows:
    d = dt.date.fromisoformat(r["date"]); wk = d - dt.timedelta(days=d.weekday())
    wcls[wk][cls(r["verdict"])] += 1
wk = sorted(wcls)
fig, ax = plt.subplots(1, 2, figsize=(12, 4.2))
bottom = [0]*len(wk)
colors = {"approve": "#4C78A8", "revise": "#F58518", "expand": "#54A24B", "outcome": "#E45756", "other": "#9D9D9D"}
for k in ("approve", "revise", "expand", "outcome", "other"):
    vals = [wcls[w][k] for w in wk]
    ax[0].bar(wk, vals, bottom=bottom, width=6, color=colors[k], label=k)
    bottom = [b+v for b, v in zip(bottom, vals)]
ax[0].set_title("ECDLP coordinator decisions per week, Jul–Sep 2026"); ax[0].set_ylabel("decisions")
ax[0].legend(frameon=False, fontsize=8); ax[0].tick_params(axis="x", rotation=30)
fams = collections.defaultdict(collections.Counter)
for r in rows:
    for f in r["families"]:
        if f in FAM: fams[f][r["month"]] += 1
top = sorted(fams, key=lambda f: -sum(fams[f].values()))[:14]
months = ("2026-07", "2026-08", "2026-09"); mcol = ("#BAB0AC", "#72B7B2", "#4C78A8")
left = [0]*len(top)
for m, c in zip(months, mcol):
    vals = [fams[f][m] for f in top]
    ax[1].barh(top, vals, left=left, color=c, label=m); left = [a+b for a, b in zip(left, vals)]
ax[1].invert_yaxis(); ax[1].set_title("Decisions by ECDLP family token (a decision can carry several)"); ax[1].legend(frameon=False, fontsize=8)
fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(os.path.join(H, f"jul_sep_decisions.{ext}"), dpi=160)
print(len(rows), "rows plotted")
