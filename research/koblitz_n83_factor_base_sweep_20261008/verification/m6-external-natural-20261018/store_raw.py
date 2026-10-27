#!/usr/bin/env python3
"""Write-once archive of one audited N83 external-SAT arm and fresh S3 GET."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
from pathlib import Path
import subprocess
import zipfile

BUCKET = "crypto-autoresearcher"
PREFIX = ("factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
          "a0/external-natural-20261018/raw")
RAW_NAMES = ("natural-mask0.xor.cnf", "export.stdout", "export.stderr",
             "solver.stdout", "solver.stderr")


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256(path: Path) -> str:
    state = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def aws(*args: str) -> dict:
    completed = subprocess.run(["aws", "s3api", *args, "--output", "json"],
                               check=True, capture_output=True, text=True,
                               timeout=180)
    return json.loads(completed.stdout)


def archive_arm(raw_root: Path, worker: Path, arm: str, output: Path) -> dict:
    require(arm in {"gray_pool", "hash_pool"}, "unknown arm")
    raw = raw_root / arm
    result = json.loads((raw / "result.json").read_text())
    audit = json.loads((raw / "audit.json").read_text())
    require(result.get("arm") == arm and audit.get("arm") == arm
            and result.get("status") == "UNKNOWN_outer_wall_cap"
            and audit.get("status") == "PASS_CUSTODY_CENSORED_ONE_MASK"
            and result.get("selected_best_total_runtime") is None
            and audit.get("selected_best_total_runtime") is None
            and result.get("worker_binary_sha256") == sha256(worker),
            "arm is not independently audited and bound to worker")
    require(set(result["raw_sha256"]) == set(RAW_NAMES), "raw coverage differs")
    for name in RAW_NAMES:
        require(sha256(raw / name) == result["raw_sha256"][name],
                f"raw {name} digest differs")
    files = {f"{arm}/{name}": raw / name for name in (*RAW_NAMES, "result.json", "audit.json")}
    files[f"{arm}/worker"] = worker
    archive_dir = raw_root / "archives"
    archive_dir.mkdir(exist_ok=True)
    staging = archive_dir / f"{arm}.zip"
    with zipfile.ZipFile(staging, "x", compression=zipfile.ZIP_DEFLATED,
                         compresslevel=6) as packed:
        for name, path in sorted(files.items()):
            info = zipfile.ZipInfo(name, date_time=(2020, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            packed.writestr(info, path.read_bytes())
    with zipfile.ZipFile(staging) as packed:
        require(set(packed.namelist()) == set(files) and packed.testzip() is None,
                "raw ZIP member coverage or CRC differs")
    archive_sha = sha256(staging)
    key = f"{PREFIX}/{arm}/{archive_sha}.zip"
    metadata = {"schema": "n83-m6-external-natural-raw-v1",
                "source-commit": result["source_commit"], "arm": arm}
    put = aws("put-object", "--bucket", BUCKET, "--key", key,
              "--body", str(staging), "--if-none-match", "*",
              "--checksum-algorithm", "SHA256",
              "--server-side-encryption", "AES256",
              "--content-type", "application/zip",
              "--metadata", json.dumps(metadata, sort_keys=True))
    head = aws("head-object", "--bucket", BUCKET, "--key", key,
               "--checksum-mode", "ENABLED")
    require(head["ContentLength"] == staging.stat().st_size
            and head["ServerSideEncryption"] == "AES256"
            and head["ContentType"] == "application/zip"
            and head["Metadata"] == metadata
            and base64.b64decode(head["ChecksumSHA256"]) == bytes.fromhex(archive_sha),
            "S3 HEAD differs from immutable archive")
    download = archive_dir / f"{arm}.fresh-get.zip"
    get = aws("get-object", "--bucket", BUCKET, "--key", key, str(download),
              "--checksum-mode", "ENABLED")
    require(sha256(download) == archive_sha
            and download.stat().st_size == staging.stat().st_size,
            "fresh S3 GET differs from exact ZIP")
    with zipfile.ZipFile(download) as packed:
        require(set(packed.namelist()) == set(files) and packed.testzip() is None,
                "fresh S3 ZIP members differ")
    row = {"schema": "n83.m6-external-natural-s3/v1", "status": "PASS_WRITE_ONCE_FRESH_GET",
           "arm": arm, "s3_bucket": BUCKET, "s3_key": key,
           "archive_sha256": archive_sha, "archive_bytes": staging.stat().st_size,
           "archive_members": {name: sha256(path) for name, path in sorted(files.items())},
           "put_etag": put.get("ETag"), "head_etag": head.get("ETag"),
           "get_etag": get.get("ETag"), "head_checksum_sha256": head["ChecksumSHA256"],
           "server_side_encryption": head["ServerSideEncryption"],
           "fresh_get_sha256": sha256(download),
           "source_commit": result["source_commit"],
           "selected_best_total_runtime": None}
    with output.open("x") as stream:
        json.dump(row, stream, indent=2, sort_keys=True)
        stream.write("\n")
    return row


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw-root", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--arm", choices=("gray_pool", "hash_pool"), required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(archive_arm(args.raw_root, args.worker, args.arm, args.output),
                     sort_keys=True))


if __name__ == "__main__":
    main()
