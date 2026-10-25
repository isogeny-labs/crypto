#!/usr/bin/env python3
"""Independently replay the bounded K1107 fixed-root4 control receipt."""

import argparse
import hashlib
import json
import math
from pathlib import Path

from research.n83_crossfamily_current_20261025 import run as wide_run
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_indexed5_20261013 import analyze as group
from research.n83_root4_small_control_20261025 import run as producer


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(output):
    folder = output / "run"
    launch = read(folder / "launch.json")
    receipt = read(folder / "receipt.json")
    expected_flags = wide_run.ic_environment(folder, "root4_standard",
                                             "k3500_root4")
    expected_flags = {k: v for k, v in expected_flags.items()
                      if k.startswith("KIC_")}
    binary = output / "executed_bins/koblitz_orbit_dlp_fast"
    command = [str(binary), "construct:83:1:1107", str(screen.TARGETS),
               "29", str(folder / "targets_result.jsonl")]
    source_ok = (
        launch.get("schema") == "n83.root4-small-control-launch/v1" and
        launch.get("source_sha256") == producer.SOURCE_SHA256 and
        sha(producer.ROOT / "examples/koblitz_orbit_dlp_fast.rs") ==
        producer.SOURCE_SHA256 and
        launch.get("binary_sha256") == sha(binary) and
        launch.get("target_sha256") == sha(screen.TARGETS) ==
        screen.TARGETS_SHA256 and
        launch.get("answer_sha256") == sha(screen.ANSWERS) ==
        screen.ANSWERS_SHA256 and
        launch.get("command") == command and
        launch.get("environment_flags") == expected_flags and
        launch.get("cap_seconds") == producer.CAP_SECONDS and
        launch.get("rss_cap_bytes") == producer.RSS_CAP_BYTES)
    resources_ok = (
        receipt.get("schema") == "n83.root4-small-control-receipt/v1" and
        receipt.get("status") == "PROCESS_EXITED" and
        receipt.get("exit_code") == 0 and
        isinstance(receipt.get("wall_seconds"), (int, float)) and
        0 < receipt["wall_seconds"] <= producer.CAP_SECONDS + 5 and
        isinstance(receipt.get("peak_sampled_rss_bytes"), int) and
        0 < receipt["peak_sampled_rss_bytes"] <= producer.RSS_CAP_BYTES and
        receipt.get("stdout_sha256") == sha(folder / "stdout.jsonl") and
        receipt.get("stderr_sha256") == sha(folder / "stderr.log") and
        receipt.get("base_dump_sha256") ==
        sha(folder / "base_dump.jsonl") == producer.BASE_DUMP_SHA256 and
        receipt.get("target_output_sha256") ==
        sha(folder / "targets_result.jsonl"))
    summaries = [json.loads(line) for line in
                 (folder / "stdout.jsonl").read_text().splitlines() if line]
    summaries = [item for item in summaries if
                 item.get("kind") == "compact_orbit_dlp_summary"]
    targets = [json.loads(line) for line in
               (folder / "targets_result.jsonl").read_text().splitlines() if line]
    if len(summaries) != 1 or len(targets) != 1:
        raise ValueError("expected exactly one summary and target result")
    summary, target = summaries[0], targets[0]
    scalar = int(read(screen.ANSWERS)["scalar"])
    point = group.point_from_json(read(screen.TARGETS))
    phase_names = ("target_query_ms", "target_pdp_ms",
                   "target_relation_check_ms", "target_descent_ms",
                   "target_recovery_check_ms")
    phases = [target.get(name) for name in phase_names]
    target_ok = (
        summary.get("n") == 83 and summary.get("a") == 1 and
        summary.get("orbit_columns") == producer.K and
        summary.get("factor_base_points") == producer.BASE_POINTS and
        summary.get("base_hash") == producer.BASE_HASH and
        summary.get("rank") == producer.K and
        summary.get("targets_solved") == 1 and
        summary.get("targets_failed") == 0 and
        summary.get("root_table_indexed4_enabled") is True and
        summary.get("indexed4_fixed31_enabled") is True and
        summary.get("root_table_slot_bytes") == 4 and
        summary.get("root_bloom_enabled") is True and
        summary.get("root_bloom_profile") == "standard" and
        summary.get("target_threads") == 2 and
        target.get("kind") == "compact_orbit_dlp_target" and
        target.get("fixture_index") == 0 and
        target.get("target") == read(screen.TARGETS) and
        target.get("recovered_scalar") == scalar and
        target.get("group_verified") is True and
        group.multiply(group.G, scalar) == point and
        group.multiply(point, group.R) is None and
        isinstance(target.get("target_ms"), (int, float)) and
        target["target_ms"] > 0 and
        all(isinstance(value, (int, float)) and math.isfinite(value) and
            value >= 0 for value in phases) and
        abs(math.fsum(phases) - target["target_ms"]) <= 1.0)
    result = {
        "schema": "n83.root4-small-control-independent-replay/v1",
        "status": "PASS_INDEPENDENT_REPLAY" if
        source_ok and resources_ok and target_ok else "BOUNDED_UNKNOWN",
        "source_verified": source_ok, "resources_verified": resources_ok,
        "target_verified": target_ok,
        "scope": "existing_public_point_shared_host_correctness_control",
        "candidate_id": None,
        "source_sha256": producer.SOURCE_SHA256,
        "base_hash": summary.get("base_hash"),
        "base_dump_sha256": receipt.get("base_dump_sha256"),
        "one_target_online_ms_exploratory": target.get("target_ms"),
        "cold_process_ms_exploratory": summary.get("timing_ms", {}).get(
            "process_total"),
        "peak_sampled_rss_bytes": receipt.get("peak_sampled_rss_bytes"),
    }
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = verify(args.output.resolve())
    path = args.output.resolve() / "independent_replay.json"
    path.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"] == "PASS_INDEPENDENT_REPLAY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
