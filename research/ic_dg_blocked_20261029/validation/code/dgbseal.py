#!/usr/bin/env python3
"""Recompute every record seal (validator's own code): a records.jsonl line with
its "record_id":"<id>" emptied must hash (sha256, first 12 hex) to <id> after
the prefix ECR1h.  Also checks each record's session_id against session.json
and that record ids are unique.  Writes work/seals.json."""
import hashlib
import json
import re

S3 = "/dev/shm/dgb-val/s3"
OUT = "/dev/shm/dgb-validation/work/seals.json"
RID = re.compile(r'"record_id":"(ECR1h[0-9a-f]{12})"')

res = {}
for b in (24, 28, 32):
    for s in ("wall", "counted"):
        d = f"{S3}/P{b}-{s}/session"
        sess = json.load(open(f"{d}/session.json"))
        n = bad = wrong_session = 0
        ids = set()
        with open(f"{d}/records.jsonl") as f:
            for line in f:
                line = line.rstrip("\n")
                m = RID.search(line)
                n += 1
                if not m:
                    bad += 1
                    continue
                rid = m.group(1)
                blank = line.replace(m.group(0), '"record_id":""', 1)
                if "ECR1h" + hashlib.sha256(blank.encode()).hexdigest()[:12] != rid:
                    bad += 1
                ids.add(rid)
                if json.loads(line)["session_id"] != sess["session_id"]:
                    wrong_session += 1
        res[f"P{b}-{s}"] = {"session_id": sess["session_id"], "records": n, "seal_failures": bad,
                            "unique_ids": len(ids), "wrong_session_id": wrong_session}
res["total_records"] = sum(v["records"] for v in res.values())
res["total_seal_failures"] = sum(v["seal_failures"] for k, v in res.items() if isinstance(v, dict))
json.dump(res, open(OUT, "w"), indent=1, sort_keys=True)
print(json.dumps(res, indent=1))
