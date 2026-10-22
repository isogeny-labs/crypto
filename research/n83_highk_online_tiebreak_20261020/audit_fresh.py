#!/usr/bin/env python3
"""Audit tie-break public points against the frozen N83 target inventory."""

import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PRIOR_AUDIT = (ROOT / "research/n83_highk_online_distribution_20261020"
               / "fresh_exclusion_audit.json")
PRIOR_AUDIT_SHA256 = "2c439041c39244b9e44fa0d50f2d861e86c0d8765e6b4cf828f9742aaaa8f926"
EXTRA = {
    "research/n83_highk_online_distribution_20261020/screen_targets.jsonl":
        "0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3",
    "research/n83_highk_online_distribution_20261020/holdout_targets.jsonl":
        "e7dcb0fe5779f847d381017b3cb5fb7dd46b6a4a7e834611d200d7cbd6244f68",
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
        raise ValueError("prior exclusion inventory differs")
    old = json.loads(PRIOR_AUDIT.read_text())
    expected = {item["path"]: item["sha256"] for item in old["prior_files"]}
    expected.update(EXTRA)
    prior, checked = set(), []
    for relative, sha in sorted(expected.items()):
        blob = subprocess.check_output(["git", "show", f"HEAD:{relative}"],
                                       cwd=ROOT)
        if digest(blob) != sha:
            raise ValueError(f"prior fixture differs: {relative}")
        points = point_rows(blob)
        prior.update(points)
        checked.append({"path": relative, "sha256": sha,
                        "point_rows": len(points)})
    blob = (HERE / "screen_targets.jsonl").read_bytes()
    points = point_rows(blob)
    if len(points) != 1536 or len(set(points)) != 1536:
        raise ValueError("screen point count or distinctness differs")
    overlap = len(set(points) & prior)
    report = {
        "schema": "n83.online-tiebreak-exclusion/v1",
        "prior_audit_sha256": PRIOR_AUDIT_SHA256,
        "prior_files": checked,
        "prior_unique_point_count": len(prior),
        "screen": {"point_count": len(points),
                   "targets_sha256": digest(blob),
                   "prior_overlap_count": overlap},
        "status": "PASS_ZERO_OVERLAP" if overlap == 0 else "OVERLAP_FOUND",
    }
    path = HERE / "fresh_exclusion_audit.json"
    write = (json.dumps(report, indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != write:
        raise ValueError("sealed exclusion audit differs")
    path.write_bytes(write)
    print(json.dumps({"status": report["status"],
                      "prior_unique_point_count": len(prior),
                      "screen": report["screen"]}, sort_keys=True))


if __name__ == "__main__":
    audit()
