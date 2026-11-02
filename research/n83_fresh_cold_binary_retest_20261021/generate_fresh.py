#!/usr/bin/env python3
"""Seal 16 fresh public N83 points for the extended finalist holdout."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
DOMAIN = b"n83-fresh-cold-finalists16-shared-binary-20261021-v3"
COUNT = 16

spec = importlib.util.spec_from_file_location(
    "n83_crossfamily_cancel_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_once(path, data):
    if path.exists() and path.read_bytes() != data:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(data)


def generate():
    points, targets, answers = [], [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"fixture point {index} is outside subgroup")
        points.append(point)
        targets.append(json.dumps([str(value) for value in point],
                                  separators=(",", ":")) + "\n")
        answers.append(json.dumps({"fixture_index": index,
                                   "scalar": str(scalar),
                                   "seed_sha256": seed.hex()},
                                  separators=(",", ":"), sort_keys=True) + "\n")
    if len(set(points)) != COUNT:
        raise ValueError("duplicate fixture point")
    write_once(HERE / "fresh_targets.jsonl", "".join(targets).encode())
    write_once(HERE / "fresh_answers.jsonl", "".join(answers).encode())
    manifest = {
        "schema": "n83.fresh-cold-binary-retest-fixture/v1",
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "subgroup_order": group.R,
        "point_count": COUNT,
        "domain": DOMAIN.decode(),
        "scalar_rule": "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)",
        "targets_sha256": digest(HERE / "fresh_targets.jsonl"),
        "answers_sha256": digest(HERE / "fresh_answers.jsonl"),
        "generator_sha256": digest(Path(__file__)),
        "status": "SEALED_UNMEASURED",
    }
    write_once(HERE / "fresh_manifest.json",
               (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode())
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    generate()
