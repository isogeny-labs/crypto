#!/usr/bin/env python3
"""Two extra replays (validation/out/jobs_extra.txt), compared with committed rows.

1. Saturation check: tune key (2254, k = 0) run at cc 4.0 instead of 2.8.  If cc 2.8 already puts
   every degree-1 place into the factor base at p = 2357, the run rows must equal the committed
   tune5_cc2.8 rows in every non-timing field.
2. The measured plain rho of G3 (curve 466, k = 0), solver seed 1 only, --threads 1.
Writes validation/out/compare_extra.json.
"""
import json, os

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
TIMING = {"wall_ns", "wall_s", "search_wall_s"}


def rows(p):
    return [json.loads(l) for l in open(p) if l.strip()]


def cmp(rep_file, com_file):
    rep, com = rows(rep_file), rows(com_file)
    cr = {(r["curve_seed"], r["target_seed"], r["seed"], r["arm"]): r for r in com if r["kind"] == "run"}
    ci = {r["curve_seed"]: r for r in com if r["kind"] == "instance"}
    res = {"done": any(r["kind"] == "done" for r in rep), "runs": 0, "identical": 0, "diffs": [], "instance_identical": None}
    for r in rep:
        if r["kind"] == "instance":
            res["instance_identical"] = r == ci[r["curve_seed"]]
        if r["kind"] == "run":
            c = cr[(r["curve_seed"], r["target_seed"], r["seed"], r["arm"])]
            res["runs"] += 1
            keys = (set(r) | set(c)) - TIMING
            d = {x: (c.get(x, "<absent>"), r.get(x, "<absent>")) for x in sorted(keys) if r.get(x, "<absent>") != c.get(x, "<absent>")}
            if d:
                res["diffs"].append({"key": [r["curve_seed"], r["target_seed"], r["seed"], r["arm"]], "diff": d})
            else:
                res["identical"] += 1
    return res


out = {}
f = os.path.join(OUT, "replay", "satcheck_tune5_cc4.0_2254_k0.jsonl")
if os.path.exists(f):
    out["saturation_cc4.0_vs_committed_cc2.8"] = cmp(f, os.path.join(S, "tune5_cc2.8.jsonl"))
f = os.path.join(OUT, "replay", "rho5_p2357_466_k0_seed1.jsonl")
if os.path.exists(f):
    out["rho5_seed1"] = cmp(f, os.path.join(S, "rho5_p2357.jsonl"))
json.dump(out, open(os.path.join(OUT, "compare_extra.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
