#!/usr/bin/env python3
"""Frozen fixed-panel wall rule for the K13 combined rootfinding comparison.

This reads, never edits, an ecbench session's records and per-record work
output. It reports a mechanical rule, not an evidence-review decision.
"""

import argparse
import json
import random
from collections import Counter, defaultdict
from pathlib import Path

ARMS = (
    "rho-strong", "matrix-f4", "matrix-f5", "inherited-f4", "f6-ic",
    "rootfind-m3", "rootfind-m2", "batch-m3", "batch-m2", "batch-m3-aa",
)
COMPARISONS = (
    ("matrix-f4", "batch-m3"),
    ("matrix-f5", "batch-m3"),
    ("inherited-f4", "batch-m3"),
    ("f6-ic", "batch-m3"),
    ("rootfind-m3", "batch-m3"),
)
AA = ("batch-m3", "batch-m3-aa")
TARGETS = 8
ROUNDS = 6
WARMUPS = 1
RESAMPLES = 20_000
SEED = 202611151603
EXPECTED_PHASES = {"factor_base", "oracle_setup", "relations", "linear_algebra", "verify"}
EXPECTED_CURVE = {"kind": "koblitz", "a": 0, "n": 13}
EXPECTED_CURVE_UID = "urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60"
TARGET_SEED = 202611151301


def read_jsonl(path):
    with Path(path).open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def percentile(sorted_values, fraction):
    position = fraction * (len(sorted_values) - 1)
    low = int(position)
    high = min(low + 1, len(sorted_values) - 1)
    return sorted_values[low] + (sorted_values[high] - sorted_values[low]) * (position - low)


def interval(by_target, lower, upper, seed):
    targets = sorted(by_target)
    rng = random.Random(seed)
    ratios = []
    for _ in range(RESAMPLES):
        draws = [targets[rng.randrange(len(targets))] for _ in targets]
        numerator = sum(by_target[t][0] for t in draws)
        denominator = sum(by_target[t][1] for t in draws)
        ratios.append(numerator / denominator)
    ratios.sort()
    return [percentile(ratios, lower), percentile(ratios, upper)]


def validity(record):
    reasons = []
    workload = record.get("workload") or {}
    if (workload.get("curve_spec") != EXPECTED_CURVE
            or (workload.get("curve") or {}).get("curve_uid") != EXPECTED_CURVE_UID
            or (workload.get("curve") or {}).get("registered") is not True
            or workload.get("target_law") != "hash_to_subgroup_v1"
            or workload.get("target_seed") != TARGET_SEED
            or type(workload.get("target_index")) is not int
            or workload["target_index"] not in range(TARGETS)
            or workload.get("planted") is not None
            or not workload.get("workload_id")
            or not workload.get("workload_sha256")
            or not isinstance(workload.get("target"), list)
            or len(workload["target"]) != 2):
        reasons.append("fixed_workload_identity_invalid")
    outcome = record.get("outcome") or {}
    if outcome.get("status") != "verified" or outcome.get("matches_target") is not True:
        reasons.append("scalar_not_verified")
    wall = (record.get("time") or {}).get("process_wall_ns")
    if not isinstance(wall, int) or wall <= 0:
        reasons.append("process_wall_missing")
    if (record.get("isolation") or {}).get("level") not in {"L2", "L3"}:
        reasons.append("below_L2")
    phases = record.get("phases")
    if not isinstance(phases, list) or not phases:
        reasons.append("phases_missing")
    elif record.get("method", {}).get("family") == "ic":
        present = {phase.get("name"): phase.get("wall_ns") for phase in phases}
        if not EXPECTED_PHASES <= present.keys() or any(
            not isinstance(present[name], int) or present[name] < 0
            for name in EXPECTED_PHASES if name in present
        ):
            reasons.append("ic_phase_missing")
    if not record.get("binary_sha256"):
        reasons.append("binary_hash_missing")
    node = (record.get("isolation") or {}).get("node")
    if type(node) is not int or node < 0:
        reasons.append("host_node_missing")
    if not record.get("env_class_id") or not record.get("algorithm_seed"):
        reasons.append("pairing_identity_missing")
    if not isinstance(record.get("warmup"), bool):
        reasons.append("warmup_flag_missing")
    return reasons


