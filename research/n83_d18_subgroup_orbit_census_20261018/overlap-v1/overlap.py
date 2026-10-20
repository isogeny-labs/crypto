#!/usr/bin/env python3
"""Stream the exact retained N83 U/D18 archives and count mixed support."""

from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import dataclass
from decimal import Decimal, localcontext
import gzip
import hashlib
import json
from math import comb
from pathlib import Path
import resource
import signal
import time


HERE = Path(__file__).resolve().parent
ORDER = 2417851639230796216685689
K = 16627
N = 83
POINTS_PER_ORBIT = 2 * N
OBJECTS = {
    "U": {"sha256": "2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369",
          "policy": "public_x_hash", "seed": 2026100801,
          "blake3": "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d"},
    "D": {"sha256": "347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7",
          "policy": "direct_subgroup_x_d18", "seed": 0,
          "blake3": "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682"},
}
SHAPES = ((2, 2), (3, 2), (2, 3), (3, 3), (4, 2), (2, 4))


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def multichoose(n: int, k: int) -> int:
    if k == 0:
        return 1
    return comb(n + k - 1, k) if n > 0 else 0


def mixed_support(only_u: int, only_d: int, common: int, a: int, b: int) -> int:
    """Count distinct multisets admitting a picks from U and b from D."""
    m = a + b
    total = 0
    for u in range(m + 1):
        for d in range(m - u + 1):
            c = m - u - d
            if u <= a <= u + c and d <= b <= d + c:
                total += (multichoose(only_u, u) * multichoose(only_d, d)
                          * multichoose(common, c))
    return total


def support_ratio(support: int) -> str:
    with localcontext() as ctx:
        ctx.prec = 25
        return format(Decimal(support) / Decimal(ORDER), ".12g")


@dataclass
class Archive:
    header: dict
    point_keys: dict[tuple[int, int], int]
    x_keys: dict[int, int]
    sha256: str
    wall_s: float
    cpu_s: float


def parse_archive(path: Path, *, arm: str, k: int = K,
                  expected_sha256: str | None = None) -> Archive:
    started = time.monotonic()
    cpu_started = time.process_time()
    expected = OBJECTS[arm]
    digest = sha256_file(path)
    require(digest == (expected_sha256 or expected["sha256"]), f"{arm} compressed SHA-256 differs")
    point_keys: dict[tuple[int, int], int] = {}
    x_keys: dict[int, int] = {}
    with gzip.open(path, "rb") as stream:
        header = json.loads(stream.readline())
        require(header["schema"] == "n83.factor-base/v1"
                and header["curve_a"] == 0 and int(header["curve_b"]) == 1
                and header["n"] == N and int(header["cofactor"]) == 4
                and header["modulus_low_terms"] == [0, 1, 2, 45]
                and header["closure"] == "signed_frobenius"
                and int(header["subgroup_order"]) == ORDER
                and header["orbit_columns"] == k
                and header["point_count"] == POINTS_PER_ORBIT * k
                and header["policy"] == expected["policy"]
                and header["seed"] == expected["seed"], f"{arm} header identity differs")
        lam = int(header["frobenius_lambda"])
        powers = [pow(lam, phase, ORDER) for phase in range(N)]
        for column in range(k):
            labels: set[tuple[int, bool]] = set()
            points: set[tuple[int, int]] = set()
            x_count: Counter[int] = Counter()
            for _ in range(POINTS_PER_ORBIT):
                line = stream.readline()
                require(bool(line), f"{arm} truncated column {column}")
                row = json.loads(line)
                phase, negative = row["phase"], row["negative"]
                require(row["column"] == column and type(phase) is int
                        and 0 <= phase < N and type(negative) is bool,
                        f"{arm} malformed label in column {column}")
                label = (phase, negative)
                require(label not in labels, f"{arm} duplicate label in column {column}")
                labels.add(label)
                coefficient = (-powers[phase] if negative else powers[phase]) % ORDER
                require(int(row["coefficient"]) == coefficient,
                        f"{arm} coefficient differs in column {column}")
                raw_point = row["point"]
                require(len(raw_point) == 2, f"{arm} malformed point in column {column}")
                point = (int(raw_point[0]), int(raw_point[1]))
                require(all(0 <= x < (1 << N) for x in point)
                        and point not in points,
                        f"{arm} duplicate/out-of-field point in column {column}")
                points.add(point)
                x_count[point[0]] += 1
            require(len(labels) == POINTS_PER_ORBIT and len(points) == POINTS_PER_ORBIT
                    and len(x_count) == N and set(x_count.values()) == {2},
                    f"{arm} incomplete signed orbit in column {column}")
            point_key = min(points)
            x_key = min(x_count)
            require(point_key[0] == x_key and point_key not in point_keys
                    and x_key not in x_keys, f"{arm} overlapping orbit in archive")
            point_keys[point_key] = column
            x_keys[x_key] = column
        require(not stream.readline(), f"{arm} trailing archive row")
    require(len(point_keys) == k and len(x_keys) == k, f"{arm} orbit count differs")
    return Archive(header, point_keys, x_keys, digest,
                   time.monotonic() - started, time.process_time() - cpu_started)


