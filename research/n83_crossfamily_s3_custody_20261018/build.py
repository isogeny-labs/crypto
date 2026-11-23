#!/usr/bin/env python3
"""Bind all provider copies and build four deterministic factor-base objects."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import zipfile

import blake3

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def digest_file(path: Path, algorithm: str) -> str:
    value = hashlib.new(algorithm)
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            value.update(block)
    return value.hexdigest()


def digest_member(archive: zipfile.ZipFile, member: str) -> str:
    value = hashlib.sha256()
    with archive.open(member) as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            value.update(block)
    return value.hexdigest()


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
                f"base metadata differs: {arm['role']} {key}")
    points = row.get("factor_base_point_coordinates")
    labels = row.get("factor_base_point_labels")
    representatives = row.get("factor_base_representatives")
    require(isinstance(points, list) and len(points) == arm["point_count"]
            and isinstance(labels, list) and len(labels) == len(points)
            and isinstance(representatives, list)
            and len(representatives) == arm["orbit_columns"],
            f"base array lengths differ: {arm['role']}")
    bound = 1 << curve["n"]
    distinct = set()
    for point in points:
        require(isinstance(point, list) and len(point) == 2
                and all(isinstance(value, str) and value.isdecimal()
                        and int(value) < bound for value in point),
                f"base coordinate differs: {arm['role']}")
        distinct.add(tuple(point))
    require(len(distinct) == len(points), f"duplicate base point: {arm['role']}")


def build(provider_root: Path, output: Path) -> dict:
    plan_bytes = (HERE / "plan.json").read_bytes()
    plan = json.loads(plan_bytes)
    require(plan["schema"] == "n83.crossfamily-s3-custody-plan/v1"
            and plan["selected_best_total_runtime"] is None
            and len(plan["source_artifacts"]) == len(plan["arms"]) == 4,
            "frozen custody plan differs")
    require([row["stratum"] for row in plan["source_artifacts"]] == list(range(4))
            and plan["copies_per_stratum"] == 6
            and plan["expected_copies_per_arm"] == 24,
            "provider coverage differs")
    for arm in plan["arms"]:
        manifest = subprocess.check_output(
            ["git", "show", plan["source_commit"] + ":" +
             arm["candidate_manifest_path"]], cwd=ROOT)
        require(hashlib.sha256(manifest).hexdigest() ==
                arm["candidate_manifest_sha256"],
                f"candidate manifest differs: {arm['role']}")
    output.mkdir(parents=True, exist_ok=False)
    archives = []
    try:
        for source in plan["source_artifacts"]:
            path = provider_root / f"stratum{source['stratum']}.zip"
            require(path.is_file() and digest_file(path, "sha256") ==
                    source["zip_sha256"],
                    f"provider ZIP differs: {source['stratum']}")
            archive = zipfile.ZipFile(path)
            names = archive.namelist()
            require(len(names) == len(set(names))
                    and all(not Path(name).is_absolute()
                            and ".." not in Path(name).parts for name in names),
                    "provider ZIP member names unsafe or duplicated")
            archives.append(archive)
        objects = []
        for arm in plan["arms"]:
            copies = []
            first = None
            for source, archive in zip(plan["source_artifacts"], archives):
                stratum = source["stratum"]
                for offset in range(plan["copies_per_stratum"]):
                    point = 6 * stratum + offset
                    member = (f"results/points/p{point:02d}/runs/"
                              f"{arm['role']}/base_dump.jsonl")
                    require(archive.getinfo(member).file_size == arm["raw_bytes"]
                            and digest_member(archive, member) == arm["raw_sha256"],
                            f"provider base copy differs: {member}")
                    copies.append({"stratum": stratum, "point": point,
                                   "member": member, "sha256": arm["raw_sha256"]})
                    if first is None:
                        first = archive.read(member)
            require(len(copies) == 24 and first is not None
                    and hashlib.sha256(first).hexdigest() == arm["raw_sha256"],
                    f"provider base coverage differs: {arm['role']}")
            check_base(first, arm, plan["curve"])
            target = output / f"{arm['role']}.jsonl.gz"
            with target.open("xb") as destination:
                with gzip.GzipFile(filename="", mode="wb", fileobj=destination,
                                   mtime=0, compresslevel=9) as compressed:
                    compressed.write(first)
            packed = target.read_bytes()
            address = blake3.blake3(packed).hexdigest()
            obj = {
                "role": arm["role"], "candidate_id": arm["candidate_id"],
                "storage_mode": arm["storage_mode"],
                "base_hash": arm["base_hash"], "provider_copies": copies,
                "raw_sha256": arm["raw_sha256"], "raw_bytes": len(first),
                "compressed_sha256": hashlib.sha256(packed).hexdigest(),
                "compressed_blake3": address, "compressed_bytes": len(packed),
                "etag_expected": '"' + hashlib.md5(packed).hexdigest() + '"',
            }
            if arm["storage_mode"] == "reuse":
                obj["s3_uri"] = arm["expected_object"]["s3_uri"]
                require(all(obj[key] == arm["expected_object"][key] for key in
                            ("compressed_sha256", "compressed_blake3",
                             "compressed_bytes", "etag_expected")),
                        f"prior S3 object cannot be reused: {arm['role']}")
            else:
                require(arm["storage_mode"] == "upload", "storage mode differs")
                obj["s3_uri"] = ("s3://" + plan["destination_bucket"] + "/" +
                                 plan["destination_prefix"] + address + ".jsonl.gz")
            objects.append(obj)
        receipt = {
            "schema": "n83.crossfamily-s3-custody-build/v1",
            "status": "PASS_FOUR_VERIFIED_PROVIDER_BASES",
            "plan_sha256": hashlib.sha256(plan_bytes).hexdigest(),
            "source_run_id": plan["source_run_id"],
            "source_commit": plan["source_commit"],
            "objects": objects, "selected_best_total_runtime": None,
        }
        (output / "build.json").write_text(
            json.dumps(receipt, sort_keys=True, indent=2) + "\n")
        return receipt
    finally:
        for archive in archives:
            archive.close()


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
