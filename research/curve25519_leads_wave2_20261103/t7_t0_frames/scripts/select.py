#!/usr/bin/env python3
"""T7 frame selection for t0-bits (PROTOCOL.md, "Selection rule" and P0/P1/P2).

Reads data/census_t{4,5,6}.jsonl (one `f4cost ... targets=0` JSON line per lo),
writes data/selection.json, prints `T<t> <L*>` lines for run.sh.

Rule (frozen): L*(t) = argmax covered_points; ties: fewer columns, then the
earlier position in scripts/lo_list.txt.  covered_points is computed by the
engine from point additions over the base and an [r]Q = O test per distinct
sum (decomposable_points + the subgroup filter in f4cost), so the rule uses
no discrete logarithm.

Exit codes: 0 ok; 3 a P0 census anchor mismatched (run.sh then stops before
the engine phase, PROTOCOL.md F2); 4 a census file is incomplete (F1).
"""
import json, math, os, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")
LOS = [int(x) for x in open(os.path.join(HERE, "lo_list.txt")).read().split()]
POS = {lo: i for i, lo in enumerate(LOS)}
R = 131203
LOG2R = math.log2(R)
# P0 anchors, from research/curve25519_t0_algebraic_20261102/cells/*.json
ANCHORS = {(4, 0): (6, 4), (5, 0): (14, 36), (5, 1): (12, 34), (6, 34): (26, 94), (6, 68): (22, 102)}
# bits translation-frame census means, k = 20 (curve25519_frame_search_20261103/analysis/summary.json)
BITS_MEAN = {4: 18.93, 5: 69.776, 6: 271.048}
BITS_BEST = {4: ("bits@797820", 78), 5: ("bits@976503", 198), 6: ("bits@908172", 536)}

out, bad_anchor, incomplete = {"cells": {}}, [], []
for t in (4, 5, 6):
    path = os.path.join(DATA, f"census_t{t}.jsonl")
    rows = {}
    if os.path.exists(path):
        for line in open(path):
            line = line.strip()
            if not line:
                continue
            d = json.loads(line)
            if d.get("t") == t and d.get("form", "").startswith("t0-bits"):
                rows[int(d["lo"])] = d
    missing = [lo for lo in LOS if lo not in rows]
    if missing:
        incomplete.append((t, len(missing)))
    for (ta, lo), (cols, cov) in ANCHORS.items():
        if ta == t and lo in rows:
            got = (rows[lo]["columns"], rows[lo]["covered_points"])
            if got != (cols, cov):
                bad_anchor.append({"t": t, "lo": lo, "expected": [cols, cov], "got": list(got)})
    if not rows:
        continue
    vals = [rows[lo] for lo in LOS if lo in rows]
    covs = [d["covered_points"] for d in vals]
    best = max(vals, key=lambda d: (d["covered_points"], -d["columns"], -POS[int(d["lo"])]))
    mean = statistics.mean(covs)
    lo0 = rows.get(0)
    # Log-free search cost (derived, PROTOCOL.md "Search cost"): per lo,
    # 2^t quotient lookups (one Legendre symbol / square root each),
    # N(N+1)/2 additions for the m = 2 sums of N base points, and one [r]Q
    # per distinct sum at 1.5*log2(r) group operations.
    adds = sum(d["base_points"] * (d["base_points"] + 1) / 2 for d in vals)
    rmul = sum(d["covered_points_all_cosets"] * 1.5 * LOG2R for d in vals)
    out["cells"][str(t)] = {
        "t": t, "n_lo": len(vals), "missing": len(missing),
        "L_star": int(best["lo"]), "L_star_columns": best["columns"],
        "L_star_covered": best["covered_points"],
        "mean_covered": mean, "sd_covered": statistics.pstdev(covs),
        "lo0_covered": lo0["covered_points"] if lo0 else None,
        "lo0_columns": lo0["columns"] if lo0 else None,
        "lo0_percentile": (sum(c < lo0["covered_points"] for c in covs) / len(covs)) if lo0 else None,
        "gain_vs_mean": best["covered_points"] / mean if mean else None,
        "gain_vs_lo0": (best["covered_points"] / lo0["covered_points"]) if lo0 and lo0["covered_points"] else None,
        "P1_ratio_mean_t0_over_bits_mean": mean / BITS_MEAN[t],
        "bits_best_form": BITS_BEST[t][0], "bits_best_covered": BITS_BEST[t][1],
        "L_star_over_bits_best": best["covered_points"] / BITS_BEST[t][1],
        "search_legendre": len(vals) * (1 << t),
        "search_group_ops_derived": adds + rmul,
    }
out["anchor_mismatches"] = bad_anchor
out["incomplete"] = incomplete
with open(os.path.join(DATA, "selection.json"), "w") as f:
    json.dump(out, f, indent=1, sort_keys=True)
for t, c in sorted(out["cells"].items()):
    print(f"T{t} {c['L_star']}")
if incomplete:
    print(f"incomplete census: {incomplete}", file=sys.stderr)
    sys.exit(4)
if bad_anchor:
    print(f"P0 anchor mismatch: {bad_anchor}", file=sys.stderr)
    sys.exit(3)
