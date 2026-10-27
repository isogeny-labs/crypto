#!/usr/bin/env python3
"""Independently replay the bridge provider and fresh-S3 byte identities."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import zipfile

import blake3

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def file_digest(path: Path, algorithm: str) -> str:
    h = hashlib.new(algorithm)
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def member_sha256(archive: zipfile.ZipFile, member: str) -> str:
    h = hashlib.sha256()
    with archive.open(member) as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def audit(provider_root: Path, downloads: Path, build_receipt: Path) -> dict:
    plan_bytes = (HERE / "plan.json").read_bytes()
    plan = json.loads(plan_bytes)
    build = json.loads(build_receipt.read_text())
    require(plan["schema"] == "n83.bridge-s3-custody-plan/v1"
            and build["schema"] == "n83.bridge-s3-custody-build/v1"
            and build["plan_sha256"] == hashlib.sha256(plan_bytes).hexdigest()
            and build["source_run_id"] == plan["source_run_id"]
            and build["source_commit"] == plan["source_commit"]
            and build["status"] == "PASS_TWO_VERIFIED_BRIDGE_BASES"
            and build["selected_best_total_runtime"] is None
            and plan["selected_best_total_runtime"] is None,
            "frozen plan or build receipt differs")
    by_role = {row["role"]: row for row in build["objects"]}
    require(len(by_role) == len(plan["arms"]) == 2,
            "build object identities differ")
    require(plan["source_artifacts"][0]["stratum"] == 0
            and {row["role"] for row in plan["arms"]}
            == {"k1107root11", "k2304root11"},
            "bridge provider or arm coverage differs")
    sources = {}
    for source in plan["source_artifacts"]:
        stratum = source["stratum"]
        path = provider_root / f"stratum{stratum}.zip"
        require(file_digest(path, "sha256") == source["zip_sha256"],
                f"provider ZIP differs: {stratum}")
        job = json.loads((HERE / "receipts/provider-job0.json").read_text())
        artifact = json.loads((HERE / "receipts/provider-artifact0.json").read_text())
        require(job["id"] == source["job_id"]
                and job["run_id"] == int(plan["source_run_id"])
                and job["head_sha"] == plan["source_commit"]
                and job["name"] == "bridge-stratum (0)"
                and job["status"] == "completed"
                and job["conclusion"] == "success"
                and job["runner_id"] > 0
                and artifact["id"] == source["artifact_id"]
                and artifact["name"] == source["artifact_name"]
                and artifact["size_in_bytes"] == source["zip_bytes"] == path.stat().st_size
                and artifact["digest"] == "sha256:" + source["zip_sha256"]
                and artifact["expired"] is False
                and artifact["workflow_run"]["id"] == job["run_id"]
                and artifact["workflow_run"]["head_sha"] == plan["source_commit"]
                and artifact["workflow_run"]["head_branch"] == plan["source_branch"],
                "GitHub provider job or artifact identity differs")
        sources[stratum] = path
    summaries = []
    expected_inventory = {}
    for arm in plan["arms"]:
        receipt = by_role[arm["role"]]
        manifest_path = REPO / arm["candidate_manifest_path"]
        require(file_digest(manifest_path, "sha256") ==
                arm["candidate_manifest_sha256"],
                f"candidate manifest bytes differ: {arm['role']}")
        manifest = json.loads(manifest_path.read_text())
        base = manifest["factor_base"]
        require(manifest["candidate_id"] == arm["candidate_id"]
                and arm["provider_run_dir"] == "ic_" + arm["role"]
                and base["effective_columns"] == arm["orbit_columns"]
                and base["actual_usable_points"] == arm["point_count"]
                and base["ordered_base_dump_sha256"] == arm["raw_sha256"]
                and base["source_base_hash_blake3"] == arm["base_hash"]
                and arm["point_count"] ==
                arm["orbit_columns"] * plan["points_per_orbit_column"],
                f"candidate manifest meaning differs: {arm['role']}")
        require(receipt["candidate_id"] == arm["candidate_id"]
                and receipt["base_hash"] == arm["base_hash"]
                and receipt["raw_sha256"] == arm["raw_sha256"]
                and receipt["raw_bytes"] == arm["raw_bytes"],
                f"build identity differs: {arm['role']}")
        expected_members = []
        for stratum, path in sources.items():
            with zipfile.ZipFile(path) as archive:
                require(len(archive.namelist()) == len(set(archive.namelist())),
                        "duplicate provider ZIP member")
                for offset in range(8):
                    point = 8 * stratum + offset
                    member = (f"results/points/p{point:02d}/runs/"
                              f"{arm['provider_run_dir']}/base_dump.jsonl")
                    require(archive.getinfo(member).file_size == arm["raw_bytes"]
                            and member_sha256(archive, member) == arm["raw_sha256"],
                            f"provider base differs: {member}")
                    expected_members.append({"stratum": stratum, "point": point,
                                             "member": member,
                                             "sha256": arm["raw_sha256"]})
        require(len(expected_members) == plan["expected_copies_per_arm"]
                and receipt["provider_copies"] == expected_members,
                f"provider copy receipt differs: {arm['role']}")
        digest = receipt["compressed_blake3"]
        expected_uri = ("s3://" + plan["destination_bucket"] + "/"
                        + plan["destination_prefix"] + digest + ".jsonl.gz")
        require(receipt["s3_uri"] == expected_uri,
                f"S3 object address differs: {arm['role']}")
        key = plan["destination_prefix"] + digest + ".jsonl.gz"
        packed = downloads / f"{digest}.jsonl.gz"
        get = json.loads((downloads / f"{digest}.get.json").read_text())
        packed_bytes = packed.read_bytes()
        require(packed.stat().st_size == receipt["compressed_bytes"]
                and file_digest(packed, "sha256") == receipt["compressed_sha256"]
                and blake3.blake3(packed_bytes).hexdigest() == digest
                and file_digest(packed, "md5") == receipt["etag_expected"].strip('"')
                and get["ETag"] == receipt["etag_expected"]
                and get["ContentLength"] == receipt["compressed_bytes"]
                and get["ContentType"] == "application/gzip"
                and get["Metadata"] == {
                    "role": arm["role"], "base-hash": arm["base_hash"],
                    "raw-sha256": arm["raw_sha256"],
                },
                f"compressed S3 object differs: {arm['role']}")
        put = json.loads((HERE / "receipts" /
                          f"{arm['role']}.put.json").read_text())
        require(put["ETag"] == receipt["etag_expected"]
                and put["ChecksumType"] == "FULL_OBJECT"
                and put["ServerSideEncryption"] == "AES256",
                f"conditional S3 PUT response differs: {arm['role']}")
        expected_inventory[key] = (receipt["compressed_bytes"],
                                   receipt["etag_expected"])
        raw = gzip.decompress(packed_bytes)
        require(len(raw) == arm["raw_bytes"]
                and hashlib.sha256(raw).hexdigest() == arm["raw_sha256"],
                f"decompressed S3 base differs: {arm['role']}")
        row = json.loads(raw)
        curve = plan["curve"]
        require(row["a"] == curve["a"] and row["n"] == curve["n"]
                and row["field_modulus_low_terms"] == curve["field_modulus_low_terms"]
                and row["subgroup_order"] == curve["subgroup_order"]
                and row["kind"] == curve["kind"]
                and row["orbit_columns"] == arm["orbit_columns"]
                and row["base_hash"] == arm["base_hash"]
                and row["factor_base_points"] == arm["point_count"],
                f"downloaded factor-base metadata differs: {arm['role']}")
        points = row["factor_base_point_coordinates"]
        require(len(points) == arm["point_count"]
                and len(row["factor_base_point_labels"]) == len(points)
                and len(row["factor_base_representatives"]) == arm["orbit_columns"]
                and len({tuple(point) for point in points}) == len(points)
                and all(isinstance(point, list) and len(point) == 2
                        and all(isinstance(v, str) and v.isdecimal()
                                and str(int(v)) == v
                                and int(v) < 1 << curve["n"] for v in point)
                        for point in points),
                f"downloaded factor-base points differ: {arm['role']}")
        summaries.append({"role": arm["role"], "s3_uri": expected_uri,
                          "compressed_sha256": receipt["compressed_sha256"],
                          "compressed_blake3": digest,
                          "raw_sha256": arm["raw_sha256"],
                          "point_count": len(points),
                          "provider_copy_count": len(expected_members),
                          "etag": get["ETag"]})
    inventory = json.loads((HERE / "receipts/inventory.json").read_text())
    listed = inventory.get("Contents", [])
    require(inventory["IsTruncated"] is False
            and len(listed) == len(expected_inventory)
            and {row["Key"]: (row["Size"], row["ETag"])
                 for row in listed} == expected_inventory,
            "live S3 prefix inventory differs")
    no_overwrite = json.loads((HERE / "receipts/no-overwrite.json").read_text())
    head = json.loads((HERE / "receipts/no-overwrite.head.json").read_text())
    k1107 = by_role["k1107root11"]
    require(no_overwrite["status"] == "PASS_PRECONDITION_REJECTED_DUPLICATE_PUT"
            and no_overwrite["request_condition"] == "If-None-Match: *"
            and no_overwrite["s3_error_code"] == "PreconditionFailed"
            and no_overwrite["aws_cli_exit_code"] != 0
            and no_overwrite["repeated_source_sha256"] == k1107["compressed_sha256"]
            and head["ETag"] == no_overwrite["head_etag"] == k1107["etag_expected"]
            and head["ContentLength"] == no_overwrite["head_content_length"]
            == k1107["compressed_bytes"]
            and head["Metadata"]["raw-sha256"] == k1107["raw_sha256"],
            "write-once rejection or post-rejection HEAD differs")
    return {"schema": "n83.bridge-s3-custody-audit/v1",
            "status": "PASS_TWO_FRESH_S3_ROUNDTRIPS",
            "plan_sha256": hashlib.sha256(plan_bytes).hexdigest(),
            "source_run_id": plan["source_run_id"],
            "objects": summaries,
            "selected_best_total_runtime": None}


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
