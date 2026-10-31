#!/usr/bin/env python3
"""Freeze disjoint N83 distribution-screen and one-target holdout fixtures."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SCREEN_COUNT = 192
HOLDOUT_COUNT = 8
DOMAINS = {
    "screen": b"n83-online-distribution-screen192-20261020-v1",
    "holdout": b"n83-online-distribution-holdout8-20261020-v1",
}

spec = importlib.util.spec_from_file_location(
    "n83_distribution_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
group = importlib.util.module_from_spec(spec)
spec.loader.exec_module(group)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_once(path, value):
    if path.exists() and path.read_bytes() != value:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(value)


def rows(kind, count):
    targets, answers, points = [], [], []
    domain = DOMAINS[kind]
    for index in range(count):
        seed = hashlib.sha256(domain + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"{kind} point {index} is outside the subgroup")
        points.append(point)
        targets.append(json.dumps([str(x) for x in point], separators=(",", ":")) + "\n")
        answers.append(json.dumps({
            "fixture_index": index, "scalar": str(scalar),
            "seed_sha256": seed.hex(),
        }, separators=(",", ":"), sort_keys=True) + "\n")
    if len(set(points)) != count:
        raise ValueError(f"duplicate {kind} fixture point")
    return points, "".join(targets).encode(), "".join(answers).encode()


def generate():
    records = {}
    all_points = set()
    for kind, count in (("screen", SCREEN_COUNT), ("holdout", HOLDOUT_COUNT)):
        points, targets, answers = rows(kind, count)
        if all_points.intersection(points):
            raise ValueError("screen and holdout share a point")
        all_points.update(points)
        target_path = HERE / f"{kind}_targets.jsonl"
        answer_path = HERE / f"{kind}_answers.jsonl"
        write_once(target_path, targets)
        write_once(answer_path, answers)
        records[kind] = {
            "domain": DOMAINS[kind].decode(), "point_count": count,
            "targets_sha256": sha256(target_path),
            "answers_sha256": sha256(answer_path),
        }
    manifest = {
        "schema": "n83.online-distribution-fixture/v1",
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "subgroup_order": group.R,
        "scalar_rule": "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)",
        "generator_sha256": sha256(Path(__file__)),
        "screen": records["screen"], "holdout": records["holdout"],
        "status": "SEALED_UNMEASURED",
    }
    write_once(HERE / "fresh_manifest.json",
               (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode())
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    generate()
