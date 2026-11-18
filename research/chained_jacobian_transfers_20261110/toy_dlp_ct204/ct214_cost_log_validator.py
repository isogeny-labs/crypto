"""CT214 fail-closed accounting of untrusted paired toy BSGS logs.

Not a trusted execution proof, hardware calibration, PCC signature or ECDLP gain.
"""
from __future__ import annotations
import json
import math
import sys
from pathlib import Path

PAIR = "CT214_PAIR_JSON "
AGGREGATE = "CT213_TOY_COST_JSON "
N = 43
ROUNDS = 7

class InvalidCostLog(ValueError):
    pass

def check(ok, why):
    if not ok:
        raise InvalidCostLog(why)

def decode(line, prefix):
    if prefix not in line:
        return None
    try:
        obj = json.loads(line.split(prefix, 1)[1].strip())
    except (ValueError, json.JSONDecodeError) as exc:
        raise InvalidCostLog("malformed " + prefix.strip()) from exc
    check(isinstance(obj, dict), "JSON record not an object")
    return obj

def num(rec, field):
    val = rec.get(field)
    check(type(val) is int, "missing/non-integer " + field)
    return val

def validate(text):
    rows, sums = [], []
    for line in text.splitlines():
        row = decode(line, PAIR)
        if row is not None:
            rows.append(row)
        total = decode(line, AGGREGATE)
        if total is not None:
            sums.append(total)
    check(len(rows) == ROUNDS*N and len(sums) == 1, "missing/extra rows or aggregate")
    seen, e_times, j_times, e_add, j_add = set(), [], [], 0, 0
    orders = {"EJ": 0, "JE": 0}
    for row in rows:
        rnd, offset, k = num(row, "round"), num(row, "offset"), num(row, "target_index")
        check(0 <= rnd < ROUNDS and 0 <= offset < N, "round/offset outside roster")
        check((rnd, offset) not in seen, "duplicate pair key")
        seen.add((rnd, offset))
        check(k == (offset + rnd*17) % N, "target roster mismatch")
        ab = "EJ" if (k+rnd)%2 == 0 else "JE"
        check(row.get("arm_order") == ab, "AB/BA schedule mismatch")
        orders[ab] += 1
        ea, ja = num(row, "direct_additions"), num(row, "jacobian_additions")
        check(ea == ja == 13 + k//7, "wrong BSGS addition count")
        check(num(row, "direct_baby_entries") == num(row, "jacobian_baby_entries") == 7,
              "wrong BSGS baby count")
        et, jt = num(row, "direct_ns"), num(row, "jacobian_ns")
        check(et > 0 and jt > 0, "nonpositive elapsed duration")
        e_times.append(et)
        j_times.append(jt)
        e_add += ea
        j_add += ja
    agg = sums[0]
    check(num(agg,"field") == 31 and num(agg,"order") == N, "wrong curve")
    check(num(agg,"rounds") == ROUNDS and num(agg,"pairs") == ROUNDS*N,
          "aggregate roster mismatch")
    e_total, j_total = sum(e_times), sum(j_times)
    check(num(agg,"direct_total_ns") == e_total and num(agg,"jacobian_total_ns") == j_total,
          "aggregate sum mismatch")
    check(num(agg,"direct_median_ns") == sorted(e_times)[150] and
          num(agg,"jacobian_median_ns") == sorted(j_times)[150], "median mismatch")
    check(num(agg,"direct_bsgs_additions") == num(agg,"jacobian_bsgs_additions") == e_add == j_add == 4690,
          "aggregate additions mismatch")
    ratio = e_total / j_total
    observed = agg.get("ratio_direct_over_jacobian")
    check(type(observed) in (float, int) and math.isfinite(observed) and
          abs(observed-ratio) <= 1e-8 * max(1., ratio), "ratio mismatch")
    check(agg.get("memory") == "unmeasured" and agg.get("pcc") == "unverified"
          and agg.get("status") == "toy_only_not_full_reference",
          "unsupported validation claim")
    return {"schema":"CT214-raw-log-audit-v1","valid_pairs":len(rows),
            "distinct_targets":N,"arm_order_counts":orders,
            "direct_total_ns":e_total,"jacobian_total_ns":j_total,
            "ratio_direct_over_jacobian":ratio,
            "direct_additions":e_add,"jacobian_additions":j_add,
            "standing":"producer_log_accounting_only_pcc_unverified"}

def main():
    if len(sys.argv) != 2:
        print("usage: python3 ct214_cost_log_validator.py <rust-log>", file=sys.stderr)
        return 2
    try:
        receipt = validate(Path(sys.argv[1]).read_text(encoding="utf-8"))
    except (InvalidCostLog, OSError) as exc:
        print("CT214 validation FAILED: " + str(exc), file=sys.stderr)
        return 1
    print("CT214_VALIDATOR_JSON " + json.dumps(receipt, sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
