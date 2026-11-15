#!/usr/bin/env python3
"""Compare the validator's replays (l3_l1_replay.sh) with the committed L1/L3 cells: every per-target counted field
(timing excluded) and every top-level structural field must be identical.
Usage: l3_l1_replay_compare.py <study_dir> <replay_dir> <out.json>"""
import glob, json, os, sys
S, RD, OUT = sys.argv[1:4]
PT = ("target_x", "field_ops", "solving_degree_max", "solving_degree", "degree_reached", "steps", "max_rows", "max_cols",
      "inconsistent", "basis_len", "complete", "timed_out")
TOP = ("k", "p", "base", "d_or_t", "gidx", "frame", "m", "form", "levels", "v_size", "n_vars", "sym_terms", "input_terms",
       "input_degree", "targets_requested", "targets_done", "seed", "cap_s", "capped", "mean_field_ops", "a_mont")
rows = []
for rf in sorted(glob.glob(os.path.join(RD, "*.json"))):
    tag = os.path.basename(rf)[:-5]
    lead = "L1_faithful" if tag.startswith("p16777141") else "L3_flat"
    cf = os.path.join(S, lead, "cells", tag + ".json")
    r = json.loads(open(rf).read()); c = json.load(open(cf))
    top = {k: (r.get(k) == c.get(k)) for k in TOP}
    pt = [tuple(t[k] for k in PT) for t in r["per_target"]] == [tuple(t[k] for k in PT) for t in c["per_target"]]
    rows.append({"cell": f"{lead}/cells/{tag}.json", "top_level_equal": all(top.values()), "top_level_diff": [k for k, v in top.items() if not v],
                 "per_target_equal": pt, "field_ops": [t["field_ops"] for t in r["per_target"]], "mean_field_ops": r["mean_field_ops"],
                 "replay_solve_s": round(sum(t["ms"] for t in r["per_target"]) / 1000, 3),
                 "recorded_solve_s": round(sum(t["ms"] for t in c["per_target"]) / 1000, 3)})
res = {"replayed": len(rows), "all_identical": all(x["top_level_equal"] and x["per_target_equal"] for x in rows), "rows": rows}
json.dump(res, open(OUT, "w"), indent=1)
for x in rows: print(x["cell"], x["top_level_equal"], x["per_target_equal"], x["top_level_diff"], f'{x["mean_field_ops"]:.6g}', x["replay_solve_s"], x["recorded_solve_s"])
print("replayed", res["replayed"], "all_identical", res["all_identical"])
