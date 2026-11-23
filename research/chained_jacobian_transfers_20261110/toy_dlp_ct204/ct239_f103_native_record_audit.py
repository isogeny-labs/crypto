#!/usr/bin/env python3
"""CT239: independent arithmetic / bookkeeping audit of frozen CT238 native rows.

This checks the recorded three cold Q73-only F103 observations; it does NOT
replay native Cantor relations or timings, certify memory, or yield signed PCC.
[pcc:unverified]
"""
from __future__ import annotations

import copy
import json
import math
import statistics
from pathlib import Path

P, R = 103, 109
G, Q = (1, 51), (42, 71)
PANEL = Path(__file__).with_name("ct239_f103_native_three_pairs.jsonl")
PHASES = ("ic_setup_ns", "ic_relation_ns", "ic_la_ns", "ic_recovery_ns", "ic_verify_ns")
COUNTERS = (
    "ic_candidates", "ic_smooth", "ic_rank", "offsets",
    "cantor_adds", "target_adds", "direct_bsgs_adds",
    "field_add", "field_mul", "field_sqr", "field_inv",
)
NUMERIC = ("round", "candidate_ns", "direct_bsgs_ns") + PHASES + COUNTERS

def check(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)

def ec_add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    u, v = b
    if x == u and (y + v) % P == 0:
        return None
    if x == u:
        check(y != 0, "bad doubled zero point")
        lam = (3*x*x + 1)*pow(2*y, -1, P) % P
    else:
        lam = (v-y)*pow((u-x) % P, -1, P) % P
    z = (lam*lam-x-u) % P
    return (z, (lam*(x-z)-y) % P)

def ec_mul(k, point):
    a = None
    while k:
        if k & 1:
            a = ec_add(a, point)
        point = ec_add(point, point)
        k >>= 1
    return a

def field_checks() -> None:
    check(all((y*y - x*x*x - x - 24) % P == 0 for x,y in (G,Q)),
          "E(F103) points not on curve")
    check(1 + sum((y*y-x*x*x-x-24)%P == 0
                  for x in range(P) for y in range(P)) == R,
          "source elliptic order wrong")
    check(ec_mul(R, G) is None and ec_mul(73, G) == Q,
          "independent source target scalar check failed")
    check(all(R % d for d in range(2,math.isqrt(R)+1)), "order not prime")

def audit(rows: list[dict]) -> dict:
    field_checks()
    check(len(rows) == 3, "expected exactly three original observations")
    seen = set()
    for row in rows:
        check(all(k in row for k in NUMERIC), "missing numeric field")
        check(all(type(row[k]) is int for k in NUMERIC), "noninteger field")
        check(all(row[k] >= 0 for k in NUMERIC), "negative native counter")
        check(type(row.get("ratio_direct_over_candidate")) is float,
              "missing/malformed ratio")
        check(row.get("pcc") == "unverified", "PCC falsely promoted")
        check(row.get("source") == "known_answer_F103", "incorrect curve provenance")
        check(row.get("memory") == "unmeasured", "unexpected memory standing")
        rd = row["round"]
        check(rd in (0,1,2) and rd not in seen, "unexpected/duplicate round")
        seen.add(rd)
        check(row.get("arm_order") == ("BSGS_IC" if rd == 1 else "IC_BSGS"),
              "wrong alternating measurement order")
        check(row["ic_candidates"] == 381 and row["ic_smooth"] == 27
              and row["ic_rank"] == 22 and row["offsets"] == 4,
              "frozen solver convergence changed")
        check(row["cantor_adds"] == 385 and row["target_adds"] == 28
              and row["direct_bsgs_adds"] == 17,
              "recorded group-operation count changed")
        check(row["candidate_ns"] == sum(row[k] for k in PHASES),
              "candidate cold total excludes or misstates a phase")
        check(all(row[k] > 0 for k in PHASES + ("direct_bsgs_ns",)),
              "zero timing phase")
        check(row["candidate_ns"] > row["direct_bsgs_ns"],
              "unexpected direction of frozen comparison")
        check(abs(row["ratio_direct_over_candidate"] -
                  row["direct_bsgs_ns"]/row["candidate_ns"]) <= 5e-11,
              "recorded ratio inconsistent with integer timings")
        check(all(row[k] > 0 for k in ("field_add","field_mul","field_inv")),
              "missing primitive counters")
        check(0 < row["ic_relation_ns"] < row["candidate_ns"],
              "invalid relation phase time")
    check(seen == {0,1,2}, "incomplete round coverage")
    check(len({tuple(row[k] for k in COUNTERS) for row in rows}) == 1,
          "nonidentical finite cold solver counters between registered rounds")
    candidates = [r["candidate_ns"] for r in rows]
    baselines = [r["direct_bsgs_ns"] for r in rows]
    ratios = [b/c for b,c in zip(baselines,candidates)]
    check(max(ratios) < 0.001, "unqualified advantage or drastic baseline drift")
    return {
        "rounds": len(rows),
        "median_candidate_ns": int(statistics.median(candidates)),
        "median_bsgs_ns": int(statistics.median(baselines)),
        "median_paired_baseline_over_candidate": statistics.median(ratios),
        "median_paired_candidate_over_baseline":
            statistics.median([1/x for x in ratios]),
        "relation_time_percent": [100*r["ic_relation_ns"]/r["candidate_ns"] for r in rows],
    }

def negative_controls(rows: list[dict]) -> int:
    mutations = [
        lambda z: z.pop(),
        lambda z: z.append(copy.deepcopy(z[0])),
        lambda z: z[1].update(round=0),
        lambda z: z[1].update(arm_order="IC_BSGS"),
        lambda z: z[0].update(source="known_answer_F101"),
        lambda z: z[0].update(pcc="verified"),
        lambda z: z[2].update(memory="measured"),
        lambda z: z[0].update(candidate_ns=z[0]["candidate_ns"]-1),
        lambda z: z[0].update(ic_relation_ns=z[0]["ic_relation_ns"]-1),
        lambda z: z[1].update(ratio_direct_over_candidate=2.0),
        lambda z: z[1].update(ic_rank=21),
        lambda z: z[2].update(ic_candidates=382),
        lambda z: z[2].update(field_mul=0),
        lambda z: z[1].pop("field_add"),
        lambda z: z[1].update(direct_bsgs_ns=-1),
        lambda z: z[0].update(field_inv=z[0]["field_inv"]+1),
    ]
    for i, mutate in enumerate(mutations):
        trial = copy.deepcopy(rows)
        mutate(trial)
        try:
            audit(trial)
        except (ValueError, KeyError):
            pass
        else:
            raise AssertionError(f"fail-closed mutation {i} was accepted")
    return len(mutations)

if __name__ == "__main__":
    rows = [json.loads(s) for s in PANEL.read_text(encoding="utf8").splitlines() if s.strip()]
    result = audit(rows)
    rejected = negative_controls(rows)
    print("CT239_RECORD_AUDIT_PASS",json.dumps(result,sort_keys=True),
          "mutations_rejected", rejected, "pcc:unverified")
