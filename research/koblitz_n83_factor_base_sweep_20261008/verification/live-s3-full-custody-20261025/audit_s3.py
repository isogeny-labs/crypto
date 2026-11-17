#!/usr/bin/env python3
"""Fresh S3 GET and content-address replay for every current N83 base archive."""

import argparse
import base64
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import time

from blake3 import blake3


BUCKET = "crypto-autoresearcher"
PREFIX = "factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
OBJECT = re.compile(r"(?P<family>.+)/objects/(?P<digest>[0-9a-f]{64})\.jsonl\.gz$")
CACHE = re.compile(r"(?P<family>.+)/inputs/archives/s3-(?P<digest>[0-9a-f]{64})\.jsonl\.gz$")


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def classify(key: str) -> tuple[str, str, str] | None:
    if not key.startswith(PREFIX):
        return None
    relative = key[len(PREFIX):]
    match = OBJECT.fullmatch(relative)
    kind = "object"
    if match is None:
        match = CACHE.fullmatch(relative)
        kind = "cached_input"
    if match is None:
        return None
    family = match.group("family")
    curve_arm = ("a1" if family == "a1" else "a0" if family == "a0"
                 or "/a0" in family or family == "linux-xor-panel-20261025"
                 else "unknown")
    return kind, family, curve_arm


def expected_digest(key: str) -> str:
    relative = key[len(PREFIX):]
    match = OBJECT.fullmatch(relative) or CACHE.fullmatch(relative)
    require(match is not None, f"unrecognized archive key: {key}")
    return match.group("digest")


def aws_json(args: list[str]) -> dict:
    result = subprocess.run(["aws", "s3api", *args, "--output", "json"],
                            capture_output=True, text=True, timeout=180,
                            check=True)
    return json.loads(result.stdout)


def get_one(index: int, listed: dict) -> dict:
    key = listed["Key"]
    kind, family, curve_arm = classify(key) or (None, None, None)
    expected = expected_digest(key)
    with tempfile.TemporaryDirectory(prefix=f"n83-s3-{index}-") as temporary:
        path = Path(temporary) / "archive.jsonl.gz"
        try:
            provider = aws_json(["get-object", "--bucket", BUCKET,
                                 "--key", key, str(path)])
            compressed = blake3()
            content_sha = hashlib.sha256()
            size = 0
            with path.open("rb") as stream:
                for block in iter(lambda: stream.read(1 << 20), b""):
                    compressed.update(block)
                    content_sha.update(block)
                    size += len(block)
            digest = compressed.hexdigest()
            sha256 = content_sha.hexdigest()
            checksum = provider.get("ChecksumSHA256")
            checksum_ok = (checksum is None
                           or base64.b64decode(checksum).hex() == sha256)
            metadata_digest = provider.get("Metadata", {}).get("blake3")
            passed = (size == listed["Size"] == provider["ContentLength"]
                      and digest == expected
                      and (metadata_digest is None
                           or metadata_digest == digest)
                      and checksum_ok
                      and provider.get("ServerSideEncryption") == "AES256"
                      and curve_arm != "unknown")
            return {"key": key, "kind": kind, "family": family,
                    "curve_arm": curve_arm, "bytes": size,
                    "listed_bytes": listed["Size"],
                    "s3_content_length": provider["ContentLength"],
                    "blake3": digest, "sha256": sha256,
                    "s3_metadata_blake3": metadata_digest,
                    "s3_checksum_sha256": checksum,
                    "sse": provider.get("ServerSideEncryption"),
                    "status": "PASS_CONTENT_ADDRESS_AND_S3" if passed
                    else "MISMATCH"}
        except Exception as error:
            return {"key": key, "kind": kind, "family": family,
                    "curve_arm": curve_arm,
                    "status": "GET_OR_REPLAY_ERROR",
                    "error_type": type(error).__name__,
                    "error": str(error)[:300]}


def online(out: Path, workers: int = 4) -> dict:
    out.mkdir(parents=True)
    begin = time.monotonic()
    listing = aws_json(["list-objects-v2", "--bucket", BUCKET,
                        "--prefix", PREFIX])
    listed = listing.get("Contents", [])
    require(listing["Prefix"] == PREFIX and not listing.get("IsTruncated"),
            "complete exact-prefix listing")
    archives = [item for item in listed if classify(item["Key"]) is not None]
    unknown_gzip = [item["Key"] for item in listed
                    if item["Key"].endswith(".jsonl.gz")
                    and classify(item["Key"]) is None]
    require(not unknown_gzip, "all JSONL-gzip archives classified")
    with ThreadPoolExecutor(max_workers=workers) as executor:
        rows = list(executor.map(lambda item: get_one(*item),
                                 enumerate(archives)))
    listing_raw = (json.dumps(listing, indent=2, sort_keys=True) + "\n").encode()
    (out / "listing.json").write_bytes(listing_raw)
    receipt = {"schema": "n83.live-s3-factor-base-replay/v2",
               "bucket": BUCKET, "prefix": PREFIX,
               "listing_sha256": sha(listing_raw),
               "listed_object_count": len(listed),
               "selected_archive_copy_count": len(rows),
               "unique_compressed_digest_count":
               len({row.get("blake3") for row in rows
                    if row["status"] == "PASS_CONTENT_ADDRESS_AND_S3"}),
               "rows": rows,
               "elapsed_wall_seconds_L0": time.monotonic() - begin,
               "selected_best_total_runtime": None}
    (out / "replay.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    require(all(row["status"] == "PASS_CONTENT_ADDRESS_AND_S3"
                for row in rows), "all current S3 factor-base archive copies replayed")
    return receipt


