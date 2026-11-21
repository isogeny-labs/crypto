#!/usr/bin/env python3
"""Field-by-field diff of replayed genus_ladder records against the committed
JSONL.  Independent of the producer's scripts.

usage: replay_diff.py REPLAY_DIR COMMITTED_JSONL [COMMITTED_JSONL ...]

Every `run` and `instance` record in REPLAY_DIR/*.jsonl is matched to the
committed record with the same key ((g, p, curve_seed, seed, arm) for runs,
(g, p, curve_seed) for instances).  All fields are compared except `wall_ns`
(wall-clock, not deterministic).  Missing keys on either side are mismatches.
"""
import glob
import json
import os
import sys

SKIP = {"wall_ns"}


def load(paths):
    runs, inst = {}, {}
    for fn in paths:
        with open(fn) as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                d = json.loads(line)
                if d["kind"] == "run":
                    k = (d["g"], d["p"], d["curve_seed"], d["seed"], d["arm"])
                    if k in runs:
                        raise SystemExit(f"duplicate run key {k} in {fn}")
                    runs[k] = (fn, d)
                elif d["kind"] == "instance":
                    k = (d["g"], d["p"], d["curve_seed"])
                    inst.setdefault(k, (fn, d))
    return runs, inst


def main():
    rdir, committed = sys.argv[1], sys.argv[2:]
    rr, ri = load(sorted(glob.glob(os.path.join(rdir, "*.jsonl"))))
    cr, ci = load(committed)
    n_rec = n_fields = 0
    bad = []
    for kind, R, Cm in (("instance", ri, ci), ("run", rr, cr)):
        for k, (fn, d) in sorted(R.items()):
            if k not in Cm:
                bad.append((kind, k, "no committed record"))
                continue
            c = Cm[k][1]
            n_rec += 1
            keys = (set(d) | set(c)) - SKIP
            for f in sorted(keys):
                n_fields += 1
                if f not in d or f not in c:
                    bad.append((kind, k, f"field {f} missing on one side"))
                elif d[f] != c[f]:
                    bad.append((kind, k, f"{f}: replay={d[f]} committed={c[f]}"))
    print(f"records compared: {n_rec}  fields compared: {n_fields}  (wall_ns excluded)")
    runs_by_cell = {}
    for k in rr:
        runs_by_cell.setdefault(k[:3], []).append(k[4])
    for k in sorted(runs_by_cell):
        print(f"  g={k[0]} p={k[1]} curve={k[2]}: {len(runs_by_cell[k])} run records")
    for b in bad:
        print("MISMATCH", b)
    print("REPLAY_DIFF", "PASS" if not bad and n_rec else "FAIL")
    return 1 if bad or not n_rec else 0


if __name__ == "__main__":
    sys.exit(main())
