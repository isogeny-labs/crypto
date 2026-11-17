#!/usr/bin/env python3
"""Seal canonical secondary blocks and sixteen single-target workloads."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def digest(blob: bytes) -> str:
    return hashlib.sha256(blob).hexdigest()


def identified(record: dict) -> dict:
    return {**record, "workload_id": digest(canonical(record))[:12]}


def seal() -> dict:
    fixture = json.loads((HERE / "fresh_manifest.json").read_text())
    screen_lines = (HERE / "screen_targets.jsonl").read_text().splitlines(
        keepends=True)
    holdout_lines = (HERE / "holdout_targets.jsonl").read_text().splitlines(
        keepends=True)
    answers = [json.loads(line) for line in
               (HERE / "holdout_answers.jsonl").read_text().splitlines()]
    if len(screen_lines) != 1536 or len(holdout_lines) != 16 or len(answers) != 16:
        raise ValueError("frozen workload streams differ")
    common = {
        "schema": "ic-workload/v1",
        "curve_id": fixture["curve_id"],
        "subgroup_order": fixture["subgroup_order"],
        "target_input_law": fixture["scalar_rule"],
        "target_input_form": "public_encoded_point",
        "target_answer_sidecar_charged": False,
    }
    blocks = []
    for block in range(2):
        first = block * 768
        blob = "".join(screen_lines[first:first + 768]).encode()
        blocks.append(identified({
            **common,
            "kind": "secondary_shared_setup_screen",
            "point_indices": [first, first + 767],
            "target_count": 768,
            "target_stream_sha256": digest(blob),
            "target_domain": fixture["screen"]["domain"],
            "cache_policy": "cold_process_per_candidate_block",
        }))
    singles = []
    for index, (line, answer) in enumerate(zip(holdout_lines, answers)):
        if answer["fixture_index"] != index:
            raise ValueError("holdout index differs")
        singles.append(identified({
            **common,
            "kind": "primary_single_target" if index == 0 else
                    "single_target_replication",
            "point_index": index,
            "target_count": 1,
            "target_point": json.loads(line),
            "target_stream_sha256": digest(line.encode()),
            "target_generation_seed_sha256": answer["seed_sha256"],
            "target_domain": fixture["holdout"]["domain"],
            "cache_policy": "reusable_factor_base_index_logs_ready_before_online_clock",
        }))
    manifest = {
        "schema": "n83.fixed31-workloads/v1",
        "screen_blocks": blocks,
        "holdout_one_target": singles,
    }
    path = HERE / "workloads.json"
    blob = json.dumps(manifest, indent=2, sort_keys=True).encode() + b"\n"
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("frozen workloads differ")
    path.write_bytes(blob)
    return manifest


if __name__ == "__main__":
    result = seal()
    print(json.dumps({
        "screen_workload_ids": [row["workload_id"] for row in
                                result["screen_blocks"]],
        "primary_workload_id": result["holdout_one_target"][0]["workload_id"],
        "one_target_workload_count": len(result["holdout_one_target"]),
    }, sort_keys=True))
