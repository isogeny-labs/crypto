#!/usr/bin/env python3
"""VALIDATOR: copy the validator's outputs from the scratch directory W into the validation directory V in compact
form (Git-sized .json, no .jsonl).
usage: compact_evidence.py <W> <V>"""
import json, sys, os, shutil

W, V = sys.argv[1:3]
shutil.copy(os.path.join(W, "recompute.json"), os.path.join(V, "recompute.json"))
shutil.copy(os.path.join(W, "results_check.json"), os.path.join(V, "results_check.json"))
shutil.copy(os.path.join(W, "ec_check.json"), os.path.join(V, "scalar_check.json"))
shutil.copy(os.path.join(W, "price_model.json"), os.path.join(V, "price_model.json"))

rep = {}
for name in ("replay_timed_all", "replay_producer", "replay_v5", "replay_dump"):
    d = json.load(open(os.path.join(W, name + ".json")))
    rs = d["results"]
    if name == "replay_timed_all":
        rs = [dict(key=x["key"], rows=x["rows"], arm=x["arm"], pb=x["pb"], T=x["T"], idx=x["idx"], param=x["param"], doff=x["doff"],
                   match=x["match"], diffs=x["diffs"]) for x in rs]
    if name == "replay_dump":
        rs = [{k: v for k, v in x.items() if k != "dump"} for x in rs]
    rep[name] = dict(bin_sha256=d["bin_sha256"], nonce=d["nonce"], summary=d["summary"], results=rs)
json.dump(rep, open(os.path.join(V, "replay_check.json"), "w"), indent=0)

F = ["round", "part", "seq", "rep", "pb", "T", "idx", "seed", "key", "targets", "ok", "I", "M", "S", "H", "L", "N", "ms_arm", "ms", "wall_s",
     "steal_jiffies_pin", "other_cpus_busy_jiffies", "loadavg"]
rows = []; extra = {}
for rnd, fn in ((1, "timing.jsonl"), (2, "timing_r2.jsonl")):
    for l in open(os.path.join(W, fn)):
        o = json.loads(l)
        if o.get("header"): extra[f"round{rnd}_header"] = o; continue
        if o.get("footer"): extra[f"round{rnd}_footer"] = o; continue
        if o.get("kind") == "opcost":
            extra.setdefault("opcost", []).append({k: v for k, v in o.items() if k not in ("phases",)}); continue
        o["round"] = rnd
        rows.append([o.get(f) for f in F])
an = json.load(open(os.path.join(W, "timing_analysis.json")))
po = json.load(open(os.path.join(W, "timing_pooled.json")))
json.dump(dict(schema="validator.mt_timed.timing/v1", fields=F, rows=rows, **extra,
               analysis_round1=dict((k, v) for k, v in an.items() if k not in ("header",)), analysis_rounds=po),
          open(os.path.join(V, "timing_reproduction.json"), "w"), separators=(",", ":"))
for f in sorted(os.listdir(V)):
    p = os.path.join(V, f)
    if os.path.isfile(p): print(f, os.path.getsize(p))
