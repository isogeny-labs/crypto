#!/usr/bin/env python3
"""Compare validator replay rows with the producer's raw rows, field by field.

Every field of a replayed run/instance record must equal the raw record
(wall_ns excluded), and the key sets must be equal.
Usage: python3 -I compare_replay.py STUDY_DIR REPLAY_DIR > compare_replay.json
"""
import glob
import json
import sys

STUDY, REP = sys.argv[1], sys.argv[2]
raw = {}
for f in ["raw/run.jsonl", "raw/run65537_odd.jsonl", "raw/run65537_even.jsonl"]:
    for l in open(f"{STUDY}/{f}"):
        if not l.strip():
            continue
        r = json.loads(l)
        if r["kind"] == "run" and r["family"] == "quartic":
            raw[("run", r["p"], r["curve_seed"], r["seed"], r["arm"])] = r
        elif r["kind"] == "instance" and r["family"] == "quartic":
            raw[("inst", r["p"], r["curve_seed"])] = r
out = {"files": [], "rows": 0, "identical": 0, "diffs": [], "uncertified": 0}
for f in sorted(glob.glob(f"{REP}/*.jsonl")):
    out["files"].append(f.rsplit("/", 1)[1])
    for l in open(f):
        if not l.strip():
            continue
        r = json.loads(l)
        if r["kind"] == "run" and r["family"] == "quartic":
            k = ("run", r["p"], r["curve_seed"], r["seed"], r["arm"])
            out["uncertified"] += not (r["certified"] is True and r["ok"] is True)
        elif r["kind"] == "instance" and r["family"] == "quartic":
            k = ("inst", r["p"], r["curve_seed"])
        else:
            continue
        o = raw[k]
        out["rows"] += 1
        if set(r) != set(o):
            out["diffs"].append({"key": k, "keyset": sorted(set(r) ^ set(o))})
            continue
        d = [x for x in r if x != "wall_ns" and r[x] != o[x]]
        if d:
            out["diffs"].append({"key": k, "fields": d})
        else:
            out["identical"] += 1
json.dump(out, sys.stdout, indent=1)
print()
