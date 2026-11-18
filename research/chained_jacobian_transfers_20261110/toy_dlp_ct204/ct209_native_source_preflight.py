#!/usr/bin/env python3
"""Fast, offline Cargo bin-source and literal include-input check for CT204 CI.

This is a sparse-checkout scope check, not a Rust compile or correctness proof.
Only string-literal include!, include_str!, include_bytes! in manifest bin roots
and build.rs are checked. Dynamic macros, transitive modules and generated paths
must still be checked by the actual compiler.
"""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

_INCLUDE = re.compile(r'\binclude(?:_str|_bytes)?!\s*\(\s*"([^"\\]+)"\s*\)')


def check(root: Path) -> tuple[int, int, list[str]]:
    root = root.resolve()
    manifest = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    bins = manifest.get("bin", [])
    sources = [root / b["path"] for b in bins]
    # Cargo runs build.rs independently of the target graph.
    if (root / "build.rs").is_file():
        sources.append(root / "build.rs")
    missing: list[str] = []
    scanned = 0
    for source in sources:
        if not source.is_file():
            missing.append(f"missing registered source: {source.relative_to(root)}")
            continue
        contents = source.read_text(encoding="utf-8")
        for match in _INCLUDE.finditer(contents):
            # Avoid a common false-positive form in line comments. This is
            # not a complete Rust lexer; the compiler remains authoritative.
            line = contents[contents.rfind("\n", 0, match.start()) + 1 : match.start()]
            if line.lstrip().startswith("//"):
                continue
            scanned += 1
            reference = source.parent / match.group(1)
            if not reference.is_file():
                missing.append(
                    f"missing include input: {source.relative_to(root)} -> {match.group(1)}"
                )
    return len(bins), scanned, missing


def main() -> int:
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
    bins, includes, missing = check(root)
    print(f"CT209 preflight: {bins} manifest binaries, {includes} literal includes checked")
    for item in missing:
        print(f"ERROR: {item}", file=sys.stderr)
    if missing:
        print("CT209: sparse checkout incomplete; Rust test not invoked", file=sys.stderr)
    else:
        print("CT209: checkout source preflight passed (not Rust compilation)")
    return 1 if missing else 0


if __name__ == "__main__":
    raise SystemExit(main())
