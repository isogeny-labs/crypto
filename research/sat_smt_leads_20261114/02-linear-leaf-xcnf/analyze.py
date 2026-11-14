#!/usr/bin/env python3
"""Merge raw/*.json -> results.json and print the summary table + slope fits."""
import json, glob, math, statistics as st, sys, os

here = os.path.dirname(os.path.abspath(__file__))
rows = []
meta = None
for f in sorted(glob.glob(os.path.join(here, "raw", "run_*.json"))):
    d = json.load(open(f))
    meta = {k: v for k, v in d.items() if k != "rows"}
    for r in d["rows"]:
        r["source"] = os.path.basename(f)
        rows.append(r)

ENUM = ("T2", "T3", "T4", "G-T2")


def work(r):
    if r["arm"] == "L":
        return r["work"]
    w = r["conflicts"]
    if r["arm"] in ENUM:
        w += r["calls"]
    return w


for r in rows:
    if r["status"] not in ("NOT_RUN",):
        r["work_unit"] = work(r)

arms = ["L", "B0", "T1", "T2", "T3", "T4", "G-B0", "G-T1", "G-T2"]
ls = sorted({r["l"] for r in rows})
summary = []
for dec in (False, True):
    for arm in arms:
        for l in ls:
            rs = [r for r in rows if r["arm"] == arm and r["l"] == l and r["decomposable"] == dec
                  and r["status"] != "NOT_RUN"]
            if not rs:
                continue
            cens = [r for r in rs if r["status"] == "CENSORED"]
            ok = [r for r in rs if r["status"] != "CENSORED"]
            ws = [r["work_unit"] for r in ok]
            e = {"decomposable": dec, "arm": arm, "l": l, "n": len(rs), "censored": len(cens),
                 "mean_work": st.mean(ws) if ws else None, "median_work": st.median(ws) if ws else None,
                 "min": min(ws) if ws else None, "max": max(ws) if ws else None,
                 "mean_calls": st.mean(r["calls"] for r in ok) if ok else None,
                 "mean_conflicts": st.mean(r.get("conflicts", 0) for r in ok) if ok else None,
                 "mean_time": st.mean(r.get("time", 0) for r in ok) if ok else None,
                 "base_size": rs[0]["base_size"]}
            if arm.startswith("G-") and ok:
                e["mean_decisions"] = st.mean(r["decisions"] for r in ok)
                e["mean_propagations"] = st.mean(r["propagations"] for r in ok)
            summary.append(e)


def fit(pts):
    if len(pts) < 2:
        return None
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    mx, my = st.mean(xs), st.mean(ys)
    return sum((x - mx) * (y - my) for x, y in pts) / sum((x - mx) ** 2 for x in xs)


slopes = {}
for dec in (False, True):
    for arm in arms:
        for stat in ("mean_work", "median_work"):
            pts = [(e["l"], math.log2(max(e[stat], 0.5))) for e in summary
                   if e["arm"] == arm and e["decomposable"] == dec and e["censored"] == 0 and e[stat] is not None]
            s = fit(pts)
            slopes[f"{arm}|{'dec' if dec else 'nondec'}|{stat}"] = {
                "slope_bits_per_l": s, "sizes": [p[0] for p in pts]}

json.dump({"meta": meta, "summary": summary, "slopes": slopes, "rows": rows},
          open(os.path.join(here, "results.json"), "w"), indent=1)

for dec in (False, True):
    print("\n## " + ("decomposable" if dec else "non-decomposable") + " targets")
    print("| arm | l | |F| | n | censored | mean work | median | min | max | mean calls | conflicts/call | 2^l | 2^(2l) | 2^(3l)/6 |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for e in summary:
        if e["decomposable"] != dec:
            continue
        l = e["l"]
        f = lambda v: "-" if v is None else (f"{v:.0f}" if v >= 10 else f"{v:.2f}")
        cpc = None if not e["mean_calls"] or e["arm"] == "L" else e["mean_conflicts"] / e["mean_calls"]
        print(f"| {e['arm']} | {l} | {e['base_size']} | {e['n']} | {e['censored']} | {f(e['mean_work'])} | "
              f"{f(e['median_work'])} | {f(e['min'])} | {f(e['max'])} | {f(e['mean_calls'])} | {f(cpc)} | "
              f"{2**l} | {4**l} | {8**l/6:.0f} |")
print("\n## slopes (bits per l, cells with zero censored rows only)")
print("| arm | targets | stat | slope | sizes |")
print("|---|---|---|---|---|")
for k, v in slopes.items():
    if v["slope_bits_per_l"] is None:
        continue
    a, d, s = k.split("|")
    print(f"| {a} | {d} | {s} | {v['slope_bits_per_l']:.2f} | {v['sizes']} |")
nr = [r for r in rows if r["status"] == "NOT_RUN"]
print("\nNOT_RUN rows:", len(nr), " total rows:", len(rows))
print("answer mismatches or failed lifts would have aborted the run (asserts).")
