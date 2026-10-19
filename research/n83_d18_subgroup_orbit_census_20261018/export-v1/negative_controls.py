#!/usr/bin/env python3
"""Rehash forged N83 objects and require the independent replay to reject them.

Requires blake3==1.0.8. The two semantic forgeries have internally consistent
object names, byte counts, and compressed/plain digests; the archive forgery
tests that changed compressed bytes fail before decompression.
"""

from __future__ import annotations

import gzip
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

import blake3


def digests(path: Path, compressed: bool) -> tuple[str, str, int]:
    b3, sha, size = blake3.blake3(), hashlib.sha256(), 0
    opener = path.open if compressed else lambda mode: gzip.open(path, mode)
    with opener("rb") as stream:
        while block := stream.read(1 << 20):
            b3.update(block)
            sha.update(block)
            size += len(block)
    return b3.hexdigest(), sha.hexdigest(), size


def forge_semantic(source: Path, root: Path, manifest: dict, mode: str) -> tuple[Path, dict]:
    root.mkdir()
    (root / "objects").mkdir()
    temporary = root / "objects" / "partial.gz"
    changed = False
    with gzip.open(source, "rb") as original, temporary.open("wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as output:
            for index, line in enumerate(original):
                if (mode == "header" and index == 0) or (mode == "row" and index == 1):
                    record = json.loads(line)
                    if mode == "header":
                        assert record["curve_a"] == 0
                        record["curve_a"] = 1
                    else:
                        assert record["coefficient"] == "1"
                        record["coefficient"] = "2"
                    line = json.dumps(record, sort_keys=True, separators=(",", ":")).encode() + b"\n"
                    changed = True
                output.write(line)
    assert changed
    compressed_b3, compressed_sha, compressed_size = digests(temporary, True)
    plain_b3, plain_sha, _ = digests(temporary, False)
    object_name = f"objects/{compressed_b3}.jsonl.gz"
    object_path = root / object_name
    temporary.rename(object_path)
    forged = dict(manifest)
    forged.update(
        object=object_name,
        compressed_blake3=compressed_b3,
        compressed_sha256=compressed_sha,
        plain_blake3=plain_b3,
        plain_sha256=plain_sha,
        bytes=compressed_size,
    )
    (root / "manifest.json").write_text(json.dumps(forged, indent=2, sort_keys=True) + "\n")
    return object_path, forged


def main() -> None:
    if len(sys.argv) != 5:
        raise SystemExit("usage: negative_controls.py GOOD_DIR CONTROL_DIR REPLAYER RECEIPT")
    good, controls, replayer, receipt = map(Path, sys.argv[1:])
    manifest = json.loads((good / "manifest.json").read_text())
    source = good / manifest["object"]
    controls.mkdir()
    records = []
    for mode in ("header", "row", "archive"):
        root = controls / mode
        if mode == "archive":
            root.mkdir()
            (root / "objects").mkdir()
            object_path = root / manifest["object"]
            shutil.copyfile(source, object_path)
            with object_path.open("r+b") as stream:
                stream.seek(-1, 2)
                original = stream.read(1)
                stream.seek(-1, 2)
                stream.write(bytes([original[0] ^ 1]))
            (root / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
            expected = "corrupted archive rejected"
        else:
            object_path, _ = forge_semantic(source, root, manifest, mode)
            expected = "object header or curve identity differs" if mode == "header" else "generic row point, label or uniqueness check failed"
        run = subprocess.run([str(replayer), "replay", str(root)], capture_output=True, text=True, timeout=1800)
        observed = run.stdout + run.stderr
        record = {
            "mode": mode,
            "mutation": {"header": "curve_a 0 to 1", "row": "first coefficient 1 to 2", "archive": "last compressed byte xor 1"}[mode],
            "object": str(object_path.relative_to(controls)),
            "compressed_sha256": digests(object_path, True)[1],
            "exit_code": run.returncode,
            "expected_rejection": expected,
            "observed": observed.strip(),
            "pass": run.returncode != 0 and (mode == "archive" or expected in observed),
        }
        records.append(record)
        receipt.write_text(json.dumps({"schema": "n83.d18-negative-controls/v1", "controls": records}, indent=2) + "\n")
        if not record["pass"]:
            raise SystemExit(f"{mode} control failed: {observed}")
    print(json.dumps({"status": "PASS", "controls": len(records)}))


if __name__ == "__main__":
    main()
