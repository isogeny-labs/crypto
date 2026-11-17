#!/usr/bin/env python3
import concurrent.futures
import hashlib
import json
import subprocess
import sys
from pathlib import Path

index_path = Path(sys.argv[1])
root = Path(sys.argv[2])
index = json.loads(index_path.read_text())
files = [entry for entry in index["files"] if entry["path"].endswith(".zst")]
if len(files) != 384:
    raise SystemExit(f"expected 384 point shards, found {len(files)}")

def fetch(entry):
    rel = entry["path"].removeprefix("data/")
    output = root / rel
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["aws", "s3", "cp", entry["snapshot_key"], str(output), "--region", "us-east-2", "--only-show-errors"],
        check=True,
    )
    digest = hashlib.sha256(output.read_bytes()).hexdigest()
    if output.stat().st_size != entry["bytes"] or digest != entry["sha256"]:
        raise RuntimeError(f"size/hash mismatch for {entry['path']}")
    return entry["bytes"]

with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:
    sizes = list(pool.map(fetch, files))
print(json.dumps({"verified_shards": len(sizes), "verified_bytes": sum(sizes)}, sort_keys=True))
