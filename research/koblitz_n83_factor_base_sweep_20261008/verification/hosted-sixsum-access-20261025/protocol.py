#!/usr/bin/env python3
"""Freeze the primary-N83 K600 Gray/hash six-summand access pilot."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from math import comb
from pathlib import Path


HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
PARENT = HERE.parent / "k600-gray-hash-paired-protocol-20261014" / "plan.json"
CAPACITY = HERE.parent / "paired-capacity-screen-20261013" / "screen.py"
SUPPORT = HERE.parent / "pilot-pair-support-screen-20261014" / "screen.json"
OUTPUT = HERE / "protocol.json"
SOURCE_COMMIT = "cb98e85d1c1c1afdd5000bc70eea00969ee86c6c"
PARENT_SHA256 = "fbd3fb943bc0f181797d106c41ee9961b8d70768b5c65a5584002e3c1b5e488f"
IMAGE = "rust:1.98.1-bookworm@sha256:93ce27a88655056a51dbdd8f5f2d7ddc071c7b0070fb288a37b5a285fc83971e"
STAGE_CAP = {"build": 600, "import": 360, "search": 1900, "cleanup": 240}
STAGE_WALL = {"build": 450, "import": 300, "search": 1850}
PREPARATION_CAP = 1800
CONFLICTS = 20000
WORKER_WALL = 1800


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def make() -> dict:
    require(sha256(PARENT.read_bytes()) == PARENT_SHA256, "parent plan changed")
    parent = json.loads(PARENT.read_text())
    screen = json.loads(SUPPORT.read_text())
    spec = importlib.util.spec_from_file_location("n83_sixsum_capacity", CAPACITY)
    require(spec is not None and spec.loader is not None, "capacity source missing")
    capacity = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(capacity)
    arms = []
    for prior in parent["arms"]:
        residual = prior["residual_signed_orbits"]
        shape = capacity.requirements(600, 600 + residual, 6, 2)
        require(shape["current_worker_cap_pass"] is True, "six-summand cap failed")
        support_rows = [row for row in screen["ordered_pairs"]
                        if row["base_object_blake3"] == prior["base_object_blake3"]
                        and row["pool_object_blake3"] == prior["pool_object_blake3"]]
        require(len(support_rows) == 1, "support pair missing or duplicated")
        support_shapes = [row for row in support_rows[0]["shape_bounds"]
                          if (row["summands"], row["residual_summands"]) == (6, 2)]
        require(len(support_shapes) == 1, "six-summand support row missing")
        point_multisets = comb(166 * 600 + 3, 4) * comb(166 * residual + 1, 2)
        require(int(support_shapes[0]["point_multisets_including_repetition"])
                == point_multisets, "support count changed")
        argv = prior["worker_search_argv"][:]
        require(argv[-3:] == ["3", "2", "/out/search"], "parent worker argv changed")
        argv[-6:-3] = [str(WORKER_WALL), str(CONFLICTS), "32"]
        argv[-3] = "4"
        arms.append({
            "role": prior["role"], "base_object_blake3": prior["base_object_blake3"],
            "pool_object_blake3": prior["pool_object_blake3"],
            "base_compressed_sha256": prior["base_compressed_sha256"],
            "pool_compressed_sha256": prior["pool_compressed_sha256"],
            "shared_signed_orbits": prior["shared_signed_orbits"],
            "residual_signed_orbits": residual,
            "point_multisets_including_repetition": str(point_multisets),
            "source_capacity": shape,
            "worker_verify_argv": prior["worker_verify_argv"],
            "worker_search_argv": argv,
        })
    require([arm["role"] for arm in arms] == ["gray_pool", "hash_pool"],
            "arm order changed")
    return {
        "schema": "n83.hosted-sixsum-access-protocol/v2",
        "status": "FROZEN_BOUNDED_ACCESS_PILOT",
        "purpose": "six_summand_natural_relation_access_before_runtime_selection",
        "source_commit": SOURCE_COMMIT,
        "parent_plan_sha256": PARENT_SHA256,
        "curve_slug": parent["curve_slug"],
        "subgroup_order": parent["subgroup_order"],
        "public_target": parent["public_target"],
        "first_trial_a": parent["first_trial_a"],
        "first_trial_b": parent["first_trial_b"],
        "container_image": IMAGE,
        "base_summands": 4, "residual_summands": 2,
        "conflict_budget": CONFLICTS, "max_models": 32,
        "worker_wall_seconds": WORKER_WALL,
        "stage_wall_caps_seconds": STAGE_WALL,
        "conservative_stage_charges_seconds": STAGE_CAP,
        "conservative_pair_charge_seconds": 2 * sum(STAGE_CAP.values()),
        "conservative_preparation_charge_seconds": PREPARATION_CAP,
        "conservative_full_charge_seconds": PREPARATION_CAP + 2 * sum(STAGE_CAP.values()),
        "build_cpus": 2,
        "search_cpus": 1, "search_memory_bytes": 4294967296,
        "search_swap_bytes": 0,
        "arms": arms,
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    encoded = (json.dumps(make(), indent=2, sort_keys=True) + "\n").encode()
    if args.write:
        with OUTPUT.open("xb") as stream:
            stream.write(encoded)
    else:
        require(OUTPUT.read_bytes() == encoded, "saved six-summand protocol changed")
    print(json.dumps({"status": "PASS_SIXSUM_SOURCE_GATE",
                      "protocol_sha256": sha256(encoded)}, sort_keys=True))


if __name__ == "__main__":
    main()
