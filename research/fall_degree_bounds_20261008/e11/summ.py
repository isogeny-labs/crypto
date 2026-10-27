#!/usr/bin/env python3
"""One line per hky_window record: draw, N, degree-2/3 dims, per-policy result."""
import json
import sys

for path in sys.argv[1:] or ["/dev/stdin"]:
    for ln in open(path):
        if not ln.startswith("{"):
            continue
        r = json.loads(ln)
        win = {k: (v.get("certified"), v["cols"], v.get("dim"), v.get("secs"), v.get("vmhwm_kb"))
               for k, v in r["window"].items()}
        print(f"{r['m']}:{r['n']}:{r['ell']} draw={r['draw']} N={r['n_vars']}",
              {k: v["dim"] for k, v in r["per_degree"].items()}, "d<=3:", r["d_f_le3"], win)
