#!/usr/bin/env python3
"""Fetch and replay the exact complete S3 panel inputs without listing S3."""

import argparse
import base64
import json
from pathlib import Path
import subprocess

from audit import HERE, audit, require


def fetch(out: Path) -> dict:
    manifest = json.loads((HERE / "manifest.json").read_bytes())
    require(manifest["schema"] == "n83.linux-xor-complete-inputs/v1",
            "manifest schema")
    out.mkdir(mode=0o700)
    prefix = ("factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/"
              "linux-xor-panel-20261025/inputs/")
    for entry in manifest["entries"]:
        key = entry["key"]
        require(key.startswith(prefix) and ".." not in Path(key).parts,
                "S3 key outside exact input prefix")
        destination = out / key[len(prefix):]
        destination.parent.mkdir(parents=True, exist_ok=True)
        result = subprocess.run(
            ["aws", "s3api", "get-object", "--bucket", manifest["bucket"],
             "--key", key, "--checksum-mode", "ENABLED", str(destination),
             "--output", "json"],
            capture_output=True, text=True, timeout=120, check=True,
        )
        response = json.loads(result.stdout)
        expected_checksum = base64.b64encode(
            bytes.fromhex(entry["sha256"])).decode("ascii")
        require(response.get("ChecksumSHA256") == expected_checksum
                and response.get("ServerSideEncryption") == "AES256",
                f"{key}: S3 checksum or encryption")
    return audit(out)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, required=True,
                        help="new, nonexistent directory for the 11 inputs")
    args = parser.parse_args()
    print(json.dumps(fetch(args.out), indent=2, sort_keys=True))
