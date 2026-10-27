#!/usr/bin/env python3
"""Supervise the exact K4096 generic replay and conditional S3 upload."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import time

import blake3


HERE = Path(__file__).resolve().parent
CHECKOUT = HERE.parents[3]
SOURCE = "examples/koblitz_n83_factor_base_export.rs"
SOURCE_COMMIT = "f61b33c1b1bb98efa22177a615270c160859a605"
BINARY_SHA256 = "5881dc22b3367b6076ef5c3d452eb270bfa3c4013ec63782a0276b4670cdc094"
MANIFEST_SHA256 = "4557a25e4eed961025da19caab6175e601fa54d07193744384385837579bd961"
OBJECT_BLAKE3 = "a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779"
OBJECT_SHA256 = "ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad"
OBJECT_NAME = f"objects/{OBJECT_BLAKE3}.jsonl.gz"
OBJECT_URI = ("s3://crypto-autoresearcher/factor-bases/icv1/etc/"
              "koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/" + OBJECT_NAME)
CAPS = {"replay": 1800, "upload": 600}


def require(value: bool, reason: str) -> None:
    if not value:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    state = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def b3(path: Path) -> str:
    state = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def write_new(path: Path, value: dict) -> None:
    with path.open("x") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")


def inputs(panel: Path, binary: Path, stage: str) -> dict:
    manifest_path = panel / "manifest.json"
    object_path = panel / OBJECT_NAME
    require(sha256(manifest_path) == MANIFEST_SHA256,
            "K4096 manifest differs from the frozen construction")
    manifest = json.loads(manifest_path.read_text())
    require(manifest["schema"] == "n83.factor-base-panel/v2-size-frontier"
            and manifest["status"] == "completed_factor_base_object"
            and manifest["source_commit"] == SOURCE_COMMIT
            and manifest["completed_base_count"] == 1
            and len(manifest["bases"]) == 1
            and manifest["selected_best_total_runtime"] is None,
            "K4096 manifest selection differs")
    row = manifest["bases"][0]
    require((row["a"], row["policy"], row["columns"], row["seed"],
             row["points"], row["object"], row["s3_uri"], row["bytes"])
            == (0, "public_x_hash", 4096, 2026100801, 679936,
                OBJECT_NAME, OBJECT_URI, 20621466)
            and row["compressed_blake3"] == OBJECT_BLAKE3
            and row["runtime_rank_eligible"] is False
            and row["total_index_calculus_runtime_ms"] is None,
            "K4096 row differs from the frozen object")
    require(object_path.stat().st_size == row["bytes"]
            and sha256(object_path) == OBJECT_SHA256
            and b3(object_path) == OBJECT_BLAKE3,
            "K4096 compressed object differs")
    require(sha256(binary) == BINARY_SHA256, "native exporter binary differs")
    source = CHECKOUT / SOURCE
    committed = subprocess.check_output(
        ["git", "show", f"{SOURCE_COMMIT}:{SOURCE}"], cwd=CHECKOUT)
    require(blake3.blake3(committed).hexdigest() == b3(source)
            == manifest["source_blake3"],
            "compiled exporter source or Git blob differs")
    result = {"manifest_sha256": MANIFEST_SHA256,
              "object_sha256": OBJECT_SHA256,
              "object_blake3": OBJECT_BLAKE3,
              "binary_sha256": BINARY_SHA256,
              "source_blake3": manifest["source_blake3"]}
    if stage == "upload":
        replay_path = panel / "replay.json"
        replay = json.loads(replay_path.read_text())
        require(replay["schema"] == "n83.factor-base-replay/v2-size-frontier"
                and replay["status"] == "PASS"
                and replay["panel_manifest_blake3"] == b3(manifest_path)
                and replay["checks"] == [{"object": OBJECT_NAME,
                       "points_checked": 679936,
                       "representatives_checked": 4096, "status": "PASS"}]
                and replay["best_total_runtime"] is None,
                "generic replay receipt does not admit publication")
        result["replay_sha256"] = sha256(replay_path)
    return result


def receipt_error(stage: str, panel: Path) -> str | None:
    path = panel / ("replay.json" if stage == "replay" else "upload-receipt.json")
    if not path.is_file():
        return "stage receipt missing"
    try:
        value = json.loads(path.read_text())
        if stage == "replay":
            valid = (value["schema"] == "n83.factor-base-replay/v2-size-frontier"
                     and value["status"] == "PASS"
                     and value["panel_manifest_blake3"] == b3(panel / "manifest.json")
                     and value["checks"] == [{"object": OBJECT_NAME,
                           "points_checked": 679936,
                           "representatives_checked": 4096, "status": "PASS"}]
                     and value["best_total_runtime"] is None)
        else:
            valid = (value["schema"] == "n83.factor-base-storage/v2-size-frontier"
                     and value["status"] == "PASS"
                     and value["bucket"] == "crypto-autoresearcher"
                     and value["manifest_blake3"] == b3(panel / "manifest.json")
                     and value["objects"] == [{"s3_uri": OBJECT_URI,
                           "compressed_blake3": OBJECT_BLAKE3,
                           "downloaded_hash_matches": True}])
    except (KeyError, TypeError, ValueError, OSError, json.JSONDecodeError):
        return "stage receipt malformed"
    return None if valid else "stage receipt content differs"


def run(stage: str, panel: Path, binary: Path, output: Path) -> dict:
    panel, binary = panel.resolve(strict=True), binary.resolve(strict=True)
    require(output.resolve() != panel and not output.resolve().is_relative_to(panel),
            "stage output must be outside the factor-base panel")
    before = inputs(panel, binary, stage)
    receipt_name = "replay.json" if stage == "replay" else "upload-receipt.json"
    require(not (panel / receipt_name).exists(), "stage receipt already exists")
    require(not (panel / ("replay-cap.json" if stage == "replay" else "upload-cap.json")).exists(),
            "stage cap already exists")
    clean = subprocess.check_output(
        ["git", "status", "--porcelain", "--untracked-files=normal"],
        cwd=CHECKOUT, text=True).strip()
    require(not clean, "source checkout must be clean for native stage")
    stage_commit = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=CHECKOUT, text=True).strip()
    output.mkdir(parents=True, exist_ok=False)
    command = ([str(binary), "v2-replay-one", str(panel), str(CAPS[stage])]
               if stage == "replay" else [str(binary), "upload", str(panel)])
    config = {"schema": "n83.k4096-native-stage-config/v1", "stage": stage,
              "source_commit": SOURCE_COMMIT, "stage_commit": stage_commit,
              "panel": str(panel), "command": command,
              "wall_seconds": CAPS[stage], "input_sha256": before,
              "selected_best_total_runtime": None}
    write_new(output / "config.json", config)
    started = time.monotonic()
    timed_out = False
    exit_code = None
    launch_error = None
    with (output / "stdout.log").open("xb") as stdout, (output / "stderr.log").open("xb") as stderr:
        try:
            child = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                     start_new_session=True)
            try:
                exit_code = child.wait(timeout=CAPS[stage])
            except subprocess.TimeoutExpired:
                timed_out = True
                os.killpg(child.pid, signal.SIGKILL)
                exit_code = child.wait(timeout=15)
        except (OSError, subprocess.SubprocessError) as error:
            launch_error = f"{type(error).__name__}: {error}"
    try:
        after = inputs(panel, binary, stage)
        input_changed = after != before
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError):
        input_changed = True
    receipt_problem = receipt_error(stage, panel)
    cap_exists = (panel / "replay-cap.json").exists() if stage == "replay" else False
    if input_changed:
        status = "PRODUCER_FAILURE_input_changed"
    elif timed_out or exit_code == 124 and cap_exists:
        status = "UNKNOWN_wall_cap"
    elif launch_error or exit_code != 0:
        status = "PRODUCER_FAILURE_stage_exit"
    elif receipt_problem or cap_exists:
        status = "PRODUCER_FAILURE_receipt"
    else:
        status = "PASS_generic_replay" if stage == "replay" else "PASS_s3_round_trip"
    outer = {"schema": "n83.k4096-native-stage-outer/v1", "stage": stage,
             "status": status, "exit_code": exit_code, "timed_out": timed_out,
             "launch_error": launch_error, "receipt_error": receipt_problem,
             "input_changed": input_changed, "config_sha256": sha256(output / "config.json"),
             "stdout_sha256": sha256(output / "stdout.log"),
             "stderr_sha256": sha256(output / "stderr.log"),
             "stage_receipt_sha256": sha256(panel / receipt_name)
                 if (panel / receipt_name).is_file() else None,
             "process_wall_ms": (time.monotonic() - started) * 1000,
             "selected_best_total_runtime": None}
    require(math.isfinite(outer["process_wall_ms"]), "stage wall is not finite")
    write_new(output / "outer.json", outer)
    return outer


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stage", choices=CAPS, required=True)
    parser.add_argument("--panel", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run(args.stage, args.panel, args.binary, args.output)
    print(json.dumps(result, sort_keys=True))
    return 1 if result["status"].startswith("PRODUCER_FAILURE") else 0


if __name__ == "__main__":
    raise SystemExit(main())
