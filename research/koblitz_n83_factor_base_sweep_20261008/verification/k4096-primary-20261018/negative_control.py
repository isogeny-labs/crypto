#!/usr/bin/env python3
"""Test that the K4096 auditor rejects a rehashed false replay receipt."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import tempfile

import audit


def write(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def run(source: Path, binary: Path, scratch_parent: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="k4096-forged-replay-",
                                     dir=scratch_parent) as scratch:
        copied = Path(scratch) / "retained-evidence"
        shutil.copytree(source, copied)
        replay_path = copied / "construction-01/object/replay.json"
        replay = json.loads(replay_path.read_text())
        assert replay["checks"][0]["points_checked"] == 679936
        original_sha256 = hashlib.sha256(replay_path.read_bytes()).hexdigest()
        replay["checks"][0]["points_checked"] = 679935
        write(replay_path, replay)
        forged_sha256 = hashlib.sha256(replay_path.read_bytes()).hexdigest()

        outer_path = copied / "replay-01/outer.json"
        outer = json.loads(outer_path.read_text())
        assert outer["stage_receipt_sha256"] == original_sha256
        outer["stage_receipt_sha256"] = forged_sha256
        write(outer_path, outer)

        try:
            audit.audit(copied, binary)
        except ValueError as error:
            if str(error) != "replay stage receipt differs":
                raise
            return {
                "schema": "n83.k4096-negative-control/v1",
                "status": "PASS_FORGED_REPLAY_REJECTED",
                "forgery": "replay point count altered and outer receipt SHA-256 recomputed",
                "original_replay_sha256": original_sha256,
                "forged_replay_sha256": forged_sha256,
                "rejection": str(error),
            }
        raise AssertionError("forged K4096 replay receipt was accepted")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--scratch-parent", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run(args.source.resolve(strict=True), args.binary.resolve(strict=True),
                 args.scratch_parent.resolve(strict=True))
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
