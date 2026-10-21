#!/usr/bin/env python3
"""Replay a current-source K5500 split-index rank and one-target control."""

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

from research.n83_highk_beyond4405_20261021 import run_k5500_full_control as runner
from research.n83_indexed5_20261013 import analyze as group

HERE = Path(__file__).resolve().parent
BASE_SHA256 = "823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd"
STATES = 1_255_377_750
PRIMARY_TABLE_BYTES = 5 * (1 << 31)
OVERFLOW_TABLE_BYTES = 5 * (1 << 30)
BLOOM_BYTES = (1 << 31) + (1 << 30)
PHASE_KEYS = ("target_query_ms", "target_pdp_ms",
              "target_relation_check_ms", "target_descent_ms",
              "target_recovery_check_ms")


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def finite_nonnegative(value: object) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and \
        math.isfinite(value) and value >= 0


def check(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def replay(output: Path) -> dict:
    launch = read(output / "launch.json")
    receipt = read(output / "receipt.json")
    if receipt.get("status") != "PROCESS_EXITED" or receipt.get("exit_code") != 0:
        return {"schema": "n83.k5500-current-source-full-replay/v1",
                "status": "BOUNDED_UNKNOWN", "process_status": receipt.get("status"),
                "receipt": receipt}
    try:
        command = launch["command"]
        check(launch["schema"] ==
              "n83.k5500-current-source-full-control-launch/v1" and
              receipt["schema"] ==
              "n83.k5500-current-source-full-control-receipt/v1",
              "runner schema differs")
        check(launch["source_sha256"] == runner.base.SOURCE_SHA256 and
              runner.base.sha256(runner.base.SOURCE) ==
              runner.base.SOURCE_SHA256, "current source differs")
        committed = subprocess.check_output(
            ["git", "show", launch["source_head_sha"] +
             ":examples/koblitz_orbit_dlp_fast.rs"], cwd=runner.ROOT)
        check(hashlib.sha256(committed).hexdigest() ==
              runner.base.SOURCE_SHA256, "committed source differs")
        check(len(command) == 5 and command[1:] == [
            "construct:83:1:5500", str(output / "target.jsonl"), "29",
            str(output / "targets_result.jsonl")], "command differs")
        check(launch["environment_flags"] == {
            **runner.FLAGS, "KIC_DUMP_BASE": str(output / "base_dump.jsonl")},
            "K5500 ID31 split flags differ")
        check(launch["cap_seconds"] == runner.CAP_SECONDS and
              launch["rss_cap_bytes"] == runner.RSS_CAP_BYTES and
              launch["min_free_bytes"] == runner.MIN_FREE_BYTES and
              0 < receipt["sampled_peak_rss_bytes"] <= runner.RSS_CAP_BYTES,
              "resource envelope differs")
        for label, path in (
            ("launch_sha256", output / "launch.json"),
            ("stdout_sha256", output / "stdout.jsonl"),
            ("stderr_sha256", output / "stderr.log"),
            ("base_dump_sha256", output / "base_dump.jsonl"),
            ("targets_result_sha256", output / "targets_result.jsonl"),
        ):
            check(receipt[label] == runner.base.sha256(path),
                  f"{label} differs")
        check(launch["target_sha256"] ==
              runner.base.sha256(output / "target.jsonl") and
              launch["public_targets_sha256"] ==
              runner.base.sha256(runner.PUBLIC) and
              launch["answer_sidecar_sha256"] ==
              runner.base.sha256(runner.ANSWERS) and
              runner.base.sha256(Path(command[0])) == launch["binary_sha256"],
              "fixture or binary differs")
        prior = read(HERE / "k5500_current_source_base" /
                     "byte_identity_replay.json")
        check(prior["status"] ==
              "PASS_BYTE_IDENTICAL_TO_PRIOR_INDEPENDENT_REPLAY" and
              prior["base_dump_sha256"] == BASE_SHA256 and
              receipt["base_dump_sha256"] == BASE_SHA256,
              "full base differs from independently replayed base")
        summary = read(output / "stdout.jsonl")
        rows = [json.loads(line) for line in
                (output / "targets_result.jsonl").read_text().splitlines()
                if line]
        check(summary.get("kind") == "compact_orbit_dlp_summary" and
              summary.get("n") == 83 and summary.get("a") == 1 and
              summary.get("orbit_columns") == 5500 and
              summary.get("factor_base_points") == 913000 and
              summary.get("base_hash") == runner.base.PRIOR_BASE_HASH and
              summary.get("base_source") == "constructed_in_process_x_scan",
              "curve or factor base differs")
        check(summary.get("regular_states") == STATES and
              summary.get("root_state_slot_bytes") == 11 and
              summary.get("root_state_allocated_bytes") == 11 * STATES and
              summary.get("root_table_slot_bytes") == 5 and
              summary.get("primary_table_allocated_bytes") ==
              PRIMARY_TABLE_BYTES and
              summary.get("overflow_table_allocated_bytes") ==
              OVERFLOW_TABLE_BYTES and
              summary.get("root_table_allocated_bytes") ==
              PRIMARY_TABLE_BYTES + OVERFLOW_TABLE_BYTES and
              summary.get("root_bloom_bytes") == BLOOM_BYTES and
              summary.get("shared_bloom_enabled") is False and
              summary.get("root_bloom_profile") == "wide3" and
              summary.get("indexed5_split_at") == 4549 and
              summary.get("root_table_indexed5_id31_enabled") is True and
              summary.get("indexed5_load80_enabled") is True and
              summary.get("root_only11_enabled") is True,
              "K5500 split-index layout differs")
        check(summary.get("root_table_entries") ==
              summary.get("primary_table_entries", -1) +
              summary.get("overflow_table_entries", -1) and
              0 < summary["root_table_entries"] <= 2 * STATES and
              summary.get("rank") == 5500 and
              summary.get("rank_relations", 0) >= 5500 and
              summary.get("targets") == 1 and
              summary.get("targets_solved") == 1 and
              summary.get("targets_failed") == 0 and len(rows) == 1,
              "index, rank or target count differs")
        target, answer = runner.checked_fixture()
        check(read(output / "target.jsonl") == target and
              group.point_from_json(target) ==
              group.multiply(group.G, int(answer["scalar"])),
              "public target or scalar differs")
        row = rows[0]
        indices = row["point_indices"]
        check(len(indices) == 4 and
              all(isinstance(i, int) and not isinstance(i, bool) and
                  0 <= i < 913000 for i in indices) and
              int(row["recovered_scalar"]) == int(answer["scalar"]) and
              row.get("group_verified") is True,
              "target relation or recovered scalar differs")
        dump = read(output / "base_dump.jsonl")
        check(dump.get("base_hash") == runner.base.PRIOR_BASE_HASH and
              len(dump["factor_base_point_coordinates"]) == 913000,
              "ordered base dump differs")
        total = None
        for index in indices:
            total = group.add(total, group.point_from_json(
                dump["factor_base_point_coordinates"][index]))
        check(total == group.point_from_json(target),
              "independent four-point relation sum differs")
        phases = {key: row[key] for key in PHASE_KEYS}
        online = row["target_ms"]
        check(finite_nonnegative(online) and online > 0 and
              all(finite_nonnegative(value) for value in phases.values()) and
              abs(math.fsum(phases.values()) - online) <= 1,
              "target phase accounting differs")
        timing = summary["timing_ms"]
        cold = timing["process_total"]
        parts = (timing["setup_total"], timing["rank_stage"],
                 timing["targets_total"])
        check(finite_nonnegative(cold) and cold > 0 and
              all(finite_nonnegative(value) for value in parts) and
              math.fsum(parts) <= cold + 1 and
              abs(timing["targets_total"] - online) <= 1,
              "cold process accounting differs")
        return {
            "schema": "n83.k5500-current-source-full-replay/v1",
            "status": "PASS_INDEPENDENT_FULL_REPLAY",
            "source_sha256": runner.base.SOURCE_SHA256,
            "binary_sha256": launch["binary_sha256"],
            "base_dump_sha256": BASE_SHA256,
            "base_hash_blake3": runner.base.PRIOR_BASE_HASH,
            "usable_points": 913000,
            "folded_columns": 5500,
            "regular_states": STATES,
            "rank": 5500,
            "recovered_scalar": str(answer["scalar"]),
            "point_indices": indices,
            "one_target_online_ms": online,
            "online_phases_ms": phases,
            "cold_process_ms": cold,
            "cold_unattributed_ms": max(0, cold - math.fsum(parts)),
            "sampled_peak_rss_bytes": receipt["sampled_peak_rss_bytes"],
        }
    except (OSError, KeyError, IndexError, ValueError, TypeError, AttributeError,
            OverflowError, subprocess.CalledProcessError) as error:
        return {"schema": "n83.k5500-current-source-full-replay/v1",
                "status": "VERIFICATION_FAILED", "error": str(error)}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve(strict=True)
    result = replay(output)
    (output / "independent_replay.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"],
                      "one_target_online_ms": result.get("one_target_online_ms"),
                      "cold_process_ms": result.get("cold_process_ms")},
                     sort_keys=True))
    return 0 if result["status"] == "PASS_INDEPENDENT_FULL_REPLAY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
