#!/usr/bin/env python3
"""Rho reference strength: steps and group operations over sqrt(pi r / 2A)
(A = 2 negation, A = 1 plain), per study, arm and prime.  [] indexing."""
import json, math, sys
for fn in sys.argv[1:]:
    rows = [json.loads(l) for l in open(fn)]
    for arm, A in (("R_rho_neg", 2), ("R_rho_plain", 1)):
        for p in (2053, 4099, 8209):
            rs = [r for r in rows if r["kind"] == "run" and r["arm"] == arm and r["p"] == p]
            if not rs:
                continue
            e = [math.sqrt(math.pi * r["r"] / (2 * A)) for r in rs]
            st = [r["steps"] / x for r, x in zip(rs, e)]
            go = [r["group_ops"] / x for r, x in zip(rs, e)]
            print(f"{fn.split('/')[-2]:34s} {arm:12s} p={p} n={len(rs)} steps/expected mean {sum(st)/len(st):.3f} "
                  f"[{min(st):.3f}, {max(st):.3f}]  group_ops/expected mean {sum(go)/len(go):.3f}")
