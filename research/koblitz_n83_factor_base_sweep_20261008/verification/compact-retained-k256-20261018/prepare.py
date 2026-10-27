#!/usr/bin/env python3
"""Prepare one exact S3-backed K256 primary panel for the guarded compact probe."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

import blake3

HERE = Path(__file__).resolve().parent
PILOT = HERE.parents[1] / "pilot-01"
BUCKET = "crypto-autoresearcher"
PREFIX = "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0"
OBJECT_HASH = "0b06b75e26cff57a62c28c2502c85f3fc3f1a5027e5620d90969ecae0947b045"
SEED = 2026100801


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def blake3_file(path: Path) -> str:
    digest = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def aws(*args: str) -> dict:
    run = subprocess.run(["aws", "s3api", *args, "--output", "json"],
                         check=True, capture_output=True, text=True, timeout=180)
    return json.loads(run.stdout)


def prepare(output: Path) -> dict:
    manifest = json.loads((PILOT / "manifest.json").read_text())
    replay = json.loads((PILOT / "replay.json").read_text())
    upload = json.loads((PILOT / "upload-receipt.json").read_text())
    rows = [row for row in manifest["bases"] if row.get("a") == 0
            and row.get("columns") == 256 and row.get("policy") == "public_x_hash"
            and row.get("seed") == SEED]
    require(len(rows) == 1, "K256 hash row is ambiguous")
    row = rows[0]
    name = f"objects/{OBJECT_HASH}.jsonl.gz"
    uri = f"s3://{BUCKET}/{PREFIX}/{name}"
    require(row["object"] == name and row["compressed_blake3"] == OBJECT_HASH
            and row["s3_uri"] == uri
            and replay["status"] == "PASS"
            and replay["panel_manifest_blake3"] == blake3_file(PILOT / "manifest.json")
            and upload["status"] == "PASS"
            and upload["manifest_blake3"] == replay["panel_manifest_blake3"],
            "frozen panel receipts do not bind selected S3 object")
    require(sum(item["object"] == name and item["status"] == "PASS"
                for item in replay["checks"]) == 1
            and sum(item["s3_uri"] == uri and item["downloaded_hash_matches"] is True
                    for item in upload["objects"]) == 1,
            "selected base lacks unique replay and S3 round trip")
    output.mkdir(parents=True, exist_ok=False)
    for filename in ("manifest.json", "replay.json", "upload-receipt.json", "probe-corpus.json"):
        shutil.copyfile(PILOT / filename, output / filename)
    object_dir = output / "objects"
    object_dir.mkdir()
    destination = object_dir / f"{OBJECT_HASH}.jsonl.gz"
    key = f"{PREFIX}/{name}"
    head = aws("head-object", "--bucket", BUCKET, "--key", key)
    get = aws("get-object", "--bucket", BUCKET, "--key", key, str(destination))
    require(head["ContentLength"] == destination.stat().st_size
            and get["ContentLength"] == destination.stat().st_size
            and head["ServerSideEncryption"] == "AES256"
            and blake3_file(destination) == OBJECT_HASH,
            "fresh S3 bytes, length or encryption differ")
    record = {"schema": "n83.primary-compact-k256-s3-input/v1",
              "status": "PASS_FRESH_EXACT_S3_INPUT", "policy": "public_x_hash",
              "seed": SEED, "columns": 256, "object": name, "s3_uri": uri,
              "object_bytes": destination.stat().st_size,
              "object_sha256": sha256(destination), "object_blake3": OBJECT_HASH,
              "s3_head_etag": head["ETag"], "s3_get_etag": get["ETag"],
              "panel_files_sha256": {filename: sha256(output / filename) for filename in
                                     ("manifest.json", "replay.json", "upload-receipt.json",
                                      "probe-corpus.json")},
              "selected_best_total_runtime": None}
    with (output / "input.json").open("x") as stream:
        json.dump(record, stream, indent=2, sort_keys=True)
        stream.write("\n")
    return record


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(prepare(args.output), sort_keys=True))


if __name__ == "__main__":
    main()
