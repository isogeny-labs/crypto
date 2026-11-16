#!/usr/bin/env python3
"""Extract exact archived N83 worker and bases for a higher-cap search."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import stat
import zipfile

HERE = Path(__file__).resolve().parent


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(blob: bytes) -> str:
    return hashlib.sha256(blob).hexdigest()


def prepare(archive: Path, output: Path) -> dict:
    dispatch_bytes = (HERE / "dispatch.json").read_bytes()
    dispatch = json.loads(dispatch_bytes)
    raw = archive.read_bytes()
    require(len(raw) == dispatch["provider_zip_bytes"]
            and sha256(raw) == dispatch["provider_zip_sha256"],
            "provider ZIP size or SHA-256 differs")
    expected = {
        "n83-m6-canonical-setup/worker":
            ("worker", dispatch["worker_sha256"]),
    }
    for key, digest in dispatch["source_archive_sha256"].items():
        expected[f"n83-archives/{key}.jsonl.gz"] = (f"{key}.jsonl.gz", digest)
    with zipfile.ZipFile(archive) as packed:
        members = packed.infolist()
        names = [member.filename for member in members]
        require(len(names) == dispatch["provider_member_count"]
                and len(names) == len(set(names)),
                "provider ZIP member coverage differs")
        require(all(not Path(name).is_absolute()
                    and ".." not in Path(name).parts
                    and not stat.S_ISLNK(member.external_attr >> 16)
                    for name, member in zip(names, members)),
                "unsafe provider ZIP member")
        require(set(expected).issubset(names), "required provider member absent")
        files = {target: packed.read(source) for source, (target, _) in expected.items()}
    require(all(sha256(files[target]) == digest
                for target, digest in expected.values()),
            "worker or base archive differs")
    output.mkdir(parents=True, exist_ok=False)
    for name, blob in files.items():
        destination = output / name
        destination.write_bytes(blob)
        if name == "worker":
            destination.chmod(0o755)
    receipt = {
        "schema": "n83.hosted-m6-highcap-input/v1",
        "status": "PASS_EXACT_ARCHIVED_INPUTS",
        "provider_run_id": dispatch["provider_run_id"],
        "provider_artifact_id": dispatch["provider_artifact_id"],
        "provider_zip_sha256": sha256(raw),
        "dispatch_sha256": sha256(dispatch_bytes),
        "files": {name: {"bytes": len(blob), "sha256": sha256(blob)}
                  for name, blob in sorted(files.items())},
        "selected_best_total_runtime": None,
    }
    (output / "input.json").write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(prepare(args.archive, args.output), sort_keys=True))


if __name__ == "__main__":
    main()
