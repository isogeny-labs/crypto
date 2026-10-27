#!/usr/bin/env python3
"""Derived validation numbers from the hky_window outputs (joined with compare.py's refs):
- per-row recall of W(k) without exact passes: a `wKx` run that certified with
  `exact_passes == 0` is exactly a `wK` per-row certification (the passes
  only start once the per-row rounds stall);
- per cell, median w1x seconds and basis bytes against the dense record's secs.
Usage: derived.py e11/validate/*.jsonl
"""
import json
import statistics
import sys
from collections import defaultdict

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from compare import SELF, build_refs, lines  # noqa: E402

ref = build_refs()
per = defaultdict(lambda: defaultdict(list))
dense_secs = defaultdict(list)  # cell -> dense degree-4 closure secs (records that store them)
for rf in SELF:
    for r in lines(rf):
        s4 = r.get("per_degree", {}).get("4", {})
        if r.get("d_f") == 4 and "secs" in s4:
            dense_secs[(r["m"], r["n"], r["ell"], r["n_vars"])].append(s4["secs"])
for p in sys.argv[1:]:
    for r in lines(p):
        key = (r["m"], tuple(r["v_basis"]), r["x_r"])
        if key not in ref or ref[key]["d_f"] != 4:
            continue
        cell = (r["m"], r["n"], r["ell"], r["n_vars"])
        for pol in ("w1x", "w2x"):
            w = r["window"].get(pol)
            if not w:
                continue
            per[cell][pol + "_perrow"].append(bool(w["certified"]) and w["exact_passes"] == 0)
            per[cell][pol + "_secs"].append(w["secs"])
            per[cell][pol + "_bytes"].append(w["basis_bytes"])
            per[cell][pol + "_ones"].append(w["basis_ones"])
            per[cell][pol + "_cols"].append(w["cols"])
            per[cell][pol + "_dim"].append(w["dim"])
tot = defaultdict(lambda: [0, 0])
print("m:n:ell N | w1 per-row | w1x secs median | dense deg-4 secs median | w1x basis MB median | density | w2 per-row")
for cell in sorted(per):
    d = per[cell]
    a = d["w1x_perrow"]
    b = d.get("w2x_perrow", [])
    tot["w1"][0] += sum(a)
    tot["w1"][1] += len(a)
    tot["w2"][0] += sum(b)
    tot["w2"][1] += len(b)
    dens = statistics.median(o / (8 * by) for o, by in zip(d["w1x_ones"], d["w1x_bytes"]))
    print(f"{cell[0]}:{cell[1]}:{cell[2]} {cell[3]} | {sum(a)}/{len(a)} | {statistics.median(d['w1x_secs']):.1f} | "
          f"{statistics.median(dense_secs[cell]) if dense_secs[cell] else float('nan'):.1f} | "
          f"{statistics.median(d['w1x_bytes']) / 1e6:.1f} | {dens:.3f} | {sum(b)}/{len(b)}")
for k, (x, y) in tot.items():
    print(f"PER-ROW RECALL {k}: {x}/{y} = {x / y:.3f}")
