#!/usr/bin/env python3
"""Compare a validator replay JSONL with the committed run.jsonl.
Every replayed run row must have a committed partner (same arm, p, curve_seed,
target_seed, seed) equal in every field except wall_ns; instance rows must be
equal in every field; cell rows in every field except search_wall_s.  Keys are
read with [] (lesson L-99ee952350b2); a field present on one side only is a
mismatch.  Usage: compare_runs.py REPLAY.jsonl COMMITTED.jsonl"""
import json, sys

rep = [json.loads(l) for l in open(sys.argv[1])]
com = [json.loads(l) for l in open(sys.argv[2])]


def key(r):
    ts = r["target_seed"] if "target_seed" in r else r["curve_seed"]
    return (r["arm"], r["p"], r["curve_seed"], ts, r["seed"])


cruns = {}
for r in com:
    if r["kind"] in ("run", "skip"):
        k = key(r)
        assert k not in cruns, k
        cruns[k] = r
cinst = {(r["p"], r["curve_seed"]): r for r in com if r["kind"] == "instance"}
ccell = {(r["p"], r["g"]): r for r in com if r["kind"] == "cell"}

n_run = n_inst = n_cell = 0
bad = []
for r in rep:
    if r["kind"] in ("run", "skip"):
        k = key(r)
        if k not in cruns:
            bad.append(("no committed partner", k)); continue
        c = cruns[k]
        fields = (set(r) | set(c)) - {"wall_ns"}
        for f in sorted(fields):
            if f not in r or f not in c or r[f] != c[f]:
                # study-1 rows carry no target_seed; the replay of the bce0eb2148 binary does not either
                bad.append(("run field", k, f, r.get(f, "<absent>"), c.get(f, "<absent>")))
        n_run += 1
    elif r["kind"] == "instance":
        c = cinst[(r["p"], r["curve_seed"])]
        if set(r) != set(c) or any(r[f] != c[f] for f in r):
            bad.append(("instance", (r["p"], r["curve_seed"])))
        n_inst += 1
    elif r["kind"] == "cell":
        c = ccell[(r["p"], r["g"])]
        fields = (set(r) | set(c)) - {"search_wall_s"}
        if any(f not in r or f not in c or r[f] != c[f] for f in fields):
            bad.append(("cell", (r["p"], r["g"]), r, c))
        n_cell += 1
print(f"replay {sys.argv[1]}: run rows {n_run}, instance rows {n_inst}, cell rows {n_cell}; "
      f"compared fields per run row = all except wall_ns")
by_arm = {}
for r in rep:
    if r["kind"] == "run":
        by_arm[(r["arm"], r["p"])] = by_arm.get((r["arm"], r["p"]), 0) + 1
print("replayed run rows by (arm, p):", dict(sorted(by_arm.items())))
print("mismatches:", len(bad))
for b in bad[:30]:
    print("  ", b)
print("RESULT:", "IDENTICAL" if not bad and n_run > 0 else "DIFFERENT")
