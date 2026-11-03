#!/usr/bin/env python3
"""Independently replay the current-source K3596 base and exposed-point pilot."""

import argparse
import hashlib
import json
import math
from pathlib import Path

from research.n83_online_finalists_20261018.generate_fresh import group

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD = ROOT / "research/n83_k992_cold_followup_20261017"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve()
    manifest_path = next(HERE.glob("candidate_IC1N83Ckb1a1fb596936*.json"))
    manifest = json.loads(manifest_path.read_text())
    pilot = json.loads((output / "pilot.json").read_text())
    launch = json.loads((output / "launch.json").read_text())
    summary = json.loads((output / "stdout.jsonl").read_text())
    target = json.loads((output / "target.jsonl").read_text())
    result = json.loads((output / "targets_result.jsonl").read_text())
    old_target = json.loads((OLD / "targets.jsonl").read_text().splitlines()[0])
    answer = json.loads((OLD / "answers.jsonl").read_text().splitlines()[0])
    point = group.point_from_json(target)
    scalar = int(result["recovered_scalar"])
    base = group.replay_base(json.loads((output / "base_dump.jsonl").read_text()),
                             summary)
    indices = result["point_indices"]
    relation = None
    if isinstance(indices, list) and len(indices) == 4 and all(
            isinstance(index, int) and 0 <= index < len(base) for index in indices):
        for index in indices:
            relation = group.add(relation, base[index])
    phases = [result[name] for name in (
        "target_query_ms", "target_pdp_ms", "target_relation_check_ms",
        "target_descent_ms", "target_recovery_check_ms")]
    checks = {
        "pilot_pass": pilot["status"] == "PASS" and all(pilot["checks"].values()),
        "candidate_id": launch["candidate_id"] == manifest["candidate_id"],
        "source_hash": launch["source_sha256"] ==
        manifest["implementation"]["source_sha256"]["ic"],
        "base_dump": digest(output / "base_dump.jsonl") ==
        manifest["factor_base"]["ordered_base_dump_sha256"],
        "base_hash": summary["base_hash"] ==
        manifest["factor_base"]["source_base_hash_blake3"],
        "base_cardinality": len(base) == 596936,
        "rank": summary["rank"] == 3596,
        "root_index_entries": summary["root_table_entries"] == 1073268803,
        "target": target == old_target and result["target"] == target,
        "scalar_sidecar": scalar == int(answer["scalar"]),
        "scalar_replay": group.multiply(group.G, scalar) == point,
        "subgroup": group.multiply(point, group.R) is None,
        "four_point_relation": relation == point,
        "online_phases": all(math.isfinite(float(value)) and float(value) >= 0
                             for value in phases) and
        abs(math.fsum(map(float, phases)) - float(result["target_ms"])) <= 1,
        "producer_success": result["exit_code"] == 0 and
        result["group_verified"] is True and summary["targets_solved"] == 1 and
        summary["targets_failed"] == 0,
    }
    record = {"schema": "n83.root4-frontier-current-pilot-verification/v1",
              "candidate_id": manifest["candidate_id"],
              "candidate_manifest_sha256": digest(manifest_path),
              "checks": checks, "status": "PASS" if all(checks.values()) else "FAILED"}
    (output / "verification.json").write_text(json.dumps(record, indent=2,
                                                        sort_keys=True) + "\n")
    print(json.dumps(record, sort_keys=True))
    return 0 if all(checks.values()) else 2


if __name__ == "__main__":
    raise SystemExit(main())
