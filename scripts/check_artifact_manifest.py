#!/usr/bin/env python3
"""Check the latest SHA-256 receipt for each path in an append-only manifest.

Older entries remain in the manifest as dated evidence, but a path can have
only one current byte sequence. The last entry for that path is its current
receipt. Every entry is parsed, and every current path must exist and match.
"""

import argparse
import hashlib
import re
from pathlib import Path


LINE = re.compile(r"([0-9a-f]{64})  (.+)")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    parser.add_argument("--base", type=Path, default=Path("."))
    args = parser.parse_args()
    base = args.base.resolve()
    current: dict[str, str] = {}
    total = 0
    for number, line in enumerate(args.manifest.read_text().splitlines(), 1):
        match = LINE.fullmatch(line)
        if match is None:
            parser.error(f"{args.manifest}:{number}: invalid SHA-256 receipt")
        digest, name = match.groups()
        file = (base / name).resolve()
        if not file.is_relative_to(base) or file == base:
            parser.error(f"{args.manifest}:{number}: path escapes base: {name}")
        current[name] = digest
        total += 1

    if not current:
        parser.error(f"{args.manifest}: empty artifact manifest")
    failures = 0
    for name, expected in current.items():
        file = base / name
        if not file.is_file():
            print(f"{name}: MISSING")
            failures += 1
            continue
        digest = hashlib.sha256()
        with file.open("rb") as handle:
            for chunk in iter(lambda: handle.read(1024 * 1024), b""):
                digest.update(chunk)
        if digest.hexdigest() != expected:
            print(f"{name}: FAILED")
            failures += 1
    print(
        f"{len(current)} current artifact hashes checked; "
        f"{total - len(current)} earlier receipts retained; {failures} failures"
    )
    return int(failures != 0)


if __name__ == "__main__":
    raise SystemExit(main())
