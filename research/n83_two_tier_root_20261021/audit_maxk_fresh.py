#!/usr/bin/env python3
"""Audit new max-K public points against the frozen N83 fixture inventory."""

import hashlib
import io
import json
import subprocess
import tarfile
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD_AUDIT = "research/n83_root4_highk_tier1_retry_20261019/fresh_exclusion_audit.json"
OLD_AUDIT_SHA256 = "7ac954283e383701cb33e8e2638ff77225357bf5d52517f5762d757843de803b"
EXTRA = {
    "research/n83_root4_highk_tier1_retry_20261019/fresh_targets.jsonl":
        "2824e714820524e81d7e3cb16821529827d82a18fc1c9de8a0e7315ab940deac",
    "research/n83_crossfamily_online_20261019/fresh_targets.jsonl":
        "36353c7fa20b31d50e0ca17ecad90608544d9f0350e6655f5f8d4cba4319a6db",
    "research/n83_root4_highk_tier1_retry_20261019/finalists_historical_targets.jsonl":
        "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58",
    "research/n83_highk_online_distribution_20261020/screen_targets.jsonl":
        "0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3",
    "research/n83_highk_online_distribution_20261020/holdout_targets.jsonl":
        "e7dcb0fe5779f847d381017b3cb5fb7dd46b6a4a7e834611d200d7cbd6244f68",
}
ARCHIVE = Path("/Volumes/data/cryptanalysis/n83_highk_tiebreak_20261020_evidence/raw_results.tar.gz")
ARCHIVE_SHA256 = "352766bd9610615b10b0aa8f6a10d28a4880f0cb8851c33ee300a199bb9e6ece"
MEMBERS = {
    "results/screen/block0/targets.jsonl":
        "20679f540248483bd1450c64d4fc7b0abbadaee8ce38c1193f64d14ab74990f3",
    "results/screen/block1/targets.jsonl":
        "6a09d3d0644ae2403aac007ac39be0f7561d6bacda9e24faecdbcfe332064871",
}
LOCAL_TARGET = Path("/Volumes/data/cryptanalysis/n83_load80_k4405_7200_20261020/target.jsonl")
LOCAL_TARGET_SHA256 = "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def git_blob(relative):
    return subprocess.check_output(["git", "show", f"HEAD:{relative}"], cwd=ROOT)


def points(blob):
    result = []
    for line in blob.decode().splitlines():
        try:
            value = json.loads(line)
        except (json.JSONDecodeError, ValueError):
            continue
        if (isinstance(value, list) and len(value) == 2 and
                all(isinstance(x, (str, int)) for x in value)):
            result.append(tuple(int(x) for x in value))
    return result


def audit():
    prior_audit_blob = git_blob(OLD_AUDIT)
    if digest(prior_audit_blob) != OLD_AUDIT_SHA256:
        raise ValueError("prior fixture inventory changed")
    prior_audit = json.loads(prior_audit_blob)
    expected = {item["path"]: item["sha256"] for item in prior_audit["prior_files"]}
    expected.update(EXTRA)
    prior = set()
    checked = []
    for relative, sha in sorted(expected.items()):
        blob = git_blob(relative)
        if digest(blob) != sha:
            raise ValueError(f"prior fixture changed: {relative}")
        rows = points(blob)
        prior.update(rows)
        checked.append({"path": relative, "sha256": sha, "point_rows": len(rows)})
    archive_blob = ARCHIVE.read_bytes()
    if digest(archive_blob) != ARCHIVE_SHA256:
        raise ValueError("high-K raw archive changed")
    archived = []
    with tarfile.open(fileobj=io.BytesIO(archive_blob), mode="r:gz") as archive:
        for member, sha in MEMBERS.items():
            blob = archive.extractfile(member).read()
            if digest(blob) != sha:
                raise ValueError(f"high-K archive member changed: {member}")
            rows = points(blob)
            prior.update(rows)
            archived.append({"member": member, "sha256": sha, "point_rows": len(rows)})
    local_blob = LOCAL_TARGET.read_bytes()
    if digest(local_blob) != LOCAL_TARGET_SHA256:
        raise ValueError("local control target changed")
    prior.update(points(local_blob))
    manifest_blob = (HERE / "maxk_fresh_manifest.json").read_bytes()
    manifest = json.loads(manifest_blob)
    fresh = {}
    fresh_seen = set()
    for kind, count in (("screen", 192), ("holdout", 8)):
        blob = (HERE / f"maxk_{kind}_targets.jsonl").read_bytes()
        rows = points(blob)
        if (len(rows) != count or len(set(rows)) != count or
                digest(blob) != manifest[kind]["targets_sha256"]):
            raise ValueError(f"{kind} target count or digest differs")
        fresh[kind] = {"point_count": count, "targets_sha256": digest(blob),
                       "prior_overlap_count": len(set(rows) & prior),
                       "other_fresh_overlap_count": len(set(rows) & fresh_seen)}
        fresh_seen.update(rows)
    result = {
        "schema": "n83.maxk-exclusion/v1",
        "old_audit_sha256": OLD_AUDIT_SHA256,
        "prior_files": checked, "highk_archive_sha256": ARCHIVE_SHA256,
        "highk_archive_members": archived,
        "local_target_sha256": LOCAL_TARGET_SHA256,
        "prior_unique_point_count": len(prior),
        "fresh_manifest_sha256": digest(manifest_blob),
        "screen": fresh["screen"], "holdout": fresh["holdout"],
    }
    result["status"] = ("PASS_ZERO_OVERLAP" if all(
        row["prior_overlap_count"] == row["other_fresh_overlap_count"] == 0
        for row in fresh.values()) else "OVERLAP_FOUND")
    path = HERE / "maxk_exclusion_audit.json"
    blob = (json.dumps(result, indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("frozen exclusion audit changed")
    path.write_bytes(blob)
    print(json.dumps({"status": result["status"],
                      "prior_unique_point_count": len(prior),
                      "screen": fresh["screen"], "holdout": fresh["holdout"]},
                     sort_keys=True))


if __name__ == "__main__":
    audit()
