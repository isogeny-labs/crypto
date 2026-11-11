#!/usr/bin/env python3
"""Compare each replayed record with the committed record of the same (arm, p, seed) or rho run,
on every field except wall_ms. Writes COMPARE.json."""
import json, sys
O = sys.argv[1]
plan = json.load(open(f"{O}/PLAN.json"))["plan"]
def strip(x):
    if isinstance(x, dict): return {k: strip(v) for k, v in x.items() if k != "wall_ms"}
    if isinstance(x, list): return [strip(v) for v in x]
    return x
res = []
for e in plan:
    try: new = json.load(open(e["out"]))
    except Exception as ex: res.append(dict(e, status="missing", error=str(ex))); continue
    src = json.load(open(e["src"]))
    old = [r for r in src if r["seed"] == e["seed"]] if e["kind"] == "cover" else src
    o, n = strip(old), strip(new)
    # The replay build may emit counters added after the study froze (2026-11-03). They are
    # recorded with their values; every field of the committed record must be identical.
    extra = [{k: v for k, v in nr.items() if k not in orow} for orow, nr in zip(o, n)]
    n_common = [{k: v for k, v in nr.items() if k in orow} for orow, nr in zip(o, n)]
    same = len(o) == len(n) and o == n_common
    diff = [] if same else sorted(k for k in o[0] if o[0].get(k) != n[0].get(k))[:20]
    res.append(dict(kind=e["kind"], arm=e.get("arm"), p=e["p"], seed=e["seed"], run=e.get("run"),
                    status="identical" if same else "differs", differing_fields=diff,
                    new_fields_in_replay=extra[0] if extra else {}, src=e["src"], replay=e["out"]))
json.dump(res, open(f"{O}/COMPARE.json", "w"), indent=1)
for r in res: print(r["kind"], r.get("arm"), r["p"], r["seed"], r.get("run"), r["status"], r.get("differing_fields", ""))
