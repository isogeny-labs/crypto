#!/usr/bin/env python3
"""Publish the r36 recovery bundle as content-addressed S3 evidence."""
import concurrent.futures
import hashlib
import json
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parent
job = "n83-fb-variants-r36-20261031-r1"
bucket = "isolab-results-104186811093-us-east-2"
prefix = f"runs/{job}/evidence/terminal-recovery-20261101T1751Z"
files = []
for path in sorted(root.rglob("*")):
    if not path.is_file() or path.name in {"PERSISTENCE_INDEX.json", "publication-receipt.json"}:
        continue
    raw = path.read_bytes()
    rel = path.relative_to(root).as_posix()
    sha = hashlib.sha256(raw).hexdigest()
    path_sha = hashlib.sha256(rel.encode()).hexdigest()
    files.append({"path": rel, "bytes": len(raw), "sha256": sha,
                  "s3_uri": f"s3://{bucket}/{prefix}/snapshots/{path_sha}/{sha}"})
index = {"schema": "n83.factor-base-recovery-persistence/v1", "job_name": job,
         "files": files,
         "structure": "recovered-s3-file-index.json maps original factor-base output paths to the exact immutable S3 snapshots; recovered/ contains the 400 hash-verified non-compressed output files; compressed shards stay in the original attempt prefix and are hash/size bound by family manifests"}
raw_index = (json.dumps(index, indent=2) + "\n").encode()
index_sha = hashlib.sha256(raw_index).hexdigest()
(root / "PERSISTENCE_INDEX.json").write_bytes(raw_index)

def cp(local, uri):
    r = subprocess.run(["aws", "s3", "cp", str(local), uri, "--region", "us-east-2", "--only-show-errors"], capture_output=True, text=True)
    return r.returncode, r.stderr

def put(item):
    return item["path"], *cp(root / item["path"], item["s3_uri"])
with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:
    results = list(pool.map(put, files))
failures = [{"path": p, "exit_code": code, "stderr": err} for p, code, err in results if code]
index_uri = f"s3://{bucket}/{prefix}/index/PERSISTENCE_INDEX-{index_sha}.json"
if not failures:
    code, err = cp(root / "PERSISTENCE_INDEX.json", index_uri)
    if code:
        failures.append({"path": "PERSISTENCE_INDEX.json", "exit_code": code, "stderr": err})
receipt = {"schema": "n83.factor-base-recovery-publication/v1", "job_name": job,
           "planned_files": len(files), "uploaded_files": sum(code == 0 for _, code, _ in results),
           "index_sha256": index_sha, "index_uri": index_uri, "failures": failures}
(root / "publication-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
receipt_sha = hashlib.sha256((root / "publication-receipt.json").read_bytes()).hexdigest()
receipt_uri = f"s3://{bucket}/{prefix}/receipts/publication-receipt-{receipt_sha}.json"
if not failures:
    code, err = cp(root / "publication-receipt.json", receipt_uri)
    if code:
        failures.append({"path": "publication-receipt.json", "exit_code": code, "stderr": err})
        receipt["failures"] = failures
        (root / "publication-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({**receipt, "receipt_uri": receipt_uri}, indent=2))
raise SystemExit(1 if failures else 0)
