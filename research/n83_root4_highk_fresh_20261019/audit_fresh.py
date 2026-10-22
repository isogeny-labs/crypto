#!/usr/bin/env python3
"""Check the new high-K points against the sealed prior N83 public fixtures."""

import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)

import subprocess  # noqa: E402

AUDIT = HERE / "fresh_exclusion_audit.json"
PRIOR_AUDIT = ROOT / "research/n83_root4_frontier_current_20261018/fresh_exclusion_audit.json"
PRIOR_FRONTIER = "research/n83_root4_frontier_current_20261018/fresh_targets.jsonl"
FINALISTS_HEAD = "9b9a0f3842de6cf921e38573cb99fcaed2a4683c"
FINALISTS_PATH = "research/n83_root11_finalists_tier1_20261017/targets.jsonl"
FINALISTS_SHA256 = "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58"
FINALISTS_COPY = HERE / "finalists_historical_targets.jsonl"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def points(data):
    result = []
    for line in data.decode().splitlines():
        try:
            row = json.loads(line)
            if (isinstance(row, list) and len(row) == 2 and
                    all(isinstance(value, (str, int)) for value in row)):
                result.append(tuple(int(value) for value in row))
        except (ValueError, TypeError, json.JSONDecodeError):
            continue
    return result


def audit():
    new_blob = (HERE / "fresh_targets.jsonl").read_bytes()
    new = points(new_blob)
    if len(new) != 16 or len(set(new)) != 16:
        raise ValueError("high-K fixture count or distinctness differs")
    old = json.loads(AUDIT.read_text()) if AUDIT.exists() else None
    sealed_prior = json.loads(PRIOR_AUDIT.read_text())
    expected = {item["path"]: item["sha256"] for item in sealed_prior["prior_files"]}
    expected[PRIOR_FRONTIER] = hashlib.sha256((ROOT / PRIOR_FRONTIER).read_bytes()).hexdigest()
    prior_files, prior_points = [], set()
    for relative, expected_sha in sorted(expected.items()):
        blob = subprocess.check_output(["git", "show", f"HEAD:{relative}"], cwd=ROOT)
        if digest(blob) != expected_sha:
            raise ValueError(f"historical fixture changed: {relative}")
        rows = points(blob)
        prior_files.append({"path": relative,
                            "sha256": digest(blob), "point_rows": len(rows)})
        prior_points.update(rows)
    finalist_blob = FINALISTS_COPY.read_bytes()
    if digest(finalist_blob) != FINALISTS_SHA256:
        raise ValueError("historical finalist fixture hash differs")
    available = subprocess.run(
        ["git", "cat-file", "-e", f"{FINALISTS_HEAD}^{{commit}}"],
        cwd=ROOT, capture_output=True, check=False)
    if available.returncode == 0:
        complete = subprocess.run(
            ["git", "show", f"{FINALISTS_HEAD}:{FINALISTS_PATH}"],
            cwd=ROOT, capture_output=True, check=True)
        if complete.stdout != finalist_blob:
            raise ValueError("archived finalist copy differs from source commit")
    finalist_rows = points(finalist_blob)
    result = {
        "schema": "n83.highk-fresh-exclusion/v1",
        "fresh_targets_sha256": digest(new_blob),
        "fresh_count": len(new),
        "prior_files": prior_files,
        "prior_unique_point_count": len(prior_points),
        "prior_overlap_count": len(set(new) & prior_points),
        "finalists_head": FINALISTS_HEAD,
        "finalists_path": FINALISTS_PATH,
        "finalists_sha256": FINALISTS_SHA256,
        "finalists_point_count": len(finalist_rows),
        "finalists_overlap_count": len(set(new) & set(finalist_rows)),
    }
    result["status"] = ("PASS_ZERO_OVERLAP" if
                        result["prior_overlap_count"] ==
                        result["finalists_overlap_count"] == 0 else
                        "OVERLAP_FOUND")
    blob = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if old and AUDIT.read_text() != blob:
        raise ValueError("sealed exclusion audit differs")
    if not old:
        AUDIT.write_text(blob)
    print(json.dumps({key: result[key] for key in (
        "status", "fresh_targets_sha256", "fresh_count", "prior_unique_point_count",
        "prior_overlap_count", "finalists_point_count", "finalists_overlap_count")},
                     sort_keys=True))


if __name__ == "__main__":
    audit()
