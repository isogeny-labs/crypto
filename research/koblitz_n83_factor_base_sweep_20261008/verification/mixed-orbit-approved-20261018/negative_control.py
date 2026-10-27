#!/usr/bin/env python3
"""Reject a forged conflict count in a copied N83 mixed-orbit event."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import tempfile

import audit


def run(source: Path, small: Path, envelope: Path, scratch: Path) -> dict:
    with tempfile.TemporaryDirectory(prefix="n83-mixed-forged-event-", dir=scratch) as folder:
        copied = Path(folder) / "evidence"
        shutil.copytree(source, copied)
        event_path = copied / "run/run/events.jsonl"
        original = event_path.read_bytes()
        event = json.loads(original)
        assert event["search"]["conflicts"] == 5000
        event["search"]["conflicts"] = 4999
        event_path.write_text(json.dumps(event, sort_keys=True) + "\n")
        try:
            audit.audit(copied, small, envelope)
        except ValueError as error:
            if str(error) != "trial event ledger differs":
                raise
            return {
                "schema": "n83.mixed-orbit-approved-negative-control/v1",
                "status": "PASS_FORGED_EVENT_REJECTED",
                "forgery": "one retained event conflict count changed from 5000 to 4999",
                "original_event_sha256": hashlib.sha256(original).hexdigest(),
                "forged_event_sha256": hashlib.sha256(event_path.read_bytes()).hexdigest(),
                "rejection": str(error),
            }
        raise AssertionError("forged mixed-orbit event was accepted")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--small-object", type=Path, required=True)
    parser.add_argument("--envelope-object", type=Path, required=True)
    parser.add_argument("--scratch-parent", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = run(args.evidence.resolve(strict=True),
                 args.small_object.resolve(strict=True),
                 args.envelope_object.resolve(strict=True),
                 args.scratch_parent.resolve(strict=True))
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
