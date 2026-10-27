#!/usr/bin/env python3
"""Check that the final binary reproduces the validation binary's w1x results
draw for draw (certified, dim, rounds, exact passes, products, degree-2/3 dims).
Usage: diff_rerun.py e11/validate e11/validate_final"""
import glob
import json
import os
import sys

a_dir, b_dir = sys.argv[1], sys.argv[2]
same = diff = 0
for bp in sorted(glob.glob(f"{b_dir}/*.jsonl")):
    ap = os.path.join(a_dir, os.path.basename(bp))
    A = {json.loads(x)["draw"]: json.loads(x) for x in open(ap) if x.startswith("{")}
    for x in open(bp):
        if not x.startswith("{"):
            continue
        b = json.loads(x)
        a = A[b["draw"]]

        def key(r):
            w = r["window"].get("w1x", {})
            return ({k: v["dim"] for k, v in r["per_degree"].items()}, r["d_f_le3"],
                    [w.get(k) for k in ("certified", "dim", "rounds", "exact_passes", "exact_products",
                                        "gens_admitted", "muls_tried")])
        if key(a) == key(b):
            same += 1
        else:
            diff += 1
            print("DIFF", bp, b["draw"], key(a), key(b))
print(f"SAME {same} DIFF {diff}")
