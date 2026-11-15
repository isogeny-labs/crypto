#!/usr/bin/env python3
"""Coverage estimands of PROTOCOL.md from runs/U_p*.txt (PARI) and runs/FW_p*.jsonl (Rust). Writes analysis.json."""
import json, math, glob
def wilson(k, n, z=1.96):
    if n == 0: return [0.0, 1.0]
    ph = k / n; d = 1 + z * z / n; c = (ph + z * z / (2 * n)) / d; h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / d
    return [max(0.0, c - h), min(1.0, c + h)]
out = {}
for p in (37, 53, 1009, 1511, 1823):
    row = {}
    try:
        U = [l.split() for l in open(f"runs/U_p{p}.txt") if l.strip()]
        n = len(U); ev = sum(u[3] == "1" for u in U); ad = sum(u[4] == "1" for u in U)
        row["U"] = {"n": n, "even": ev / n, "admitted": ad / n, "admitted_ci": wilson(ad, n), "admitted_k": ad}
    except FileNotFoundError:
        pass
    try:
        F = [json.loads(l) for l in open(f"runs/FW_p{p}.jsonl") if l.strip()]
        if not F: raise FileNotFoundError
        n = len(F); ad = [x for x in F if x["admitted"]]; wk = [x for x in ad if x["walked"]]; fd = [x for x in wk if x["found"]]
        row["F"] = {"n": n, "admitted": len(ad) / n, "admitted_ci": wilson(len(ad), n), "start_weak": sum(x["start_weak"] for x in F)}
        row["W"] = {"walked": len(wk), "reached": len(fd), "reached_frac": len(fd) / max(1, len(wk)), "reached_ci": wilson(len(fd), len(wk)),
                    "path_max": max([x["path"] for x in fd], default=0),
                    "muls_walk_median": sorted(x["muls_walk"] for x in wk)[len(wk) // 2] if wk else 0}
        if "U" in row:
            row["operational_coverage"] = row["U"]["admitted"] * row["W"]["reached_frac"]
    except FileNotFoundError:
        pass
    out[str(p)] = row
json.dump(out, open("analysis.json", "w"), indent=1)
for p, r in out.items():
    print(p, {k: ({kk: (round(vv, 4) if isinstance(vv, float) else vv) for kk, vv in v.items()} if isinstance(v, dict) else round(v, 5)) for k, v in r.items()})
