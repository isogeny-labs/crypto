#!/usr/bin/env python3
"""Round 7 set T: median F4 time over 6 systems per map (3 passes x 2 targets), W, rows, area, and the T gates."""
import json, os, statistics
D = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(D, "runs", "T")
MAPS = ("none", "quad", "cubic", "quart", "generic3"); res = {}
for m in MAPS:
    rows = []
    for p in (1, 2, 3):
        f = os.path.join(R, f"T-p{p}-{m}.jsonl")
        rows += [r for r in map(json.loads, open(f)) if "control" in r]
    assert len(rows) == 6 and not any(r["timed_out"] for r in rows), m
    assert all(r["planted_ok"] is not False for r in rows if r["target"] == "planted"), m
    W = {r["max_cols"] for r in rows}; H = {r["max_rows"] for r in rows}
    t = sorted(r["ms"] for r in rows)
    res[m] = dict(W=max(W), rows=max(H), Wset=W, Hset=H, med=statistics.median(t), lo=t[0], hi=t[-1])
ref = res["none"]
print("map\tW\trows\tarea_ratio\tmedian_s\tmin_s\tmax_s\ttime_ratio\tgate")
for m in MAPS:
    x = res[m]; ar = x["W"] * x["rows"] / (ref["W"] * ref["rows"]); tr = x["med"] / ref["med"]
    g = "-" if m == "none" else ("T-reopen" if tr <= 0.8 and ar <= 1 else "T-closed" if tr >= 1 or ar >= 1.1 else "T-mixed")
    print(f"{m}\t{x['W']}\t{x['rows']}\t{ar:.3f}\t{x['med']/1000:.1f}\t{x['lo']/1000:.1f}\t{x['hi']/1000:.1f}\t{tr:.3f}\t{g}"
          + ("" if len(x["Wset"]) == 1 and len(x["Hset"]) == 1 else f"\t(W/rows vary: {sorted(x['Wset'])} {sorted(x['Hset'])})"))
