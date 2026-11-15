#!/usr/bin/env python3
"""Hand-check of one calibrated block (target B, check B3).

Validator V, 2026-11-14. Recomputes, from records.jsonl alone, the
`ops.band.calibrated` grid of `ecbench compare` and the `calibrated` pairs of
`ecbench band --w-cal` for a two-arm session, and compares them with the
binary's JSON. Own reading of the record format; the only producer input is
the class list FIELD_ARITHMETIC_COUNTERS (the definition under test), read
from band.rs at the given commit.

Pricing replicated (band.rs at 97a8e0d841, read, not imported):
  per run  G = cost.total_gae, C = n_C = 0 (asserted: no calibration_pinned_units)
           counts of cost.unpriced: counters[name], else the phase native sums of
           name or name-without-_uncharged, else solver.ops for solver_*;
           minus NON_OPERATION_COUNTERS; plus the TABLE_WORK counter sums where
           a phase carries `counted:<name>` (default pricing)
           S(wf, wo) = (G + wo*U_other + wf*U_field) / sqrt(r)
  grid     ratio[i][j] = sum_B S(wf_i, wo_j) / sum_A S(wf_i, wo_j) over pairs
           matched on (workload_id, round), both verified and not warm-up
  order_at per field weight: "B < A" if every ratio < 1, "A < B" if every > 1,
           else flips; ranked iff the same strict order at all three.

Usage: b3_handcheck.py RECORDS.jsonl BAND_RS A_ARM B_ARM W_CAL COMPARE_JSON [BAND_JSON] OUT
"""
import json, math, re, sys

rec_path, band_rs, A, B, wcal, cmp_path = sys.argv[1:7]
band_json = sys.argv[7] if len(sys.argv) > 8 else None
out_path = sys.argv[-1]
wcal = float(wcal)
src = open(band_rs).read()
blk = src[src.index("pub const FIELD_ARITHMETIC_COUNTERS"):]
blk = blk[:blk.index("];")]
FIELD = set(re.findall(r'"([^"]+)"', blk))
NONOP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"}
TABLE = ["table_insert_attempts", "sibling_second_points", "sibling_lift_products"]


def is_field(n):
    n = n.rsplit(".", 1)[-1]
    return n in FIELD and n not in NONOP


recs = [json.loads(l) for l in open(rec_path) if l.strip()]


def counts(r):
    out = {}
    for name in r["cost"]["unpriced"]:
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in r["counters"]:
            out[name] = r["counters"][name]; continue
        tot, found = 0, False
        for p in r["phases"]:
            nat = p.get("native", {})
            if name in nat: tot += nat[name]; found = True
            elif base in nat: tot += nat[base]; found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]; continue
        out[name] = tot if found else None
    out = {k: v for k, v in out.items() if k not in NONOP}
    for t in TABLE:
        if any(("counted:" + t) in p.get("work_native", {}) for p in r["phases"]):
            out[t] = out.get(t, 0) + sum(p.get("work_native", {}).get(t, 0) for p in r["phases"])
    return out


def split(r):
    assert not (r.get("detail") or {}).get("calibration_pinned_units"), "pinned calibration not replicated"
    c = counts(r)
    assert all(v is not None for v in c.values()), c
    f = sum(v for k, v in c.items() if is_field(k))
    o = sum(v for k, v in c.items() if not is_field(k))
    return r["cost"]["total_gae"], o, f, math.sqrt(r["workload"]["curve"]["r"]), c


def S(r, wf, wo):
    g, o, f, sq, _ = split(r)
    return (g + wo * o + wf * f) / sq


key = lambda r: (r["workload"]["workload_id"], r["round"])
side = lambda arm: {key(r): r for r in recs if r["arm"] == arm and not r["warmup"] and r["outcome"]["status"] == "verified"}
a, b = side(A), side(B)
pairs = [(a[k], b[k]) for k in sorted(a) if k in b]
FW = [0.0, wcal, 10 * wcal]
WO = [0.0, 0.1, 1.0]
grid = [[sum(S(rb, wf, wo) for _, rb in pairs) / sum(S(ra, wf, wo) for ra, _ in pairs) for wo in WO] for wf in FW]
order = {}
dirs = []
for lab, row in zip(["0", "w_cal", "10*w_cal"], grid):
    if all(x < 1 for x in row): order[lab] = f"{B} < {A}"; dirs.append(-1)
    elif all(x > 1 for x in row): order[lab] = f"{A} < {B}"; dirs.append(1)
    else: order[lab] = "flips"; dirs.append(0)
ranked = dirs[0] != 0 and all(d == dirs[0] for d in dirs)
mine = {"pairs": len(pairs), "field_weights": FW, "other_weights": WO, "ratio_b_over_a": grid, "order_at": order,
        "ranked_at_calibrated_price": ranked,
        "per_run": [{"arm": r["arm"], "key": list(key(r)), "G": split(r)[0], "U_other": split(r)[1], "U_field": split(r)[2],
                     "sqrt_r": split(r)[3], "counts": split(r)[4]} for p in pairs for r in p]}
cmpj = json.load(open(cmp_path))
cal = cmpj["ops"]["band"]["calibrated"]
res = {"mine": mine, "binary_compare": cal, "checks": []}
ok = True
for i in range(3):
    for j in range(3):
        bv = cal["ratio_b_over_a"][i][j]
        good = bv is not None and abs(bv - grid[i][j]) <= 1e-12 * max(1, abs(grid[i][j]))
        ok &= good
        res["checks"].append({"cell": [FW[i], WO[j]], "binary": bv, "mine": grid[i][j], "ok": good})
for k in order:
    ok &= cal["order_at"][k] == order[k]
ok &= cal["ranked_at_calibrated_price"] == ranked
ok &= abs(cal["w_cal"] - wcal) < 1e-15 and cal["field_weights"] == FW
# the band order (governing) of the same comparison, for the side-by-side reading
res["band_order_binary"] = cmpj["ops"]["band"].get("order")
if band_json:
    bj = json.load(open(band_json))
    res["binary_band_calibrated"] = bj.get("calibrated")
    # band: per-arm mean S over verified runs (not pairs); order per field weight must hold at every other weight
    def mean_s(arm, wf, wo):
        rs = [r for r in recs if r["arm"] == arm and not r["warmup"] and r["outcome"]["status"] == "verified"]
        return sum(S(r, wf, wo) for r in rs) / len(rs)
    border = {}
    for lab, wf in zip(["0", "w_cal", "10*w_cal"], FW):
        sa = [mean_s(A, wf, wo) for wo in WO]; sb = [mean_s(B, wf, wo) for wo in WO]
        res.setdefault("band_mean_s", {})[lab] = {A: sa, B: sb}
        border[lab] = (A, B) if all(x < y for x, y in zip(sa, sb)) else ((B, A) if all(y < x for x, y in zip(sa, sb)) else None)
    res["band_order_mine"] = {k: (f"{v[0]} < {v[1]}" if v else "flips") for k, v in border.items()}
res["ok"] = ok
json.dump(res, open(out_path, "w"), indent=1)
print("pairs", len(pairs), "ok", ok)
for row in grid: print(["%.10g" % x for x in row])
print(order, ranked)
