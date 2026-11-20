#!/usr/bin/env python3
"""PROTOCOL section 2: covered range and admitted h = 1 seeds per (genus, prime); writes curves.json."""
import glob, gzip, json, os, re
HERE = os.path.dirname(os.path.abspath(__file__))
NEED = {(4, 54293): 3, (5, 2357): 5, (5, 4099): 3}
out = {}
for (g, p), need in NEED.items():
    adm, tried, ranges = set(), 0, set()
    for f in glob.glob(os.path.join(HERE, f"search_g{g}_p{p}_r*.jsonl*")):
        if f.endswith(".jsonl") and os.path.exists(f + ".gz"):
            continue
        ranges.add(re.search(r"_r(\d+)-(\d+)_", f).groups())
        op = gzip.open if f.endswith(".gz") else open
        for l in op(f, "rt"):
            r = json.loads(l)
            if r.get("kind") == "instance" and r.get("g") == g and r.get("h") == 1:
                adm.add(r["curve_seed"])
            if r.get("kind") == "cell":
                tried += r["tried"]
    span = sum(int(b) - int(a) + 1 for a, b in ranges)
    seeds = sorted(adm)
    cell = {"admitted": seeds, "tried": tried, "span": span, "complete": tried == span, "need": need}
    if g == 5 and p == 2357:
        cell["confirm"], cell["tune"] = seeds[:3], seeds[3:5]
    else:
        cell["confirm"] = seeds[:3]
    cell["enough"] = len(seeds) >= need and cell["complete"]
    out[f"g{g}_p{p}"] = cell
json.dump(out, open(os.path.join(HERE, "curves.json"), "w"), indent=1)
for k, v in out.items():
    print(k, {x: v[x] for x in ("admitted", "tried", "span", "complete", "enough")})
