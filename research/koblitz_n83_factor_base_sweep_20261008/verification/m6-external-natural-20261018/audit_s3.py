#!/usr/bin/env python3
"""Fresh independent S3 replay of both N83 external-SAT raw archives."""

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
ARMS = ("gray_pool", "hash_pool")


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def aws(*args: str) -> dict:
    run = subprocess.run(["aws", "s3api", *args, "--output", "json"],
                         capture_output=True, text=True, check=True, timeout=180)
    return json.loads(run.stdout)


def audit_arm(receipts: Path, download_dir: Path, arm: str) -> dict:
    custody = json.loads((receipts / f"{arm}-s3.json").read_text())
    result = json.loads((receipts / f"{arm}-result.json").read_text())
    local_audit = json.loads((receipts / f"{arm}-audit.json").read_text())
    digest = custody["archive_sha256"]
    key = f"{PREFIX}/{arm}/{digest}.zip"
    require(custody["status"] == "PASS_WRITE_ONCE_FRESH_GET"
            and custody["s3_bucket"] == BUCKET and custody["s3_key"] == key
            and custody["arm"] == arm
            and custody["fresh_get_sha256"] == digest
            and local_audit["status"] == "PASS_CUSTODY_CENSORED_ONE_MASK"
            and result["status"] == "UNKNOWN_outer_wall_cap"
            and result["selected_best_total_runtime"] is None,
            "archived S3 receipt or censored local audit differs")
    head = aws("head-object", "--bucket", BUCKET, "--key", key,
               "--checksum-mode", "ENABLED")
    expected_meta = {"schema": "n83-m6-external-natural-raw-v1",
                     "source-commit": result["source_commit"], "arm": arm}
    require(head["ContentLength"] == custody["archive_bytes"]
            and head["ServerSideEncryption"] == "AES256"
            and head["ContentType"] == "application/zip"
            and head["Metadata"] == expected_meta
            and base64.b64decode(head["ChecksumSHA256"]) == bytes.fromhex(digest)
            and head["ETag"] == custody["put_etag"] == custody["head_etag"],
            "fresh S3 HEAD differs from uploaded object")
    destination = download_dir / f"{arm}-independent.zip"
    fetched = aws("get-object", "--bucket", BUCKET, "--key", key,
                  str(destination), "--checksum-mode", "ENABLED")
    require(sha256(destination) == digest
            and destination.stat().st_size == custody["archive_bytes"]
            and fetched["ETag"] == head["ETag"],
            "independent S3 GET differs")
    expected_members = custody["archive_members"]
    with zipfile.ZipFile(destination) as archive:
        require(set(archive.namelist()) == set(expected_members)
                and archive.testzip() is None, "independent ZIP member coverage differs")
        for name, expected in expected_members.items():
            require(hashlib.sha256(archive.read(name)).hexdigest() == expected,
                    f"independent ZIP member {name} differs")
    require(expected_members[f"{arm}/worker"] == result["worker_binary_sha256"]
            and expected_members[f"{arm}/natural-mask0.xor.cnf"]
            == result["raw_sha256"]["natural-mask0.xor.cnf"],
            "worker or exact formula not bound by S3 archive")
    return {"arm": arm, "s3_key": key, "archive_sha256": digest,
            "archive_bytes": custody["archive_bytes"],
            "member_count": len(expected_members),
            "fresh_get_sha256": sha256(destination),
            "head_etag": head["ETag"]}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipts", type=Path, required=True)
    parser.add_argument("--downloads", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.downloads.mkdir(parents=True, exist_ok=False)
    rows = [audit_arm(args.receipts, args.downloads, arm) for arm in ARMS]
    record = {"schema": "n83.m6-external-natural-s3-audit/v1",
              "status": "PASS_TWO_FRESH_INDEPENDENT_GETS",
              "arms": rows, "selected_best_total_runtime": None}
    with args.output.open("x") as stream:
        json.dump(record, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(record, sort_keys=True))


if __name__ == "__main__":
    main()
