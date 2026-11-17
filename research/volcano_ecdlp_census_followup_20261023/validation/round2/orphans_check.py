#!/usr/bin/env python3
"""Round-2 validator: compare each restart-orphaned partial session (runs/orphans/*.orphan.tar.xz)
with its redone .r2 session, record by record (seq, round, warmup, arm, workload_id, target_index,
total_gae, recovered). Streams both archives; nothing is unpacked to disk. Validator-written.
Usage: orphans_check.py STUDY_DIR"""
import json, os, sys, tarfile


def records(path):
    out = {}
    status = None
    with tarfile.open(path, "r|xz") as tf:
        for m in tf:
            if not m.isfile():
                continue
            if m.name.endswith("session.json"):
                status = json.load(tf.extractfile(m)).get("status")
            if m.name.endswith("records.jsonl"):
                for line in tf.extractfile(m).read().splitlines():
                    if line.strip():
                        r = json.loads(line)
                        out[r["seq"]] = (r["round"], r["warmup"], r["arm"], r["workload"]["workload_id"],
                                         r["workload"]["target_index"], r["cost"]["total_gae"], r["outcome"]["recovered"])
    return out, status


study = sys.argv[1]
res = {}
for f in sorted(os.listdir(os.path.join(study, "runs", "orphans"))):
    if not f.endswith(".orphan.tar.xz"):
        continue
    spec = f[: -len(".orphan.tar.xz")]
    o, so = records(os.path.join(study, "runs", "orphans", f))
    r, sr = records(os.path.join(study, "sessions", spec + ".r2.tar.xz"))
    same = sum(1 for k, v in o.items() if r.get(k) == v)
    res[spec] = {"orphan_records": len(o), "orphan_status": so, "r2_records": len(r), "r2_status": sr,
                 "orphan_records_identical_in_r2": same, "all_identical": same == len(o)}
print(json.dumps(res, indent=1))
