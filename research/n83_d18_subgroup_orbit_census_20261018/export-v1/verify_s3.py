#!/usr/bin/env python3
"""Verify a fresh S3 GET against the frozen local N83 export manifest.

Requires blake3==1.0.8. This program reads the saved aws CLI PUT, GET and HEAD
responses; the network operations are performed separately and kept as receipts.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

import blake3


def digest(path: Path) -> dict:
    b3, sha, size = blake3.blake3(), hashlib.sha256(), 0
    with path.open("rb") as stream:
        while block := stream.read(1 << 20):
            b3.update(block)
            sha.update(block)
            size += len(block)
    return {"blake3": b3.hexdigest(), "sha256": sha.hexdigest(), "bytes": size}


def main() -> None:
    if len(sys.argv) != 5:
        raise SystemExit("usage: verify_s3.py EXPORTED_DIR FRESH_GET EVIDENCE_DIR RECEIPT")
    exported, fresh, evidence, receipt = map(Path, sys.argv[1:])
    manifest = json.loads((exported / "manifest.json").read_text())
    put = json.loads((evidence / "s3-put.json").read_text())
    get = json.loads((evidence / "s3-get.json").read_text())
    head = json.loads((evidence / "s3-head.json").read_text())
    local = digest(exported / manifest["object"])
    downloaded = digest(fresh)
    expected = {
        "blake3": manifest["compressed_blake3"],
        "sha256": manifest["compressed_sha256"],
        "bytes": manifest["bytes"],
    }
    key = (
        "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
        "d18-direct-subgroup-v1/a0/" + manifest["object"]
    )
    valid = (
        local == expected
        and downloaded == expected
        and head["ContentLength"] == expected["bytes"]
        and head["ServerSideEncryption"] == "AES256"
        and put["ServerSideEncryption"] == "AES256"
        and get["ServerSideEncryption"] == "AES256"
        and put["ETag"] == head["ETag"] == get["ETag"]
    )
    result = {
        "schema": "n83.d18-s3-custody/v1",
        "status": "PASS" if valid else "FAIL",
        "s3_uri": "s3://crypto-autoresearcher/" + key,
        "source_commit": manifest["source_commit"],
        "curve_id": manifest["curve_id"],
        "local": local,
        "fresh_get": downloaded,
        "manifest_expected": expected,
        "head_content_length": head["ContentLength"],
        "server_side_encryption": head["ServerSideEncryption"],
        "etag": head["ETag"],
    }
    receipt.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"], "bytes": expected["bytes"]}))
    if not valid:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
