#!/usr/bin/env python3
"""Derive current-source IC candidate, workload, and run identities."""

import argparse
import copy
import gzip
import hashlib
import json
import math
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_online_finalists_20261018.generate_candidates import canonical_id

HERE = Path(__file__).resolve().parent
IDENTITIES = HERE / "identities"
ARMS = (1104, 1107, 1120)
K1107_ID = "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a"
K1107_MANIFEST = base.HERE / "identities" / f"candidate_{K1107_ID}.json"


def canonical(record, excluded):
    return json.dumps({key: value for key, value in record.items()
                       if key not in excluded}, sort_keys=True,
                      separators=(",", ":"), ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def read(path):
    return json.loads(path.read_text())


def rows(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def dump(k):
    path = (base.HERE / "k1107_current_source_base/base_dump.jsonl.gz"
            if k == 1107 else HERE / f"base_dump_k{k}.jsonl.gz")
    with gzip.open(path, "rb") as stream:
        blob = stream.read()
    return blob, json.loads(blob)


def set_digest(ordered_dump):
    points = sorted((int(x), int(y)) for x, y in
                    ordered_dump["factor_base_point_coordinates"])
    encoded = [[str(x), str(y)] for x, y in points]
    return sha(json.dumps(encoded, sort_keys=True, separators=(",", ":"),
                          ensure_ascii=False).encode())


def candidates():
    baseline = read(K1107_MANIFEST)
    if baseline["candidate_id"] != canonical_id(baseline):
        raise ValueError("K1107 candidate identity differs")
    result = {}
    for k in ARMS:
        blob, ordered_dump = dump(k)
        summary = read(HERE / "single" / f"k{k}" / "stdout.jsonl")
        if (summary["rank"] != k or summary["factor_base_points"] != 166*k or
                summary["base_hash"] != ordered_dump["base_hash"]):
            raise ValueError(f"K{k} base or rank differs")
        candidate = copy.deepcopy(baseline)
        factor_base = candidate["factor_base"]
        factor_base.update({
            "actual_usable_points": 166*k,
            "effective_columns": k,
            "enumerated_set_sha256": set_digest(ordered_dump),
            "ordered_base_dump_sha256": sha(blob),
            "source_base_hash_blake3": summary["base_hash"],
        })
        candidate["point_decomposition"]["root_slot"]["request_entries"] = \
            summary["root_table_request_entries"]
        candidate["point_decomposition"]["root_slot"]["allocated_bytes"] = \
            summary["root_table_allocated_bytes"]
        candidate["relation_collection"]["stop_criterion"] = f"rank_{k}"
        candidate["relation_linear_algebra"]["columns"] = k
        candidate["relation_linear_algebra"]["rank_criterion"] = k
        candidate["candidate_id"] = canonical_id(candidate)
        if k == 1107 and candidate != baseline:
            raise ValueError("derived K1107 identity differs from sealed record")
        result[k] = candidate
    return result


def workloads():
    definitions = {
        "single": ("earlier_public_control_fixture_0",
                   base.ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl",
                   "one_cold_process_one_target"),
        "batch": ("first_16_public_highk_screen_points",
                  base.ROOT / "research/n83_highk_beyond4405_20261021/screen_answers.jsonl",
                  "one_cold_process_shared_factor_logs_16_targets"),
        "batch192": ("first_192_public_highk_screen_points",
                     base.ROOT / "research/n83_highk_beyond4405_20261021/screen_answers.jsonl",
                     "one_cold_process_shared_factor_logs_192_targets"),
    }
    result = {}
    for kind, (law, answers_path, cache) in definitions.items():
        directory = HERE / ("batch192_attempt" if kind == "batch192" else kind) / "k1107"
        input_name = "target.jsonl" if kind == "single" else "targets.jsonl"
        points = rows(directory / input_name)
        answers = rows(answers_path)[:len(points)]
        if len(points) != len(answers) or not all(
                answer["fixture_index"] == i for i, answer in enumerate(answers)):
            raise ValueError(f"{kind} target or answer count differs")
        record = {
            "schema": "ic1.workload/v1",
            "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
            "target_group": "prime_order_r_subgroup",
            "input_law": law,
            "target_count": len(points),
            "target_points": points,
            "target_seed_sha256": [answer.get("seed_sha256") for answer in answers],
            "cache_policy": cache,
            "target_generation_timed": False,
        }
        record["workload_id"] = sha(canonical(record, {"workload_id"}))[:12]
        result[kind] = record
    return result


def measurements(candidate_records, workload_records):
    result = []
    for kind in ("single", "batch"):
        workload_id = workload_records[kind]["workload_id"]
        for k in ARMS:
            directory = HERE / kind / f"k{k}"
            candidate_id = candidate_records[k]["candidate_id"]
            launch, receipt = (read(directory / filename) for filename in
                               ("launch.json", "receipt.json"))
            summary = read(directory / "stdout.jsonl")
            target_rows = rows(directory / "targets_result.jsonl")
            replay = read(directory / "independent_replay.json")
            if (receipt["status"] != "PROCESS_EXITED" or
                    replay["status"] not in
                    ("PASS_EXPLORATORY_ONE_TARGET_REPLAY",
                     "PASS_SECONDARY_PROBE_REPLAY")):
                raise ValueError(f"{kind}/K{k} is not a verified completed run")
            run_number = 2 if kind == "single" and k == 1107 else 1
            timing = summary["timing_ms"]
            cold_parts = [timing[name] for name in
                          ("setup_total", "rank_stage", "targets_total")]
            record = {
                "schema": "ic1.empirical_run/v1",
                "candidate_id": candidate_id,
                "workload_id": workload_id,
                "run_id": f"{candidate_id}W{workload_id}R{run_number}",
                "run_number": run_number,
                "status": "PASS_EXPLORATORY_REPLAY",
                "hardware_tier": "ordinary_shared_host_diagnostic",
                "source_sha256": launch["source_sha256"],
                "binary_sha256": launch["binary_sha256"],
                "target_count": len(target_rows),
                "actual_usable_points": summary["factor_base_points"],
                "folded_columns": summary["orbit_columns"],
                "rank": summary["rank"],
                "rank_relations": summary["rank_relations"],
                "root_state_allocated_bytes": summary["root_state_allocated_bytes"],
                "root_table_allocated_bytes": summary["root_table_allocated_bytes"],
                "root_table_entries": summary["root_table_entries"],
                "base_hash_blake3": summary["base_hash"],
                "base_dump_sha256": receipt["base_dump_sha256"],
                "probes_by_target": [row["probes"] for row in target_rows],
                "online_ms_by_target": [row["target_ms"] for row in target_rows],
                "online_phases_ms_by_target": [
                    {key: row[key] for key in (
                        "target_query_ms", "target_pdp_ms",
                        "target_relation_check_ms", "target_descent_ms",
                        "target_recovery_check_ms")}
                    for row in target_rows],
                "one_target_online_ms": (target_rows[0]["target_ms"]
                                         if kind == "single" else None),
                "cold_process_ms": timing["process_total"],
                "cold_setup_ms": timing["setup_total"],
                "cold_rank_ms": timing["rank_stage"],
                "cold_targets_ms": timing["targets_total"],
                "cold_unattributed_ms": max(
                    0, timing["process_total"] - math.fsum(cold_parts)),
                "paired_rho_online_ms": None,
                "online_speedup": None,
                "peak_rss_bytes": summary.get("peak_rss_bytes"),
                "process_wall_seconds": receipt["wall_seconds"],
                "receipt_path": str((directory / "receipt.json").relative_to(HERE)),
                "replay_path": str((directory / "independent_replay.json").relative_to(HERE)),
            }
            result.append(record)
    directory = HERE / "batch192_attempt/k1107"
    launch, receipt, guard = (read(directory / filename) for filename in
                              ("launch.json", "receipt.json",
                               "../host_resource_guard.json"))
    if (receipt["status"] != "PROCESS_FAILED" or
            receipt["exit_code"] != -15 or
            guard["status"] != "STOPPED_HOST_STARTUP_DISK_GUARD" or
            guard["receipt_sha256"] != sha((directory / "receipt.json").read_bytes()) or
            guard["remaining_arms_not_launched"] != [1120, 1104]):
        raise ValueError("batch192 resource-guard receipt differs")
    candidate_id = candidate_records[1107]["candidate_id"]
    workload_id = workload_records["batch192"]["workload_id"]
    result.append({
        "schema": "ic1.empirical_run/v1",
        "candidate_id": candidate_id,
        "workload_id": workload_id,
        "run_id": f"{candidate_id}W{workload_id}R1",
        "run_number": 1,
        "status": "STOPPED_HOST_STARTUP_DISK_GUARD",
        "hardware_tier": "ordinary_shared_host_diagnostic",
        "source_sha256": launch["source_sha256"],
        "binary_sha256": launch["binary_sha256"],
        "target_count": 192,
        "exit_code": -15,
        "process_wall_seconds": receipt["wall_seconds"],
        "actual_usable_points": None,
        "folded_columns": None,
        "rank": None,
        "probes_by_target": None,
        "online_ms_by_target": None,
        "one_target_online_ms": None,
        "cold_process_ms": None,
        "paired_rho_online_ms": None,
        "online_speedup": None,
        "receipt_path": str((directory / "receipt.json").relative_to(HERE)),
        "guard_path": str((HERE / "batch192_attempt/host_resource_guard.json").relative_to(HERE)),
    })
    return result


def derive():
    if base.sha256(base.SOURCE) != base.SOURCE_SHA256:
        raise ValueError("current IC source differs")
    candidate_records = candidates()
    workload_records = workloads()
    run_records = measurements(candidate_records, workload_records)
    return candidate_records, workload_records, run_records


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    candidate_records, workload_records, run_records = derive()
    expected = {
        **{f"candidate_{record['candidate_id']}.json": record
           for record in candidate_records.values()},
        **{f"workload_{record['workload_id']}.json": record
           for record in workload_records.values()},
        "measurement_rows.json": run_records,
    }
    if args.write:
        IDENTITIES.mkdir(exist_ok=True)
    for filename, record in expected.items():
        path = IDENTITIES / filename
        blob = json.dumps(record, indent=2, sort_keys=True) + "\n"
        if args.write:
            path.write_text(blob)
        elif not path.exists() or path.read_text() != blob:
            raise ValueError(f"identity or measurement row differs: {filename}")
    print(json.dumps({"candidate_ids": {k: value["candidate_id"] for
                                        k, value in candidate_records.items()},
                      "workload_ids": {k: value["workload_id"] for
                                       k, value in workload_records.items()},
                      "run_count": len(run_records)}, sort_keys=True))


if __name__ == "__main__":
    main()
