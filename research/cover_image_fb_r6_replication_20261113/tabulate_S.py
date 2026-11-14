#!/usr/bin/env python3
"""Set S: W per (cell, map, seed), r_s = W_X/W_none per seed, closed/reopen/mixed gates, S3 null ratios."""
import json, os, statistics
D = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(D, "runs", "S")
SEEDS = (101, 102, 103, 104, 105); MAPS = ("quad", "cubic", "quart", "generic3")
def cell(c, m, s):
    p = os.path.join(R, f"{c}-{m}-s{s}.jsonl")
    if not os.path.exists(p):
        return None
    rows = [r for r in map(json.loads, open(p)) if "control" in r]
    if not rows or any(r["timed_out"] for r in rows) or any(r["target"] == "planted" and r["planted_ok"] is False for r in rows):
        return None
    out = {}
    for ctl in {r["control"] for r in rows}:
        rs = [r for r in rows if r["control"] == ctl]
        out[ctl] = dict(W=max(r["max_cols"] for r in rs), rows=max(r["max_rows"] for r in rs),
                        D=max(r["solving_degree_max"] for r in rs), ms=statistics.median(r["ms"] for r in rs))
    return out
lines = ["cell\tmap\tseed\tW\trows\tD\tW_over_PKM\tW_over_null"]
verdicts = []
for c in ("S3", "S4"):
    ref = {s: cell(c, "none", s) for s in SEEDS}
    ws = [ref[s]["tower"]["W"] for s in SEEDS if ref[s]]
    print(f"{c} PKM W across seeds: {ws}  (spread {min(ws)}–{max(ws)})" if ws else f"{c} PKM: no data")
    for s in SEEDS:
        if ref[s]:
            t = ref[s]["tower"]; n = ref[s].get("null")
            lines.append(f"{c}\tnone\t{s}\t{t['W']}\t{t['rows']}\t{t['D']}\t1\t{t['W']/n['W']:.3f}" if n else f"{c}\tnone\t{s}\t{t['W']}\t{t['rows']}\t{t['D']}\t1\t-")
    for m in MAPS:
        rs, rn = [], []
        for s in SEEDS:
            x = cell(c, m, s)
            if not x or not ref[s]:
                continue
            t = x["tower"]; r = t["W"] / ref[s]["tower"]["W"]; rs.append(r)
            nn = x.get("null"); q = t["W"] / nn["W"] if nn else None
            if q: rn.append(q)
            lines.append(f"{c}\t{m}\t{s}\t{t['W']}\t{t['rows']}\t{t['D']}\t{r:.3f}\t{q:.3f}" if q else f"{c}\t{m}\t{s}\t{t['W']}\t{t['rows']}\t{t['D']}\t{r:.3f}\t-")
        if len(rs) < 5:
            v = f"PENDING/censored ({len(rs)}/5 seeds)"
        elif sum(r >= 1 for r in rs) >= 4:
            v = "closed"
        elif sum(r <= 0.8 for r in rs) >= 4:
            v = "reopen"
        else:
            v = "mixed"
        verdicts.append(f"{c} {m}: {v}  W/PKM " + " ".join(f"{r:.3f}" for r in rs)
                        + (f"  W/null " + " ".join(f"{q:.3f}" for q in rn) if rn else ""))
open(os.path.join(D, "cells_S.tsv"), "w").write("\n".join(lines) + "\n")
print("\n".join(verdicts))
