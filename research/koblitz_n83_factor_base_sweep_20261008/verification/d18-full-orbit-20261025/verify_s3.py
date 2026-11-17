#!/usr/bin/env python3
"""Independently bind the full-D18 S3 object, manifest and fresh GETs."""

import argparse
import base64
import hashlib
import json
from pathlib import Path

import blake3


PREFIX = ("factor-bases/icv1/etc/"
          "koblitz_n83_factor_base_sweep_20261008/"
          "d18-full-direct-subgroup-v1/a0/")
CURVE = "icv1-f2m83-tm6151469093347-debefd74"
POLICY = "direct_subgroup_x_d18_full"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def digest(path: Path) -> dict:
    b3, sha, size = blake3.blake3(), hashlib.sha256(), 0
    with path.open("rb") as stream:
        while block := stream.read(1 << 20):
            b3.update(block)
            sha.update(block)
            size += len(block)
    return {"blake3": b3.hexdigest(), "sha256": sha.hexdigest(),
            "bytes": size}


def loaded(path: Path) -> dict:
    return json.loads(path.read_bytes())


def verify(exported: Path, fresh: Path, evidence: Path) -> dict:
    manifest_raw = (exported / "manifest.json").read_bytes()
    manifest = json.loads(manifest_raw)
    replay_raw = (exported / "replay.json").read_bytes()
    replay = json.loads(replay_raw)
    fresh_replay = loaded(evidence / "fresh-replay.json")
    put = loaded(evidence / "s3-put.json")
    get = loaded(evidence / "s3-get.json")
    head = loaded(evidence / "s3-head.json")
    s3_manifest_raw = (evidence / "s3-manifest.json").read_bytes()
    s3_manifest = json.loads(s3_manifest_raw)
    s3_manifest_download = (evidence / "s3-manifest-fresh-get.json").read_bytes()
    manifest_put = loaded(evidence / "s3-manifest-put.json")
    manifest_get = loaded(evidence / "s3-manifest-get.json")
    manifest_head = loaded(evidence / "s3-manifest-head.json")
    expected = {"blake3": manifest["compressed_blake3"],
                "sha256": manifest["compressed_sha256"],
                "bytes": manifest["bytes"]}
    local = digest(exported / manifest["object"])
    downloaded = digest(fresh)
    object_key = PREFIX + manifest["object"]
    checksum = base64.b64encode(bytes.fromhex(expected["sha256"])).decode()
    require(manifest["schema"] == "n83.direct-subgroup-orbit-export/v1"
            and manifest["curve_id"] == CURVE
            and manifest["policy"] == POLICY
            and manifest["columns"] == 32643
            and manifest["points"] == 5418738
            and manifest["object"]
            == f"objects/{expected['blake3']}.jsonl.gz"
            and manifest["selected_best_total_runtime"] is None,
            "exact primary full-D18 archive identity")
    require(local == downloaded == expected,
            "fresh S3 object bytes and compressed hashes")
    require(replay["status"] == fresh_replay["status"] == "PASS"
            and replay["points_checked"] == fresh_replay["points_checked"]
            == manifest["points"]
            and replay["point_set_blake3"]
            == fresh_replay["point_set_blake3"]
            == manifest["point_set_blake3"]
            and replay["compressed_sha256"]
            == fresh_replay["compressed_sha256"] == expected["sha256"],
            "local and fresh-object generic group replay")
    require(all(item["ETag"] == head["ETag"]
                and item["ServerSideEncryption"] == "AES256"
                and item["ChecksumSHA256"] == checksum
                for item in (put, get, head))
            and get["ContentLength"] == head["ContentLength"]
            == expected["bytes"]
            and head["Metadata"]["sha256"] == expected["sha256"]
            and head["Metadata"]["curve"] == CURVE
            and head["Metadata"]["policy"] == POLICY,
            "provider object checksum, metadata and encryption")
    manifest_sha = hashlib.sha256(s3_manifest_raw).hexdigest()
    manifest_checksum = base64.b64encode(
        bytes.fromhex(manifest_sha)).decode()
    require(s3_manifest_raw == s3_manifest_download
            and s3_manifest["schema"] == "n83.d18-full-s3-manifest/v1"
            and s3_manifest["status"] == "PASS_FULL_REPLAY_AND_FRESH_S3_CUSTODY"
            and s3_manifest["curve_id"] == CURVE
            and s3_manifest["policy"] == POLICY
            and s3_manifest["object_key"] == object_key
            and s3_manifest["compressed_sha256"] == expected["sha256"]
            and s3_manifest["compressed_blake3"] == expected["blake3"]
            and s3_manifest["compressed_bytes"] == expected["bytes"]
            and s3_manifest["point_set_blake3"]
            == manifest["point_set_blake3"]
            and s3_manifest["producer_manifest_sha256"]
            == hashlib.sha256(manifest_raw).hexdigest()
            and s3_manifest["generic_replay_sha256"]
            == hashlib.sha256(replay_raw).hexdigest()
            and s3_manifest["selected_best_total_runtime"] is None,
            "immutable hash-bound S3 manifest")
    require(all(item["ETag"] == manifest_head["ETag"]
                and item["ServerSideEncryption"] == "AES256"
                and item["ChecksumSHA256"] == manifest_checksum
                for item in (manifest_put, manifest_get, manifest_head))
            and manifest_get["ContentLength"]
            == manifest_head["ContentLength"] == len(s3_manifest_raw),
            "provider manifest checksum and encryption")
    return {"schema": "n83.d18-full-s3-custody-audit/v1",
            "status": "PASS_FULL_OBJECT_AND_MANIFEST",
            "s3_object_uri": "s3://crypto-autoresearcher/" + object_key,
            "s3_manifest_uri": "s3://crypto-autoresearcher/" +
            PREFIX + "manifest.json", "source_commit": manifest["source_commit"],
            "curve_id": CURVE, "policy": POLICY, "columns": 32643,
            "points": 5418738, "local": local, "fresh_get": downloaded,
            "manifest_sha256": manifest_sha,
            "point_set_blake3": manifest["point_set_blake3"],
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("exported", type=Path)
    parser.add_argument("fresh_get", type=Path)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("receipt", type=Path)
    args = parser.parse_args()
    result = verify(args.exported, args.fresh_get, args.evidence)
    args.receipt.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: result[key] for key in
                      ("status", "columns", "points", "manifest_sha256")},
                     sort_keys=True))
