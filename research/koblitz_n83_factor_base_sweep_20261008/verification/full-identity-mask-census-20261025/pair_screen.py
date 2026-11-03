#!/usr/bin/env python3
"""Exact two-summand necessary-condition screen for N83 masks 4 and 5."""

import argparse
from collections import defaultdict
import gzip
import hashlib
import json
from pathlib import Path
import sys

from run import HERE, MANIFEST, PARENT, STUDY, file_sha, require

sys.path.insert(0, str(STUDY))
from chain_search_supervisor import parse_point, point_add  # noqa: E402


GROUP_ARITHMETIC_SHA256 = "fa0443e453e5fb7011019a08b61a219834e1594acc99e007f405f27e44b97a66"


def negate(point: tuple[int, int]) -> tuple[int, int]:
    x, y = point
    return x, x ^ y


def exact_points(archive: Path) -> set[tuple[int, int]]:
    points = set()
    with gzip.open(archive, "rt") as stream:
        for line in stream:
            row = json.loads(line)
            if "point" in row:
                point = parse_point(row["point"])
                require(point is not None, "affine factor-base point")
                points.add(point)
    require(len(points) == 99600 and all(negate(p) in points for p in points),
            "complete signed pool orbit point set")
    return points


def pair_hits(target: tuple[int, int], left: set[tuple[int, int]],
              right: set[tuple[int, int]]) -> int:
    require(target is not None, "affine target")
    return sum(point_add(target, negate(point)) in right for point in left)


def screen(census: dict, protocol: dict, parent: dict, manifest: dict,
           inputs: Path, census_sha256: str) -> dict:
    arithmetic = STUDY / "chain_search_supervisor.py"
    require(file_sha(arithmetic) == GROUP_ARITHMETIC_SHA256,
            "independently pinned Python group arithmetic")
    require(file_sha(MANIFEST) == protocol["input_manifest_sha256"]
            and manifest["curve"] == protocol["curve"]
            and census["schema"] == "n83.full-identity-mask-census/v1"
            and census["formula_count"] == len(census["rows"]) == 192
            and census["selected_best_total_runtime"] is None,
            "exact primary formula census and S3 manifest")
    archives = {Path(row["key"]).name: row for row in manifest["entries"]
                if row["kind"] == "archive"}
    require(len(archives) == 3, "three source archives")
    points = {}
    for name, row in archives.items():
        path = inputs / "archives" / name
        require(path.is_file() and path.stat().st_size == row["bytes"]
                and file_sha(path) == row["sha256"],
                f"{name}: exact cached S3 archive")
        points[name] = exact_points(path)
    base_name = f"s3-{parent['base_object_blake3']}.jsonl.gz"
    base = points[base_name]
    base_x = {p[0] for p in base}
    targets = defaultdict(set)
    for row in census["rows"]:
        target = parse_point(row["group_target"])
        require(target is not None, "affine public trial target")
        targets[row["trial"]].add(target)
    require(len(targets) == 3 and all(len(xs) == 1 for xs in targets.values()),
            "one exact group target per trial")
    results = []
    for pool in protocol["pools"]:
        name = f"s3-{parent['pool_objects_blake3'][pool]}.jsonl.gz"
        pool_points = points[name]
        residual = {p for p in pool_points if p[0] not in base_x}
        require(len(residual) == (99600 if pool == "hash" else 16102)
                and all(negate(p) in residual for p in residual),
                "exact signed-orbit residual difference")
        for trial, target_set in sorted(targets.items()):
            target = next(iter(target_set))
            results.append({"pool": pool, "trial": trial,
                            "base_points": len(base),
                            "pool_points": len(pool_points),
                            "residual_points": len(residual),
                            "base_plus_residual_ordered_pair_hits":
                                pair_hits(target, base, residual),
                            "residual_plus_residual_ordered_pair_hits":
                                pair_hits(target, residual, residual)})
    require(len(results) == 6, "all trial/pool pair-sum necessary gates")
    return {"schema": "n83.full-identity-mask-pair-screen/v1",
            "census_sha256": census_sha256,
            "input_manifest_sha256": protocol["input_manifest_sha256"],
            "group_arithmetic_sha256": GROUP_ARITHMETIC_SHA256,
            "rule": "masks 4 and 5 make the first four summands sum to O; the last two must sum to the public trial target",
            "results": results,
            "relation_search_executed": False,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--census", type=Path, default=HERE / "census.json")
    args = parser.parse_args()
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    parent = json.loads(PARENT.read_bytes())
    manifest = json.loads(MANIFEST.read_bytes())
    census = json.loads(args.census.read_bytes())
    result = screen(census, protocol, parent, manifest, args.inputs,
                    hashlib.sha256(args.census.read_bytes()).hexdigest())
    print(json.dumps(result, indent=2, sort_keys=True))
