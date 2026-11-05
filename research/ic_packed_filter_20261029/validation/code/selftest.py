"""Negative controls on the validator's own checkers (TASK-20261029-654f73).
A checker that reports 0 mismatches is only evidence if it can report one:
each test perturbs a copy of real records and requires the checker to flag it.

usage: selftest.py OUT.json
"""
import copy
import sys

import j3_work as j3
import scalars
import vcommon as v


def main():
    recs = v.load_wall(32)
    arms = j3.all_by_key(recs)
    k = sorted(arms["dgbf"])[10]
    base, r = arms["dgb"][k], arms["dgbf"][k]
    out = {}
    out["identical_pair_no_diff"] = j3.diff(j3.cond8_fields(base, True), j3.cond8_fields(r, True)) == []
    # 1. native count off by one
    r2 = copy.deepcopy(r)
    r2["phases"][2]["native"]["lookups"] += 1
    out["native_plus_one_detected"] = bool(j3.diff(j3.cond8_fields(base, True), j3.cond8_fields(r2, True)))
    # 2. work_native counter added
    r3 = copy.deepcopy(r)
    r3["phases"][1]["work_native"]["extra"] = 1
    out["work_native_extra_key_detected"] = bool(j3.diff(j3.cond8_fields(base, True), j3.cond8_fields(r3, True)))
    # 3. recovered log changed
    r4 = copy.deepcopy(r)
    r4["outcome"]["recovered"] = str(int(r4["outcome"]["recovered"]) + 1)
    out["recovered_changed_detected"] = bool(j3.diff(j3.cond8_fields(base, True), j3.cond8_fields(r4, True)))
    # 4. mismatched pairing (another target) must differ
    k2 = (k[0], (k[1] + 1) % 24, k[2])
    out["wrong_pairing_detected"] = bool(j3.diff(j3.cond8_fields(arms["dgb"][k2], True), j3.cond8_fields(r, True)))
    # 5. re-pricing catches a one-bitop change
    r5 = copy.deepcopy(r)
    r5["work"]["phases"][1]["bitops"] = str(int(r5["work"]["phases"][1]["bitops"]) + 1)
    out["reprice_detects_one_bitop"] = bool(j3.reprice(r5))
    out["reprice_clean_record_ok"] = j3.reprice(r) == []
    # 6. scalar check rejects a wrong log
    sess = scalars.check_session([r4] + [x for x in recs if x["workload"]["curve"]["slug"] == k[0]][:20])
    out["scalar_check_rejects_wrong_log"] = len(sess["failures"]) == 1
    # 7. tfbs on the unfiltered arm would be seen
    r6 = copy.deepcopy(base)
    r6["phases"][1]["work_native"]["table_filter_bit_sets"] = 5
    out["tfbs_on_dgb_seen"] = v.tfbs(r6)[1] == ["oracle_setup"]
    # 8. the counted view removes every wall/timing field
    cv = v.counted_view(r)
    def has_timing(o):
        if isinstance(o, dict):
            return any(v._is_timing_key(kk) or has_timing(vv) for kk, vv in o.items() if kk != "isolation")
        if isinstance(o, list):
            return any(has_timing(x) for x in o)
        return False
    out["counted_view_has_no_timing_field"] = not has_timing(cv) and set(cv["isolation"]) == {"level", "blockers"}
    out["counted_view_masks_too_short"] = v._mask_blockers(["L2: solve too short to admit wall time: solve 1.0 µs"]) == [v.TOO_SHORT]
    out["all_pass"] = all(out.values())
    v.dump(out, sys.argv[1])
    print(out)


if __name__ == "__main__":
    main()
