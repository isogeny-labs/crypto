#!/usr/bin/env python3
"""Freeze exact ic.prime-workload/v1 inputs from the selected PKM roster.

This is a deterministic format conversion. The ic workload loader must still
validate curve arithmetic and every target before any measured invocation.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROSTER_SHA256 = "271fdaf1b23b87fb5b2b8a6ecb7ac1f6492e146d7e8b4deb2f6f251d618d9123"
ROSTER_SCHEMA = "pkm-native-f4-roster/v1"
WORKLOAD_SCHEMA = "ic.prime-workload/v1"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def encoded(document):
    return (json.dumps(document, indent=2, sort_keys=True) + "\n").encode("utf-8")


def preserve(path, data):
    if path.exists():
        if path.read_bytes() != data:
            raise ValueError(f"frozen file differs: {path}")
        return
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def point(pair):
    if len(pair) != 2 or any(not isinstance(x, int) or x < 0 for x in pair):
        raise ValueError("point must contain two nonnegative integers")
    return {"x": hex(pair[0]), "y": hex(pair[1])}


def workload(row):
    p, r, group_order = (row[key] for key in ("p", "r", "group_order"))
    t, curve_index = row["t"], row["curve_index"]
    if group_order != r or t not in (3, 4, 5) or curve_index not in (0, 1, 2):
        raise ValueError("unexpected curve selection or subgroup order")
    targets = sorted(row["targets"], key=lambda target: target["target_index"])
    if [target["target_index"] for target in targets] != list(range(16)):
        raise ValueError("expected exactly 16 indexed targets")
    scalars = [target["scalar"] for target in targets]
    if len(set(scalars)) != 16 or any(not 0 < scalar < r for scalar in scalars):
        raise ValueError("target scalars must be distinct and in 1..r")
    return {
        "schema": WORKLOAD_SCHEMA,
        "curve": {
            "field": {"kind": "prime", "modulus": hex(p)},
            "a": hex(row["a"]),
            "b": hex(row["b"]),
            "group_order": hex(group_order),
            "subgroup_order": hex(r),
            "cofactor": hex(group_order // r),
            "generator": point(row["generator"]),
        },
        "sampling": (
            f"{ROSTER_SCHEMA}; roster_sha256={ROSTER_SHA256}; "
            f"t={t}; curve_index={curve_index}; 16 frozen targets"
        ),
        "targets": [
            {"scalar": target["scalar"], "point": point(target["point"])}
            for target in targets
        ],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--roster", type=Path, default=HERE / "roster.json")
    parser.add_argument("--out-dir", type=Path, default=HERE / "workloads")
    args = parser.parse_args()
    roster_bytes = args.roster.read_bytes()
    if digest(roster_bytes) != ROSTER_SHA256:
        parser.error("roster SHA-256 differs from the frozen selection")
    roster = json.loads(roster_bytes)
    if roster.get("schema") != ROSTER_SCHEMA or len(roster.get("curves", [])) != 9:
        parser.error("roster schema or curve count differs")
    args.out_dir.mkdir(parents=True, exist_ok=True)
    entries = []
    seen = set()
    for row in sorted(roster["curves"], key=lambda curve: (curve["t"], curve["curve_index"])):
        key = (row["t"], row["curve_index"])
        if key in seen:
            raise ValueError(f"duplicate curve {key}")
        seen.add(key)
        name = f"t{key[0]}_c{key[1]}.json"
        document = workload(row)
        data = encoded(document)
        preserve(args.out_dir / name, data)
        entries.append({
            "path": name,
            "sha256": digest(data),
            "t": key[0],
            "curve_index": key[1],
            "p": row["p"],
            "r": row["r"],
            "g": row["g"],
            "target_scalars": [target["scalar"] for target in document["targets"]],
        })
    manifest = {
        "schema": "pkm-f4-prime-workloads/v1",
        "roster_sha256": ROSTER_SHA256,
        "workload_schema": WORKLOAD_SCHEMA,
        "entries": entries,
        "status": "frozen inputs; arithmetic validation by the ic workload loader pending",
    }
    preserve(args.out_dir / "manifest.json", encoded(manifest))
    print(json.dumps({"curves": len(entries), "targets": sum(len(e["target_scalars"]) for e in entries),
                      "manifest_sha256": digest(encoded(manifest))}, sort_keys=True))


if __name__ == "__main__":
    main()