def make(u_path: Path, d_path: Path) -> dict:
    wall_start, cpu_start = time.monotonic(), time.process_time()
    u = parse_archive(u_path, arm="U", k=K)
    d = parse_archive(d_path, arm="D", k=K)
    identity = ("curve_slug", "field_encoding", "generator", "frobenius_lambda",
                "modulus_low_terms", "subgroup_order")
    require(all(u.header[key] == d.header[key] for key in identity),
            "U/D curve or subgroup identity differs")
    shared = sorted(u.point_keys.keys() & d.point_keys.keys())
    shared_x = sorted(u.x_keys.keys() & d.x_keys.keys())
    require(len(shared) == len(shared_x)
            and [p[0] for p in shared] == shared_x,
            "point-key and x-key overlap differ")
    common = len(shared)
    only_u, only_d = K - common, K - common
    common_points = POINTS_PER_ORBIT * common
    only_u_points = POINTS_PER_ORBIT * only_u
    only_d_points = POINTS_PER_ORBIT * only_d
    shapes = {}
    for a, b in SHAPES:
        value = mixed_support(only_u_points, only_d_points, common_points, a, b)
        shapes[f"{a}+{b}"] = {
            "distinct_point_multisets": str(value),
            "support_over_subgroup_order": support_ratio(value),
            "uniform_target_union_bound": support_ratio(min(value, ORDER)),
        }
    same = {str(m): str(multichoose(K * POINTS_PER_ORBIT, m)) for m in (4, 5, 6)}
    witnesses = [{"u_column": u.point_keys[key], "d_column": d.point_keys[key],
                  "canonical_point": [str(key[0]), str(key[1])]}
                 for key in shared[:16]]
    shared_bytes = json.dumps([[str(x), str(y)] for x, y in shared],
                              separators=(",", ":")).encode()
    return {
        "schema": "n83.u-d18-exact-orbit-overlap/v1", "status": "PASS_PRODUCER_PENDING_INDEPENDENT_AUDIT",
        "protocol_commit": "edce217", "candidate_id": None,
        "selected_best_total_runtime": None,
        "input_compressed_sha256": {"U": u.sha256, "D": d.sha256},
        "input_compressed_blake3_pinned": {arm: OBJECTS[arm]["blake3"] for arm in ("U", "D")},
        "curve_slug": u.header["curve_slug"], "subgroup_order": str(ORDER),
        "orbit_columns_each": K, "points_each": K * POINTS_PER_ORBIT,
        "shared_orbits": common, "u_only_orbits": only_u, "d_only_orbits": only_d,
        "union_orbits": 2 * K - common,
        "shared_points": common_points, "u_only_points": only_u_points,
        "d_only_points": only_d_points,
        "union_points": POINTS_PER_ORBIT * (2 * K - common),
        "shared_canonical_point_sha256": hashlib.sha256(shared_bytes).hexdigest(),
        "shared_witnesses_first_16": witnesses,
        "mixed_support": shapes, "same_base_support": same,
        "counting_scope": "exact distinct point-multisets, repeated points allowed; not a relation or runtime measurement",
        "timing": {"U": {"wall_s": u.wall_s, "cpu_s": u.cpu_s},
                   "D": {"wall_s": d.wall_s, "cpu_s": d.cpu_s},
                   "total_wall_s": time.monotonic() - wall_start,
                   "total_cpu_s": time.process_time() - cpu_start,
                   "ru_maxrss_bytes_macos": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss},
        "producer_sha256": sha256_file(Path(__file__)),
    }


def deadline_expired(_signum: int, _frame: object) -> None:
    raise TimeoutError("1200-second census cap")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--u", type=Path, required=True)
    parser.add_argument("--d", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    require(not args.output.exists(), "overlap receipt already exists")
    signal.signal(signal.SIGALRM, deadline_expired)
    signal.alarm(1200)
    try:
        result = make(args.u, args.d)
        args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    except Exception as exc:
        error = args.output.with_suffix(".error.json")
        error.write_text(json.dumps({"status": "INCOMPLETE_ERROR", "error": str(exc)},
                                    indent=2, sort_keys=True) + "\n")
        raise
    print(json.dumps({"status": result["status"], "shared_orbits": result["shared_orbits"],
                      "receipt_sha256": sha256_file(args.output)}))


if __name__ == "__main__":
    main()
