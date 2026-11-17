#!/usr/bin/env python3
"""Audit both new N83 streams against the frozen public fixture inventory."""

import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PRIOR_AUDIT = ROOT / "research/n83_root4_highk_tier1_retry_20261019/fresh_exclusion_audit.json"
PRIOR_AUDIT_SHA256 = "7ac954283e383701cb33e8e2638ff77225357bf5d52517f5762d757843de803b"
EXTRA = {
    "research/n83_root4_highk_tier1_retry_20261019/fresh_targets.jsonl":
        "2824e714820524e81d7e3cb16821529827d82a18fc1c9de8a0e7315ab940deac",
    "research/n83_crossfamily_online_20261019/fresh_targets.jsonl":
        "36353c7fa20b31d50e0ca17ecad90608544d9f0350e6655f5f8d4cba4319a6db",
    "research/n83_root4_highk_tier1_retry_20261019/finalists_historical_targets.jsonl":
        "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58",
}


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def point_rows(blob):
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


def audit():
    if digest(PRIOR_AUDIT.read_bytes()) != PRIOR_AUDIT_SHA256:
        raise ValueError("prior fixture inventory changed")
    old = json.loads(PRIOR_AUDIT.read_text())
    expected = {item["path"]: item["sha256"] for item in old["prior_files"]}
    expected.update(EXTRA)
    prior = set()
    checked = []
    for relative, sha in sorted(expected.items()):
        blob = subprocess.check_output(
            ["git", "show", f"HEAD:{relative}"], cwd=ROOT)
        if digest(blob) != sha:
            raise ValueError(f"prior fixture differs: {relative}")
        points = point_rows(blob)
        prior.update(points)
        checked.append({"path": relative, "sha256": sha,
                        "point_rows": len(points)})
    fresh = {}
    both = set()
    for kind, count in (("screen", 192), ("holdout", 8)):
        blob = (HERE / f"{kind}_targets.jsonl").read_bytes()
        points = point_rows(blob)
        if len(points) != count or len(set(points)) != count:
            raise ValueError(f"{kind} count or distinctness differs")
        overlap_prior = len(set(points) & prior)
        overlap_other = len(set(points) & both)
        fresh[kind] = {"point_count": count, "targets_sha256": digest(blob),
                       "prior_overlap_count": overlap_prior,
                       "other_fresh_overlap_count": overlap_other}
        both.update(points)
    report = {
        "schema": "n83.online-distribution-exclusion/v1",
        "prior_audit_sha256": PRIOR_AUDIT_SHA256,
        "prior_files": checked, "prior_unique_point_count": len(prior),
        "screen": fresh["screen"], "holdout": fresh["holdout"],
    }
    report["status"] = ("PASS_ZERO_OVERLAP" if all(
        item["prior_overlap_count"] == item["other_fresh_overlap_count"] == 0
        for item in fresh.values()) else "OVERLAP_FOUND")
    path = HERE / "fresh_exclusion_audit.json"
    blob = (json.dumps(report, indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("sealed exclusion audit differs")
    path.write_bytes(blob)
    print(json.dumps({"status": report["status"],
                      "prior_unique_point_count": len(prior),
                      "screen": fresh["screen"],
                      "holdout": fresh["holdout"]}, sort_keys=True))


if __name__ == "__main__":
    audit()
