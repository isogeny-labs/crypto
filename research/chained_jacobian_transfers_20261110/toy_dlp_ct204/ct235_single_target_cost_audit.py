#!/usr/bin/env python3
"""CT235 single-target, frozen-native-record audit.

Recomputes *record arithmetic* only, not independent native relations,
wall-clock replay, optimized rho/frontier IC or signed PCC.
[pcc:unverified]
"""
import json
import math
import statistics
import sys
from collections import defaultdict
from pathlib import Path

P, R = 101, 109
G, Q = (2, 37), (19, 37)
BASES = (24, 36, 48, 54)
CANDIDATES = {24: 2882, 36: 1596, 48: 1421, 54: 1421}
NUMERIC = (
    "basis", "round", "rank", "candidates", "offsets", "full",
    "single_lp", "double_lp", "multi_lp_discard", "b_only_rows",
    "lp_pivot_rank", "known_lp", "graph_edges", "graph_cycles",
    "graph_components", "verified_relations", "target73",
    "target_candidates73", "setup_ns", "relations_ns", "la_ns",
    "verification_ns", "target_ns", "total_ns", "bsgs_ns",
    "cantor_adds", "target_adds", "logical_bytes_peak",
    "field_add", "field_mul", "field_sqr", "field_inv"
)
PHASES = ("setup_ns", "relations_ns", "la_ns", "verification_ns", "target_ns")
COUNT_KEYS = (
    "rank", "candidates", "offsets", "full", "single_lp", "double_lp",
    "multi_lp_discard", "b_only_rows", "lp_pivot_rank", "known_lp",
    "graph_edges", "graph_cycles", "graph_components", "verified_relations",
    "target73", "target_candidates73", "cantor_adds", "target_adds",
    "field_add", "field_mul", "field_sqr", "field_inv",
)

def ec_add(a, z):
    if a is None:
        return z
    if z is None:
        return a
    x, y = a
    u, v = z
    if x == u and (y + v) % P == 0:
        return None
    if x == u:
        slope = (3*x*x+1)*pow(2*y, -1, P) % P
    else:
        slope = (v-y)*pow((u-x) % P, -1, P) % P
    new_x = (slope*slope-x-u) % P
    return (new_x, (slope*(x-new_x)-y) % P)

def ec_mul(k, a):
    b = None
    while k:
        if k & 1:
            b = ec_add(b, a)
        a = ec_add(a, a)
        k >>= 1
    return b

def check(condition, message):
    if not condition:
        raise ValueError(message)

def audit(rows):
    check(len(rows) == 16, "expected exactly 16 original native records")
    check(ec_mul(73, G) == Q and ec_mul(R, G) is None, "independent E/F101 point oracle")
    by_base = defaultdict(list)
    observed = set()
    for row in rows:
        check(all(k in row for k in NUMERIC), "missing numeric field")
        check(all(type(row[k]) is int for k in NUMERIC), "non-integral native counter")
        check(all(row[k] >= 0 for k in NUMERIC), "negative native counter")
        check(row.get("pcc") == "unverified", "unsupported PCC standing")
        check(row.get("model") == "F101_signed_two_lp_Q73_only", "model mismatch")
        b, rd = row["basis"], row["round"]
        check(b in BASES and rd in range(4), "unexpected arm or round")
        check((b, rd) not in observed, "duplicate arm/round")
        observed.add((b, rd))
        expected_order = "IC_BSGS" if rd in (0, 3) else "BSGS_IC"
        check(row.get("arm_order") == expected_order, "ABBA order mismatch")
        check(row["target73"] == 73, "wrong recovered source scalar")
        check(ec_mul(row["target73"], G) == Q, "source target failed independent check")
        check(row["rank"] == b, "incomplete factor-base rank")
        check(row["candidates"] == CANDIDATES[b], "drift from frozen CT233 collector")
        check(0 < row["target_candidates73"] <= 6540, "invalid target search budget")
        check(row["candidates"] <= 6540, "relation budget exceeded")
        total_used = row["full"] + row["single_lp"] + row["double_lp"]
        check(total_used <= row["candidates"], "more relations than candidates")
        check(total_used + row["multi_lp_discard"] <= row["candidates"],
              "more classified support attempts than candidates")
        outside = 54-b
        check(row["graph_edges"] == row["single_lp"] + row["double_lp"],
              "graph edge count mismatch")
        check(row["graph_components"] == outside+1-row["graph_edges"]+row["graph_cycles"],
              "graph Euler identity mismatch")
        check(1 <= row["graph_components"] <= outside+1, "impossible graph components")
        check(row["lp_pivot_rank"] <= outside, "LP rank exceeds available columns")
        check(row["known_lp"] <= row["lp_pivot_rank"], "nonunique LP asserted known")
        check(row["b_only_rows"] + row["lp_pivot_rank"] == total_used,
              "Schur-complement row accounting mismatch")
        check(b <= row["b_only_rows"], "rank exceeds base-only rows")
        check(row["verified_relations"] <= total_used, "too many verified relations")
        check(row["total_ns"] > 0 and row["bsgs_ns"] > 0, "invalid elapsed time")
        check(sum(row[k] for k in PHASES) <= row["total_ns"],
              "timed phases exceed total wall time")
        check(row["logical_bytes_peak"] > 0, "logical memory missing")
        rss = row.get("vmhwm_kib")
        check(rss is None or (type(rss) is int and rss > 0),
              "malformed process-wide VmHWM")
        check(all(row[k] > 0 for k in ("field_add", "field_mul", "field_sqr", "field_inv")),
              "missing native field counters")
        ratio = row.get("ratio_bsgs_over_candidate")
        check(type(ratio) in (int,float) and math.isfinite(ratio), "invalid ratio")
        actual = row["bsgs_ns"] / row["total_ns"]
        check(math.isclose(ratio, actual, rel_tol=1e-9, abs_tol=1e-11),
              "reported ratio not backed by native times")
        if b == 54:
            check((row["full"], row["single_lp"], row["double_lp"]) == (62, 0, 0),
                  "unchanged 54-column native control drift")
        by_base[b].append(row)
    check(observed == {(b, rd) for b in BASES for rd in range(4)},
          "arm/round coverage failure")
    result = {}
    for b in BASES:
        records = sorted(by_base[b], key=lambda r: r["round"])
        check(len(records) == 4, "missing base repeats")
        reference = records[0]
        for rec in records[1:]:
            check(all(rec[k] == reference[k] for k in COUNT_KEYS),
                  "non-deterministic relation/counter replay")
        ratios = [r["bsgs_ns"]/r["total_ns"] for r in records]
        result[b] = {
            "median_candidate_ns": statistics.median(r["total_ns"] for r in records),
            "median_bsgs_ns": statistics.median(r["bsgs_ns"] for r in records),
            "median_paired_baseline_over_candidate": statistics.median(ratios),
            "min_paired_ratio": min(ratios),
            "max_paired_ratio": max(ratios),
        }
    return result

