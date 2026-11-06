#!/usr/bin/env python3
"""Reconstruct exact current-source N83 factor-base identities from K1120."""

import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path

from blake3 import blake3
from research.n83_online_finalists_20261018.generate_candidates import canonical_id

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
PARENT = ROOT / "research/n83_nearby_k_current_source_20261021"
HIGHK = ROOT / "research/n83_highk_beyond4405_20261021"
PARENT_GZ_SHA256 = "c42b85a515f82c91b49aa0797477ed022ffe84b1f601e5631f52fa10bdb645df"
PARENT_RAW_SHA256 = "a1264aef4163f14f11738459fdd6620ca557c55c59e5f1e133fd3aa918b0ff34"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
BASELINE_ID = "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a"
ANCHORS = {
    1104: (PARENT / "base_dump_k1104.jsonl.gz",
           PARENT / "identities/candidate_IC1N83Ckb1a1fb183264PDP4rootRCguidedLAgaussTDdirectISO0h2233626bba52.json"),
    1107: (HIGHK / "k1107_current_source_base/base_dump.jsonl.gz",
           HIGHK / f"identities/candidate_{BASELINE_ID}.json"),
    1120: (PARENT / "base_dump_k1120.jsonl.gz",
           PARENT / "identities/candidate_IC1N83Ckb1a1fb185920PDP4rootRCguidedLAgaussTDdirectISO0hc21c54dc0a94.json"),
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def compact(record):
    return json.dumps(record, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def pretty(record):
    return (json.dumps(record, sort_keys=True, indent=2,
                       ensure_ascii=False) + "\n").encode()


def read_gzip(path):
    with gzip.open(path, "rb") as stream:
        return stream.read()


def set_digest(coordinates):
    points = sorted((int(x), int(y)) for x, y in coordinates)
    encoded = [[str(x), str(y)] for x, y in points]
    return sha(compact(encoded))


def slot_request(k):
    # K*(83*K+1)/2 retained states, then ceil(4*states/3).
    return (2 * k * (83 * k + 1) + 2) // 3


def slot_allocated_bytes(k):
    slots = 1 << (2 * slot_request(k) - 1).bit_length()
    return slots * 5


def produce():
    source_path = PARENT / "base_dump_k1120.jsonl.gz"
    if sha(source_path.read_bytes()) != PARENT_GZ_SHA256:
        raise ValueError("K1120 compressed parent differs")
    raw_parent = read_gzip(source_path)
    if sha(raw_parent) != PARENT_RAW_SHA256:
        raise ValueError("K1120 uncompressed parent differs")
    parent = json.loads(raw_parent)
    coordinates = parent["factor_base_point_coordinates"]
    labels = parent["factor_base_point_labels"]
    if (len(coordinates) != 166 * 1120 or len(labels) != len(coordinates) or
            len(parent["factor_base_representatives"]) != 1120 or
            parent["orbit_columns"] != 1120 or
            parent["factor_base_points"] != len(coordinates)):
        raise ValueError("K1120 ordered base shape differs")
    baseline = json.loads(ANCHORS[1107][1].read_bytes())
    if (canonical_id(baseline) != BASELINE_ID or
            baseline["implementation"]["source_sha256"]["ic"] != SOURCE_SHA256):
        raise ValueError("K1107 baseline identity or source differs")
    for col, coefficient in labels:
        if not (0 <= col < 1120 and 0 < coefficient < parent["subgroup_order"]):
            raise ValueError("invalid parent point label")
    seen = set()
    for x, y in coordinates:
        point = (int(x), int(y))
        if point in seen:
            raise ValueError("duplicate parent point")
        seen.add(point)
    if len(seen) != 166 * 1120:
        raise ValueError("parent point count differs")

    hasher = blake3()
    hasher.update(b"compact-orbit-constructed-base-v1")
    output = {}
    catalog = []
    for k in range(1102, 1121):
        # Hash the whole first prefix once; subsequent K values add one orbit.
        start = 0 if k == 1102 else 166 * (k - 1)
        for i in range(start, 166 * k):
            x, y = coordinates[i]
            column, coefficient = labels[i]
            if column != i // 166:
                raise ValueError(f"K{k} orbit labels are not contiguous")
            for word in (int(x), int(y)):
                hasher.update(word.to_bytes(16, "little"))
            hasher.update(int(column).to_bytes(8, "little"))
            hasher.update(int(coefficient).to_bytes(8, "little"))
        prefix_hash = hasher.hexdigest()
        dump = copy.copy(parent)
        dump.update({
            "base_hash": prefix_hash,
            "factor_base_point_coordinates": coordinates[:166 * k],
            "factor_base_point_labels": labels[:166 * k],
            "factor_base_points": 166 * k,
            "factor_base_representatives": parent["factor_base_representatives"][:k],
            "orbit_columns": k,
        })
        raw_dump = compact(dump) + b"\n"
        manifest = copy.deepcopy(baseline)
        fb = manifest["factor_base"]
        fb.update({
            "actual_usable_points": 166 * k,
            "effective_columns": k,
            "enumerated_set_sha256": set_digest(dump["factor_base_point_coordinates"]),
            "ordered_base_dump_sha256": sha(raw_dump),
            "source_base_hash_blake3": prefix_hash,
        })
        slot = manifest["point_decomposition"]["root_slot"]
        slot["request_entries"] = slot_request(k)
        slot["allocated_bytes"] = slot_allocated_bytes(k)
        manifest["relation_collection"]["stop_criterion"] = f"rank_{k}"
        manifest["relation_linear_algebra"]["columns"] = k
        manifest["relation_linear_algebra"]["rank_criterion"] = k
        manifest["candidate_id"] = canonical_id(manifest)
        if k in ANCHORS:
            anchor_dump, anchor_manifest = ANCHORS[k]
            if raw_dump != read_gzip(anchor_dump):
                raise ValueError(f"K{k} producer dump differs byte for byte")
            if manifest != json.loads(anchor_manifest.read_bytes()):
                raise ValueError(f"K{k} published candidate differs")
        output[f"identities/candidate_{manifest['candidate_id']}.json"] = pretty(manifest)
        catalog.append({
            "k": k,
            "candidate_id": manifest["candidate_id"],
            "actual_usable_points": 166 * k,
            "effective_columns": k,
            "ordered_base_dump_sha256": sha(raw_dump),
            "source_base_hash_blake3": prefix_hash,
            "root_slot_allocated_bytes": slot["allocated_bytes"],
            "direct_producer_timing": None,
            "identity_status": "PUBLISHED_PRODUCER_ANCHOR" if k in ANCHORS
                               else "DERIVED_VERIFIED_PREFIX",
        })
    if len({row["candidate_id"] for row in catalog}) != 19:
        raise ValueError("candidate identity collision")
    output["catalog.json"] = pretty({
        "schema": "n83.nearby-grid-identities/v1",
        "status": "PASS_DERIVED_PREFIX_IDENTITIES",
        "source_sha256": SOURCE_SHA256,
        "parent_compressed_sha256": PARENT_GZ_SHA256,
        "parent_raw_sha256": PARENT_RAW_SHA256,
        "candidate_count": len(catalog),
        "anchor_ks": sorted(ANCHORS),
        "candidates": catalog,
        "selected_best_total_runtime": None,
    })
    return output


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    output = produce()
    for relative, content in output.items():
        path = HERE / relative
        if args.write:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(content)
        elif not path.is_file() or path.read_bytes() != content:
            raise ValueError(f"derived artifact differs: {relative}")
    print(json.dumps({"status": "PASS_DERIVED_PREFIX_IDENTITIES",
                      "candidates": 19, "anchor_ks": sorted(ANCHORS)},
                     sort_keys=True))


if __name__ == "__main__":
    main()
