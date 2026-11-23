#!/usr/bin/env python3
"""Post-run recount of every auxiliary curve the R4 rows selected (Amendment R4 draft, edit E4.7).

For each distinct (p, aux_a, aux_b, aux_order) in the isogeny rows it runs the independent naive counter
(naive_count.c, built static) and requires equality. A mismatch voids every row built on that curve.
    python3 recount.py NAIVE_COUNT_BINARY OUT.json ROWS.jsonl...   (exit 1 on any mismatch)
"""
import json, subprocess, sys

exe, out, paths = sys.argv[1], sys.argv[2], sys.argv[3:]
curves = {}
for path in paths:
    for line in open(path):
        r = json.loads(line)
        if r.get("kind") != "isogeny" or "tower" not in r or "aux_order" not in r["tower"]:
            continue
        tw = r["tower"]
        key = (r["p"], tw["aux_a"], tw["aux_b"])
        curves.setdefault(key, {"aux_order": set(), "files": set(), "t": set()})
        curves[key]["aux_order"].add(tw["aux_order"])
        curves[key]["files"].add(path.split("/")[-1])
        curves[key]["t"].add(r["t"])
res, bad = [], 0
for (p, a, b), info in sorted(curves.items()):
    naive = int(subprocess.run([exe, str(p), str(a), str(b), "8"], capture_output=True, text=True, check=True).stdout)
    ok = info["aux_order"] == {naive}
    bad += not ok
    res.append({"p": p, "aux_a": a, "aux_b": b, "aux_order_rows": sorted(info["aux_order"]), "naive": naive,
                "equal": ok, "t": sorted(info["t"]), "files": sorted(info["files"])})
json.dump({"schema": "pfd.iso-recount/v1", "curves": len(res), "mismatched": bad, "rows": res}, open(out, "w"), indent=1)
print(f"recount: {len(res)} curves, {bad} mismatched")
sys.exit(1 if bad or not res else 0)
