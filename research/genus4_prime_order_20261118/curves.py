#!/usr/bin/env python3
"""PROTOCOL section 2: the 5 smallest admitted h = 1 seeds per prime (2640 excluded at 16411). Writes curves.json."""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
out = {}
for p in (16411, 32771):
    inst, scanned = {}, set()
    for f in glob.glob(os.path.join(HERE, f"search_p{p}_*.jsonl*")):
        op = __import__("gzip").open if f.endswith(".gz") else open
        for l in op(f, "rt"):
            r = json.loads(l)
            if r.get("kind") == "instance" and r.get("g") == 4 and r.get("h") == 1:
                inst[r["curve_seed"]] = r
            if r.get("kind") == "config":
                pass
    seeds = sorted(s for s in inst if not (p == 16411 and s == 2640))
    out[str(p)] = {"admitted": seeds, "chosen": seeds[:5], "orders": {str(s): inst[s]["order"] for s in seeds[:5]}}
json.dump(out, open(os.path.join(HERE, "curves.json"), "w"), indent=1)
for p, v in out.items():
    print(p, "admitted", v["admitted"], "chosen", v["chosen"])
