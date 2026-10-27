#!/usr/bin/env python3
"""Replay a hosted six-summand candidate with independent Python group arithmetic."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
sys.path.insert(0, str(STUDY))
from chain_search_supervisor import (  # noqa: E402
    PRIMARY_GENERATOR, PRIMARY_ORDER, field_mul, parse_point, point_add, point_mul,
)

CURVE = "icv1-f2m83-tm6151469093347-debefd74"
ORBIT = 166


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_scalar(value: object) -> int:
    require(isinstance(value, str) and value.isdecimal()
            and str(int(value)) == value and int(value) < PRIMARY_ORDER,
            "noncanonical subgroup scalar")
    return int(value)


def archive_points(path: Path, digest: str) -> list[tuple[int, int]]:
    require(sha256(path) == digest, "compressed factor-base bytes differ")
    points = []
    with gzip.open(path, "rt", encoding="utf-8") as stream:
        header = json.loads(next(stream))
        require(header["schema"] == "n83.factor-base/v1"
                and header["curve_slug"] == CURVE
                and header["point_count"] == 600 * ORBIT
                and header["subgroup_order"] == str(PRIMARY_ORDER)
                and header["closure"] == "signed_frobenius",
                "factor-base header differs")
        for index, line in enumerate(stream):
            row = json.loads(line)
            coordinate = row["point"]
            require(isinstance(coordinate, list) and len(coordinate) == 2
                    and all(isinstance(word, str) and word.isdecimal()
                            and str(int(word)) == word and int(word) < 1 << 83
                            for word in coordinate)
                    and row["column"] == index // ORBIT
                    and row["phase"] == (index % ORBIT) // 2
                    and row["negative"] is (index % 2 == 1),
                    "factor-base point or orbit label differs")
            points.append((int(coordinate[0]), int(coordinate[1])))
    require(len(points) == 600 * ORBIT and len(set(points)) == len(points),
            "factor-base point count or uniqueness differs")
    return points


def residual_domain(base: list[tuple[int, int]],
                    pool: list[tuple[int, int]], expected_orbits: int) -> set[tuple[int, int]]:
    base_set = set(base)
    residual = set()
    for offset in range(0, len(pool), ORBIT):
        orbit = pool[offset:offset + ORBIT]
        overlap = sum(point in base_set for point in orbit)
        require(overlap in (0, ORBIT), "partial signed-orbit overlap")
        if overlap == 0:
            residual.update(orbit)
    require(len(residual) == expected_orbits * ORBIT,
            "residual domain cardinality differs")
    return residual


def s3_residual(x: int, y: int, z: int) -> int:
    xy = field_mul(x, y)
    pair = xy ^ field_mul(x, z) ^ field_mul(y, z)
    return field_mul(xy, z) ^ field_mul(pair, pair) ^ 1


def check_candidate(candidate: dict, worker: dict,
                    base: list[tuple[int, int]], residual: set[tuple[int, int]],
                    public_target: tuple[int, int]) -> dict:
    require(candidate["source_id"] == 1
            and len(candidate["base_point_indices"]) == 4
            and len(candidate["residual_points"]) == 2,
            "candidate shape differs")
    indices = candidate["base_point_indices"]
    require(all(type(index) is int and 0 <= index < len(base) for index in indices),
            "candidate base index differs")
    selected = [base[index] for index in indices]
    selected.extend(parse_point(row) for row in candidate["residual_points"])
    require(all(point is not None for point in selected)
            and all(point in residual for point in selected[4:])
            and selected[0][0] <= selected[1][0],
            "candidate point membership or canonical order differs")
    a = canonical_scalar(candidate["a"])
    b = canonical_scalar(candidate["b"])
    require(a == canonical_scalar(worker["a"])
            and b == canonical_scalar(worker["b"]),
            "candidate source coefficients differ")
    for point in selected:
        require(parse_point([str(point[0]), str(point[1])]) == point
                and point_mul(point, PRIMARY_ORDER) is None,
                "candidate point outside curve or subgroup")
    require(point_mul(public_target, PRIMARY_ORDER) is None,
            "public query outside subgroup")
    target = point_add(point_mul(PRIMARY_GENERATOR, a), point_mul(public_target, b))
    total = None
    partials = []
    for point in selected:
        total = point_add(total, point)
        partials.append(total)
    require(total == target, "six-point group relation differs")
    checked_s3 = 0
    for index in range(1, len(selected)):
        left = partials[index - 1]
        right = selected[index]
        summed = partials[index]
        if left is None or summed is None:
            continue
        require(s3_residual(left[0], right[0], summed[0]) == 0,
                "original-field S3 node differs")
        checked_s3 += 1
    encoded = json.dumps([[str(x), str(y)] for x, y in selected],
                         separators=(",", ":")).encode()
    return {"status": "PASS_INDEPENDENT_GROUP_RELATION",
            "selected_points_sha256": hashlib.sha256(encoded).hexdigest(),
            "s3_nodes_checked": checked_s3,
            "identity_transitions": 5 - checked_s3,
            "selected_best_total_runtime": None}


def replay(base_archive: Path, pool_archive: Path, worker_result: Path,
           corpus_path: Path, role: str) -> dict:
    dispatch = json.loads((HERE / "dispatch.json").read_text())
    plan_path = HERE.parent / "hosted-m6-paired-protocol-20261017/plan.json"
    require(sha256(plan_path) == dispatch["frozen_plan_sha256"]
            and sha256(corpus_path) == dispatch["corpus_sha256"],
            "plan or public corpus differs")
    plan = json.loads(plan_path.read_text())
    arm = next(row for row in plan["arms"] if row["role"] == role)
    require(base_archive.name == f"{arm['base_object_blake3']}.jsonl.gz"
            and pool_archive.name == f"{arm['pool_object_blake3']}.jsonl.gz",
            "factor-base object identity differs")
    base = archive_points(base_archive,
                          dispatch["source_archive_sha256"][arm["base_object_blake3"]])
    pool = archive_points(pool_archive,
                          dispatch["source_archive_sha256"][arm["pool_object_blake3"]])
    residual = residual_domain(base, pool, arm["residual_signed_orbits"])
    corpus = json.loads(corpus_path.read_text())
    require(corpus["schema"] == "n83.public-probe-corpus/v1"
            and corpus["targets"][0]["a"] == 0
            and corpus["targets"][0]["fixture"] == 0,
            "public target corpus differs")
    public_target = parse_point(corpus["targets"][0]["point"])
    require(public_target is not None, "public target absent")
    worker = json.loads(worker_result.read_text())
    require(worker["schema"] == "n83.cross-stream-pilot-partial-result/v2"
            and worker["ordering"] == "equal_domain_runs"
            and worker["base_object_blake3"] == arm["base_object_blake3"]
            and worker["pool_object_blake3"] == arm["pool_object_blake3"]
            and worker["selected_best_total_runtime"] is None,
            "worker result identity differs")
    candidate = worker["candidate"]
    if candidate is None:
        require(worker["status"] != "PASS_one_bounded_partial",
                "passing worker status lacks candidate")
        return {"schema": "n83.hosted-m6-highcap-model-replay/v1",
                "status": "NO_MODEL_TO_REPLAY", "role": role,
                "worker_status": worker["status"],
                "selected_best_total_runtime": None}
    require(worker["status"] == "PASS_one_bounded_partial",
            "candidate with nonpassing worker status")
    result = check_candidate(candidate, worker, base, residual, public_target)
    return {"schema": "n83.hosted-m6-highcap-model-replay/v1",
            "role": role, "worker_status": worker["status"],
            "base_archive_sha256": sha256(base_archive),
            "pool_archive_sha256": sha256(pool_archive),
            "worker_result_sha256": sha256(worker_result),
            **result}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base-archive", type=Path, required=True)
    parser.add_argument("--pool-archive", type=Path, required=True)
    parser.add_argument("--worker-result", type=Path, required=True)
    parser.add_argument("--corpus", type=Path, required=True)
    parser.add_argument("--role", choices=["gray_pool", "hash_pool"], required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = replay(args.base_archive, args.pool_archive, args.worker_result,
                    args.corpus, args.role)
    text = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output.exists():
        require(args.output.read_text() == text, "sealed model replay differs")
    else:
        args.output.write_text(text)
    print(json.dumps({"status": result["status"], "role": args.role}, sort_keys=True))


if __name__ == "__main__":
    main()