def pair_summary(rows, baseline, candidate, ci, seed):
    keys = {(target, round_no) for target in {k[1] for k in rows}
            for round_no in range(WARMUPS, WARMUPS + ROUNDS)}
    pairs = []
    for target, round_no in sorted(keys):
        first = rows.get((baseline, target, round_no))
        second = rows.get((candidate, target, round_no))
        if first is None or second is None or validity(first) or validity(second):
            continue
        if first["binary_sha256"] != second["binary_sha256"]:
            continue
        if first["isolation"]["node"] != second["isolation"]["node"]:
            continue
        if first.get("env_class_id") != second.get("env_class_id"):
            continue
        if first.get("algorithm_seed") != second.get("algorithm_seed"):
            continue
        pairs.append((target, first["time"]["process_wall_ns"], second["time"]["process_wall_ns"]))
    result = {"baseline": baseline, "candidate": candidate, "admitted_pairs": len(pairs),
              "expected_pairs": TARGETS * ROUNDS, "complete": len(pairs) == TARGETS * ROUNDS}
    if not result["complete"]:
        return result
    by_target = defaultdict(lambda: [0, 0])
    for target, base_ns, cand_ns in pairs:
        by_target[target][0] += cand_ns
        by_target[target][1] += base_ns
    if len(by_target) != TARGETS or any(v[1] <= 0 for v in by_target.values()):
        result["complete"] = False
        return result
    result["ratio"] = sum(v[0] for v in by_target.values()) / sum(v[1] for v in by_target.values())
    result["target_ratio_range"] = [min(v[0] / v[1] for v in by_target.values()),
                                    max(v[0] / v[1] for v in by_target.values())]
    result["all_pairs_lower"] = all(cand_ns < base_ns for _, base_ns, cand_ns in pairs)
    result["interval"] = interval(by_target, ci[0], ci[1], seed)
    return result


