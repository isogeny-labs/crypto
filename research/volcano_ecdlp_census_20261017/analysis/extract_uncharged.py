#!/usr/bin/env python3
"""Per-run uncharged-counter extract (ANALYSIS_PLAN s9).

For one session archive: unpack into a scratch dir, read records.jsonl, write
runs/uncharged/<spec>.csv.xz with one row per measured (non-warmup) record:
spec,workload_id,target_index,arm,round,total_gae,s,unc_total,unc_detail
unc_detail = name=count;... for every counter ecbench lists in cost.unpriced
(value from the record's top-level counters, else the sum of the phases'
native counters of the same name without the _uncharged suffix).
Usage: extract_uncharged.py SPEC SCRATCH_DIR
"""
import csv, io, json, lzma, os, subprocess, sys
spec, scratch = sys.argv[1], sys.argv[2]
root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.makedirs(scratch, exist_ok=True)
subprocess.run(["tar", "-C", scratch, "-xf", f"{root}/sessions/{spec}.tar.xz"], check=True)
rows = []
with open(f"{scratch}/{spec}/records.jsonl") as f:
    for line in f:
        r = json.loads(line)
        if r.get("warmup"):
            continue
        c = r["cost"]
        det = {}
        for name in c.get("unpriced", []):
            if name in r.get("counters", {}):
                v = r["counters"][name]
            else:
                base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
                v = sum(p.get("native", {}).get(base, 0) for p in r.get("phases", []))
            det[name] = v
        w = r["workload"]
        rows.append([spec, w["workload_id"], w["target_index"], r["arm"], r["round"],
                     c["total_gae"], c["s"], sum(det.values()),
                     ";".join(f"{k}={v}" for k, v in det.items())])
os.makedirs(f"{root}/runs/uncharged", exist_ok=True)
buf = io.StringIO()
w = csv.writer(buf)
w.writerow(["spec", "workload_id", "target_index", "arm", "round", "total_gae", "s", "unc_total", "unc_detail"])
w.writerows(rows)
with lzma.open(f"{root}/runs/uncharged/{spec}.csv.xz", "wt", preset=9) as o:
    o.write(buf.getvalue())
print(spec, len(rows))
