#!/usr/bin/env python3
"""Seal fresh same-point N83 targets for the current-source cross-family panel."""

import argparse
import hashlib
import json
from pathlib import Path

from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_confirm16_20261024 import generate_fixture as prior
from research.n83_nearby_grid_confirm16_20261024 import protocol as prior_protocol


HERE = Path(__file__).resolve().parent
DOMAIN = b"n83-crossfamily-current16-20261025-v1"
COUNT = 16
CURVE_ID = "EC1N83Ckb1a1haacb69ee9f37"
SCALAR_RULE = "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def encoded(value):
    return json.dumps(value, sort_keys=True, indent=2) + "\n"


def produce():
    seen, inventory = prior.earlier_points()
    prior_manifest, prior_points = prior_protocol.fixture()
    prior_target_sha = sha((prior_protocol.HERE / "targets.jsonl").read_bytes())
    if prior_target_sha != prior_manifest["artifacts_sha256"]["targets.jsonl"]:
        raise ValueError("prior confirmation target SHA differs")
    earlier_count = len(seen)
    for _, point, _, _ in prior_points:
        if point in seen:
            raise ValueError("prior confirmation point overlaps inventory")
        seen.add(point)
    if len(seen) != earlier_count + len(prior_points):
        raise ValueError("prior confirmation points repeat")

    targets, answers, workloads, points = [], [], [], []
    for index in range(COUNT):
        seed = hashlib.sha256(DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or not group.on_curve(point) or
                group.multiply(point, group.R) is not None):
            raise ValueError(f"fixture point {index} fails subgroup replay")
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
        "schema": "n83.crossfamily-current16-exclusion/v1",
        "prior_inventory_sha256": sha(prior.screen.EXCLUSION_AUDIT.read_bytes()),
        "prior_confirm16_targets_sha256": prior_target_sha,
        "prior_unique_point_count": len(seen),
        "new_point_count": COUNT, "overlap_count": overlap,
        "duplicate_count": duplicates,
        "status": "PASS_ZERO_OVERLAP" if overlap == duplicates == 0
                  else "OVERLAP_FOUND",
        "prior_files": inventory["prior_files"],
    }
    if audit["status"] != "PASS_ZERO_OVERLAP":
        raise ValueError("cross-family fixture overlaps prior public points")
    blobs = {
        "targets.jsonl": "".join(targets).encode(),
        "answers.jsonl": "".join(answers).encode(),
        "workloads.jsonl": "".join(workloads).encode(),
        "exclusion_audit.json": encoded(audit).encode(),
    }
    manifest = {
        "schema": "n83.crossfamily-current16-fixture/v1",
        "status": "SEALED_UNMEASURED", "domain": DOMAIN.decode(),
        "curve_id": CURVE_ID, "subgroup_order": group.R,
        "point_count": COUNT, "scalar_rule": SCALAR_RULE,
        "generator_sha256": sha(Path(__file__).read_bytes()),
        "prior_confirm16_manifest_sha256": sha((prior_protocol.HERE / "fixture_manifest.json").read_bytes()),
        "artifacts_sha256": {name: sha(blob) for name, blob in blobs.items()},
        "workload_ids": [json.loads(line)["workload_id"] for line in workloads],
    }
    blobs["fixture_manifest.json"] = encoded(manifest).encode()
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
                raise ValueError(f"sealed fixture differs: {name}")
            path.write_bytes(blob)
        elif not path.exists() or path.read_bytes() != blob:
            raise ValueError(f"fixture replay differs: {name}")
    print(json.dumps({"status": manifest["status"],
                      "point_count": COUNT,
                      "prior_unique_point_count":
                      json.loads(blobs["exclusion_audit.json"])["prior_unique_point_count"],
                      "workload_ids": manifest["workload_ids"]}, sort_keys=True))


if __name__ == "__main__":
    main()
