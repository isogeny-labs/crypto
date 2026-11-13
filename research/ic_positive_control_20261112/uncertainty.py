#!/usr/bin/env python3
"""Bootstrap 95% intervals of the C/R cost ratio per p (resampling runs), from control.json."""
import json, random, sys
d = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "control.json"))
rows = d["rows"]
random.seed(20261112)
by = {}
for r in rows:
    by.setdefault((r["p"], r["arm"]), []).append(r)
ps = sorted({r["p"] for r in rows})
def mean(v): return sum(v) / len(v)
print("| p | n_C | n_R | key | C/R | 95% interval | CV(C) | CV(R) |")
print("|--:|--:|--:|--|--:|--|--:|--:|")
for p in ps:
    C = by[(p, "C_double_lp")]; R = by[(p, "R_rho_negation")]
    for key in ["total_w0", "total_w01", "total_w1", "total_cal"]:
        c = [x[key] for x in C]; r = [x[key] for x in R]
        est = mean(c) / mean(r)
        bs = []
        for _ in range(2000):
            cc = [random.choice(c) for _ in c]; rr = [random.choice(r) for _ in r]
            bs.append(mean(cc) / mean(rr))
        bs.sort()
        lo, hi = bs[int(0.025 * len(bs))], bs[int(0.975 * len(bs))]
        sd = lambda v: (sum((x - mean(v)) ** 2 for x in v) / max(1, len(v) - 1)) ** 0.5 / mean(v)
        print(f"| {p} | {len(c)} | {len(r)} | {key} | {est:.3f} | [{lo:.3f}, {hi:.3f}] | {sd(c):.3f} | {sd(r):.3f} |")
