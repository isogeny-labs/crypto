#!/usr/bin/env python3
"""AGENTS.md §5a calibrated reading (owner 2026-11-12): order of each IC arm
and the matched rho at w = 0, w_cal and 10 w_cal, from the per-p informational
calibration recorded in each run file (best-of-N timing on the shared VM, not
perf-ipc). IC: group + k*(w_rel*rel_mul + w_la*la_mul); rho: group + k*w_rel*native.
Checks total_cal (k = 1) against the recomputation.
Usage: calibrated_reading.py REPO_ROOT OUT_JSON"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

ROOT = sys.argv[1]
studies = {
    "lucas": ["research/ic_positive_control_lucas_20261116/run.json"],
    "la": ["research/ic_positive_control_la_20261116/run.json"],
    "unit_t": ["research/ic_positive_control_unit_t_20261116/run.json"],
    "capped": ["research/ic_positive_control_capped_20261116/run_w.json", "research/ic_positive_control_unit_t_20261116/run.json"],
}
out = {}
for st, files in studies.items():
    rows, cal = [], {}
    for i, f in enumerate(files):
        d, rr = V.load(os.path.join(ROOT, f))
        if st == "capped" and i == 1:
            rr = [r for r in rr if r["arm"] == "R_rho_negation"]
            # W rows use their own run's calibration; rho rows the unit-T run's
            cal_r = d["calibration_ns_informational"]
            for r in rr:
                r["_cal"] = cal_r[str(r["p"])]
        else:
            cal_r = d["calibration_ns_informational"]
            for r in rr:
                r["_cal"] = cal_r[str(r["p"])]
        rows += rr
    by = V.index(rows)
    bad_cal = 0
    def tc(r, k):
        c = r["_cal"]
        if r["kind"] == "rho":
            return r["group_ops"] + k * c["w_rel"] * r["native_mul"]
        return r["group_ops"] + k * (c["w_rel"] * r["rel_mul"] + c["w_la"] * r["la_mul"])
    for r in rows:
        if abs(tc(r, 1) - r["total_cal"]) > 1e-6 * r["total_cal"]:
            bad_cal += 1
    res = {"total_cal_mismatch_rows": bad_cal, "per_p": {}}
    ps = sorted({r["p"] for r in rows})
    for p in ps:
        e = {}
        cR = V.cell(by, "R_rho_negation", p)
        for a in sorted(by):
            if a == "R_rho_negation":
                continue
            cA = V.cell(by, a, p)
            if not cA:
                continue
            rat = [V.ratio_of_means(cA, cR, lambda r, k=k: tc(r, k)) for k in (0, 1, 10)]
            e[a] = {"w0": rat[0], "w_cal": rat[1], "10w_cal": rat[2],
                    "ranked_below_rho_at_calibrated_price": all(x < 1 for x in rat),
                    "w_rel": cA[0]["_cal"]["w_rel"], "w_la": cA[0]["_cal"]["w_la"]}
        res["per_p"][p] = e
    out[st] = res
json.dump(out, open(sys.argv[2], "w"), indent=1)
for st, res in out.items():
    print("==", st, "total_cal mismatches:", res["total_cal_mismatch_rows"])
    for p, e in res["per_p"].items():
        if p < 4099:
            continue
        print("  ", p, "; ".join(f"{a}: {v['w0']:.3f}/{v['w_cal']:.3f}/{v['10w_cal']:.3f}{' RANKED<rho' if v['ranked_below_rho_at_calibrated_price'] else ''}" for a, v in e.items()))
