import json, glob, math, os
R = "/home/user/crypto/research/ic_m3_algebraic_vs_mitm_20261014"
for f in sorted(glob.glob(R + "/runs/m3/truth-*.json")):
    d = json.load(open(f))
    rows = d["rows"]
    ns = [r["ns"] for r in rows]
    yes = sum(1 for r in rows if r["yes"])
    mean = sum(ns) / len(ns)
    p = d["decomposable"] / d["pool"]
    fc = d["fb_columns"]
    amort = mean / p + d["table_build_ns"] / fc
    online = mean / p
    print(os.path.basename(f), "pool", d["pool"], "decomp", d["decomposable"], "p", round(p, 4), "yes_rows", yes,
          "mean_ns", round(mean, 1), "floor_sum_ns", d["floor_sum_ns"], "fcols", fc, "fpts", d["fb_points"],
          "log2 amort", round(math.log2(amort), 3), "log2 online", round(math.log2(online), 3), "exh", d["exhaustive_check"][:1] if d.get("exhaustive_check") else None)
