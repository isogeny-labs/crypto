#!/usr/bin/env python3
"""Bind cross-family points against prior public N83 target fixtures."""

import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
AUDIT = HERE / "fresh_exclusion_audit.json"
FINALISTS_HEAD = "9b9a0f3842de6cf921e38573cb99fcaed2a4683c"
FINALISTS_PATH = "research/n83_root11_finalists_tier1_20261017/targets.jsonl"
FINALISTS_SHA256 = "4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58"


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
    if len(new) != 24 or len(set(new)) != 24:
        raise ValueError("fresh fixture count or distinctness differs")
    old = json.loads(AUDIT.read_text()) if AUDIT.exists() else None
    paths = ([ROOT / item["path"] for item in old["prior_files"]] if old else
             sorted(path for path in (ROOT / "research").glob("n83*/*target*.jsonl")
                    if path != HERE / "fresh_targets.jsonl"))
    prior_files, prior_points = [], set()
    for path in paths:
        blob = path.read_bytes()
        rows = points(blob)
        prior_files.append({"path": str(path.relative_to(ROOT)),
                            "sha256": digest(blob), "point_rows": len(rows)})
        prior_points.update(rows)
    prior_overlap = len(set(new) & prior_points)
    complete = subprocess.run(
        ["git", "show", f"{FINALISTS_HEAD}:{FINALISTS_PATH}"],
        cwd=ROOT, capture_output=True, check=True)
    if digest(complete.stdout) != FINALISTS_SHA256:
        raise ValueError("live finalist fixture hash differs")
    finalist_rows = points(complete.stdout)
    finalist_overlap = len(set(new) & set(finalist_rows))
    result = {
        "schema": "n83.crossfamily-cancel-exclusion/v1",
        "fresh_targets_sha256": digest(new_blob),
        "fresh_count": len(new),
        "prior_files": prior_files,
        "prior_unique_point_count": len(prior_points),
        "prior_overlap_count": prior_overlap,
        "finalists_head": FINALISTS_HEAD,
        "finalists_path": FINALISTS_PATH,
        "finalists_sha256": FINALISTS_SHA256,
        "finalists_point_count": len(finalist_rows),
        "finalists_overlap_count": finalist_overlap,
        "status": "PASS_ZERO_OVERLAP" if prior_overlap == finalist_overlap == 0
                  else "OVERLAP_FOUND",
    }
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
