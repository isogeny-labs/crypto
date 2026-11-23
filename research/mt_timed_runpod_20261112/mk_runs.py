#!/usr/bin/env python3
"""Compact per-execution timing/counter table (kept in Git) from runs.jsonl.  usage: mk_runs.py runs.jsonl out.json"""
import json, sys, hashlib
src, out = sys.argv[1], sys.argv[2]
F = ["seq", "rep", "key", "build", "arm", "pb", "T", "idx", "seed", "param", "doff", "targets", "ok", "I", "M", "S", "H", "L", "N",
     "ms_arm", "ms", "wall_s", "loadavg", "steal_ticks", "bin_sha256"]
rows = [json.loads(l) for l in open(src)]
json.dump({"schema": "mt_timed_runpod.runs/v1", "source": "results/holdout/runs.jsonl",
           "source_sha256": hashlib.sha256(open(src, "rb").read()).hexdigest(),
           "commit": sorted({r.get("commit") for r in rows}), "bin_sha256": sorted({r.get("bin_sha256") for r in rows}),
           "fields": F, "rows": [[r.get(f) for f in F] for r in rows]}, open(out, "w"), separators=(",", ":"))
print(len(rows), "rows ->", out)
