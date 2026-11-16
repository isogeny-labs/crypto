#!/usr/bin/env python3
"""Compare a replayed full_jacobian.json with the committed one (Validator V, 2026-11-14).

Every value is compared exactly except the timing-derived ones (class times,
addition times, calibrated totals and ratios, wall times, the args string and
the host), which are listed separately: their calibrated order classes are
reported, not required equal.
Usage: compare_replay.py COMMITTED REPLAY OUT
"""
import json, sys

a, b, out = json.load(open(sys.argv[1])), json.load(open(sys.argv[2])), sys.argv[3]
TIMING = {"class_times_ns", "t_add_ns", "total_cal", "ratio_neg_cal", "wall_s", "arm_wall_s", "cal_negation", "args", "host"}
CALCLS = {"class_neg_cal", "class_best_cal", "ranked_at_calibrated_price"}
diffs, cal_diffs, n, ncal = [], [], 0, 0


def walk(x, y, path):
    global n, ncal
    if isinstance(x, dict) and isinstance(y, dict):
        for k in sorted(set(x) | set(y)):
            if k in TIMING:
                continue
            if k not in x or k not in y:
                diffs.append({"path": path + "/" + k, "committed": x.get(k, "<absent>"), "replay": y.get(k, "<absent>"), "kind": "key"})
                continue
            if k in CALCLS:
                ncal += 1
                if x[k] != y[k]:
                    cal_diffs.append({"path": path + "/" + k, "committed": x[k], "replay": y[k]})
                continue
            walk(x[k], y[k], path + "/" + k)
    elif isinstance(x, list) and isinstance(y, list):
        if len(x) != len(y):
            diffs.append({"path": path, "committed_len": len(x), "replay_len": len(y)}); return
        for i, (u, v) in enumerate(zip(x, y)):
            walk(u, v, f"{path}[{i}]")
    else:
        n += 1
        if x != y:
            diffs.append({"path": path, "committed": x, "replay": y})


walk(a, b, "")
res = {"compared_values": n, "differences": diffs, "calibrated_class_values": ncal, "calibrated_class_differences": cal_diffs}
json.dump(res, open(out, "w"), indent=1)
print(f"compared {n} non-timing values: {len(diffs)} differences; calibrated classes {ncal}: {len(cal_diffs)} differ")
for d in diffs[:20]: print("DIFF", d)
for d in cal_diffs[:20]: print("CALDIFF", d)
