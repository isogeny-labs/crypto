#!/usr/bin/env python3
"""Reduces results.json (frozen analyze.py) to summary.json and applies the frozen decision rule (PROTOCOL.md section 3):
candidate sw / incumbent ls passes on a curve iff the upper 95% bound is < 1 at w = 0, 0.1 and 1, pooled AND in each
replicate. Upper bound = the larger of ecbench compare's two 95% upper ends (linearised-t ci95 and cluster-bootstrap
ci95_bootstrap). Recorded ecbench outputs only; no new statistics."""
import json, os, gzip
HERE = os.path.dirname(os.path.abspath(__file__)); _p = os.path.join(HERE, "results.json")
R = json.load(open(_p) if os.path.exists(_p) else gzip.open(_p + ".gz", "rt"))
W = ["0", "0.1", "1"]
def norm(c):
    o = c.get("ops", c)  # single-session compare nests under "ops"; pooled may be flat
    hi = max(o["ci95"][1], o["ci95_bootstrap"][1]); lo = min(o["ci95"][0], o["ci95_bootstrap"][0])
    return {"ratio": o["ratio_b_over_a"], "lo": lo, "hi": hi, "ci95_t": o["ci95"], "ci95_boot": o["ci95_bootstrap"], "pairs": o["pairs"]}
S = {"ratios": {}, "phases": R["phases"], "native": R["native"], "wall": R["wall"], "sessions": R["sessions"], "status": R["status"],
     "identity": R["identity"], "same_walk": R["same_walk"], "band_S": {}, "band_ranks": {}, "decision": {}}
for key, c in R["compare"].items():
    n, scope, pair, w = key.split("|"); w = w[2:]
    S["ratios"].setdefault(n, {}).setdefault(scope, {}).setdefault(pair, {})[w] = norm(c)
for name, b in R["band"].items():
    S["band_S"][name] = {row["arm"]: dict(zip(W, row["mean_s"])) for row in b["rows"]}
    S["band_ranks"][name] = {f'{x["a"]} vs {x["b"]}': x["verdict"] for x in b["ranks"]}
for n in ("23", "29", "37"):
    per = {scope: {w: S["ratios"][n][scope]["sw/ls"][w]["hi"] < 1 for w in W} for scope in ("pooled", "r1", "r2", "r3")}
    S["decision"][n] = {"per_scope_upper_below_1": per, "passes": all(all(v.values()) for v in per.values())}
json.dump(S, open(os.path.join(HERE, "summary.json"), "w"), indent=1, sort_keys=True)
for n in ("23", "29", "37"):
    for scope in ("pooled", "r1", "r2", "r3"):
        for pair, v in sorted(S["ratios"][n][scope].items()):
            print(f"n{n} {scope:6s} {pair:20s} " + "  ".join(f"w={w}: {v[w]['ratio']:.3f} [{v[w]['lo']:.3f},{v[w]['hi']:.3f}]" for w in W if w in v))
    print(f"n{n} decision", S["decision"][n]["passes"], "identity", S["identity"][n], "same_walk", S["same_walk"][n])
