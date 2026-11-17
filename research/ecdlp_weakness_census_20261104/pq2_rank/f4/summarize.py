#!/usr/bin/env python3
"""Merge the msolve runs (msolve_n8_34.json and the per-n/per-instance part
files) into msolve_all.json, one row per main-sweep instance, and print the
comparison with the census lower bound d_iter. Computes nothing new."""
import collections
import glob
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
rows = {}
# the full sweep first, then the part files, which re-ran instances and so take precedence
for p in sorted(glob.glob(os.path.join(HERE, "msolve_*.json")), key=lambda q: (not q.endswith("msolve_n8_34.json"), q)):
    if p.endswith("msolve_all.json"):
        continue
    for r in json.load(open(p))["rows"]:
        rows[(r["n"], r["instance"])] = r
out = [rows[k] for k in sorted(rows)]
json.dump({"source": "msolve 0.6.5 (F4, grevlex) on f4/dumps, the main sweep's own instances",
           "rows": out}, open(os.path.join(HERE, "msolve_all.json"), "w"), indent=1)

ok_lin = sum(r["linear_in_gb"] == r["k"] for r in out)
by_band = collections.defaultdict(collections.Counter)
agree = disagree = 0
for r in out:
    band = "n<=16" if r["n"] <= 16 else "n>=17"
    by_band[band][r["max_step_degree"]] += 1
    d = r.get("census_d_iter")
    if d and d.isdigit():
        if int(d) == r["max_step_degree"]:
            agree += 1
        else:
            disagree += 1
    # where the census says ">3", msolve must be >= 4 if containment holds
    if d == ">3" and r["max_step_degree"] < 4:
        print("CONTRADICTION", r["n"], r["instance"])
print(f"instances: {len(out)}, n = {out[0]['n']}..{out[-1]['n']}")
print(f"linear polynomials in msolve's reduced basis == k: {ok_lin}/{len(out)}")
print("max step degree by band:", {b: dict(c) for b, c in by_band.items()})
print(f"where the census d_iter is exact: msolve max step degree equal {agree}, different {disagree}")
print("census '>3' rows with msolve < 4:", sum(1 for r in out if r.get("census_d_iter") == ">3" and r["max_step_degree"] < 4))
