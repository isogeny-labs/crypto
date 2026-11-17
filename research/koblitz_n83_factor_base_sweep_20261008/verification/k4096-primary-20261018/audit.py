#!/usr/bin/env python3
"""Independently audit the retained K4096 construction, replay and S3 copy."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import math
from pathlib import Path
import subprocess

import native_stage as frozen


MODULUS = (1 << 83) | (1 << 45) | 7
FIELD_LIMIT = 1 << 83
LINUX_BINARY_SHA256 = "21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed"
STAGE_COMMIT = "a79115bf38e5c335983a35e7744328e46b53adfa"
STAGE_SOURCE = "research/koblitz_n83_factor_base_sweep_20261008/verification/k4096-primary-20261018/native_stage.py"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def square(word: int) -> int:
    value = 0
    for bit in range(83):
        if (word >> bit) & 1:
            value ^= 1 << (2 * bit)
    for bit in range(164, 82, -1):
        if (value >> bit) & 1:
            value ^= MODULUS << (bit - 83)
    return value


def point(value: object) -> tuple[int, int]:
    require(isinstance(value, list) and len(value) == 2
            and all(isinstance(x, str) and x.isdecimal() for x in value),
            "point coordinate encoding differs")
    x, y = map(int, value)
    require(0 <= x < FIELD_LIMIT and 0 <= y < FIELD_LIMIT,
            "point coordinate exceeds the field")
    return x, y


def scan_fresh_object(path: Path) -> dict:
    seen: set[tuple[int, int]] = set()
    with gzip.open(path, "rt") as stream:
        header = json.loads(next(stream))
        require(header["schema"] == "n83.factor-base/v1"
                and header["curve_a"] == 0
                and header["policy"] == "public_x_hash"
                and header["seed"] == 2026100801
                and header["n"] == 83
                and header["orbit_columns"] == 4096
                and header["point_count"] == 679936
                and header["subgroup_order"] == "2417851639230796216685689"
                and header["modulus_low_terms"] == [0, 1, 2, 45]
                and header["closure"] == "signed_frobenius"
                and len(header["representatives"]) == 4096,
                "fresh S3 header differs from the frozen primary base")
        order = int(header["subgroup_order"])
        lam = int(header["frobenius_lambda"])
        count = 0
        for column, rep_value in enumerate(header["representatives"]):
            rep = point(rep_value)
            previous = None
            coefficient = 1
            for phase in range(83):
                for negative in (False, True):
                    try:
                        row = json.loads(next(stream))
                    except StopIteration as error:
                        raise ValueError("fresh S3 object has a missing point row") from error
                    current = point(row["point"])
                    require(row["column"] == column and row["phase"] == phase
                            and row["negative"] is negative
                            and row["coefficient"] == str(
                                (order - coefficient) % order if negative else coefficient),
                            "fresh S3 point row label differs")
                    if negative:
                        require(current == (positive[0], positive[0] ^ positive[1]),
                                "fresh S3 signed point differs")
                    else:
                        positive = current
                        if phase == 0:
                            require(positive == rep,
                                    "fresh S3 representative differs")
                        else:
                            require(positive == (square(previous[0]),
                                                 square(previous[1])),
                                    "fresh S3 Frobenius step differs")
                        previous = positive
                    require(current not in seen, "fresh S3 point is duplicated")
                    seen.add(current)
                    count += 1
                coefficient = coefficient * lam % order
            require((square(previous[0]), square(previous[1])) == rep,
                    "fresh S3 Frobenius orbit does not close")
        require(next(stream, None) is None and count == 679936,
                "fresh S3 object has trailing or missing point rows")
    return {"points": count, "representatives": 4096,
            "unique_points": len(seen),
            "signed_and_frobenius_label_replay": "PASS"}


def check_stage(root: Path, name: str, panel: Path, binary: Path) -> dict:
    stage = root / name
    config, outer = read(stage / "config.json"), read(stage / "outer.json")
    role = "replay" if name == "replay-01" else "upload"
    expected = frozen.inputs(panel, binary, role)
    require(config["schema"] == "n83.k4096-native-stage-config/v1"
            and config["stage"] == outer["stage"] == role
            and config["source_commit"] == frozen.SOURCE_COMMIT
            and config["stage_commit"] == STAGE_COMMIT
            and config["input_sha256"] == expected
            and config["wall_seconds"] == frozen.CAPS[role]
            and config["selected_best_total_runtime"] is None
            and outer["schema"] == "n83.k4096-native-stage-outer/v1"
            and outer["status"] == ("PASS_generic_replay" if role == "replay"
                                    else "PASS_s3_round_trip")
            and outer["exit_code"] == 0
            and outer["timed_out"] is False
            and outer["launch_error"] is None
            and outer["receipt_error"] is None
            and outer["input_changed"] is False
            and outer["config_sha256"] == frozen.sha256(stage / "config.json")
            and outer["stdout_sha256"] == frozen.sha256(stage / "stdout.log")
            and outer["stderr_sha256"] == frozen.sha256(stage / "stderr.log")
            and outer["selected_best_total_runtime"] is None
            and math.isfinite(outer["process_wall_ms"])
            and 0 < outer["process_wall_ms"] < (frozen.CAPS[role] + 15) * 1000
            and frozen.receipt_error(role, panel) is None,
            f"{role} stage receipt differs")
    receipt = panel / ("replay.json" if role == "replay" else "upload-receipt.json")
    require(outer["stage_receipt_sha256"] == frozen.sha256(receipt),
            f"{role} stage receipt hash differs")
    return {"status": outer["status"], "process_wall_ms": outer["process_wall_ms"],
            "receipt_sha256": outer["stage_receipt_sha256"]}


def audit(root: Path, binary: Path) -> dict:
    stage_blob = subprocess.check_output(
        ["git", "show", f"{STAGE_COMMIT}:{STAGE_SOURCE}"],
        cwd=frozen.CHECKOUT)
    require(hashlib.sha256(stage_blob).hexdigest()
            == frozen.sha256(Path(frozen.__file__)),
            "native-stage supervisor differs from its run commit")
    construction = root / "construction-01"
    panel = construction / "object"
    config, outer = read(construction / "config.json"), read(construction / "outer.json")
    frozen.inputs(panel, binary, "replay")
    require(config["schema"] == "n83.v2-object-construction-config/v1"
            and config["source_commit"] == frozen.SOURCE_COMMIT
            and config["columns"] == 4096
            and config["curve_a"] == 0
            and config["policy"] == "public_x_hash"
            and config["seed"] == 2026100801
            and config["wall_seconds"] == 900
            and config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            and config["memory_cgroup_swap_limit_bytes"] == 0
            and config["cpu_limit"] == 1.0
            and config["binary_sha256"] == LINUX_BINARY_SHA256
            and outer["schema"] == "n83.v2-object-construction-outer/v1"
            and outer["status"] == "PASS_construction_only"
            and outer["source_commit"] == config["source_commit"]
            and outer["worker_exit_code"] == 0
            and outer["cleanup_error"] is None
            and outer["launch_error"] is None
            and outer["manifest_error"] is None
            and outer["source_changed"] is False
            and outer["config_sha256"] == frozen.sha256(construction / "config.json")
            and outer["manifest_sha256"] == frozen.MANIFEST_SHA256
            and outer["stdout_sha256"] == frozen.sha256(construction / "stdout.log")
            and outer["stderr_sha256"] == frozen.sha256(construction / "stderr.log")
            and outer["replay_executed"] is False
            and outer["s3_upload_executed"] is False
            and outer["selected_best_total_runtime"] is None
            and math.isfinite(outer["process_wall_ms"])
            and 0 < outer["process_wall_ms"] < 915000,
            "construction source, caps or outer receipt differs")
    live = read(construction / "container-inspect-raw.json")[0]
    host = live["HostConfig"]
    require(live["State"]["Running"] is True
            and live["Image"] == config["container_image_id"]
            and host["Memory"] == 4 * 1024**3
            and host["MemorySwap"] == host["Memory"]
            and host["NanoCpus"] == 1_000_000_000
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is True
            and host["CapDrop"] == ["ALL"]
            and host["PidsLimit"] == 64,
            "live Docker resource inspection differs")
    mounts = {item["Destination"]: item for item in live["Mounts"]}
    require(set(mounts) == {config["repository_root"], "/worker", "/out"}
            and mounts[config["repository_root"]]["RW"] is False
            and mounts["/worker"]["RW"] is False
            and mounts["/out"]["RW"] is True,
            "construction mounts differ")
    remaining = subprocess.check_output(
        ["docker", "ps", "-aq", "--filter", f"name=^{config['container_name']}$"],
        text=True).strip()
    require(not remaining, "construction container remains")
    replay = check_stage(root, "replay-01", panel, binary)
    upload = check_stage(root, "upload-01", panel, binary)
    fresh = read(root / "fresh-s3-receipt.json")
    downloaded = root / "fresh-get.jsonl.gz"
    require(fresh["schema"] == "n83.k4096-fresh-s3-get/v1"
            and fresh["status"] == "PASS_S3_BYTE_IDENTITY"
            and fresh["s3_uri"] == frozen.OBJECT_URI
            and fresh["content_length"] == 20621466
            and fresh["head_etag"] == fresh["get_etag"]
            and fresh["server_side_encryption"] == "AES256"
            and fresh["s3_metadata_blake3"] == frozen.OBJECT_BLAKE3
            and fresh["compressed_sha256"] == frozen.sha256(downloaded)
                == frozen.OBJECT_SHA256
            and fresh["compressed_blake3"] == frozen.b3(downloaded)
                == frozen.OBJECT_BLAKE3
            and fresh["selected_best_total_runtime"] is None,
            "fresh S3 GET receipt or bytes differ")
    structural = scan_fresh_object(downloaded)
    return {"schema": "n83.k4096-primary-independent-audit/v1",
            "status": "PASS_CONSTRUCTION_REPLAY_S3",
            "source_commit": frozen.SOURCE_COMMIT,
            "object_blake3": frozen.OBJECT_BLAKE3,
            "object_sha256": frozen.OBJECT_SHA256,
            "construction_wall_ms": outer["process_wall_ms"],
            "generic_replay": replay,
            "s3_upload": upload,
            "fresh_s3": fresh,
            "independent_object_scan": structural,
            "total_index_calculus_runtime_ms": None,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.root, args.binary)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
