#!/usr/bin/env python3
"""Finish shard 18 custody after an interrupted pointer client, without a PUT."""

import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
from datetime import datetime, timezone

HERE = Path(__file__).resolve().parent
ROOT = Path("/mnt/spot-codex-state/tmp/ml-dataset-koblitz23-m3-15-enum-20261031")
INDEX = 18
BUCKET = "setec-astronomy-artifacts"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    marker = ROOT / "shard-18" / "attempt-start.json"
    status = ROOT / "shard-18" / "local-status.json"
    assert marker.is_file() and status.is_file()
    assert not (HERE / "checkpoints" / "shard-18.json").exists()
    archive = ROOT / "archives" / "shard-18.tar.gz"
    pointer = ROOT / "custody" / "shard-18.json"
    custody = json.loads(pointer.read_text())
    assert custody["index"] == INDEX and custody["archive_sha256"] == digest(archive.read_bytes())
    env = os.environ.copy()
    env.update(AWS_CONFIG_FILE="/tmp/ml-dataset-aws-config-20261028",
               AWS_PROFILE="ml-dataset-checkpoint", AWS_DEFAULT_REGION="us-east-2")
    checks = []
    for role, path in (("archive", archive), ("pointer", pointer)):
        local = path.read_bytes()
        sha = digest(local)
        key = f"store/sha256/{sha[:2]}/{sha}"
        with tempfile.TemporaryDirectory(prefix="koblitz23-shard18-readback-") as td:
            downloaded = Path(td) / "object"
            result = subprocess.run(["aws", "s3api", "get-object", "--bucket", BUCKET,
                                     "--key", key, str(downloaded)], env=env,
                                    capture_output=True, text=True, timeout=120, check=False)
            if result.returncode or not downloaded.is_file() or downloaded.read_bytes() != local:
                raise RuntimeError(f"{role} was absent or not byte-identical in S3; no local promotion")
        checks.append({"role": role, "s3_uri": f"s3://{BUCKET}/{key}",
                       "sha256": sha, "bytes": len(local), "fresh_get_byte_identical": True})
    published = ROOT / "custody" / "shard-18-published.json"
    value = {"schema": "ml-dataset.koblitz23-m3-15-enum-campaign-published/v1",
             "archive_sha256": checks[0]["sha256"], "pointer_sha256": checks[1]["sha256"],
             "archive_fresh_get": True, "pointer_fresh_get": True}
    encoded = (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()
    if published.exists():
        assert published.read_bytes() == encoded
    else:
        published.write_bytes(encoded)
    receipt = {"schema": "ml-dataset.koblitz23-shard18-interrupted-custody-reconciliation/v1",
               "index": INDEX, "observed_at": datetime.now(timezone.utc).isoformat(),
               "cause": "controller interrupted during pointer store client after sole native attempt",
               "new_s3_puts": 0, "new_native_attempts": 0, "objects": checks,
               "published_receipt_sha256": digest(encoded)}
    target = HERE / "S3-INTERRUPTED-CUSTODY-18.json"
    if target.exists():
        raise RuntimeError("immutable reconciliation receipt already exists")
    target.write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"index": INDEX, "both_s3_objects_verified": True,
                      "new_s3_puts": 0, "new_native_attempts": 0}, sort_keys=True))


if __name__ == "__main__":
    main()
