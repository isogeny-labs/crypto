#!/usr/bin/env python3
"""Bind the current-source K5500 dump to the prior independent replay."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from research.n83_highk_beyond4405_20261021 import run_k5500_base_stage as runner
from research.n83_indexed5_20261013 import analyze as group

PRIOR_RECORD = "research/n83_two_tier_root_20261021/k5500_base_record.json"
PRIOR_REPLAY = ("research/n83_two_tier_root_20261021/"
                "runs/base_only_k5500_20261021/verification.json")


def committed(path: str) -> tuple[dict, str]:
    blob = subprocess.check_output(["git", "show", "HEAD:" + path],
                                   cwd=runner.ROOT)
    return json.loads(blob), hashlib.sha256(blob).hexdigest()


def verify(stage: Path) -> dict:
    receipt = json.loads((stage / "receipt.json").read_text())
    summary = json.loads((stage / "stdout.jsonl").read_text())
    record, _ = committed(PRIOR_RECORD)
    replay, replay_sha = committed(PRIOR_REPLAY)
    custody = json.loads((runner.HERE / "source_binary_custody.json").read_text())
    command = receipt.get("command", [])
    expected_raw = record["factor_base"]["ordered_base_dump_sha256"]
    raw_sha = runner.sha256(stage / "base_dump.jsonl")
    if (receipt.get("status") != "PROCESS_EXITED" or
            receipt.get("source_sha256") != runner.SOURCE_SHA256 or
            runner.sha256(runner.SOURCE) != runner.SOURCE_SHA256 or
            receipt.get("binary_sha256") != custody["binary_sha256"] or
            custody.get("status") != "MATCHED_LOCAL_AND_FRESH_S3_DOWNLOAD" or
            receipt.get("exit_code") != 0 or
            receipt.get("cap_seconds") != runner.CAP_SECONDS or
            len(command) != 5 or
            command[1:4] != [f"construct:83:1:{runner.K}",
                             str(runner.TARGET), "29"] or
            receipt.get("environment_flags") !=
            {"KIC_BASE_BUILD_ONLY": "1",
             "KIC_DUMP_BASE": str(stage / "base_dump.jsonl")} or
            receipt.get("target_sha256") != runner.sha256(runner.TARGET) or
            receipt.get("stdout_sha256") != runner.sha256(stage / "stdout.jsonl") or
            receipt.get("base_dump_sha256") != raw_sha or
            receipt.get("summary") != summary or
            raw_sha != expected_raw or
            summary.get("kind") != "compact_orbit_base_stage" or
            summary.get("n") != group.N or summary.get("a") != 1 or
            summary.get("subgroup_order") != group.R or
            group.point_from_json(summary["generator"]) != group.G or
            summary.get("orbit_columns") != runner.K or
            summary.get("factor_base_points") != 166 * runner.K or
            summary.get("base_hash") != runner.PRIOR_BASE_HASH or
            summary.get("base_hash") !=
            record["factor_base"]["producer_base_hash_blake3"] or
            record["factor_base"]["actual_usable_points"] != 166 * runner.K or
            record["factor_base"]["prefix_k4405_status"] !=
            "PASS_EXACT_PREFIX" or
            record["verification"]["status"] !=
            "PASS_INDEPENDENT_BASE_REPLAY" or
            record["verification"]["receipt_sha256"] != replay_sha or
            replay.get("status") != "PASS_INDEPENDENT_BASE_REPLAY" or
            replay.get("base_dump_sha256") != expected_raw):
        raise ValueError("K5500 current-source dump or prior replay differs")
    return {
        "schema": "n83.fixed31-split-finalist-base-replay/v1",
        "status": "PASS_BYTE_IDENTICAL_TO_PRIOR_INDEPENDENT_REPLAY",
        "source_sha256": runner.SOURCE_SHA256,
        "binary_sha256": receipt["binary_sha256"],
        "usable_points": 166 * runner.K,
        "folded_columns": runner.K,
        "base_hash_blake3": summary["base_hash"],
        "base_dump_sha256": raw_sha,
        "prior_replay_sha256": replay_sha,
        "ordered_k4405_prefix": True,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-stage", type=Path, required=True)
    args = parser.parse_args()
    stage = args.base_stage.resolve(strict=True)
    result = verify(stage)
    output = stage / "byte_identity_replay.json"
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if output.exists() and output.read_text() != blob:
        raise ValueError("K5500 byte-identity replay differs")
    output.write_text(blob)
    print(json.dumps({"status": result["status"],
                      "usable_points": result["usable_points"],
                      "ordered_k4405_prefix": result["ordered_k4405_prefix"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
