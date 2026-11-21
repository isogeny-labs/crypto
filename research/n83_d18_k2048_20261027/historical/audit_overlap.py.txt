#!/usr/bin/env python3
"""Compare the exact point sets of the frozen primary N83 K2048 bases."""

import argparse
import gzip
import importlib.metadata
import json
from pathlib import Path

import blake3


CURVE = "icv1-f2m83-tm6151469093347-debefd74"
OBJECTS = {
    "U": ("public_x_hash", "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851"),
    "D": ("direct_subgroup_x_d18_prefix2048", "a3c06a35db07b7aa0d9e7f45e8b747d5c136f2ea1a0fdc0e8822ffed84659f68"),
}
POINTS = 339_968
COLUMNS = 2_048
SIGNED_ORBIT_SIZE = 166


def load(arm, path):
    policy, digest = OBJECTS[arm]
    hasher = blake3.blake3()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            hasher.update(chunk)
    if hasher.hexdigest() != digest:
        raise ValueError(f"{arm} archive BLAKE3 differs")
    points = set()
    rows = 0
    columns = set()
    with gzip.open(path, "rt") as stream:
        header = json.loads(next(stream))
        if (
            header["curve_slug"] != CURVE
            or header["policy"] != policy
            or header["orbit_columns"] != COLUMNS
            or header["point_count"] != POINTS
            or header["subgroup_order"] != "2417851639230796216685689"
        ):
            raise ValueError(f"{arm} header identity differs")
        for line in stream:
            row = json.loads(line)
            point = tuple(map(int, row["point"]))
            column = row["column"]
            if (
                len(point) != 2
                or any(word < 0 or word >= 1 << 83 for word in point)
                or not isinstance(column, int)
                or column < 0 or column >= COLUMNS
            ):
                raise ValueError(f"{arm} point or column outside frozen range")
            points.add(point)
            columns.add(column)
            rows += 1
    if rows != POINTS or len(points) != POINTS or len(columns) != COLUMNS:
        raise ValueError(f"{arm} row count, uniqueness or column coverage differs")
    return points, {"policy": policy, "rows": rows, "distinct_points": len(points)}


def audit(u_path, d_path):
    if importlib.metadata.version("blake3") != "1.0.8":
        raise ValueError("BLAKE3 verifier version differs")
    u, u_meta = load("U", u_path)
    d, d_meta = load("D", d_path)
    common = len(u & d)
    if common % SIGNED_ORBIT_SIZE:
        raise ValueError("partial signed-orbit overlap")
    return {
        "schema": "n83.d18-k2048-structural-overlap/v1", "status": "PASS",
        "curve_id": CURVE, "arms": {"U": u_meta, "D": d_meta},
        "u_object_blake3": OBJECTS["U"][1], "d_object_blake3": OBJECTS["D"][1],
        "shared_points": common, "shared_signed_orbits": common // SIGNED_ORBIT_SIZE,
        "unique_u_points": len(u) - common, "unique_d_points": len(d) - common,
        "selected_best_total_runtime": None,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--u-object", type=Path, required=True)
    parser.add_argument("--d-object", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError("overlap output already exists")
    result = audit(args.u_object, args.d_object)
    args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
