"""Block positions of the arms of a wall session (XP-20261029-packed-filter PROTOCOL.md section 10: the README's position table).
A block is the set of records that share (workload_id, round); its position order is the order of their seq. Per arm, over the measured (not warm-up)
records: how many sit at each position, and how often each other arm is its immediate predecessor in the block (and how often it is first). Warm-up
records are tabulated the same way, separately. It reads only seq, arm, round, warmup and workload.workload_id: no wall field. It decides nothing.

usage: positions.py EKS_DIR ANALYSIS_DIR
Writes ANALYSIS_DIR/positions.json and prints it as lines. Exits 1 with a line beginning STOP: only when records.jsonl is absent or unreadable."""
import collections, json, os, sys

eks, an = sys.argv[1:3]
try:
    recs = [json.loads(l) for l in open(os.path.join(eks, "session", "records.jsonl")) if l.strip()]
except Exception as e:
    print("STOP: cannot read records.jsonl: " + type(e).__name__)
    sys.exit(1)
blocks = collections.defaultdict(list)
for r in recs:
    blocks[((r.get("workload") or {}).get("workload_id"), r.get("round"), bool(r.get("warmup")))].append(r)
out = {}
sizes = collections.Counter()
for scope in ("measured", "warmup"):
    pos = collections.defaultdict(collections.Counter)
    pred = collections.defaultdict(collections.Counter)
    for (wid, rd, wu), rs in blocks.items():
        if (scope == "warmup") != wu:
            continue
        rs = sorted(rs, key=lambda r: r.get("seq"))
        sizes[(scope, len(rs))] += 1
        for i, r in enumerate(rs):
            pos[r.get("arm")][i] += 1
            pred[r.get("arm")]["first" if i == 0 else rs[i - 1].get("arm")] += 1
    out[scope] = {
        "position_counts": {a: {str(i): n for i, n in sorted(c.items())} for a, c in sorted(pos.items())},
        "predecessor_counts": {a: dict(sorted(c.items())) for a, c in sorted(pred.items())},
    }
out["block_size_counts"] = {"%s:%d" % k: n for k, n in sorted(sizes.items())}
json.dump(out, open(os.path.join(an, "positions.json"), "w"), indent=1, sort_keys=True)
print("POSITIONS_BLOCK_SIZES " + json.dumps(out["block_size_counts"], sort_keys=True))
for scope in ("measured", "warmup"):
    for a, c in out[scope]["position_counts"].items():
        print("POSITIONS_%s %s %s" % (scope.upper(), a, json.dumps(c, sort_keys=True)))
    for a, c in out[scope]["predecessor_counts"].items():
        print("PREDECESSORS_%s %s %s" % (scope.upper(), a, json.dumps(c, sort_keys=True)))
print("POSITIONS_OK")
