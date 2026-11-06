#!/usr/bin/env python3
"""Replay exact K128 index identity across serial and four-thread root building."""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
SERIAL = HERE / "runs/k128_shared_a"
PARALLEL = HERE / "runs/k128_root_threads4"
KEYS = (
    "n", "base_hash", "orbit_columns", "factor_base_points",
    "regular_states", "state_digest_blake3", "table_digest_blake3",
    "overflow_digest_blake3", "root_table_entries", "primary_table_entries",
    "overflow_table_entries", "root_table_allocated_bytes",
    "root_bloom_bytes", "shared_bloom_enabled", "indexed5_split_at",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify():
    old = json.loads((SERIAL / "result.json").read_text())
    new = json.loads((PARALLEL / "result.json").read_text())
    old_input = json.loads((SERIAL / "input.json").read_text())
    new_input = json.loads((PARALLEL / "input.json").read_text())
    if (old["status"] != "PASS_INDEX_STAGE_CONTROL" or
            new["status"] != "PASS_INDEX_STAGE_CONTROL" or
            old_input["source_sha256"] != new_input["source_sha256"] or
            old_input["target_sha256"] != new_input["target_sha256"] or
            new_input["root_threads"] != 4 or
            digest(PARALLEL / "runner.py") != new_input["runner_sha256"]):
        raise ValueError("serial/parallel source or process gate differs")
    old_by_name = {row["arm"]: row for row in old["arms"]}
    new_by_name = {row["arm"]: row for row in new["arms"]}
    for name in ("monolithic", "split"):
        first, second = old_by_name[name], new_by_name[name]
        if (first["exit_code"] != 0 or second["exit_code"] != 0 or
                second["status"] != "PROCESS_EXITED" or
                second["stage"].get("status") != "STAGE_DIAGNOSTIC_ONLY"):
            raise ValueError(f"{name}: stage process failed")
        for key in KEYS:
            if first["stage"][key] != second["stage"][key]:
                raise ValueError(f"{name}: {key} differs")
    return {
        "schema": "n83.root-threads-control/v1",
        "status": "PASS_EXACT_SERIAL_PARALLEL_INDEX",
        "serial_result_sha256": digest(SERIAL / "result.json"),
        "parallel_result_sha256": digest(PARALLEL / "result.json"),
        "source_sha256": new_input["source_sha256"],
        "target_sha256": new_input["target_sha256"],
        "root_threads": 4,
        "matched_fields": list(KEYS),
    }


if __name__ == "__main__":
    result = verify()
    output = PARALLEL / "verification.json"
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True))
