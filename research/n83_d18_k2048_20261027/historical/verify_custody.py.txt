#!/usr/bin/env python3
"""Independently verify the K2048 D18 archive and fresh S3 download."""

import argparse
import hashlib
import importlib.metadata
import json
import subprocess
from pathlib import Path

import blake3


BUCKET = "crypto-autoresearcher"
PREFIX = (
    "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
    "d18-direct-subgroup-k2048-v1/a0"
)
CURVE = "icv1-f2m83-tm6151469093347-debefd74"
POLICY = "direct_subgroup_x_d18_prefix2048"


def hashes(path):
    sha = hashlib.sha256()
    b3 = blake3.blake3()
    size = 0
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            sha.update(chunk)
            b3.update(chunk)
            size += len(chunk)
    return {"sha256": sha.hexdigest(), "blake3": b3.hexdigest(), "bytes": size}


def verify(root):
    if importlib.metadata.version("blake3") != "1.0.8":
        raise ValueError("BLAKE3 verifier version differs")
    manifest_path = root / "base/manifest.json"
    replay_path = root / "base/replay.json"
    manifest = json.loads(manifest_path.read_text())
    replay = json.loads(replay_path.read_text())
    export_guard = json.loads((root / "export-guard.json").read_text())
    replay_guard = json.loads((root / "replay-guard.json").read_text())
    if (
        manifest["schema"] != "n83.direct-subgroup-orbit-export-k2048/v1"
        or manifest["curve_id"] != CURVE
        or manifest["policy"] != POLICY
        or manifest["columns"] != 2048
        or manifest["points"] != 339968
        or manifest["selected_best_total_runtime"] is not None
        or replay["schema"] != "n83.direct-subgroup-orbit-replay-k2048/v1"
        or replay["status"] != "PASS"
        or replay["representatives_checked"] != 2048
        or replay["points_checked"] != 339968
        or replay["point_set_blake3"] != manifest["point_set_blake3"]
        or replay["compressed_sha256"] != manifest["compressed_sha256"]
        or export_guard["status"] != "PASS"
        or replay_guard["status"] != "PASS"
        or export_guard["source_head_sha"] != manifest["source_commit"]
        or replay_guard["source_head_sha"] != manifest["source_commit"]
        or export_guard["binary_sha256"] != replay_guard["binary_sha256"]
    ):
        raise ValueError("export, replay, or guard identity differs")
    object_name = manifest["compressed_blake3"] + ".jsonl.gz"
    object_rel = "objects/" + object_name
    if manifest["object"] != object_rel:
        raise ValueError("object name is not content addressed")
    local_object = root / "base" / object_rel
    fresh_object = root / "fresh" / ("0-" + object_name)
    expected = {
        "sha256": manifest["compressed_sha256"],
        "blake3": manifest["compressed_blake3"],
        "bytes": manifest["bytes"],
    }
    if hashes(local_object) != expected or hashes(fresh_object) != expected:
        raise ValueError("local or fresh object hash differs")
    for index, name in ((1, "manifest.json"), (2, "replay.json")):
        if (root / "fresh" / f"{index}-{name}").read_bytes() != (
            root / "base" / name
        ).read_bytes():
            raise ValueError(f"fresh {name} differs")
    key = f"{PREFIX}/{object_rel}"
    head = subprocess.run(
        ["aws", "s3api", "head-object", "--bucket", BUCKET,
         "--key", key, "--region", "us-west-2", "--output", "json"],
        check=True, capture_output=True, text=True, timeout=30,
    )
    metadata = json.loads(head.stdout)
    if (
        metadata["ContentLength"] != expected["bytes"]
        or metadata.get("ServerSideEncryption") != "AES256"
    ):
        raise ValueError("S3 HEAD length or encryption differs")
    return {
        "schema": "n83.d18-k2048-s3-custody/v1", "status": "PASS",
        "curve_id": CURVE, "policy": POLICY,
        "source_commit": manifest["source_commit"],
        "binary_sha256": export_guard["binary_sha256"],
        "s3_uri": f"s3://{BUCKET}/{key}",
        "object": expected, "point_set_blake3": manifest["point_set_blake3"],
        "columns": 2048, "points": 339968,
        "manifest_sha256": hashes(manifest_path)["sha256"],
        "replay_sha256": hashes(replay_path)["sha256"],
        "head_content_length": metadata["ContentLength"],
        "server_side_encryption": metadata["ServerSideEncryption"],
        "selected_best_total_runtime": None,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError("custody output already exists")
    result = verify(args.run_dir.resolve(strict=True))
    args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