def synthetic_records():
    # Only for fail-closed mutation tests; NOT observations or benchmark evidence.
    fixture = (
        (24, 28, 2, 21, 43, 46, 38, 28, 3, 36, 66),
        (36, 15, 8, 32, 20, 9, 43, 17, 2, 35, 60),
        (48, 14, 36, 20, 6, 0, 56, 6, 1, 20, 62),
        (54, 14, 62, 0, 0, 0, 62, 0, 1, 0, 62),
    )
    out = []
    for b, offs, full, single, double, discarded, b_rows, lp_rank, comps, cycles, verified in fixture:
        for rd in range(4):
            out.append({
                "basis": b, "round": rd,
                "arm_order": "IC_BSGS" if rd in (0,3) else "BSGS_IC",
                "rank": b, "candidates": CANDIDATES[b], "offsets": offs,
                "full": full, "single_lp": single, "double_lp": double,
                "multi_lp_discard": discarded, "b_only_rows": b_rows,
                "lp_pivot_rank": lp_rank, "known_lp": lp_rank, "graph_edges": single+double,
                "graph_cycles": cycles, "graph_components": comps,
                "verified_relations": verified, "target73": 73, "target_candidates73": 21,
                "setup_ns": 100, "relations_ns": 1000, "la_ns": 100,
                "verification_ns": 100, "target_ns": 100, "total_ns": 1500,
                "bsgs_ns": 10, "ratio_bsgs_over_candidate": 10/1500,
                "cantor_adds": CANDIDATES[b]+offs, "target_adds": 26,
                "logical_bytes_peak": 49208, "vmhwm_kib": 3636,
                "field_add": 20000, "field_mul": 13000, "field_sqr": 200,
                "field_inv": 400, "model": "F101_signed_two_lp_Q73_only",
                "pcc": "unverified",
            })
    return out

def self_test():
    # Controlled synthetic base: every modification must reject decisively.
    source = synthetic_records()
    audit(source)
    patches = [
        (0, "target73", 37), (1, "rank", 23), (2, "ratio_bsgs_over_candidate", 3.0),
        (3, "graph_cycles", 35), (4, "lp_pivot_rank", 37),
        (5, "bsgs_ns", 0), (6, "total_ns", 1),
        (7, "known_lp", 19), (8, "vmhwm_kib", -1),
        (9, "arm_order", "IC_BSGS"), (10, "pcc", "verified"),
        (11, "candidates", 2883),
    ]
    failures = 0
    for i,key,value in patches:
        rows = [dict(r) for r in source]
        rows[i][key] = value
        try:
            audit(rows)
        except ValueError:
            failures += 1
        else:
            raise AssertionError(f"mutation accepted: {i}:{key}")
    for bad in (source[:-1], source + [dict(source[0])]):
        try:
            audit(bad)
        except ValueError:
            failures += 1
        else:
            raise AssertionError("missing/duplicate record accepted")
    print("CT235_SYNTHETIC_AUDIT_SELFTEST_PASS", failures, "rejections")

if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "--self-test":
        self_test()
    else:
        path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).with_name("ct235_native_panel.jsonl")
        data = [json.loads(x) for x in path.read_text().splitlines() if x.strip()]
        # A provenance header is allowed only if it explicitly binds source and run.
        if data and "source_sha" in data[0]:
            header = data.pop(0)
            check(isinstance(header.get("run_id"), int) and
                  isinstance(header.get("native_job_id"), int) and
                  isinstance(header.get("source_sha"), str) and
                  len(header["source_sha"]) == 40, "invalid source provenance")
        print(json.dumps(audit(data), sort_keys=True, indent=2))
        print("CT235_RECORD_ONLY_AUDIT_PASS", len(data), "unsigned observations")
