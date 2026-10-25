#!/usr/bin/env python3
"""Name and register the exact IC candidates after four verified processes."""

import argparse
import copy
import hashlib
import json
from pathlib import Path

from research.n83_fp8_e2e_control_20261025 import run, verify
from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


HERE = Path(__file__).resolve().parent
BASE_IDENTITIES = run.ROOT / "research/n83_nearby_full_grid_20261024/identities"
SOURCE_COMMIT = "08e1ef3aa4426eb7670436405f7f1f4ca0e95419"
PREFIX = "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def named_candidate(base, arm):
    record = copy.deepcopy(base)
    record.pop("candidate_id")
    record["implementation"]["source_commit"] = SOURCE_COMMIT
    record["implementation"]["source_sha256"]["ic"] = run.SOURCE_SHA256
    record["implementation"]["algorithm_flags"] = dict(run.FLAGS)
    if arm == "fp8":
        record["implementation"]["algorithm_flags"]["KIC_INDEXED4_FINGERPRINT8"] = "1"
    pdp = record["point_decomposition"]
    pdp["solver_source_sha256"] = run.SOURCE_SHA256
    pdp["variant"] = "S3_four_summand_symmetric_root_only11_four_byte_relative30_split512_compact4_bloom" + (
        "_byte_fingerprint_sidecar" if arm == "fp8" else "_packed_fingerprint_only")
    pdp["root_slot"] = {
        "bytes": 4, "state_id_bits": 30, "root_selector_bits": 1,
        "packed_fingerprint_bits": 1,
        "sidecar_fingerprint_bits": 8 if arm == "fp8" else 0,
        "sidecar_bytes_per_slot": 1 if arm == "fp8" else 0,
        "state_id_rule": "relative_to_primary_or_extension_segment",
        "primary_segment_columns": 512, "extension_segment_columns": 595,
        "request_rule": "ceil_five_times_segment_states_over_four_then_two_power_of_two_slots",
        "duplicate_policy": "first_inserted_exact_root_wins_primary_before_extension",
        "empty_marker": "zero_packed_word",
        "lookup_equality": "full_canonical_root_from_retained_state_after_fingerprint_filter",
    }
    pdp["root_bloom_filter"] = {
        "profile": "compact4", "bits_per_request": 4, "hashes": 3,
        "policy": "per_segment_membership_prefilter_then_exact_root_lookup",
    }
    for section in ("relation_collection", "relation_linear_algebra", "target_descent"):
        record[section]["source_digest"] = run.SOURCE_SHA256
    digest = hashlib.sha256(canonical(record)).hexdigest()[:12]
    record["candidate_id"] = PREFIX + digest
    return record


def seal(output):
    replay = verify.verify(output)
    if replay["status"] != "PASS_EXACT_FULL_PROCESS_PARITY":
        raise ValueError("four full-process arms have not passed independent replay")
    candidate, expected_sha = fixture.CANDIDATES[1107]
    base_path = BASE_IDENTITIES / f"candidate_{candidate}.json"
    if run.sha(base_path) != expected_sha:
        raise ValueError("exact K1107 base manifest differs")
    base = json.loads(base_path.read_text())
    if (base["factor_base"]["source_base_hash_blake3"] !=
            "b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e" or
            base["factor_base"]["actual_usable_points"] != 183762):
        raise ValueError("existing base identity differs")
    if any(row["base_hash"] != base["factor_base"]["source_base_hash_blake3"] or
           row["factor_base_points"] != 183762 for row in replay["rows"]):
        raise ValueError("measured base differs from exact candidate base")
    manifests = {}
    for arm in ("baseline", "fp8"):
        record = named_candidate(base, arm)
        path = HERE / f"candidate_{record['candidate_id']}.json"
        blob = json.dumps(record, sort_keys=True, indent=2) + "\n"
        if path.exists() and path.read_text() != blob:
            raise ValueError("candidate manifest has changed")
        path.write_text(blob)
        manifests[arm] = {"candidate_id": record["candidate_id"],
                          "candidate_manifest_sha256": run.sha(path)}
    target = json.loads(fixture.TARGETS.read_text())
    answer = json.loads(fixture.ANSWERS.read_text())
    workload = {
        "schema_version": 1,
        "curve_id": base["curve"]["curve_id"],
        "subgroup_order": str(base["curve"]["r"]),
        "generator": base["curve"]["G"],
        "targets": [target], "target_count": 1,
        "input_law": "fixed_existing_public_point_from_known_scalar",
        "fixture_scalar": str(answer["scalar"]),
        "target_generation_seed": None,
        "online_scope": "one_target_after_reusable_base_index_and_factor_logs",
    }
    workload_id = hashlib.sha256(canonical(workload)).hexdigest()[:12]
    summary = json.loads((Path(output) / "summary.json").read_text())
    repetitions = {"baseline": 0, "fp8": 0}
    rows = []
    for child, check in zip(summary["rows"], replay["rows"]):
        arm = child["arm"]
        repetitions[arm] += 1
        candidate_id = manifests[arm]["candidate_id"]
        rows.append({
            "candidate_id": candidate_id,
            "workload_id": workload_id,
            "run_id": f"{candidate_id}W{workload_id}R{repetitions[arm]}",
            "arm": arm, "status": child["status"],
            "process_wall_ms": child["wall_seconds"] * 1000,
            "process_internal_ms": check["process_ms"],
            "index_build_ms": check["index_ms"],
            "one_target_online_ms": check["online_ms"],
            "rank": check["rank"], "scalar": check["recovered_scalar"],
            "peak_sampled_rss_bytes": child["peak_sampled_rss_bytes"],
        })
    result = {"schema": "n83.fp8-full-abba-sealed/v1",
              "scope": "shared_host_exploratory",
              "replay_status": replay["status"],
              "source_sha256": run.SOURCE_SHA256,
              "binary_sha256": summary["binary_sha256"],
              "workload_id": workload_id, "workload_record": workload,
              "candidates": manifests, "rows": rows}
    (Path(output) / "sealed_result.json").write_text(
        json.dumps(result, sort_keys=True, indent=2) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = seal(args.output)
    print(result["workload_id"], *(value["candidate_id"] for value in result["candidates"].values()))
