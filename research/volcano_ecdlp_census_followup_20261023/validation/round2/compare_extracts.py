#!/usr/bin/env python3
"""Round-2 validator: compare the re-derived rows with runs/extract and runs/uncharged.

Validator-written. Key: (session, workload_id, target_index, arm, round, warmup).
Compares ops_gae (= total_gae), S, solve_wall_ns, isolation, verified, curve slug,
volcano/family/level and the A2.1 uncharged count (measured rows only).
Usage: compare_extracts.py STUDY_DIR ROWS_CSV_GZ OUT_JSON
"""
import json, os, sys
import pandas as pd

study, rows_path, out = sys.argv[1:4]
mine = pd.read_csv(rows_path)
mine["session"] = mine["session"].astype(str)
res = {"sessions": 0, "extract_rows": 0, "mine_rows": int(len(mine)), "mismatch": {}, "uncharged_rows": 0,
       "uncharged_mismatch": 0, "uncharged_detail_mismatch": 0, "missing_in_extract": 0, "missing_in_mine": 0}
mm = {k: 0 for k in ["ops_gae", "S", "solve_wall_ns", "iso", "verified", "slug", "volcano", "family", "level"]}
for sess, g in mine.groupby("session"):
    res["sessions"] += 1
    ex = pd.read_csv(os.path.join(study, "runs", "extract", sess + ".csv.xz"))
    res["extract_rows"] += len(ex)
    ex["warmup"] = ex["warmup"].astype(str).str.lower().map({"true": 1, "false": 0})
    k = ["workload_id", "target_index", "arm", "round", "warmup"]
    j = g.merge(ex, on=k, how="outer", indicator=True, suffixes=("", "_x"))
    res["missing_in_extract"] += int((j["_merge"] == "left_only").sum())
    res["missing_in_mine"] += int((j["_merge"] == "right_only").sum())
    j = j[j["_merge"] == "both"]
    mm["ops_gae"] += int((j["total_gae"] != j["ops_gae"]).sum())
    mm["S"] += int((j["S"] != j["S_x"]).sum())
    mm["solve_wall_ns"] += int((j["solve_wall_ns"] != j["solve_wall_ns_x"]).sum())
    mm["iso"] += int((j["iso"] != j["isolation"]).sum())
    mm["verified"] += int((j["verified"].astype(bool) != j["verified_x"].astype(str).str.lower().eq("true")).sum())
    mm["slug"] += int((j["slug"] != j["curve_slug"]).sum())
    mm["volcano"] += int((j["V"] != j["volcano"]).sum())
    mm["family"] += int((j["fam"] != j["family"]).sum())
    mm["level"] += int((j["level"] != j["level_x"].astype(int)).sum())
    un = pd.read_csv(os.path.join(study, "runs", "uncharged", sess + ".csv.xz"))
    res["uncharged_rows"] += len(un)
    gm = g[g["warmup"] == 0]
    ju = gm.merge(un, on=["workload_id", "target_index", "arm", "round"], how="outer", indicator=True,
                  suffixes=("", "_u"))
    res["missing_in_extract"] += int((ju["_merge"] != "both").sum())
    ju = ju[ju["_merge"] == "both"]
    res["uncharged_mismatch"] += int(((ju["unc"] != ju["unc_total"]) | (ju["total_gae"] != ju["total_gae_u"]) |
                                       (ju["S"] != ju["s"])).sum())
    res["uncharged_detail_mismatch"] += int((ju["unc_detail"].fillna("") != ju["unc_detail_u"].fillna("")).sum())
res["mismatch"] = mm
json.dump(res, open(out, "w"), indent=1)
print(json.dumps(res))
