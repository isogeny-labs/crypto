#!/usr/bin/env python3
"""Bind fresh S3 downloads to the two audited N83 high-cap provider ZIPs."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


BUCKET = "crypto-autoresearcher"
PREFIX = ("factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
          "a0/hosted-m6-highcap-20261018/provider-artifacts/")
ROLES = ("gray_pool", "hash_pool")


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def audit(root: Path, provider_audit: Path) -> dict:
    bound = read(provider_audit)
    artifacts = read(root / "artifacts.json")
    inventory = read(root / "provider-inventory.json")
    require(bound["schema"] == "n83.hosted-m6-highcap-audit/v1"
            and bound["status"] == "PASS_BOUND_TWO_HOSTED_SOLVER_RECEIPTS"
            and bound["selected_best_total_runtime"] is None
            and bound["conservative_full_charge_seconds"] == 21_600
            and len(bound["arms"]) == 2
            and set(row["arm"] for row in bound["arms"]) == set(ROLES)
            and artifacts["total_count"] == len(artifacts["artifacts"]) == 2,
            "paired provider audit or artifact coverage differs")
    run_id = bound["run_id"]
    provider = {row["arm"]: row for row in bound["arms"]}
    by_name = {row["name"]: row for row in artifacts["artifacts"]}
    require(len(by_name) == 2, "duplicate provider artifact")
    expected_inventory = {}
    rows = []
    for role in ROLES:
        original = (root / f"{role}.zip").read_bytes()
        fresh = (root / f"{role}.s3.zip").read_bytes()
        put = read(root / f"{role}.put.json")
        get = read(root / f"{role}.get.json")
        digest = hashlib.sha256(original).hexdigest()
        etag = '"' + hashlib.md5(original).hexdigest() + '"'
        key = PREFIX + digest + ".zip"
        name = f"n83-hosted-m6-highcap-{role}-{run_id}-1"
        meta = by_name[name]
        require(digest == provider[role]["artifact_zip_sha256"]
                and meta["id"] == provider[role]["artifact_id"]
                and meta["digest"] == "sha256:" + digest
                and meta["size_in_bytes"] == len(original)
                and fresh == original
                and put["ETag"] == get["ETag"] == etag
                and put["ServerSideEncryption"] == "AES256"
                and get["ServerSideEncryption"] == "AES256"
                and put["ChecksumType"] == "FULL_OBJECT"
                and get["ContentLength"] == len(original)
                and get["ContentType"] == "application/zip"
                and get["Metadata"] == {
                    "run-id": str(run_id), "arm": role,
                    "head-sha": bound["head_sha"],
                }, f"{role} S3 fresh download or provider binding differs")
        expected_inventory[key] = (len(original), etag)
        rows.append({"arm": role, "artifact_id": meta["id"],
                     "artifact_bytes": len(original),
                     "artifact_sha256": digest,
                     "s3_uri": "s3://" + BUCKET + "/" + key,
                     "etag": etag})
    objects = inventory.get("Contents", [])
    require(inventory.get("Prefix") == PREFIX
            and not inventory.get("IsTruncated", False)
            and "NextContinuationToken" not in inventory
            and len(objects) == 2
            and {row["Key"]: (row["Size"], row["ETag"])
                 for row in objects} == expected_inventory
            and all(row["ChecksumAlgorithm"] == ["SHA256"]
                    and row["ChecksumType"] == "FULL_OBJECT"
                    for row in objects), "S3 provider prefix inventory differs")
    return {"schema": "n83.hosted-m6-highcap-s3-audit/v1",
            "status": "PASS_TWO_FRESH_S3_PROVIDER_ROUNDTRIPS",
            "run_id": run_id, "head_sha": bound["head_sha"],
            "arms": rows, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--provider-audit", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.root, args.provider_audit)
    serialized = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output.exists():
        require(args.output.read_text() == serialized,
                "sealed S3 audit output differs")
    else:
        args.output.write_text(serialized)
    print(json.dumps({"status": result["status"], "run_id": result["run_id"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
