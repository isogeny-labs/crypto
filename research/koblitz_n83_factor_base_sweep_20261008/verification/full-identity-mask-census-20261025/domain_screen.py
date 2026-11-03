#!/usr/bin/env python3
"""Prove final-identity exclusions from exact N83 pool archive x-domains."""

import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path

from analyze import status as structural_status
from run import HERE, MANIFEST, PARENT, file_sha, require, specs


def pool_x_domain(archive: Path) -> set[str]:
    xs = set()
    with gzip.open(archive, "rt") as stream:
        for line in stream:
            row = json.loads(line)
            if "point" in row:
                point = row["point"]
                require(isinstance(point, list) and len(point) == 2
                        and all(isinstance(value, str) and value.isdigit()
                                and 0 <= int(value) < 1 << 83 for value in point),
                        "rational binary-field point representation")
                xs.add(point[0])
    require(xs, "nonempty pool x-domain")
    return xs


def classify(mask: int, target_in_pool: bool) -> str:
    if structural_status(mask) == "UNSAT_BY_ADJACENT_IDENTITIES":
        return "UNSAT_BY_ADJACENT_IDENTITIES"
    if mask & 8 and not target_in_pool:
        return "UNSAT_TARGET_X_ABSENT_FROM_POOL"
    return "NO_PROVEN_IMPOSSIBILITY_FROM_THESE_GATES"


def screen(census: dict, protocol: dict, parent: dict, manifest: dict,
           inputs: Path, census_sha256: str) -> dict:
    require(file_sha(MANIFEST) == protocol["input_manifest_sha256"]
            and manifest["curve"] == protocol["curve"],
            "exact primary S3 manifest")
    require(census["schema"] == "n83.full-identity-mask-census/v1"
            and census["formula_count"] == len(census["rows"]) == 192
            and census["selected_best_total_runtime"] is None,
            "complete formula-only census")
    archive_rows = {Path(row["key"]).name: row for row in manifest["entries"]
                    if row["kind"] == "archive"}
    require(len(archive_rows) == 3, "three source archives")
    archives = {}
    for name, row in archive_rows.items():
        path = inputs / "archives" / name
        require(path.is_file() and path.stat().st_size == row["bytes"]
                and file_sha(path) == row["sha256"],
                f"{name}: exact cached S3 archive")
        archives[name] = path
    pool_domains = {}
    pool_receipts = []
    for pool in protocol["pools"]:
        pool_hash = parent["pool_objects_blake3"][pool]
        name = f"s3-{pool_hash}.jsonl.gz"
        require(name in archives, "qualified residual-pool archive")
        pool_domains[pool] = pool_x_domain(archives[name])
        pool_receipts.append({"pool": pool, "archive_sha256": archive_rows[name]["sha256"],
                              "unique_x": len(pool_domains[pool])})
    targets = defaultdict(set)
    counts = Counter()
    survivors = defaultdict(list)
    for recipe, row in zip(specs(protocol), census["rows"], strict=True):
        require(all(row[key] == value for key, value in recipe.items())
                and len(row["group_target"]) == 2,
                "ordered complete formula and affine target")
        target_x = row["group_target"][0]
        targets[row["trial"]].add(target_x)
        in_pool = target_x in pool_domains[row["pool"]]
        state = classify(row["identity_mask"], in_pool)
        counts[state] += 1
        if state == "NO_PROVEN_IMPOSSIBILITY_FROM_THESE_GATES":
            survivors[(row["trial"], row["pool"],
                       row["base_summands"], row["residual_summands"])].append(row)
    require(len(targets) == 3 and all(len(xs) == 1 for xs in targets.values())
            and counts == {"UNSAT_BY_ADJACENT_IDENTITIES": 96,
                           "UNSAT_TARGET_X_ABSENT_FROM_POOL": 36,
                           "NO_PROVEN_IMPOSSIBILITY_FROM_THESE_GATES": 60}
            and len(survivors) == 12
            and all({row["identity_mask"] for row in rows} == {0, 1, 2, 4, 5}
                    for rows in survivors.values()),
            "exact target-domain exclusion and surviving mask set")
    target_receipts = []
    for trial, xs in sorted(targets.items()):
        x = next(iter(xs))
        target_receipts.append({"trial": trial, "target_x": x,
                                "in_hash_pool": x in pool_domains["hash"],
                                "in_gray_pool": x in pool_domains["gray"]})
    shape_screen = []
    for key, rows in sorted(survivors.items()):
        minimum = min(row["variables"] for row in rows)
        shape_screen.append({"trial": key[0], "pool": key[1],
                             "base_summands": key[2], "residual_summands": key[3],
                             "remaining_masks": [0, 1, 2, 4, 5],
                             "fewest_variables": minimum,
                             "fewest_variables_masks": sorted(
                                 row["identity_mask"] for row in rows
                                 if row["variables"] == minimum)})
    return {"schema": "n83.full-identity-mask-domain-screen/v1",
            "census_sha256": census_sha256,
            "input_manifest_sha256": protocol["input_manifest_sha256"],
            "pool_receipts": pool_receipts,
            "target_receipts": target_receipts,
            "rule": "final intermediate identity forces sixth affine summand x to equal target x; sixth summand is in the residual pool",
            "counts": dict(sorted(counts.items())),
            "shape_screen": shape_screen,
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
