#!/usr/bin/env python3
"""Compare every replayed row with the committed row: all fields except wall-clock ones must be equal.

Committed rows are the producer's working-tree .jsonl files (untracked; hashes in raw_manifest.json).
Run rows are matched on (curve_seed, target_seed, seed, arm); instance rows on curve_seed; the cell
row of a replay covers only the replayed curve, so only its admitted / rejects are compared with the
instance's presence.  Writes validation/out/replay_compare.json.
"""
import json, os

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
TIMING = {"wall_ns", "wall_s", "search_wall_s"}


def rows(path):
    return [json.loads(l) for l in open(path) if l.strip()]


jobs = [json.loads(l) for l in open(os.path.join(OUT, "jobs.txt"))]
BLOCKS = sorted(json.load(open(os.path.join(OUT, "plan.json")))["blocks"], key=len, reverse=True)
res, tot, same = [], 0, 0
for j in jobs:
    if j["kind"] == "valpo":
        continue
    tag = j["tag"]
    block = next(b for b in BLOCKS if tag.startswith(b + "_"))
    f = os.path.join(OUT, "replay", tag + ".jsonl")
    if not os.path.exists(f):
        res.append({"tag": tag, "status": "not run"})
        continue
    rep = rows(f)
    if not any(r["kind"] == "done" for r in rep):
        res.append({"tag": tag, "status": "incomplete"})
        continue
    com = rows(os.path.join(S, block + ".jsonl"))
    cr = {(r["curve_seed"], r["target_seed"], r["seed"], r["arm"]): r for r in com if r["kind"] == "run"}
    ci = {r["curve_seed"]: r for r in com if r["kind"] == "instance"}
    ent = {"tag": tag, "binary": j["kind"], "runs": 0, "identical": 0, "diffs": [], "instance_identical": None,
           "extra_fields_in_replay": set()}
    for r in rep:
        if r["kind"] == "instance":
            c = ci[r["curve_seed"]]
            ent["instance_identical"] = r == c
        elif r["kind"] == "run":
            k = (r["curve_seed"], r["target_seed"], r["seed"], r["arm"])
            c = cr[k]
            ent["runs"] += 1
            extra = set(r) - set(c)
            ent["extra_fields_in_replay"] |= extra
            keys = (set(r) | set(c)) - TIMING - (extra if j["kind"] == "blpt" else set())
            d = {x: (c.get(x, "<absent>"), r.get(x, "<absent>")) for x in sorted(keys) if r.get(x, "<absent>") != c.get(x, "<absent>")}
            if d:
                ent["diffs"].append({"key": k, "diff": d})
            else:
                ent["identical"] += 1
        elif r["kind"] == "cell":
            ent["cell"] = {"tried": r["tried"], "admitted": r["admitted"], "rejects": r["rejects"]}
    ent["extra_fields_in_replay"] = sorted(ent["extra_fields_in_replay"])
    ent["status"] = "identical" if ent["runs"] == ent["identical"] == 2 * (2 if block.startswith("tune5") else 1) and ent["instance_identical"] else "DIFFERS"
    tot += ent["runs"]
    same += ent["identical"]
    res.append(ent)
summary = {"run_rows_compared": tot, "run_rows_identical": same, "jobs": len(res),
           "jobs_identical": sum(e["status"] == "identical" for e in res),
           "by_binary": {b: sum(e.get("identical", 0) for e in res if e.get("binary") == b) for b in ("po", "blpt")}}
json.dump({"summary": summary, "jobs": res}, open(os.path.join(OUT, "replay_compare.json"), "w"), indent=1, default=list)
print(json.dumps(summary, indent=1))
for e in res:
    print(e["tag"], e["status"], e.get("identical"), "/", e.get("runs"), "inst", e.get("instance_identical"), e.get("extra_fields_in_replay"), e.get("diffs", [])[:1])
