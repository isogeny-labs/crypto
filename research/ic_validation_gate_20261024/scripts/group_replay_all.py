#!/usr/bin/env python3
"""Group an `ecbench verify --replay-all` receipt by arm and by the receipt's detail text (stdlib only).
usage: group_replay_all.py SESSION_DIR RECEIPT.json OUT.json"""
import json, sys
from collections import Counter, defaultdict

sess, rec_p, out_p = sys.argv[1:4]
recs = {}
for l in open(f"{sess}/records.jsonl"):
    d = json.loads(l); recs[d["seq"]] = d
rc = json.load(open(rec_p))
res = rc.get("replays") or rc.get("results") or []
key = next(k for k, v in rc.items() if isinstance(v, list) and v and isinstance(v[0], dict) and "reproduced" in v[0])
res = rc[key]
by_arm = defaultdict(Counter); detail = defaultdict(Counter); solver_arm = defaultdict(lambda: [0, 0])
for r in res:
    d = recs[r["seq"]]
    by_arm[d["arm"]]["identical" if r["reproduced"] else "differs"] += 1
    if not r["reproduced"]: detail[d["arm"]][r["detail"]] += 1
    s = d.get("solver") or {}
    if s: solver_arm[d["arm"]][0] += 1
out = {"session": rc.get("session_id"), "receipt_ok": rc.get("ok"), "replayed": len(res),
       "identical": sum(r["reproduced"] for r in res), "differs": sum(not r["reproduced"] for r in res),
       "auditor_binary_sha256": rc.get("auditor_binary_sha256"),
       "by_arm": {a: dict(c) for a, c in sorted(by_arm.items())},
       "detail_by_arm": {a: dict(c) for a, c in sorted(detail.items())},
       "records_with_solver_block_by_arm": {a: v[0] for a, v in sorted(solver_arm.items())},
       "arm_methods": {d["arm"]: d["method"]["id"] for d in recs.values()}}
json.dump(out, open(out_p, "w"), indent=1)
print(json.dumps({k: out[k] for k in ("replayed", "identical", "differs", "by_arm", "detail_by_arm")}, indent=1))
