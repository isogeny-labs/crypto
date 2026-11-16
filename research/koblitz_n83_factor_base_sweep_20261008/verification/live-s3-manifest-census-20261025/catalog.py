#!/usr/bin/env python3
"""Reconcile every saved live N83 base object with exact source manifests."""

import argparse
import base64
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys

from blake3 import blake3


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence-01"
CUSTODY = HERE.parent / "live-s3-full-custody-20261025"
sys.path.insert(0, str(CUSTODY))
from audit_s3 import check as check_custody  # noqa: E402

BUCKET = "crypto-autoresearcher"
PREFIX = "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
PRIMARY = "icv1-f2m83-tm6151469093347-debefd74"
DIRECT_SOURCE_COMMIT = "46c86f47c5530f309453d94ee7246957b03544af"
DIRECT_SHA = {
    "manifest": "2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045",
    "replay": "1f3e5cad62b9deb93cfa255e02e1ae1490faab53b625f2e01f40ea7b2aee7e6d",
    "s3-custody": "6b467e6e30d2af0f641fa503ebccf1f998fb6745b95abf8a6f5174e604d67f63",
}


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def manifest_keys(listing: dict) -> set[str]:
    return {item["Key"] for item in listing["Contents"]
            if item["Key"].endswith("/manifest.json")
            or "/manifests/" in item["Key"]}


def source_row(row: dict, source: str, objects: dict) -> dict:
    uri = row["s3_uri"]
    require(uri.startswith(f"s3://{BUCKET}/{PREFIX}"),
            "manifest object in exact N83 S3 prefix")
    key = uri[len(f"s3://{BUCKET}/"):]
    require(key in objects, f"manifest object exists: {key}")
    obj = objects[key]
    bytes_claim = row.get("bytes", row.get("compressed_bytes"))
    point_set = row.get("point_set_blake3", row.get("projected_point_set_blake3"))
    points = row.get("points", row.get("point_records", row.get("projected_point_count")))
    columns = row.get("columns", row.get("orbit_columns", row.get("signed_columns")))
    require(row["compressed_blake3"] == obj["blake3"]
            and bytes_claim == obj["bytes"]
            and isinstance(point_set, str) and len(point_set) == 64
            and isinstance(points, int) and points > 0
            and isinstance(columns, int) and columns > 0
            and ("a" not in row or row["a"] == (0 if obj["curve_arm"] == "a0" else 1))
            and ("curve_slug" not in row or row["curve_slug"] == PRIMARY)
            and row.get("selected_best_total_runtime") is None,
            f"manifest claims match downloaded content address: {key}")
    return {"key": key, "curve_arm": obj["curve_arm"],
            "family": obj["family"], "compressed_blake3": obj["blake3"],
            "compressed_sha256": obj["sha256"], "compressed_bytes": obj["bytes"],
            "point_set_blake3_manifest_claim": point_set,
            "point_count_manifest_claim": points,
            "orbit_columns_manifest_claim": columns,
            "policy": row.get("policy", row.get("schema")),
            "manifest_source": source,
            "selected_best_total_runtime": None}


