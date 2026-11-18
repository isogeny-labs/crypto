#!/usr/bin/env python3
"""Hash-check two archived L0 record objects and read K13 batch solve clocks.

This performs no elliptic-curve execution and makes no x86 prediction.
"""

import argparse
import hashlib
import json
from pathlib import Path


INPUTS = (
    ("local_repeat", "0be204a394b2ef7ef223cb30960f12747ea30f736a135f46c2c0d76ecf0dbbde", 202611160401, 200, 32),
    ("local_eight_arm", "dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44", 202611160511, 48, 4),
)
CURVE = "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294"
FLOOR_NS = 1_000_000


def read_one(store_root, name, digest, target_seed, total_rows, measured_rows):
    path = store_root / digest[:2] / digest
    raw = path.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == digest, name
    records = [json.loads(line) for line in raw.splitlines() if line]
    assert len(records) == total_rows, (name, len(records))
    batch = [r for r in records if r["arm"] == "batch-m3" and not r["warmup"]]
    assert len(batch) == measured_rows, (name, len(batch))
    assert {r["session_id"] for r in batch}.__len__() == 1
    assert {r["workload"]["target_seed"] for r in batch} == {target_seed}
    assert {r["workload"]["curve"]["icv1"] for r in batch} == {CURVE}
    assert {r["isolation"]["level"] for r in batch} == {"L0"}
    assert {r["outcome"]["status"] for r in batch} == {"verified"}
    values = [r["time"]["solve_wall_ns"] for r in batch]
    assert all(isinstance(v, int) and v > 0 for v in values)
    return {
        "name": name,
        "sha256": digest,
        "target_seed": target_seed,
        "session_id": batch[0]["session_id"],
        "measured_rows": len(batch),
        "solve_min_ns": min(values),
        "solve_max_ns": max(values),
        "below_floor": sum(v < FLOOR_NS for v in values),
        "all_l0": True,
        "record_ids": [r["record_id"] for r in batch],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--store-root", type=Path, required=True)
    args = parser.parse_args()
    sessions = [read_one(args.store_root, *entry) for entry in INPUTS]
    print(json.dumps({
        "schema": "rootfind-k13-duration-readback/v1",
        "status": "hash_checked_archived_l0_data",
        "ecbench_grade_v3_min_solve_ns": FLOOR_NS,
        "frozen_panel_target_seed": 202611151301,
        "sessions": sessions,
        "measured_batch_rows": sum(s["measured_rows"] for s in sessions),
        "below_floor": sum(s["below_floor"] for s in sessions),
        "inference": "Local L0 durations flag an x86 L2 admission risk; they cannot decide x86 duration or any solver comparison.",
    }, indent=2))


if __name__ == "__main__":
    main()
