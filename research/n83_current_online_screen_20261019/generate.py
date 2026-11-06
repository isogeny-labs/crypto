#!/usr/bin/env python3
"""Freeze new public N83 points and an independently checked answer sidecar."""

import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ANALYZER = ROOT / "research/n83_indexed5_20261013/analyze.py"
ANALYZER_SHA256 = "e34d06b3275d86dcb3a659b79adf87dcdc078cebb6f510c4b5c42403d83a1804"
DOMAIN = b"n83-current-online-screen-fresh-8000-20261019-v1"
COUNT = 8000
PINNED_PRIOR = {
    "research/n83_compact4_crosslayout_20261016/targets.jsonl":
        "6c5558613d4f1115dc7bdb09b74d569645c0e85328ddad6555a2484317803a08",
    "research/n83_online_finalists_20261018/fresh_targets.jsonl":
        "bfd299b20bae05af249dea7941451d5fc6cbbebe44b483c2e84651cc4c2d3478",
    "research/n83_final_extended_20261018/fresh_targets.jsonl":
        "10a0cb758b3dfcff8b79e06a2437c3786ee03049dba7e096e7858024cdd9e811",
    "research/n83_root4_frontier_current_20261018/fresh_targets.jsonl":
        "fadc5caf48c0241c1bae6c7059de93abbb3cdc14e8900c4d3152b200b2e5ea25",
}


def module():
    if hashlib.sha256(ANALYZER.read_bytes()).hexdigest() != ANALYZER_SHA256:
        raise ValueError("independent group implementation changed")
    spec = importlib.util.spec_from_file_location("n83_online_screen_group", ANALYZER)
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    if not group.on_curve(group.G) or group.multiply(group.G, group.R) is not None:
        raise ValueError("generator or subgroup order differs")
    return group


def prior_points():
    seen = set()
    hashes = {}
    for relative, expected in PINNED_PRIOR.items():
        path = ROOT / relative
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != expected:
            raise ValueError(f"prior public panel changed: {relative}")
        hashes[relative] = actual
        for line in path.read_text().splitlines():
            point = json.loads(line)
            if not isinstance(point, list) or len(point) != 2:
                raise ValueError(f"malformed prior point: {relative}")
            seen.add(tuple(point))
    return seen, hashes


def bytes_for(rows):
    return "".join(json.dumps(row, separators=(",", ":")) + "\n"
                   for row in rows).encode()


def freeze(path, rows):
    blob = bytes_for(rows)
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"frozen fixture changed: {path}")
    if not path.exists():
        path.write_bytes(blob)
    return hashlib.sha256(blob).hexdigest()


def main():
    group = module()
    seen, prior_hashes = prior_points()
    points, answers = [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        encoded = [str(point[0]), str(point[1])]
        if tuple(encoded) in seen or not group.on_curve(point):
            raise ValueError(f"point {index} repeated or outside curve")
        seen.add(tuple(encoded))
        points.append(encoded)
        answers.append({"fixture_index": index, "point": encoded,
                        "scalar": str(scalar), "seed_sha256": seed.hex()})
    public_hash = freeze(HERE / "targets.jsonl", points)
    answer_hash = freeze(HERE / "answers.jsonl", answers)
    manifest = {"schema": "n83.current-online-screen-fixture/v1",
                "domain": DOMAIN.decode(), "point_count": COUNT,
                "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
                "prior_point_count": sum(len((ROOT / path).read_text().splitlines())
                                         for path in PINNED_PRIOR),
                "prior_sha256": prior_hashes, "targets_sha256": public_hash,
                "answers_sha256": answer_hash,
                "generator_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                "group_sha256": ANALYZER_SHA256}
    freeze(HERE / "fixture_manifest.json", [manifest])
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    main()
