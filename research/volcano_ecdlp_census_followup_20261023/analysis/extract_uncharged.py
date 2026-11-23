#!/usr/bin/env python3
"""Per-run uncharged-counter extract (ANALYSIS_PLAN s1, census A2.1 in force from the start).

For every done session in runs/progress.jsonl, one archive at a time: unpack
into a scratch dir, read records.jsonl, write runs/uncharged/<session>.csv.xz
with one row per measured (non-warmup) record, delete the scratch copy, then
go on to the next archive.  <session> is the basename of the progress line's
`out` (so the three redone sessions are `<spec>.r2`).

Columns: spec,workload_id,target_index,arm,round,total_gae,s,unc_total,unc_detail
unc_detail = name=count;... for every counter ecbench lists in cost.unpriced
(value from the record's top-level counters, else the sum of the phases'
native counters of the same name without the _uncharged suffix).
Also writes runs/slugmap/<session>.json: curve slug -> (p, a, b, gx, gy, r,
group_order), the only link from the CSV extracts' curve_slug to a census
vertex or an M model (matched on (a, b)).

Usage: extract_uncharged.py SCRATCH_DIR [--only SESSION ...]
Refuses to run with under 150 MB free on the scratch filesystem.
"""
import csv, io, json, lzma, os, shutil, subprocess, sys

root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
scratch = sys.argv[1]
only = set(sys.argv[3:]) if len(sys.argv) > 2 and sys.argv[2] == "--only" else None
os.makedirs(scratch, exist_ok=True)
prog = [json.loads(l) for l in open(f"{root}/runs/progress.jsonl")]
done = [d for d in prog if d.get("status") == "done" and d.get("audit_ok") is True and "spec" in d]
os.makedirs(f"{root}/runs/uncharged", exist_ok=True)
os.makedirs(f"{root}/runs/slugmap", exist_ok=True)
for d in done:
    name = os.path.basename(d["out"])
    if only and name not in only:
        continue
    if shutil.disk_usage(scratch).free < 150 * 2**20:
        sys.exit("STOP: under 150 MB free; not unpacking another archive")
    subprocess.run(["tar", "-C", scratch, "-xf", f"{root}/sessions/{name}.tar.xz"], check=True)
    rows, slugs = [], {}
    with open(f"{scratch}/{name}/records.jsonl") as f:
        for line in f:
            r = json.loads(line)
            if r.get("warmup"):
                continue
            c = r["cost"]
            det = {}
            for k in c.get("unpriced", []):
                if k in r.get("counters", {}):
                    v = r["counters"][k]
                else:
                    base = k[:-len("_uncharged")] if k.endswith("_uncharged") else k
                    v = sum(p.get("native", {}).get(base, 0) for p in r.get("phases", []))
                det[k] = v
            w = r["workload"]
            cs = w["curve_spec"]
            slugs[w["curve"]["slug"]] = {k: cs.get(k) for k in ("p", "a", "b", "gx", "gy", "r", "group_order")}
            rows.append([name, w["workload_id"], w["target_index"], r["arm"], r["round"],
                         c["total_gae"], c["s"], sum(det.values()),
                         ";".join(f"{k}={v}" for k, v in det.items())])
    shutil.rmtree(f"{scratch}/{name}")
    buf = io.StringIO()
    wr = csv.writer(buf)
    wr.writerow(["spec", "workload_id", "target_index", "arm", "round", "total_gae", "s", "unc_total", "unc_detail"])
    wr.writerows(rows)
    with lzma.open(f"{root}/runs/uncharged/{name}.csv.xz", "wt", preset=9) as o:
        o.write(buf.getvalue())
    json.dump(slugs, open(f"{root}/runs/slugmap/{name}.json", "w"), sort_keys=True)
    print(name, len(rows), flush=True)
