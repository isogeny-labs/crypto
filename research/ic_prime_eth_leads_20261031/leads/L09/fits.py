#!/usr/bin/env python3
"""log-log slopes of s vs b from table.json (split mode)."""
import json, math, sys
rows = [r for r in json.load(open(sys.argv[1] if len(sys.argv) > 1 else "table.json")) if r["mode"] in ("split", "both")]
def slope(a, b): return math.log(b[1] / a[1]) / math.log(b[0] / a[0])
def series(pred):
    pts = sorted({(r["b"], r["s"]) for r in rows if pred(r)}); return pts
out = {}
for name, pred in {
  "PDP chained-S3 m=3 plain": lambda r: r["group"].startswith("prime PDP chained-S3") and "plain" in r["group"],
  "PDP chained-S3 m=3 xor": lambda r: r["group"].startswith("prime PDP chained-S3") and "XOR" in r["group"],
  "PDP quotient/group-law m=2 plain": lambda r: r["group"].startswith("prime PDP quotient") and "plain" in r["group"],
  "PDP quotient/group-law m=2 xor": lambda r: r["group"].startswith("prime PDP quotient") and "XOR" in r["group"],
  "Bochum m=2 (crate) b=6..16": lambda r: r["group"].startswith("hashed Bochum") and r["b"] >= 6,
  "Bochum m=2 (crate) b=16..131 (incl. ECCp-131)": lambda r: (r["group"].startswith("hashed Bochum") and r["b"] == 16) or r["label"].startswith("ECCp-131"),
}.items():
    p = series(pred)
    out[name] = dict(points=p, slopes=[round(slope(p[i], p[i+1]), 3) for i in range(len(p) - 1)], overall=round(slope(p[0], p[-1]), 3))
print(json.dumps(out, indent=1))
