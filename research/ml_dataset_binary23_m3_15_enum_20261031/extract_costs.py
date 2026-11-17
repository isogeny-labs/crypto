#!/usr/bin/env python3
"""Project bounded per-phase cost observations without upgrading PMU scope."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = Path("/mnt/spot-codex-state/tmp/ml-dataset-binary23-m3-15-enum-20261031")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text())


def main():
    rows = []
    for index in range(26):
        checkpoint = read(HERE / "checkpoints" / f"shard-{index:02d}.json")
        cell = ROOT / f"shard-{index:02d}" / "runner" / "attempt" / "cells" / f"{index:012d}"
        manifest = read(cell / "manifest.json")
        status = checkpoint["custody"]["native_status"]
        assert checkpoint["index"] == manifest["cell_index"] == index
        assert status["verified_and_exported"] and status["cell_spec"]["solver"] == manifest["config"]["solver"]
        phase_observations = []
        for artifact in manifest["artifacts"]:
            if artifact["role"] != "hardware-and-resource-telemetry":
                continue
            path = cell / artifact["path"]
            assert path.stat().st_size == artifact["bytes"] and sha(path) == artifact["sha256"]
            telemetry = read(path)
            assert telemetry["binding"]["cell_index"] == index
            observations = [{"event": o["event"], "status": o["status"],
                "raw": o.get("raw"), "estimate": o.get("estimate"),
                "lifecycle_clean": o.get("lifecycle_clean")}
                for o in telemetry["observations"]]
            phase_observations.append({"phase": telemetry["phase"],
                "artifact_sha256": artifact["sha256"],
                "elapsed_ns_diagnostic": telemetry["elapsed_ns_diagnostic"],
                "observations": observations,
                "whole_worker_retired_instructions": telemetry["whole_worker_retired_instructions"],
                "whole_worker_coverage_verified": telemetry["whole_worker_coverage_verified"],
                "independently_verified": telemetry["independently_verified"],
                "platform_admitted": telemetry["platform_admitted"],
                "training_cost_eligible": telemetry["training_cost_eligible"],
                "clock_valid_for_comparison": telemetry["clock_valid_for_comparison"]})
        assert 1 <= len(phase_observations) <= 4
        rows.append({"schema": "ml-dataset.binary23-cell-cost-observation/v1",
            "index": index, "m": manifest["config"]["m"],
            "method": manifest["config"]["solver"],
            "twist": manifest["config"]["twist"],
            "generator_index": manifest["config"]["generator_index"],
            "key_index": manifest["config"]["key_index"],
            "archive_sha256": checkpoint["custody"]["archive_sha256"],
            "native_status": manifest["outcome"]["status"],
            "adapter_counter_scope": manifest["outcome"].get("counter_scope"),
            "adapter_counters": manifest["outcome"].get("counters"),
            "binary_field_operation_tally": manifest["outcome"].get("binary_field_operation_tally"),
            "phase_observations": sorted(phase_observations, key=lambda p: p["phase"])})
    target = HERE / "cost-observations.jsonl"
    data = b"".join((json.dumps(row, sort_keys=True) + "\n").encode() for row in rows)
    if target.exists() and target.read_bytes() != data:
        raise ValueError("cost-observations.jsonl differs; review source changes")
    target.write_bytes(data)
    print(json.dumps({"cells": 26, "phase_rows": sum(len(r["phase_observations"]) for r in rows),
        "whole_worker_instruction_labels": sum(any(p["training_cost_eligible"] for p in r["phase_observations"]) for r in rows)},
        sort_keys=True))


if __name__ == "__main__":
    main()
