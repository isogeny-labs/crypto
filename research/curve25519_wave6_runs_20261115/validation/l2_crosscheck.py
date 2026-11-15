#!/usr/bin/env python3
"""Validator cross-check of the producer's ecbench read-outs (band-k*.json, comparisons/*.json) against
numbers recomputed here from records.jsonl alone (stdlib only; no producer code imported).
Usage: l2_crosscheck.py <L2_dir> <out.json>"""
import json, math, sys, glob, os
from collections import defaultdict

L2, OUT = sys.argv[1], sys.argv[2]
NON_OP = {"residual_table_bytes_uncharged", "factor_base_vec_bytes_floor_uncharged", "native_table_bytes_uncharged"}


def U(rec):
    tot = 0
    for name in rec["cost"].get("unpriced", []):
        if name in NON_OP:
            continue
        base = name[:-10] if name.endswith("_uncharged") else name
        if name in rec.get("counters", {}):
            tot += rec["counters"][name]; continue
        for ph in rec["phases"]:
            nat = ph.get("native") or {}
            tot += nat.get(name, nat.get(base, 0))
    if any("counted:table_insert_attempts" in (ph.get("work_native") or {}) for ph in rec["phases"]):
        tot += sum((ph.get("work_native") or {}).get("table_insert_attempts", 0) for ph in rec["phases"])
    return tot


out = {}
for k in (20, 24):
    recs = [json.loads(l) for l in open(f"{L2}/sessions/k{k}/records.jsonl")]
    meas = [r for r in recs if not r["warmup"]]
    rows = defaultdict(lambda: {"n": 0, "s": [0.0, 0.0, 0.0], "charged": 0.0, "native": 0})
    for r in meas:
        key = (r["workload"]["curve"]["slug"], r["arm"])
        sq = math.sqrt(r["workload"]["curve"]["r"])
        u = U(r)
        rw = rows[key]; rw["n"] += 1; rw["charged"] += r["cost"]["total_gae"]; rw["native"] += u
        for i, w in enumerate((0.0, 0.1, 1.0)):
            rw["s"][i] += (r["cost"]["total_gae"] + w * u) / sq
    band = json.load(open(f"{L2}/band-k{k}.json"))
    maxd, maxc, maxn, n_rows = 0.0, 0.0, 0, 0
    for row in band["rows"]:
        rw = rows[(row["curve"], row["arm"])]
        n_rows += 1
        for i in range(3):
            maxd = max(maxd, abs(rw["s"][i] / rw["n"] - row["mean_s"][i]))
        maxc = max(maxc, abs(rw["charged"] - row["charged_sum"]))
        maxn = max(maxn, abs(rw["native"] - row["native_sum"]))
        if rw["n"] != row["runs"]:
            maxn = max(maxn, 10 ** 9)
    res = {"band_rows": n_rows, "band_max_abs_mean_s_diff": maxd, "band_max_abs_charged_sum_diff": maxc,
           "band_max_abs_native_sum_diff": maxn, "band_ranks": band.get("ranks")}
    # comparisons: recompute ratio of sum S over matched (workload, round) pairs at the file's weight
    comp = {}
    idx = defaultdict(dict)
    for r in meas:
        idx[(r["round"], r["workload"]["workload_id"])][r["arm"]] = r
    for f in sorted(glob.glob(f"{L2}/sessions/k{k}/comparisons/*.json")):
        c = json.load(open(f))
        a, b, w = c["a"]["arm"], c["b"]["arm"], c["pricing_weight"] or 0.0
        sa = sb = 0.0
        for v in idx.values():
            for arm, acc in ((a, "a"), (b, "b")):
                pass
            sqa = math.sqrt(v[a]["workload"]["curve"]["r"])
            sa += (v[a]["cost"]["total_gae"] + w * U(v[a])) / sqa
            sb += (v[b]["cost"]["total_gae"] + w * U(v[b])) / sqa
        comp[os.path.basename(f)] = {"a": a, "b": b, "w": w, "file_ratio": c["ops"]["ratio_b_over_a"],
                                     "recomputed_ratio": sb / sa, "abs_diff": abs(sb / sa - c["ops"]["ratio_b_over_a"]),
                                     "file_ci95": c["ops"]["ci95"], "file_band": c["ops"]["band"]["ratio_b_over_a"],
                                     "file_order": c["ops"]["band"]["order"], "same_seeds": c["ops"]["same_seeds"],
                                     "pairs": c["ops"]["pairs"], "wall_ratio_ns_per_gae": c["wall"].get("ratio_b_over_a"),
                                     "wall_total_ratio": c["wall"].get("total_wall_ratio"), "wall_status": c["wall"].get("status"),
                                     "quality_score": c["quality"]["score"]}
    res["comparisons"] = comp
    out[f"k{k}"] = res
json.dump(out, open(OUT, "w"), indent=1)
print(json.dumps(out, indent=1))
