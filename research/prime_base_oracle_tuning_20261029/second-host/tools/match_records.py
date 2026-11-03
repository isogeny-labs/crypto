# Identity check only, no result computed (ratios come from `ecbench compare`).
# Pairs each original record with the second-host record of the same
# (arm, workload, round, algorithm_seed) and asks whether every deterministic field
# agrees: everything S and the §5a band are built from. Excluded and listed in the
# output: identifiers, wall/OS timing, host and isolation capsules, binary identity,
# and the bitop/v1 `work` / phase `work_native` fields, which the second host's older
# binary (commit cb2ec687) does not emit (the band reads `counters` and phase `native`).
# Usage: python3 -I match_records.py ORIGINAL/records.jsonl SECOND/records.jsonl
import json, sys
TOP = ("arm", "role", "round", "warmup", "method", "workload", "algorithm_seed", "outcome", "cost",
       "boundaries", "counters", "factor_base", "detail", "solver", "field_ops")
PHASE = ("name", "adds", "doubles", "scalar_mults", "gae", "native")
def load(p):
    out = {}
    for ln in open(p):
        r = json.loads(ln)
        if r.get("schema") == "ecbench.record/v1":
            out[(r["arm"], r["workload"]["workload_id"], r["round"], r["algorithm_seed"])] = r
    return out
def det(r):
    d = {k: r.get(k) for k in TOP}
    d["phases"] = [{k: p.get(k) for k in PHASE} for p in r["phases"]]
    return d
a, b = load(sys.argv[1]), load(sys.argv[2])
same = diff = missing = 0
first = None
for k, ra in a.items():
    rb = b.get(k)
    if rb is None:
        missing += 1
    elif det(ra) == det(rb):
        same += 1
    else:
        diff += 1
        first = first or [list(k), det(ra), det(rb)]
print(json.dumps({"schema": "pbo-second-host-match/v1", "original_records": len(a),
                  "second_host_records": len(b), "matched_identical": same, "matched_different": diff,
                  "original_without_partner": missing,
                  "second_host_measured_records_without_original": sum(1 for k in b if k not in a),
                  "compared_top_fields": list(TOP), "compared_phase_fields": list(PHASE),
                  "first_difference": first}, indent=1))
