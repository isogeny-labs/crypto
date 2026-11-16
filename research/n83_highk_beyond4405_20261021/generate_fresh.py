#!/usr/bin/env python3
"""Seal independent N83 screen points and single-target holdout points."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
COUNTS = {"screen": 1536, "holdout": 16}
DOMAINS = {
    "screen": b"n83-fixed31-screen1536-20261021-v1",
    "holdout": b"n83-fixed31-holdout16-20261021-v1",
}

spec = importlib.util.spec_from_file_location(
    "n83_fixed31_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def digest(blob: bytes) -> str:
    return hashlib.sha256(blob).hexdigest()


def write_once(path: Path, blob: bytes) -> None:
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(blob)


def generate() -> dict:
    seen = set()
    streams = {}
    for kind, count in COUNTS.items():
        target_rows = []
        answer_rows = []
        domain = DOMAINS[kind]
        for index in range(count):
            seed = hashlib.sha256(domain + index.to_bytes(4, "little")).digest()
            scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
            point = group.multiply(group.G, scalar)
            if (point is None or not group.on_curve(point) or
                    group.multiply(point, group.R) is not None or point in seen):
                raise ValueError(f"invalid or repeated {kind} point {index}")
            seen.add(point)
            target_rows.append(json.dumps([str(x) for x in point],
                                          separators=(",", ":")) + "\n")
            answer_rows.append(json.dumps({
                "fixture_index": index,
                "scalar": str(scalar),
                "seed_sha256": seed.hex(),
            }, separators=(",", ":"), sort_keys=True) + "\n")
        target_blob = "".join(target_rows).encode()
        answer_blob = "".join(answer_rows).encode()
        write_once(HERE / f"{kind}_targets.jsonl", target_blob)
        write_once(HERE / f"{kind}_answers.jsonl", answer_blob)
        streams[kind] = {
            "domain": domain.decode(),
            "point_count": count,
            "targets_sha256": digest(target_blob),
            "answers_sha256": digest(answer_blob),
        }
    manifest = {
        "schema": "n83.fixed31-fixture/v1",
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "subgroup_order": group.R,
        "scalar_rule": "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)",
        "generator_sha256": digest(Path(__file__).read_bytes()),
        "screen": streams["screen"],
        "holdout": streams["holdout"],
        "status": "SEALED_UNMEASURED",
    }
    write_once(HERE / "fresh_manifest.json",
               (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode())
    return manifest


if __name__ == "__main__":
    print(json.dumps(generate(), sort_keys=True))