def analyze(records, work_rows):
    invalid = []
    rows = {}
    warmup_slots = set()
    work = {}
    for row in work_rows:
        key = row.get("record_id")
        if not isinstance(key, str) or not key:
            invalid.append({"record_id": key, "reasons": ["work_record_id_invalid"]})
        if key in work:
            invalid.append({"record_id": key, "reasons": ["duplicate_work_row"]})
        result = row.get("work")
        if not isinstance(result, dict) or set(result) not in ({"Ok"}, {"Err"}):
            invalid.append({"record_id": key, "reasons": ["work_result_missing"]})
        work[key] = row
    work_completeness = Counter()
    unpriced = defaultdict(set)
    binaries = set()
    nodes = set()
    sessions = set()
    workloads = set()
    workload_by_index = {}
    for record in records:
        rid = record.get("record_id")
        if not isinstance(rid, str) or not rid:
            invalid.append({"record_id": rid, "reasons": ["record_id_invalid"]})
        arm = record.get("arm")
        target = (record.get("workload") or {}).get("workload_id")
        round_no = record.get("round")
        warmup = record.get("warmup")
        if arm not in ARMS or not target or not isinstance(round_no, int):
            invalid.append({"record_id": rid, "reasons": ["slot_identity_invalid"]})
            continue
        workloads.add(target)
        workload = record["workload"]
        index = workload.get("target_index")
        if type(index) is int and index in range(TARGETS):
            identity = (target, workload.get("workload_sha256"),
                        json.dumps(workload.get("target"), sort_keys=True),
                        workload.get("target_counter"))
            previous = workload_by_index.setdefault(index, identity)
            if identity != previous:
                invalid.append({"record_id": rid, "reasons": ["workload_index_conflict"]})
        sessions.add(record.get("session_id"))
        binaries.add(record.get("binary_sha256"))
        nodes.add((record.get("isolation") or {}).get("node"))
        reasons = validity(record)
        if reasons:
            invalid.append({"record_id": rid, "reasons": reasons})
        work_record = (work.get(rid) or {}).get("work") or {}
        ok = work_record.get("Ok") or {}
        if ok.get("complete") is True:
            work_completeness[arm] += 1
        for item in ok.get("unpriced") or []:
            unpriced[arm].add(str(item))
        slot = (arm, target, round_no)
        if warmup:
            if slot in warmup_slots:
                invalid.append({"record_id": rid, "reasons": ["duplicate_warmup_slot"]})
            warmup_slots.add(slot)
        else:
            if slot in rows:
                invalid.append({"record_id": rid, "reasons": ["duplicate_measured_slot"]})
            rows[slot] = record
    expected = len(ARMS) * TARGETS * (ROUNDS + WARMUPS)
    record_ids = [record.get("record_id") for record in records]
    work_records_complete = (len(work_rows) == expected
                             and len(work) == expected
                             and len(set(record_ids)) == expected
                             and set(record_ids) == set(work))
    expected_measured = {(arm, target, round_no) for arm in ARMS for target in workloads
                         for round_no in range(WARMUPS, WARMUPS + ROUNDS)}
    expected_warmup = {(arm, target, round_no) for arm in ARMS for target in workloads
                       for round_no in range(WARMUPS)}
    panel_complete = (len(records) == expected and work_records_complete
                      and len(workloads) == TARGETS
                      and set(workload_by_index) == set(range(TARGETS))
                      and set(rows) == expected_measured and warmup_slots == expected_warmup
                      and len(binaries) == 1 and None not in binaries
                      and len(nodes) == 1 and None not in nodes
                      and len(sessions) == 1 and None not in sessions and not invalid)
    aa = pair_summary(rows, *AA, (0.025, 0.975), SEED)
    aa_pass = (aa["complete"] and aa["interval"][0] <= 1 <= aa["interval"][1])
    comparisons = [pair_summary(rows, base, cand, (0.005, 0.995), SEED)
                   for base, cand in COMPARISONS]
    for result in comparisons:
        result["mechanical_rule_met"] = bool(panel_complete and aa_pass and result["complete"]
            and result["all_pairs_lower"] and result["interval"][1] < 1
            and result["interval"][1] < aa["interval"][0])
    return {
        "schema": "rootfind-f456-batch.fixed-wall-analysis/v1",
        "planned_executions": expected,
        "observed_executions": len(records),
        "observed_work_rows": len(work_rows),
        "work_records_complete": work_records_complete,
        "missing_work_records": sorted(set(record_ids) - set(work), key=str),
        "extra_work_records": sorted(set(work) - set(record_ids), key=str),
        "panel_complete": panel_complete,
        "invalid_rows": invalid,
        "binary_sha256": sorted(x for x in binaries if x),
        "nodes": sorted(x for x in nodes if type(x) is int and x >= 0),
        "work_complete_count_by_arm": dict(sorted(work_completeness.items())),
        "unpriced_by_arm": {key: sorted(value) for key, value in sorted(unpriced.items())},
        "aa": aa,
        "aa_pass": aa_pass,
        "comparisons": comparisons,
        "all_four_f456_rules_met": all(c["mechanical_rule_met"] for c in comparisons[:4]),
        "requires_ecbench_replay_and_independent_review": True,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("records_jsonl")
    parser.add_argument("work_jsonl", help="ecbench work SESSION > work.jsonl, including failed rows")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    result = analyze(read_jsonl(args.records_jsonl), read_jsonl(args.work_jsonl))
    Path(args.out).write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(f"panel_complete={result['panel_complete']} four_rules_met={result['all_four_f456_rules_met']}")


if __name__ == "__main__":
    main()