def check(out: Path) -> dict:
    listing_raw = (out / "listing.json").read_bytes()
    listing = json.loads(listing_raw)
    receipt = json.loads((out / "replay.json").read_bytes())
    listed = listing.get("Contents", [])
    selected = [item for item in listed if classify(item["Key"]) is not None]
    require(receipt["schema"] == "n83.live-s3-factor-base-replay/v2"
            and receipt["bucket"] == BUCKET and receipt["prefix"] == PREFIX
            and receipt["listing_sha256"] == sha(listing_raw)
            and receipt["listed_object_count"] == len(listed)
            and receipt["selected_archive_copy_count"] == len(selected)
            and receipt["selected_best_total_runtime"] is None
            and len(receipt["rows"]) == len(selected),
            "saved listing and receipt identity")
    require(not any(item["Key"].endswith(".jsonl.gz")
                    and classify(item["Key"]) is None for item in listed),
            "no unclassified archive")
    by_digest: dict[str, set[tuple[int, str]]] = {}
    for source, row in zip(selected, receipt["rows"]):
        kind, family, curve_arm = classify(source["Key"])
        require(row["key"] == source["Key"]
                and row["kind"] == kind and row["family"] == family
                and row["curve_arm"] == curve_arm
                and row["status"] == "PASS_CONTENT_ADDRESS_AND_S3"
                and row["blake3"] == expected_digest(row["key"])
                and row["bytes"] == row["listed_bytes"]
                == row["s3_content_length"] == source["Size"]
                and row["sse"] == "AES256"
                and (row["s3_metadata_blake3"] is None
                     or row["s3_metadata_blake3"] == row["blake3"])
                and (row["s3_checksum_sha256"] is None
                     or base64.b64decode(row["s3_checksum_sha256"]).hex()
                     == row["sha256"]),
                f"saved receipt row {row['key']}")
        by_digest.setdefault(row["blake3"], set()).add(
            (row["bytes"], row["sha256"]))
    require(all(len(values) == 1 for values in by_digest.values())
            and len(by_digest) == receipt["unique_compressed_digest_count"],
            "duplicate archived bytes agree")
    repeat_count = repeat(out) if (out / "first-pass.json").exists() else None
    return {"schema": "n83.live-s3-factor-base-receipt-check/v1",
            "status": "PASS_SAVED_RECEIPT_INTERNAL_CONSISTENCY",
            "listed_object_count": len(listed),
            "archive_copies": len(selected),
            "unique_compressed_digests": len(by_digest),
            "repeated_get_object_count": repeat_count,
            "groups": dict(sorted(Counter(row["family"] for row in
                                          receipt["rows"]).items())),
            "selected_best_total_runtime": None}


def repeat(out: Path) -> int:
    first_raw = (out / "first-pass.json").read_bytes()
    first_listing_raw = (out / "first-listing.json").read_bytes()
    second_raw = (out / "replay.json").read_bytes()
    first = json.loads(first_raw)
    first_listing = json.loads(first_listing_raw)
    second = json.loads(second_raw)
    comparison = json.loads((out / "repeat-comparison.json").read_bytes())
    require(first["schema"] == "n83.live-s3-factor-base-replay/v1"
            and first["listing_sha256"] == sha(first_listing_raw)
            and first["selected_best_total_runtime"] is None
            and comparison["schema"] == "n83.live-s3-factor-base-repeat/v1"
            and comparison["status"] == "PASS_77_REPEAT_GET_BYTES"
            and comparison["first_pass_receipt_sha256"] == sha(first_raw)
            and comparison["first_pass_listing_sha256"] == sha(first_listing_raw)
            and comparison["second_pass_receipt_sha256"] == sha(second_raw)
            and comparison["selected_best_total_runtime"] is None,
            "two saved GET rounds and comparison digests")
    first_listed = {item["Key"]: item["Size"]
                    for item in first_listing["Contents"]
                    if classify(item["Key"]) is not None
                    and classify(item["Key"])[0] == "object"}
    earlier = {}
    for row in first["rows"]:
        key = row["key"]
        require(row["status"] == "PASS_CONTENT_ADDRESS_AND_S3"
                and classify(key) is not None
                and classify(key)[0] == "object"
                and row["blake3"] == expected_digest(key)
                and row["bytes"] == row["listed_bytes"]
                == row["s3_content_length"] == first_listed[key]
                and row["sse"] == "AES256", "first GET content receipt")
        earlier[key] = (row["bytes"], row["blake3"], row["sha256"])
    later = {row["key"]: (row["bytes"], row["blake3"], row["sha256"])
             for row in second["rows"] if row["kind"] == "object"}
    require(len(earlier) == len(later) == comparison["matching_object_keys"]
            and set(first_listed) == set(earlier)
            and earlier == later
            and comparison["cached_input_copies_in_second_pass"]
            == len(second["rows"]) - len(later),
            "same content bytes in both fresh GET rounds")
    return len(earlier)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    actions = parser.add_mutually_exclusive_group(required=True)
    actions.add_argument("--online", action="store_true")
    actions.add_argument("--check", action="store_true")
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()
    require(1 <= args.workers <= 8, "bounded S3 GET concurrency")
    print(json.dumps(online(args.out, args.workers) if args.online
                     else check(args.out), indent=2, sort_keys=True))
