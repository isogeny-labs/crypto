#!/usr/bin/env python3
"""Check cheap source bindings before native catalog checks run on main."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys


def check_catalog(root: Path) -> None:
    catalog = root / "docs/curves"
    registry_bytes = (catalog / "registry.json").read_bytes()
    registry = json.loads(registry_bytes)
    report = json.loads((catalog / "covers.json").read_bytes())
    graph = json.loads((catalog / "cover-links.yaml").read_bytes())
    registry_sha = hashlib.sha256(registry_bytes).hexdigest()
    for name, document in (("covers.json", report), ("cover-links.yaml", graph)):
        if document.get("registry_sha256") != registry_sha:
            raise ValueError(f"{name} binds a stale registry; regenerate native cover metadata")
    sources = [
        "src/bin/curve_cover_check/checker.rs",
        "src/bin/curve_cover_check/models.rs",
        "src/bin/curve_cover_check/links.rs",
        "src/bin/curve_cover_check.rs",
    ]
    source_sha = hashlib.sha256(b"".join((root / p).read_bytes() for p in sources)).hexdigest()
    if report.get("checker_source_sha256") != source_sha:
        raise ValueError("covers.json binds stale checker source; regenerate native cover metadata")
    if len(report["curves"]) != len(registry["curves"]):
        raise ValueError("cover report does not cover every registry row")
    summary = report["summary"]
    if sum(summary[k] for k in ("verified", "unsupported", "invalid_input")) != len(report["curves"]):
        raise ValueError("cover summary does not account for every report row")


if __name__ == "__main__":
    try:
        check_catalog(Path(__file__).resolve().parents[1])
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"curve catalog consistency: {error}", file=sys.stderr)
        sys.exit(1)
    print("Curve catalog source bindings and row coverage are current; native proof checks remain separate.")
