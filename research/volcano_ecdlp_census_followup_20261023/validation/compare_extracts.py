#!/usr/bin/env python3
"""Compare the validator's raw-record re-derivation (rederive.py output) with
the committed producer extracts, row by row, for every session:

* runs/extract/<session>.csv.xz (the ANALYSIS_PLAN s1 rows): ops_gae vs
  total_gae, S vs s, verified, status, isolation, solve_wall_ns, curve_slug,
  warmup, volcano/family/level;
* runs/uncharged/<session>.csv.xz (A2.1): total_gae, s and unc_total vs this
  validator's own reading of A2.1.

Key: (workload_id, target_index, arm, round, warmup). Usage:
python3 -I compare_extracts.py STUDY_DIR ROWS_DIR OUT_JSON
"""
import json
import lzma
import os
import sys

import pandas as pd

STUDY, ROWS, OUT = sys.argv[1:4]
prog = [json.loads(l) for l in open(os.path.join(STUDY, "runs/progress.jsonl"))]
done = [d for d in prog if d.get("status") == "done"]
tot = {"sessions": 0, "extract_rows": 0, "extract_measured_rows": 0, "uncharged_rows": 0,
       "records": 0, "records_measured": 0}
mism = {k: 0 for k in ["extract_missing_record", "record_missing_extract", "ops_gae", "S", "verified",
                       "status", "isolation", "solve_wall_ns", "slug", "spec_fields",
                       "unc_missing_record", "record_missing_unc", "unc_total_gae", "unc_s", "unc_total"]}
per = []
for d in done:
    sess = os.path.basename(d["out"])
    mine = pd.read_csv(os.path.join(ROWS, sess + ".csv.gz"))
    mine["warm"] = mine["warmup"].astype(bool)
    with lzma.open(os.path.join(STUDY, "runs/extract", sess + ".csv.xz"), "rt") as f:
        ex = pd.read_csv(f)
    with lzma.open(os.path.join(STUDY, "runs/uncharged", sess + ".csv.xz"), "rt") as f:
        un = pd.read_csv(f)
    ex["warm"] = ex["warmup"].astype(str).str.lower().eq("true")
    k = ["wid", "tidx", "arm", "round", "warm"]
    ex = ex.rename(columns={"workload_id": "wid", "target_index": "tidx"})
    un = un.rename(columns={"workload_id": "wid", "target_index": "tidx"})
    un["warm"] = False
    j = mine.merge(ex, on=k, how="outer", indicator=True, suffixes=("", "_ex"))
    m = {}
    m["extract_missing_record"] = int((j["_merge"] == "right_only").sum())
    m["record_missing_extract"] = int((j["_merge"] == "left_only").sum())
    b = j[j["_merge"] == "both"]
    m["ops_gae"] = int((b["total_gae"] != b["ops_gae"]).sum())
    m["S"] = int((b["s"] != b["S"]).sum())
    m["verified"] = int(((b["status"] == "verified") != b["verified"].astype(str).str.lower().eq("true")).sum())
    m["status"] = int((b["status"] != b["status_ex"]).sum())
    m["isolation"] = int((b["iso"] != b["isolation"]).sum())
    m["solve_wall_ns"] = int((b["solve_wall_ns"] != b["solve_wall_ns_ex"]).sum())
    m["slug"] = int((b["slug"] != b["curve_slug"]).sum())
    m["spec_fields"] = int(((b["V"] != b["volcano"]) | (b["fam"] != b["family"]) | (b["lvl"] != b["level"])).sum())
    mm = mine[~mine["warm"]]
    u = mm.merge(un, on=k, how="outer", indicator=True, suffixes=("", "_un"))
    m["unc_missing_record"] = int((u["_merge"] == "right_only").sum())
    m["record_missing_unc"] = int((u["_merge"] == "left_only").sum())
    ub = u[u["_merge"] == "both"]
    m["unc_total_gae"] = int((ub["total_gae"] != ub["total_gae_un"]).sum())
    m["unc_s"] = int((ub["s"] != ub["s_un"]).sum())
    m["unc_total"] = int((ub["unc"] != ub["unc_total"]).sum())
    for kk, vv in m.items():
        mism[kk] += vv
    tot["sessions"] += 1
    tot["extract_rows"] += len(ex)
    tot["extract_measured_rows"] += int((~ex["warm"]).sum())
    tot["uncharged_rows"] += len(un)
    tot["records"] += len(mine)
    tot["records_measured"] += int((~mine["warm"]).sum())
    per.append({"session": sess, **m, "extract_rows": len(ex), "uncharged_rows": len(un), "records": len(mine)})
res = {"totals": tot, "mismatches": mism, "all_match": all(v == 0 for v in mism.values()),
       "sessions_with_any_mismatch": [p["session"] for p in per if any(p[k] for k in mism)]}
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res, indent=1))
