#!/usr/bin/env python3
"""Check fixed31 fixtures against sealed earlier N83 public-point inputs."""

import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PRIOR_AUDIT = ROOT / "research/n83_highk_online_tiebreak_20261020/fresh_exclusion_audit.json"
PRIOR_AUDIT_SHA256 = "82919779c004b8ed6b703bbfa5d4bd24b2b24f38e8fa3b4e8fb1b10f78b42b9e"
EXTRA = (
    ("HEAD", "research/n83_highk_online_tiebreak_20261020/screen_targets.jsonl",
     "41b160dc56fadb0e8beec4c86bfee47457c6ade8290dd17cfa30c728787d5582"),
    ("753b6223493688aaff3b3c8881cb487228ef90e6",
     "research/n83_cold_base_confirm_20261021/targets.jsonl",
     "613bb1ad135f3bfc4632fdc3543eb91953571f1ac91fd36b5d31fece5a04f67c"),
    ("48501c90100223ebf9597ed223692c0f24ce1f8f",
     "research/n83_fresh_cold_binary_retest_20261021/fresh_targets.jsonl",
     "c604d7d42468d16636f4d0a8f6fe4caa75b91104d85287743b868e04ce5098e4"),
    ("HEAD", "research/n83_fresh_cold_finalists_20261019/fresh_targets.jsonl",
     "174a6c56bbd54d419df3363e700660a08578612fbb5b8d7f25ed54c5eb622e4a"),
    ("HEAD", "research/n83_fresh_cold_finalists_20261019/historical_finalist_targets.jsonl",
     "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58"),
    ("HEAD", "research/n83_global_bridge_20261018/fresh_targets.jsonl",
     "a767c6acb37174593b8dd50cddfac7ed5be02498a00d199754a234606a8cbd64"),
    ("HEAD", "research/n83_k5086_base_20261021/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_load80_online_pair_20261020/targets.jsonl",
     "5d5482355731bc2362f7a3b612c70fba13ba326f97e153a4610138bda5a14abd"),
    ("HEAD", "research/n83_root4_load80_20261020/reports/run-20261020T1604Z/raw/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_two_tier_root_20261021/maxk_holdout_targets.jsonl",
     "96375c178e8315c136f190927dc3470fc84f1b7096c2e563387808928aa56d6a"),
    ("HEAD", "research/n83_two_tier_root_20261021/maxk_screen_targets.jsonl",
     "b99b196b43e69414c101e948ca74ea32703d1ea87ad71e0109e01b37f8cafe38"),
    ("HEAD", "research/n83_two_tier_root_20261021/runs/base_only_k128_20261021/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_two_tier_root_20261021/runs/base_only_k4405_20261021/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_two_tier_root_20261021/runs/base_only_k5500_20261021/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_two_tier_root_20261021/runs/k4405_monolithic_local_7200/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
    ("HEAD", "research/n83_two_tier_root_20261021/runs/k4405_split_shared_local_7200/target.jsonl",
     "1c9b53960953fd252d09af08afbfdff9229fbeca151285e698b3e90cb79cd656"),
)


def digest(blob: bytes) -> str:
    return hashlib.sha256(blob).hexdigest()


def point_rows(blob: bytes) -> list[tuple[int, int]]:
    points = []
    for line in blob.decode().splitlines():
        try:
            value = json.loads(line)
            if (isinstance(value, list) and len(value) == 2 and
                    all(isinstance(x, (str, int)) for x in value)):
                points.append(tuple(int(x) for x in value))
        except (TypeError, ValueError, json.JSONDecodeError):
            continue
    return points


def verified_git_points(rev: str, path: str, sha256: str) -> list[tuple[int, int]]:
    blob = subprocess.check_output(["git", "show", f"{rev}:{path}"], cwd=ROOT)
    if digest(blob) != sha256:
        raise ValueError(f"sealed fixture differs: {rev}:{path}")
    return point_rows(blob)


def write_once(path: Path, blob: bytes) -> None:
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"sealed audit differs: {path}")
    path.write_bytes(blob)


def audit() -> dict:
    if digest(PRIOR_AUDIT.read_bytes()) != PRIOR_AUDIT_SHA256:
        raise ValueError("prior exclusion inventory differs")
    old = json.loads(PRIOR_AUDIT.read_text())
    inventory = [("HEAD", row["path"], row["sha256"])
                 for row in old["prior_files"]] + list(EXTRA)
    seen = set()
    checked = []
    for rev, path, sha256 in inventory:
        points = verified_git_points(rev, path, sha256)
        seen.update(points)
        checked.append({"revision": rev, "path": path,
                        "sha256": sha256, "point_rows": len(points)})
    fresh = {}
    new_seen = set()
    for kind, count in (("screen", 1536), ("holdout", 16)):
        blob = (HERE / f"{kind}_targets.jsonl").read_bytes()
        points = point_rows(blob)
        if len(points) != count or len(set(points)) != count:
            raise ValueError(f"{kind} point count or distinctness differs")
        overlap_prior = len(set(points) & seen)
        overlap_fresh = len(set(points) & new_seen)
        fresh[kind] = {"point_count": count, "targets_sha256": digest(blob),
                       "prior_overlap_count": overlap_prior,
                       "other_fresh_overlap_count": overlap_fresh}
        new_seen.update(points)
    report = {
        "schema": "n83.fixed31-exclusion/v2",
        "prior_audit_sha256": PRIOR_AUDIT_SHA256,
        "prior_files": checked,
        "prior_unique_point_count": len(seen),
        "screen": fresh["screen"],
        "holdout": fresh["holdout"],
        "status": "PASS_ZERO_OVERLAP" if all(
            row["prior_overlap_count"] == row["other_fresh_overlap_count"] == 0
            for row in fresh.values()) else "OVERLAP_FOUND",
    }
    write_once(HERE / "fresh_exclusion_audit_v2.json",
               (json.dumps(report, indent=2, sort_keys=True) + "\n").encode())
    return report


if __name__ == "__main__":
    result = audit()
    print(json.dumps({"status": result["status"],
                      "prior_unique_point_count": result["prior_unique_point_count"],
                      "screen": result["screen"], "holdout": result["holdout"]},
                     sort_keys=True))
