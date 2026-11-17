#!/usr/bin/env python3
"""Publish the immutable r36 failure inventory and structure records to S3."""
import concurrent.futures
import hashlib
import json
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parent
job = "n83-fb-variants-r36-20261031-r1"
bucket = "isolab-results-104186811093-us-east-2"
prefix = f"runs/{job}/evidence/terminal-failure-20261101T0715Z"
files = []
for path in sorted(root.rglob("*")):
    if not path.is_file() or path.name in {"PERSISTENCE_INDEX.json", "publication-receipt.json"}:
        continue
    data = path.read_bytes()
    rel = path.relative_to(root).as_posix()
    sha = hashlib.sha256(data).hexdigest()
    path_sha = hashlib.sha256(rel.encode()).hexdigest()
    key = f"{prefix}/snapshots/{path_sha}/{sha}"
    files.append({"path": rel, "bytes": len(data), "sha256": sha, "s3_uri": f"s3://{bucket}/{key}"})

index = {"schema": "n83.factor-base-failure-evidence-index/v1", "job_name": job,
         "job_uid": "e97b3bf9-a613-4645-a07e-4c9e19a1cba3", "files": files,
         "layout": "listing-chunks preserve paginated object-list slices; s3-object-listing.json is their ordered reconstruction; RUN_OUTPUT_FORMAT.json describes the intended data layout"}
raw = (json.dumps(index, indent=2) + "\n").encode()
index_sha = hashlib.sha256(raw).hexdigest()
(root / "PERSISTENCE_INDEX.json").write_bytes(raw)

def upload(item):
    result = subprocess.run(["aws", "s3", "cp", str(root / item["path"]), item["s3_uri"], "--region", "us-east-2", "--only-show-errors"], capture_output=True, text=True)
    return item["path"], result.returncode, result.stderr

with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    outcomes = list(pool.map(upload, files))
failures = [{"path": p, "exit_code": rc, "stderr": err} for p, rc, err in outcomes if rc]
index_uri = f"s3://{bucket}/{prefix}/index/PERSISTENCE_INDEX-{index_sha}.json"
if not failures:
    local = root / "PERSISTENCE_INDEX.json"
    result = subprocess.run(["aws", "s3", "cp", str(local), index_uri, "--region", "us-east-2", "--only-show-errors"], capture_output=True, text=True)
    if result.returncode:
        failures.append({"path": "PERSISTENCE_INDEX.json", "exit_code": result.returncode, "stderr": result.stderr})
receipt = {"schema": "n83.factor-base-failure-evidence-publication/v1", "job_name": job,
           "planned_files": len(files), "uploaded_files": sum(1 for _, rc, _ in outcomes if rc == 0),
           "index_sha256": index_sha, "index_uri": index_uri, "failures": failures}
(root / "publication-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt, indent=2))
raise SystemExit(1 if failures else 0)
