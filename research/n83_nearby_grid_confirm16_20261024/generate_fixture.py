#!/usr/bin/env python3
"""Seal 16 independent N83 points before the nearby-grid screen is read."""

import hashlib
import json
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import audit_fresh
from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_tier1_20261024 import protocol as screen

HERE = Path(__file__).resolve().parent
DOMAIN = b"n83-nearby-grid-confirm16-20261024-v1"
COUNT = 16
CURVE_ID = "EC1N83Ckb1a1haacb69ee9f37"
SCALAR_RULE = "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)"
SCREEN_AUDIT_SHA256 = "8987b1ef6beb2c6f16d2c9eeac7ca8c6809f616177ce8ddb3e658ac1361a9a34"
SCREEN_HEAD_SHA = "921bf2d29b4ef22c4f4adb85f71ccceb33106a54"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def write_once(path, blob):
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(blob)


def earlier_points():
    blob = screen.EXCLUSION_AUDIT.read_bytes()
    if sha(blob) != SCREEN_AUDIT_SHA256:
        raise ValueError("nearby-grid screen exclusion audit differs")
    old = json.loads(blob)
    if old["status"] != "PASS_ZERO_OVERLAP" or old["overlap_count"] != 0:
        raise ValueError("nearby-grid screen overlap gate failed")
    seen = set()
    for row in old["prior_files"]:
        points = audit_fresh.verified_git_points(
            row["revision"], row["path"], row["sha256"])
        if len(points) != row["point_rows"]:
            raise ValueError(f"prior point count differs: {row['path']}")
        seen.update(points)
    if len(seen) != old["prior_unique_point_count"]:
        raise ValueError("prior unique point count differs")
    screen_blob = screen.TARGETS.read_bytes()
    if sha(screen_blob) != screen.TARGETS_SHA256:
        raise ValueError("screen point differs")
    screen_points = audit_fresh.point_rows(screen_blob)
    if len(screen_points) != 1 or screen_points[0] in seen:
        raise ValueError("screen point overlaps prior inventory")
    seen.add(screen_points[0])
    return seen, old


def generate():
    seen, old = earlier_points()
    targets, answers, workloads, points = [], [], [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"point {index} fails subgroup replay")
        points.append(point)
        encoded = [str(x) for x in point]
        target_line = json.dumps(encoded, separators=(",", ":")) + "\n"
        targets.append(target_line)
        answers.append(json.dumps({"fixture_index": index,
                                   "scalar": str(scalar),
                                   "seed_sha256": seed.hex()},
                                  sort_keys=True, separators=(",", ":")) + "\n")
        workload = {
            "schema": "ic-workload/v1", "curve_id": CURVE_ID,
            "subgroup_order": group.R, "target_input_law": SCALAR_RULE,
            "target_input_form": "public_encoded_point",
            "target_answer_sidecar_charged": False,
            "kind": "primary_single_target", "point_index": index,
            "target_count": 1, "target_point": encoded,
            "target_stream_sha256": sha(target_line.encode()),
            "target_generation_seed_sha256": seed.hex(),
            "target_domain": DOMAIN.decode(),
            "cache_policy": "reusable_factor_base_index_logs_ready_before_online_clock",
        }
        workload["workload_id"] = sha(canonical(workload))[:12]
        workloads.append(json.dumps(workload, sort_keys=True,
                                    separators=(",", ":")) + "\n")
    overlap = len(set(points) & seen)
    duplicates = COUNT - len(set(points))
    audit = {
        "schema": "n83.nearby-grid-confirm16-exclusion/v1",
        "screen_exclusion_audit_sha256": SCREEN_AUDIT_SHA256,
        "prior_unique_point_count": len(seen),
        "screen_point_sha256": screen.TARGETS_SHA256,
        "new_point_count": COUNT, "overlap_count": overlap,
        "duplicate_count": duplicates,
        "status": "PASS_ZERO_OVERLAP" if overlap == duplicates == 0
                  else "OVERLAP_FOUND",
        "prior_files": old["prior_files"],
    }
    if audit["status"] != "PASS_ZERO_OVERLAP":
        raise ValueError("confirmation points overlap earlier public points")
    blobs = {
        "targets.jsonl": "".join(targets).encode(),
        "answers.jsonl": "".join(answers).encode(),
        "workloads.jsonl": "".join(workloads).encode(),
        "exclusion_audit.json": (json.dumps(audit, sort_keys=True,
                                            indent=2) + "\n").encode(),
    }
    manifest = {
        "schema": "n83.nearby-grid-confirm16-fixture/v1",
        "status": "SEALED_UNMEASURED", "domain": DOMAIN.decode(),
        "curve_id": CURVE_ID, "subgroup_order": group.R,
        "point_count": COUNT, "scalar_rule": SCALAR_RULE,
        "generator_sha256": sha(Path(__file__).read_bytes()),
        "screen_head_sha": SCREEN_HEAD_SHA,
        "screen_workload_id": screen.WORKLOAD_ID,
        "screen_catalog_sha256": screen.CATALOG_SHA256,
        "screen_protocol_sha256": sha(Path(screen.__file__).read_bytes()),
        "artifacts_sha256": {name: sha(blob) for name, blob in blobs.items()},
        "workload_ids": [json.loads(line)["workload_id"] for line in workloads],
    }
    for name, blob in blobs.items():
        write_once(HERE / name, blob)
    write_once(HERE / "fixture_manifest.json",
               (json.dumps(manifest, sort_keys=True, indent=2) + "\n").encode())
    return {"status": manifest["status"], "point_count": COUNT,
            "prior_unique_point_count": len(seen),
            "workload_ids": manifest["workload_ids"]}


if __name__ == "__main__":
    print(json.dumps(generate(), sort_keys=True))
