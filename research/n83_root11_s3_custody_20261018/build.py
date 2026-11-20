#!/usr/bin/env python3
"""Bind repeated provider factor-base dumps and build immutable S3 objects."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import zipfile

import blake3

HERE = Path(__file__).resolve().parent
PLAN = HERE / "plan.json"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sha256_member(archive: zipfile.ZipFile, member: str) -> str:
    digest = hashlib.sha256()
    with archive.open(member) as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def check_base(raw: bytes, arm: dict, curve: dict) -> None:
    row = json.loads(raw)
    expected = {
        "a": curve["a"], "n": curve["n"],
        "field_modulus_low_terms": curve["field_modulus_low_terms"],
        "subgroup_order": curve["subgroup_order"], "kind": curve["kind"],
        "orbit_columns": arm["orbit_columns"], "base_hash": arm["base_hash"],
        "factor_base_points": arm["point_count"],
        "field_value_encoding": "base-10 strings for u128 field values",
    }
    for key, value in expected.items():
        require(type(row.get(key)) is type(value) and row[key] == value,
                f"factor-base metadata differs: {arm['role']} {key}")
    points = row.get("factor_base_point_coordinates")
    labels = row.get("factor_base_point_labels")
    representatives = row.get("factor_base_representatives")
    require(isinstance(points, list) and len(points) == arm["point_count"]
            and isinstance(labels, list) and len(labels) == len(points)
            and isinstance(representatives, list)
            and len(representatives) == arm["orbit_columns"],
            f"factor-base array lengths differ: {arm['role']}")
    bound = 1 << curve["n"]
    unique = set()
    for point in points:
        require(isinstance(point, list) and len(point) == 2
                and all(isinstance(value, str) and value.isdecimal()
                        and 0 <= int(value) < bound for value in point),
                f"factor-base coordinate differs: {arm['role']}")
        unique.add(tuple(point))
    require(len(unique) == len(points), f"duplicate factor-base point: {arm['role']}")


def build(provider_root: Path, output: Path) -> dict:
    plan_bytes = PLAN.read_bytes()
    plan = json.loads(plan_bytes)
    require(plan["schema"] == "n83.root11-s3-custody-plan/v1"
            and plan["selected_best_total_runtime"] is None
            and len(plan["source_artifacts"]) == 4
            and len(plan["arms"]) == 2,
            "frozen custody plan differs")
    output.mkdir(parents=True, exist_ok=False)
    archives = []
    for source in plan["source_artifacts"]:
        path = provider_root / f"stratum{source['stratum']}.zip"
        require(path.is_file() and sha256_file(path) == source["zip_sha256"],
                f"provider ZIP identity differs: stratum {source['stratum']}")
        archive = zipfile.ZipFile(path)
        names = archive.namelist()
        require(len(names) == len(set(names)), "duplicate provider member")
        archives.append(archive)
    objects = []
    try:
        for arm in plan["arms"]:
            rows = []
            first = None
            for source, archive in zip(plan["source_artifacts"], archives):
                stratum = source["stratum"]
                for offset in range(8):
                    point = 8 * stratum + offset
                    member = (f"results/points/p{point:02d}/runs/"
                              f"{arm['role']}/base_dump.jsonl")
                    info = archive.getinfo(member)
                    require(info.file_size == arm["raw_bytes"],
                            f"provider member size differs: {member}")
                    digest = sha256_member(archive, member)
                    require(digest == arm["raw_sha256"],
                            f"provider member hash differs: {member}")
                    rows.append({"stratum": stratum, "point": point,
                                 "member": member, "sha256": digest})
                    if first is None:
                        first = archive.read(member)
            require(len(rows) == plan["expected_copies_per_arm"] and first is not None,
                    f"provider copy count differs: {arm['role']}")
            check_base(first, arm, plan["curve"])
            target = output / f"{arm['role']}.jsonl.gz"
            with target.open("xb") as destination:
                with gzip.GzipFile(filename="", mode="wb", fileobj=destination,
                                   mtime=0, compresslevel=9) as compressed:
                    compressed.write(first)
            packed = target.read_bytes()
            compressed_blake3 = blake3.blake3(packed).hexdigest()
            key = (plan["destination_prefix"] + compressed_blake3 + ".jsonl.gz")
            objects.append({
                "role": arm["role"], "candidate_id": arm["candidate_id"],
                "base_hash": arm["base_hash"],
                "provider_copies": rows,
                "raw_sha256": arm["raw_sha256"], "raw_bytes": len(first),
                "compressed_sha256": hashlib.sha256(packed).hexdigest(),
                "compressed_blake3": compressed_blake3,
                "compressed_bytes": len(packed),
                "etag_expected": '"' + hashlib.md5(packed).hexdigest() + '"',
                "s3_uri": "s3://" + plan["destination_bucket"] + "/" + key,
            })
    finally:
        for archive in archives:
            archive.close()
    receipt = {
        "schema": "n83.root11-s3-custody-build/v1",
        "plan_sha256": hashlib.sha256(plan_bytes).hexdigest(),
        "source_run_id": plan["source_run_id"],
        "source_commit": plan["source_commit"],
        "objects": objects,
        "selected_best_total_runtime": None,
        "status": "PASS_TWO_VERIFIED_PROVIDER_BASES",
    }
    (output / "build.json").write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--provider-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    receipt = build(args.provider_root, args.output)
    print(json.dumps({"status": receipt["status"],
                      "objects": [row["s3_uri"] for row in receipt["objects"]]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
