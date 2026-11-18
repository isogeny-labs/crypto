"""Compare two result files row by row on (cell, m, n, k, seed, target, D): solved, rank (canonical when the closure was
run to a fixed point, i.e. not early_one), target_rank, deficit. Usage: compare.py A.jsonl B.jsonl [label]"""
import json, sys
def load(p):
    d = {}
    for l in open(p):
        r = json.loads(l)
        if "solved" in r:
            d[(r["cell"], r["m"], r["n"], r["k"], r["seed"], r["target"], r["D"])] = r
    return d
a, b = load(sys.argv[1]), load(sys.argv[2])
bad = 0; n = 0; skipped = 0
for key, ra in sorted(a.items()):
    rb = b.get(key)
    if rb is None:
        skipped += 1; continue
    n += 1
    same_solved = ra["solved"] == rb["solved"]
    same_t = ra["target_rank"] == rb["target_rank"] and ra["cols"] == rb["cols"]
    if rb.get("early_one"):
        same_rank = ra["solved"] and ra["rank"] == ra["cols"]   # 1 in W: rank = cols by closure
    else:
        same_rank = ra["rank"] == rb["rank"]
    if not (same_solved and same_t and same_rank):
        bad += 1; print("MISMATCH", key, ra["solved"], rb["solved"], ra["rank"], rb["rank"], ra["target_rank"], rb["target_rank"])
print(f"{sys.argv[3] if len(sys.argv) > 3 else ''} compared {n} (cell,target,D) rows, mismatches {bad}, rows only in first file {skipped}")
