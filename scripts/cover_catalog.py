"""The cover catalog: the registry rows docs/curves/cover-catalog.json names.

docs/curves/covers.json and docs/curves/cover-links.yaml (written by
`cargo run --bin curve_cover_check --`) bind themselves to these rows, not
to the whole of docs/curves/registry.json, so census curves the registry
builder sweeps in leave them current.  `rows_sha256` is the same digest the
Rust checker writes as `catalog_rows_sha256`: SHA-256 of the sorted-key
compact UTF-8 JSON of {"schema_version": ..., "curves": [rows]}.
"""
from __future__ import annotations

import hashlib
import json

MANIFEST = "docs/curves/cover-catalog.json"
SCHEMA = "curve-cover-catalog/v1"


def subset(registry: dict, manifest: dict) -> dict:
    """The named rows, in registry order, as a sub-registry."""
    if manifest.get("schema_version") != SCHEMA:
        raise ValueError(f"{MANIFEST}: unsupported schema")
    slugs = manifest["slugs"]
    wanted = set(slugs)
    if len(wanted) != len(slugs) or manifest.get("count") != len(slugs):
        raise ValueError(f"{MANIFEST}: duplicate slugs or a count that does not match them")
    rows = [c for c in registry["curves"] if c["slug"] in wanted]
    if len(rows) != len(wanted):
        missing = sorted(wanted - {c["slug"] for c in rows})[:5]
        raise ValueError(f"{MANIFEST} names slugs the registry does not hold: {missing}")
    return {"schema_version": registry["schema_version"], "curves": rows}


def rows_sha256(sub: dict) -> str:
    text = json.dumps(sub, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(text.encode()).hexdigest()
