#!/usr/bin/env python3
"""Seal fresh N83 public points for the later small-versus-wide bridge."""

import argparse
import hashlib
import json
from pathlib import Path

from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_confirm16_20261024 import generate_fixture as nearby
from research.n83_nearby_grid_confirm16_20261024 import protocol as nearby_protocol
from research.n83_crossfamily_current_20261025 import generate_fixture as wide


HERE = Path(__file__).resolve().parent
DOMAIN = b"n83-crossfamily-bridge16-20261025-v1"
COUNT = 16
CURVE_ID = "EC1N83Ckb1a1haacb69ee9f37"
SCALAR_RULE = "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def pretty(value):
    return (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()


def prior_points():
    seen, inventory = nearby.earlier_points()
    _, nearby_points = nearby_protocol.fixture()
    for _, point, _, _ in nearby_points:
        if point in seen:
            raise ValueError("nearby confirmation overlaps the historical inventory")
        seen.add(point)

    wide_blobs, wide_manifest = wide.produce()
    for name, blob in wide_blobs.items():
        if (wide.HERE / name).read_bytes() != blob:
            raise ValueError(f"sealed wide fixture differs: {name}")
    if wide_manifest["status"] != "SEALED_UNMEASURED":
        raise ValueError("wide fixture status differs")
    for line in wide_blobs["targets.jsonl"].splitlines():
        point = group.point_from_json(json.loads(line))
        if point in seen:
            raise ValueError("wide point repeats an earlier public point")
        seen.add(point)
    if len(seen) != inventory["prior_unique_point_count"] + 1 + 16 + 16:
        raise ValueError("prior public-point inventory count differs")
    return seen, inventory


def produce():
    seen, inventory = prior_points()
    targets, answers, workloads, points = [], [], [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"bridge point {index} fails subgroup replay")
        points.append(point)
        pair = [str(x) for x in point]
        line = json.dumps(pair, separators=(",", ":")) + "\n"
        targets.append(line)
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
            "target_count": 1, "target_point": pair,
            "target_stream_sha256": sha(line.encode()),
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
        "schema": "n83.crossfamily-bridge16-exclusion/v1",
        "historical_inventory_sha256": sha(nearby.screen.EXCLUSION_AUDIT.read_bytes()),
        "nearby_confirm16_targets_sha256":
        sha((nearby_protocol.HERE / "targets.jsonl").read_bytes()),
        "wide_confirm16_targets_sha256":
        sha((wide.HERE / "targets.jsonl").read_bytes()),
        "prior_unique_point_count": len(seen),
        "new_point_count": COUNT, "overlap_count": overlap,
        "duplicate_count": duplicates,
        "status": "PASS_ZERO_OVERLAP" if overlap == duplicates == 0
                  else "OVERLAP_FOUND",
        "historical_files": inventory["prior_files"],
    }
    if audit["status"] != "PASS_ZERO_OVERLAP":
        raise ValueError("bridge fixture overlaps earlier public points")
    blobs = {
        "targets.jsonl": "".join(targets).encode(),
        "answers.jsonl": "".join(answers).encode(),
        "workloads.jsonl": "".join(workloads).encode(),
        "exclusion_audit.json": pretty(audit),
    }
    manifest = {
        "schema": "n83.crossfamily-bridge16-fixture/v1",
        "status": "SEALED_UNMEASURED", "domain": DOMAIN.decode(),
        "curve_id": CURVE_ID, "subgroup_order": group.R,
        "point_count": COUNT, "scalar_rule": SCALAR_RULE,
        "generator_sha256": sha(Path(__file__).read_bytes()),
        "nearby_confirm16_manifest_sha256":
        sha((nearby_protocol.HERE / "fixture_manifest.json").read_bytes()),
        "wide_confirm16_manifest_sha256":
        sha((wide.HERE / "fixture_manifest.json").read_bytes()),
        "artifacts_sha256": {name: sha(blob) for name, blob in blobs.items()},
        "workload_ids": [json.loads(line)["workload_id"] for line in workloads],
    }
    blobs["fixture_manifest.json"] = pretty(manifest)
    return blobs, manifest


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    blobs, manifest = produce()
    for name, blob in blobs.items():
        path = HERE / name
        if args.write:
            if path.exists() and path.read_bytes() != blob:
                raise ValueError(f"sealed bridge fixture differs: {name}")
            path.write_bytes(blob)
        elif not path.exists() or path.read_bytes() != blob:
            raise ValueError(f"bridge fixture replay differs: {name}")
    print(json.dumps({"status": manifest["status"],
                      "point_count": COUNT,
                      "prior_unique_point_count":
                      json.loads(blobs["exclusion_audit.json"])["prior_unique_point_count"],
                      "workload_ids": manifest["workload_ids"]}, sort_keys=True))


if __name__ == "__main__":
    main()
