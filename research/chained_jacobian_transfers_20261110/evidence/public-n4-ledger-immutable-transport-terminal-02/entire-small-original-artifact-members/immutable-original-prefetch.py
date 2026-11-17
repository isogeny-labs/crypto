#!/usr/bin/env python3
"""Batch-fetch changed PR blobs before trusted artifact migration.

Only object IDs from Git's raw tree diff are sent to the existing origin.
No PR code, path, hook or checkout command is executed. The canonical artifact
tool still selects policy files, uploads them and verifies remote bytes.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess

OID = re.compile(rb"(?:[0-9a-f]{40}|[0-9a-f]{64})\Z")


def git(root: Path, *args: str, data: bytes | None = None) -> bytes:
    env = dict(os.environ, GIT_NO_LAZY_FETCH="1")
    try:
        result = subprocess.run(
            ["git", "-C", str(root), *args],
            input=data, capture_output=True, env=env, timeout=180,
        )
    except subprocess.TimeoutExpired as exc:
        raise RuntimeError("Git artifact prefetch command exceeded 180 seconds") from exc
    if result.returncode:
        raise RuntimeError(
            f"Git artifact prefetch failed ({result.returncode}): "
            + result.stderr.decode(errors="replace")[-2000:]
        )
    return result.stdout


def changed_blobs(root: Path, base: str, head: str) -> list[bytes]:
    raw = git(root, "diff", "--raw", "--no-abbrev", "-z", "--no-renames",
              "--no-ext-diff", "--no-textconv", "--diff-filter=AM", base, head)
    fields = raw.split(b"\0")
    if fields[-1] != b"" or (len(fields) - 1) % 2:
        raise RuntimeError("Malformed raw Git diff")
    blobs = []
    for meta, _path in zip(fields[0:-1:2], fields[1:-1:2]):
        parts = meta.split()
        if len(parts) != 5 or not parts[0].startswith(b":"):
            raise RuntimeError("Malformed raw Git record")
        mode, oid, status = parts[1], parts[3], parts[4]
        if not OID.fullmatch(oid) or status not in {b"A", b"M"}:
            raise RuntimeError("Unexpected raw Git object or status")
        if mode == b"160000":
            continue
        if mode not in {b"100644", b"100755", b"120000"}:
            raise RuntimeError("Unexpected Git file mode")
        blobs.append(oid)
    return list(dict.fromkeys(blobs))


def missing_blobs(root: Path, blobs: list[bytes]) -> list[bytes]:
    if not blobs:
        return []
    output = git(root, "cat-file",
                 "--batch-check=%(objectname) %(objecttype) %(objectsize)",
                 data=b"\n".join(blobs) + b"\n")
    lines = output.splitlines()
    if len(lines) != len(blobs):
        raise RuntimeError("Git did not return every object-size result")
    missing = []
    for oid, line in zip(blobs, lines):
        answer = line.split()
        if answer == [oid, b"missing"]:
            missing.append(oid)
        elif (len(answer) != 3 or answer[0] != oid or answer[1] != b"blob"
              or not answer[2].isdigit()):
            raise RuntimeError("Git returned an unexpected object-size result")
    return missing


def prefetch(root: Path, base: str, head: str, batch_size: int = 256) -> dict:
    if not 1 <= batch_size <= 256:
        raise ValueError("batch size must be between 1 and 256")
    blobs = changed_blobs(root, base, head)
    missing = missing_blobs(root, blobs)
    batches = 0
    for start in range(0, len(missing), batch_size):
        batch = missing[start:start + batch_size]
        git(root, "-c", "fetch.negotiationAlgorithm=noop", "fetch", "--no-tags",
            "--no-write-fetch-head", "--no-prune", "--recurse-submodules=no",
            "--filter=blob:none", "--stdin", "origin",
            data=b"\n".join(batch) + b"\n")
        batches += 1
    if missing_blobs(root, blobs):
        raise RuntimeError("Changed blobs are still missing after prefetch")
    return {"changed_blobs": len(blobs), "missing_blobs": len(missing), "batches": batches}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base")
    parser.add_argument("head")
    args = parser.parse_args()
    try:
        print(json.dumps(prefetch(Path.cwd(), args.base, args.head), sort_keys=True))
    except (RuntimeError, ValueError) as exc:
        parser.exit(1, f"artifact prefetch: {exc}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
