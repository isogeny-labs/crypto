#!/usr/bin/env python3
"""Register two matched-index factor-base candidates and the single-point runs."""

import argparse
import hashlib
import json
from pathlib import Path

from research.n83_fp8_e2e_control_20261025 import (
    run, run_base_abba as control, seal, verify_base_abba,
)
from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


HERE = Path(__file__).resolve().parent


def register(output):
    replay = verify_base_abba.verify(output)
    if replay["status"] != "PASS_INDEPENDENT_REPLAY":
        raise ValueError("four full-process factor-base arms have not passed replay")
    manifests = {}
    for k in sorted(control.BASE_HASHES):
        base_path = control.base_manifest_path(k)
        if run.sha(base_path) != control.BASE_MANIFEST_SHA256[k]:
            raise ValueError("base manifest differs")
        base = json.loads(base_path.read_text())
        if (base["factor_base"]["source_base_hash_blake3"] != control.BASE_HASHES[k] or
                base["factor_base"]["effective_columns"] != k or
                base["factor_base"]["actual_usable_points"] != 166 * k):
            raise ValueError("base identity differs")
        candidate = seal.named_candidate(base, "fp8")
        path = HERE / f"candidate_{candidate['candidate_id']}.json"
        blob = json.dumps(candidate, sort_keys=True, indent=2) + "\n"
        if path.exists() and path.read_text() != blob:
            raise ValueError("candidate manifest differs")
        path.write_text(blob)
        manifests[k] = {"candidate_id": candidate["candidate_id"],
                        "candidate_manifest_sha256": run.sha(path)}
    target = json.loads(fixture.TARGETS.read_text())
    answer = json.loads(fixture.ANSWERS.read_text())
    curve = json.loads(control.base_manifest_path(1111).read_text())["curve"]
    workload = {
        "schema_version": 1, "curve_id": curve["curve_id"],
        "subgroup_order": str(curve["r"]), "generator": curve["G"],
        "targets": [target], "target_count": 1,
        "input_law": "fixed_existing_public_point_from_known_scalar",
        "fixture_scalar": str(answer["scalar"]),
        "target_generation_seed": None,
        "online_scope": "one_target_after_reusable_base_index_and_factor_logs",
    }
    workload_id = hashlib.sha256(seal.canonical(workload)).hexdigest()[:12]
    receipt = json.loads((Path(output) / "summary.json").read_text())
    repetitions = {1111: 0, 1115: 0}
    rows = []
    for child, check in zip(receipt["rows"], replay["rows"]):
        k = child["k"]
        repetitions[k] += 1
        candidate_id = manifests[k]["candidate_id"]
        rows.append({
            "candidate_id": candidate_id, "workload_id": workload_id,
            "run_id": f"{candidate_id}W{workload_id}R{repetitions[k]}",
            "k": k, "status": child["status"],
            "process_wall_ms": child["wall_seconds"] * 1000,
            "process_internal_ms": check["process_ms"],
            "index_build_ms": check["index_ms"],
            "one_target_online_ms": check["online_ms"],
            "rank": check["rank"], "scalar": check["scalar"],
            "peak_sampled_rss_bytes": child["peak_sampled_rss_bytes"],
        })
    result = {
        "schema": "n83.fp8-base-abba-sealed/v1",
        "scope": "shared_host_exploratory",
        "comparison_variable": "factor_base",
        "replay_status": replay["status"],
        "source_sha256": run.SOURCE_SHA256,
        "binary_sha256": receipt["binary_sha256"],
        "workload_id": workload_id, "workload_record": workload,
        "candidates": {str(k): v for k, v in manifests.items()}, "rows": rows,
    }
    (Path(output) / "sealed_result.json").write_text(
        json.dumps(result, sort_keys=True, indent=2) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = register(args.output)
    print(result["workload_id"],
          *(record["candidate_id"] for record in result["candidates"].values()))