def catalog(evidence: Path = EVIDENCE, custody: Path | None = None) -> dict:
    custody = custody or CUSTODY / "evidence-01"
    custody_check = check_custody(custody)
    require(custody_check["status"] == "PASS_SAVED_RECEIPT_INTERNAL_CONSISTENCY"
            and custody_check["archive_copies"] == 80
            and custody_check["unique_compressed_digests"] == 77
            and custody_check["repeated_get_object_count"] == 77,
            "prior two-round compressed-byte custody")
    listing = json.loads((custody / "listing.json").read_bytes())
    observed = json.loads((custody / "replay.json").read_bytes())
    objects = {row["key"]: row for row in observed["rows"]
               if row["kind"] == "object"}
    require(len(objects) == 77, "77 unique current content-addressed keys")
    listed = {item["Key"]: item for item in listing["Contents"]}
    index = json.loads((evidence / "index.json").read_bytes())
    require(len(index) == 23
            and {row["key"] for row in index} == manifest_keys(listing),
            "every current live S3 manifest fetched")
    records = []
    for position, item in enumerate(index):
        require(item["index"] == position and item["key"] in listed,
                "manifest receipt index and listing key")
        raw = (evidence / f"{position:02d}.json").read_bytes()
        provider = item["metadata"]
        checksum = provider.get("ChecksumSHA256")
        require(len(raw) == item["bytes"] == provider["ContentLength"]
                == listed[item["key"]]["Size"]
                and sha(raw) == item["sha256"]
                and (checksum is None or base64.b64decode(checksum).hex() == sha(raw))
                and provider.get("ServerSideEncryption") == "AES256",
                f"fresh manifest GET receipt {item['key']}")
        family = item["key"][len(PREFIX):]
        if "/panels/" in family and family.endswith("/manifest.json"):
            require(blake3(raw).hexdigest() == family.split("/")[-2],
                    "content-addressed panel manifest")
        document = json.loads(raw)
        require(document["selected_best_total_runtime"] is None,
                "no manifest promotes a runtime winner")
        rows = document.get("bases", [document])
        require(rows, "nonempty manifest")
        for row in rows:
            records.append(source_row(row, item["key"], objects))

    for name, expected in DIRECT_SHA.items():
        require(sha((evidence / f"direct-{name}.json").read_bytes()) == expected,
                f"exact prior independent direct-base {name} receipt")
    direct = json.loads((evidence / "direct-manifest.json").read_bytes())
    replay = json.loads((evidence / "direct-replay.json").read_bytes())
    s3 = json.loads((evidence / "direct-s3-custody.json").read_bytes())
    require(direct["curve_id"] == s3["curve_id"] == PRIMARY
            and direct["source_commit"] == s3["source_commit"]
            == "b901026b6b05cd1fe9b909620d6746a5a931520c"
            and replay["status"] == s3["status"] == "PASS"
            and replay["points_checked"] == direct["points"] == 2760082
            and replay["representatives_checked"] == direct["columns"] == 16627
            and replay["point_set_blake3"] == direct["point_set_blake3"]
            and replay["compressed_blake3"] == direct["compressed_blake3"]
            and replay["compressed_sha256"] == direct["compressed_sha256"]
            and s3["fresh_get"] == {
                "blake3": direct["compressed_blake3"],
                "sha256": direct["compressed_sha256"],
                "bytes": direct["bytes"]}
            and s3["server_side_encryption"] == "AES256",
            "prior generic full-row direct-base replay and S3 receipt")
    direct_uri = s3["s3_uri"]
    require(direct_uri.startswith(f"s3://{BUCKET}/{PREFIX}"),
            "direct base exact S3 prefix")
    direct_key = direct_uri[len(f"s3://{BUCKET}/"):]
    require(direct_key in objects, "direct base current S3 object")
    obj = objects[direct_key]
    require(obj["blake3"] == direct["compressed_blake3"]
            and obj["sha256"] == direct["compressed_sha256"]
            and obj["bytes"] == direct["bytes"]
            and obj["curve_arm"] == "a0", "direct base current bytes")
    records.append({"key": direct_key, "curve_arm": "a0",
                    "family": obj["family"],
                    "compressed_blake3": obj["blake3"],
                    "compressed_sha256": obj["sha256"],
                    "compressed_bytes": obj["bytes"],
                    "point_set_blake3_manifest_claim": direct["point_set_blake3"],
                    "point_count_manifest_claim": direct["points"],
                    "orbit_columns_manifest_claim": direct["columns"],
                    "policy": direct["policy"],
                    "manifest_source": f"git:{DIRECT_SOURCE_COMMIT}:export-v1/manifest.json",
                    "independent_full_row_replay": True,
                    "selected_best_total_runtime": None})
    require(len(records) == len(objects) == 77
            and {row["key"] for row in records} == set(objects),
            "every current content-addressed base has one exact manifest")
    counts = Counter(row["curve_arm"] for row in records)
    unique = {arm: len({row["point_set_blake3_manifest_claim"]
                        for row in records if row["curve_arm"] == arm})
              for arm in ("a0", "a1")}
    require(counts == {"a0": 50, "a1": 27}
            and unique == {"a0": 50, "a1": 15},
            "observed archive and declared point-set census")
    return {"schema": "n83.live-s3-manifest-census/v1",
            "status": "PASS_77_BASES_RECONCILED",
            "s3_manifest_gets": len(index),
            "content_addressed_objects": len(objects),
            "curve_arm_object_counts": dict(counts),
            "manifest_declared_unique_point_sets": unique,
            "diagnostic_a1_redundant_archives": counts["a1"] - unique["a1"],
            "direct_source_commit": DIRECT_SOURCE_COMMIT,
            "records": sorted(records, key=lambda row: row["key"]),
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--evidence", type=Path, default=EVIDENCE)
    parser.add_argument("--custody", type=Path)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    result = catalog(args.evidence, args.custody)
    raw = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.out:
        require(not args.out.exists(), "new catalog output path")
        args.out.write_text(raw)
    print(json.dumps({key: value for key, value in result.items()
                      if key != "records"}, indent=2, sort_keys=True))
