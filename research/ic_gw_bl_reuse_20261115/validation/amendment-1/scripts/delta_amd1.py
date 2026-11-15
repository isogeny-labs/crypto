#!/usr/bin/env python3
"""BLR1 (amendment 1) vs the frozen BLR records, per (q, seed): counted fields that differ, DL linear_algebra deltas,
s.bl_narrowed, and the change of the DL ratio. usage: delta_amd1.py BLR_RUNS_DIR BLR1_RUNS_DIR RECOMPUTE_BLR RECOMPUTE_BLR1"""
import json, os, sys
a_dir, b_dir, ra, rb = sys.argv[1:5]
QS = [262111, 524287, 1048507, 2097091, 4194271]
ck = lambda r: [k for k in r if not (k.endswith("wall") or k.endswith("cpu")) and k != "load1" and not k.startswith("s.controls")]
ld = lambda d, q, s: [json.loads(l) for l in open(os.path.join(d, "units", f"BLR_{q}_{s}.jsonl")) if '"rec":"ic"' in l]
RA, RB = json.load(open(ra)), json.load(open(rb))
cells, other = [], 0
for q in QS:
    for s in range(1, 7):
        A, B = ld(a_dir, q, s), ld(b_dir, q, s)
        assert [x["arm"] for x in A] == [x["arm"] for x in B]
        diff = [f"{x['arm']}/{k}" for x, z in zip(A, B) for k in ck(x) if x[k] != z[k]]
        other += sum(1 for d in diff if d not in ("DL/linear_algebra.lmul", "DL/linear_algebra.ladd"))
        dl_a = [x for x in A if x["arm"] == "DL"][0]; dl_b = [x for x in B if x["arm"] == "DL"][0]
        cells.append({"q": q, "seed": s, "bl_narrowed": dl_b["s.bl_narrowed"], "fields_differing": diff,
                      "d_lmul": dl_b["linear_algebra.lmul"] - dl_a["linear_algebra.lmul"],
                      "d_ladd": dl_b["linear_algebra.ladd"] - dl_a["linear_algebra.ladd"],
                      "d_lmul_rel": (dl_b["linear_algebra.lmul"] - dl_a["linear_algebra.lmul"]) / dl_a["linear_algebra.lmul"]})
maxchg = max(abs(RB["ratios"][k]["per_seed"][i] - RA["ratios"][k]["per_seed"][i]) for k in RA["ratios"] for i in range(6))
narrowed = [c for c in cells if c["bl_narrowed"] > 0]; plain = [c for c in cells if c["bl_narrowed"] == 0]
out = {"cells": cells, "other_field_differences": other,
       "narrowed_cells": len(narrowed), "narrowed_cells_differing": sum(1 for c in narrowed if c["fields_differing"]),
       "unnarrowed_cells": len(plain), "unnarrowed_cells_differing": sum(1 for c in plain if c["fields_differing"]),
       "d_lmul_range_narrowed": [min(c["d_lmul"] for c in narrowed), max(c["d_lmul"] for c in narrowed)],
       "d_lmul_rel_max": max(c["d_lmul_rel"] for c in cells),
       "max_abs_change_any_ratio_cell": maxchg}
json.dump(out, sys.stdout, indent=1); print()
