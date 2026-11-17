#!/usr/bin/env python3
"""Independently check small-base pilot identity, relation rank and scalar."""

import argparse
import hashlib
import json
from pathlib import Path

from research.n83_final_extended_20261018 import arms
from research.n83_online_finalists_20261018.generate_fresh import group

ROOT = Path(__file__).resolve().parents[2]
OLD = ROOT / "research/n83_k992_cold_followup_20261017"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--k", type=int, choices=(992, 1024), required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    arm = next(item for item in arms.SMALL if item[1] == args.k)
    manifest_path, manifest = arms.manifest(arm[3])
    launch = json.loads((output / "launch.json").read_text())
    pilot = json.loads((output / "pilot.json").read_text())
    summary = json.loads((output / "stdout.jsonl").read_text())
    target = json.loads((output / "target.jsonl").read_text())
    result = json.loads((output / "targets_result.jsonl").read_text())
    old_target = json.loads((OLD / "targets.jsonl").read_text().splitlines()[0])
    answer = json.loads((OLD / "answers.jsonl").read_text().splitlines()[0])
    scalar = int(result["recovered_scalar"])
    point = tuple(map(int, target))
    checks = {
        "pilot_pass": pilot["status"] == "PASS" and all(pilot["checks"].values()),
        "candidate_id": launch["candidate_id"] == manifest["candidate_id"],
        "source_hash": launch["source_sha256"] == arms.SOURCE_SHA256,
        "base_dump": digest(output / "base_dump.jsonl") ==
        manifest["factor_base"]["ordered_base_dump_sha256"],
        "base_hash": summary["base_hash"] ==
        manifest["factor_base"]["source_base_hash_blake3"],
        "rank": summary["rank"] == args.k,
        "target": target == old_target and result["target"] == target,
        "scalar_sidecar": scalar == int(answer["scalar"]),
        "scalar_replay": group.multiply(group.G, scalar) == point,
        "subgroup": group.multiply(point, group.R) is None,
        "producer_group_check": result["group_verified"] is True,
        "producer_success": result["exit_code"] == 0 and
        summary["targets_solved"] == 1 and summary["targets_failed"] == 0,
    }
    record = {"schema": "n83.final-extended-small-base-pilot-verification/v1",
              "k": args.k, "candidate_manifest_sha256": digest(manifest_path),
              "checks": checks, "status": "PASS" if all(checks.values()) else "FAILED"}
    (output / "verification.json").write_text(json.dumps(record, indent=2,
                                                        sort_keys=True) + "\n")
    print(json.dumps(record, sort_keys=True))
    return 0 if all(checks.values()) else 2


if __name__ == "__main__":
    raise SystemExit(main())
