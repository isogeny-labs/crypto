#!/usr/bin/env python3
"""Independently audit four provider bases against fresh S3 GETs."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import zipfile

import blake3

HERE = Path(__file__).resolve().parent


def insist(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def member_digest(archive: zipfile.ZipFile, name: str) -> tuple[int, str]:
    digest = hashlib.sha256()
    length = 0
    with archive.open(name) as source:
        for chunk in iter(lambda: source.read(1 << 20), b""):
            length += len(chunk)
            digest.update(chunk)
    return length, digest.hexdigest()


def deterministic_gzip(raw: bytes) -> bytes:
    output = io.BytesIO()
    with gzip.GzipFile(filename="", mode="wb", fileobj=output,
                       mtime=0, compresslevel=9) as stream:
        stream.write(raw)
    return output.getvalue()


def audit(provider_root: Path, downloads: Path, build_receipt: Path) -> dict:
    plan_bytes = (HERE / "plan.json").read_bytes()
    plan = json.loads(plan_bytes)
    build = json.loads(build_receipt.read_text())
    insist(plan["schema"] == "n83.crossfamily-s3-custody-plan/v1"
           and build["schema"] == "n83.crossfamily-s3-custody-build/v1"
           and build["plan_sha256"] == hashlib.sha256(plan_bytes).hexdigest()
           and build["source_run_id"] == plan["source_run_id"]
           and build["source_commit"] == plan["source_commit"]
           and build["status"] == "PASS_FOUR_VERIFIED_PROVIDER_BASES"
           and build["selected_best_total_runtime"] is None
           and plan["selected_best_total_runtime"] is None,
           "plan or build receipt differs")
    by_role = {row["role"]: row for row in build["objects"]}
    insist(len(by_role) == len(build["objects"]) == len(plan["arms"]) == 4,
           "object roles differ")
    archives = {}
    try:
        for source in plan["source_artifacts"]:
            number = source["stratum"]
            path = provider_root / f"stratum{number}.zip"
            insist(path.stat().st_size == source["zip_bytes"]
                   and sha256_file(path) == source["zip_sha256"],
                   f"provider archive differs: {number}")
            archive = zipfile.ZipFile(path)
            names = archive.namelist()
            insist(len(names) == len(set(names)) and all(
                not Path(name).is_absolute() and ".." not in Path(name).parts
                for name in names), f"unsafe archive members: {number}")
            archives[number] = archive
        insist(set(archives) == {0, 1, 2, 3}, "stratum coverage differs")
        summaries = []
        for arm in plan["arms"]:
            role = arm["role"]
            receipt = by_role[role]
            insist(all(receipt[key] == arm[key] for key in
                       ("candidate_id", "base_hash", "raw_sha256", "raw_bytes",
                        "storage_mode")), f"build identity differs: {role}")
            copies = []
            for stratum in range(4):
                for offset in range(plan["copies_per_stratum"]):
                    point = stratum * plan["copies_per_stratum"] + offset
                    name = (f"results/points/p{point:02d}/runs/"
                            f"{role}/base_dump.jsonl")
                    size, digest = member_digest(archives[stratum], name)
                    insist(size == arm["raw_bytes"]
                           and digest == arm["raw_sha256"],
                           f"provider copy differs: {stratum}:{point}:{role}")
                    copies.append({"stratum": stratum, "point": point,
                                   "member": name, "sha256": digest})
            insist(len(copies) == plan["expected_copies_per_arm"]
                   and receipt["provider_copies"] == copies,
                   f"provider copy coverage differs: {role}")
            packed_hash = receipt["compressed_blake3"]
            if arm["storage_mode"] == "reuse":
                expected = arm["expected_object"]
                insist(all(receipt[key] == expected[key] for key in
                           ("s3_uri", "compressed_sha256", "compressed_blake3",
                            "compressed_bytes", "etag_expected")),
                       f"reused object identity differs: {role}")
                uri = expected["s3_uri"]
            else:
                insist(arm["storage_mode"] == "upload", "invalid storage mode")
                uri = ("s3://" + plan["destination_bucket"] + "/" +
                       plan["destination_prefix"] + packed_hash + ".jsonl.gz")
                insist(receipt["s3_uri"] == uri,
                       f"content-addressed URI differs: {role}")
            packed_path = downloads / f"{packed_hash}.jsonl.gz"
            packed = packed_path.read_bytes()
            get = json.loads((downloads / f"{packed_hash}.get.json").read_text())
            insist(len(packed) == receipt["compressed_bytes"]
                   and hashlib.sha256(packed).hexdigest() ==
                   receipt["compressed_sha256"]
                   and blake3.blake3(packed).hexdigest() == packed_hash
                   and hashlib.md5(packed).hexdigest() ==
                   receipt["etag_expected"].strip('"')
                   and get["ETag"] == receipt["etag_expected"]
                   and get["ContentLength"] == len(packed)
                   and get["ContentType"] == "application/gzip"
                   and get["Metadata"] == {
                       "role": arm["s3_metadata_role"],
                       "base-hash": arm["base_hash"],
                       "raw-sha256": arm["raw_sha256"],
                   }, f"S3 GET differs: {role}")
            raw = gzip.decompress(packed)
            insist(len(raw) == arm["raw_bytes"]
                   and hashlib.sha256(raw).hexdigest() == arm["raw_sha256"]
                   and deterministic_gzip(raw) == packed,
                   f"raw base or deterministic compression differs: {role}")
            row = json.loads(raw)
            curve = plan["curve"]
            for key in ("a", "n", "field_modulus_low_terms", "subgroup_order",
                        "kind"):
                insist(row[key] == curve[key], f"curve field differs: {role}:{key}")
            insist(row["orbit_columns"] == arm["orbit_columns"]
                   and row["base_hash"] == arm["base_hash"]
                   and row["factor_base_points"] == arm["point_count"],
                   f"base metadata differs: {role}")
            coordinates = row["factor_base_point_coordinates"]
            insist(len(coordinates) == arm["point_count"]
                   and len(row["factor_base_point_labels"]) == len(coordinates)
                   and len(row["factor_base_representatives"]) ==
                   arm["orbit_columns"]
                   and len({tuple(point) for point in coordinates}) ==
                   len(coordinates)
                   and all(isinstance(point, list) and len(point) == 2
                           and all(isinstance(value, str) and value.isdecimal()
                                   and int(value) < (1 << curve["n"])
                                   for value in point)
                           for point in coordinates),
                   f"point array differs: {role}")
            summaries.append({"role": role, "s3_uri": uri,
                              "storage_mode": arm["storage_mode"],
                              "compressed_sha256": receipt["compressed_sha256"],
                              "compressed_blake3": packed_hash,
                              "raw_sha256": arm["raw_sha256"],
                              "point_count": len(coordinates),
                              "provider_copy_count": len(copies),
                              "etag": get["ETag"]})
        return {"schema": "n83.crossfamily-s3-custody-audit/v1",
                "status": "PASS_FOUR_FRESH_S3_ROUNDTRIPS",
                "plan_sha256": hashlib.sha256(plan_bytes).hexdigest(),
                "source_run_id": plan["source_run_id"],
                "objects": summaries, "selected_best_total_runtime": None}
    finally:
        for archive in archives.values():
            archive.close()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--provider-root", type=Path, required=True)
    parser.add_argument("--downloads", type=Path, required=True)
    parser.add_argument("--build-receipt", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.provider_root, args.downloads,
                           args.build_receipt), sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
