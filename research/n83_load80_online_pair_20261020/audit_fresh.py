#!/usr/bin/env python3
"""Audit the two sealed points against the pinned N83 public inventory."""

import argparse
import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
TIE_COMMIT = "5e6d7f7bdc7d92f2bea75ce756a14965b6ff6644"
AUDIT_PATH = "research/n83_highk_online_tiebreak_20261020/fresh_exclusion_audit.json"
AUDIT_SHA256 = "82919779c004b8ed6b703bbfa5d4bd24b2b24f38e8fa3b4e8fb1b10f78b42b9e"
TIE_TARGET_PATH = "research/n83_highk_online_tiebreak_20261020/screen_targets.jsonl"


def digest(blob):
    return hashlib.sha256(blob).hexdigest()


def pinned(repo, path):
    return subprocess.check_output(["git", "show", f"{TIE_COMMIT}:{path}"],
                                   cwd=repo)


def point_rows(blob):
    points = []
    for line in blob.decode().splitlines():
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if (isinstance(value, list) and len(value) == 2 and
                all(isinstance(x, (str, int)) for x in value)):
            points.append(tuple(int(x) for x in value))
    return points


def audit(repo):
    old_blob = pinned(repo, AUDIT_PATH)
    if digest(old_blob) != AUDIT_SHA256:
        raise ValueError("pinned prior exclusion audit differs")
    old = json.loads(old_blob)
    expected = {item["path"]: item["sha256"] for item in old["prior_files"]}
    expected[TIE_TARGET_PATH] = old["screen"]["targets_sha256"]
    prior = set()
    checked = []
    for path, sha in sorted(expected.items()):
        blob = pinned(repo, path)
        if digest(blob) != sha:
            raise ValueError(f"pinned inventory item differs: {path}")
        rows = point_rows(blob)
        prior.update(rows)
        checked.append({"path": path, "sha256": sha,
                        "point_rows": len(rows)})
    fixture = json.loads((HERE / "fixture.json").read_text())
    fresh_blob = (HERE / "targets.jsonl").read_bytes()
    fresh = point_rows(fresh_blob)
    if (digest(fresh_blob) != fixture["targets_sha256"] or
            len(fresh) != 2 or len(set(fresh)) != 2):
        raise ValueError("two-point fixture differs")
    overlap = len(set(fresh) & prior)
    report = {
        "schema": "n83.load80-online-pair-exclusion/v1",
        "tie_commit": TIE_COMMIT,
        "prior_audit_sha256": AUDIT_SHA256,
        "prior_files": checked,
        "prior_unique_point_count": len(prior),
        "fresh_targets_sha256": digest(fresh_blob),
        "fresh_point_count": len(fresh),
        "prior_overlap_count": overlap,
        "status": "PASS_ZERO_OVERLAP" if overlap == 0 else "OVERLAP_FOUND",
    }
    blob = (json.dumps(report, indent=2, sort_keys=True) + "\n").encode()
    path = HERE / "fresh_exclusion_audit.json"
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("sealed exclusion audit differs")
    path.write_bytes(blob)
    print(json.dumps({key: report[key] for key in
                      ("status", "prior_unique_point_count",
                       "fresh_point_count", "prior_overlap_count")},
                     sort_keys=True))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--tie-root", type=Path, required=True)
    args = parser.parse_args()
    audit(args.tie_root.resolve())
