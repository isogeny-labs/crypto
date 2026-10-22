#!/usr/bin/env python3
"""Freeze 1,536 independent public N83 points for the conditional tie-break."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
COUNT = 1536
DOMAIN = b"n83-online-tiebreak-screen1536-20261020-v1"

spec = importlib.util.spec_from_file_location(
    "n83_tiebreak_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_once(path, blob):
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(blob)


def generate():
    targets, answers, points = [], [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"screen point {index} is outside the subgroup")
        points.append(point)
        targets.append(json.dumps([str(x) for x in point],
                                  separators=(",", ":")) + "\n")
        answers.append(json.dumps({
            "fixture_index": index, "scalar": str(scalar),
            "seed_sha256": seed.hex(),
        }, separators=(",", ":"), sort_keys=True) + "\n")
    if len(set(points)) != COUNT:
        raise ValueError("duplicate screen point")
    target_path = HERE / "screen_targets.jsonl"
    answer_path = HERE / "screen_answers.jsonl"
    write_once(target_path, "".join(targets).encode())
    write_once(answer_path, "".join(answers).encode())
    manifest = {
        "schema": "n83.online-tiebreak-fixture/v1",
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "subgroup_order": group.R,
        "domain": DOMAIN.decode(),
        "point_count": COUNT,
        "scalar_rule": "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)",
        "generator_sha256": digest(Path(__file__)),
        "targets_sha256": digest(target_path),
        "answers_sha256": digest(answer_path),
        "status": "SEALED_UNMEASURED",
    }
    write_once(HERE / "fresh_manifest.json",
               (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode())
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    generate()
