#!/usr/bin/env python3
"""Reject altered paired-run query, phase, and archive artifacts."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import tempfile

import audit_pair

CASES = (
    ("public_query_x", "out/native-paired/arms/a1-k2048/worker/events.jsonl"),
    ("terminal_conflicts", "out/native-paired/arms/b1-k4096/worker.stderr"),
    ("archive_byte", "out/native-paired/panel-k2048/objects/"
     "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz"),
)


def tamper(name: str, path: Path) -> None:
    original = path.read_bytes()
    path.unlink()  # The rest of the tree uses hard links; isolate this file.
    if name == "public_query_x":
        rows = [json.loads(line) for line in original.decode().splitlines()]
        matches = [row for row in rows if "RelationQueryStarted" in row.get("event", "")]
        assert len(matches) == 1
        event = matches[0]["event"]
        match = re.search(r'target_x: "([0-9]+)"', event)
        assert match is not None
        matches[0]["event"] = (event[:match.start(1)] + str(int(match.group(1)) + 1)
                               + event[match.end(1):])
        path.write_text("".join(json.dumps(row, sort_keys=True) + "\n" for row in rows))
    elif name == "terminal_conflicts":
        assert original.count(b'"conflicts":1000') == 1
        path.write_bytes(original.replace(b'"conflicts":1000', b'"conflicts":999'))
    elif name == "archive_byte":
        content = bytearray(original)
        content[len(content) // 2] ^= 1
        path.write_bytes(content)
    else:
        raise ValueError(name)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fetched", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    rows = []
    for name, relative in CASES:
        with tempfile.TemporaryDirectory(prefix="n83-paired-forgery-",
                                         dir=args.fetched.parent) as temp:
            clone = Path(temp) / "fetched"
            shutil.copytree(args.fetched, clone, copy_function=os.link)
            tamper(name, clone / relative)
            try:
                audit_pair.audit(clone)
            except ValueError as error:
                rows.append({"mutation": name, "status": "PASS_TAMPER_REJECTED",
                             "reason": str(error)})
            else:
                raise AssertionError(f"forged {name} passed the independent audit")
    result = {"schema": "n83.native-paired-negative-controls/v1",
              "status": "PASS_ALL_TAMPER_REJECTED", "controls": rows,
              "selected_best_total_runtime": None}
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
