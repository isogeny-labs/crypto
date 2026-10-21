#!/usr/bin/env python3
"""Independently replay the current-source K1107 full-process control."""

import argparse
import gzip
import hashlib
import json
import math
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_highk_beyond4405_20261021 import run_k1107_full_control as control
from research.n83_indexed5_20261013 import analyze as group

BASE_DUMP_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
SET_SHA256 = "d5584f33527fa38f174007aeaf071a42ab772be1d1021b586fd4142b80e31e2b"


def verify(output: Path, base_path: Path | None = None) -> dict:
    launch = json.loads((output / "launch.json").read_text())
    receipt = json.loads((output / "receipt.json").read_text())
    summary = json.loads((output / "stdout.jsonl").read_text())
    result = json.loads((output / "targets_result.jsonl").read_text())
    target = json.loads((output / "target.jsonl").read_text())
    public = json.loads(control.PUBLIC.read_text().splitlines()[0])
    answer = json.loads(control.ANSWERS.read_text().splitlines()[0])
    point = group.point_from_json(target)
    scalar = int(result["recovered_scalar"])
    base_path = base_path or output / "base_dump.jsonl"
    base_blob = (gzip.open(base_path, "rb").read()
                 if base_path.suffix == ".gz" else base_path.read_bytes())
    base_dump = json.loads(base_blob)
    base_dump_sha256 = hashlib.sha256(base_blob).hexdigest()
    points = group.replay_base(base_dump, summary)
    ordered_points = [[str(x), str(y)] for x, y in sorted(points)]
    set_sha = hashlib.sha256(json.dumps(
        ordered_points, sort_keys=True, separators=(",", ":"),
        ensure_ascii=False).encode()).hexdigest()
    relation = None
    indices = result.get("point_indices")
    if (isinstance(indices, list) and len(indices) == 4 and
            all(isinstance(index, int) and 0 <= index < len(points)
                for index in indices)):
        for index in indices:
            relation = group.add(relation, points[index])
    phases = [result.get(name) for name in (
        "target_query_ms", "target_pdp_ms", "target_relation_check_ms",
        "target_descent_ms", "target_recovery_check_ms")]
    checks = {
        "current_source": base.sha256(base.SOURCE) == base.SOURCE_SHA256 ==
                          launch["source_sha256"],
        "binary_custody": launch["binary_sha256"] ==
                          json.loads((base.HERE / "source_binary_custody.json")
                                     .read_text())["binary_sha256"],
        "prior_public_fixture": target == public == answer["point"] and
                                launch["public_targets_sha256"] ==
                                base.sha256(control.PUBLIC) and
                                launch["answer_sidecar_sha256"] ==
                                base.sha256(control.ANSWERS),
        "target_file_hash": launch["target_sha256"] ==
                            base.sha256(output / "target.jsonl"),
        "receipt_hashes": receipt["stdout_sha256"] ==
                          base.sha256(output / "stdout.jsonl") and
                          receipt["stderr_sha256"] ==
                          base.sha256(output / "stderr.log") and
                          receipt["base_dump_sha256"] == base_dump_sha256 and
                          receipt["targets_result_sha256"] ==
                          base.sha256(output / "targets_result.jsonl"),
        "base_identity": len(points) == 183762 and
                         summary["base_hash"] == base.PRIOR_BASE_HASH and
                         receipt["base_dump_sha256"] == BASE_DUMP_SHA256 and
                         set_sha == SET_SHA256,
        "root11_layout": launch["environment_flags"].get("KIC_ROOT_ONLY11") == "1" and
                         launch["environment_flags"].get("KIC_INDEXED5_LOAD75") == "1" and
                         launch["environment_flags"].get("KIC_INDEXED5_FIXED30") == "0" and
                         launch["environment_flags"].get("KIC_INDEXED5_FIXED31") is None,
        "rank": summary["rank"] == summary["orbit_columns"] == 1107,
        "index_layout": summary["root_table_layout"] == "indexed83_5byte_exact" and
                        summary["root_table_entries"] == 101708823 and
                        summary["root_table_allocated_bytes"] == 1342177280,
        "target": result["target"] == target,
        "scalar_sidecar": scalar == int(answer["scalar"]),
        "scalar_replay": group.multiply(group.G, scalar) == point,
        "subgroup": group.multiply(point, group.R) is None,
        "four_point_relation": relation == point,
        "online_phases": all(isinstance(value, (int, float)) and
                             math.isfinite(float(value)) and float(value) >= 0
                             for value in phases) and
                         abs(math.fsum(map(float, phases)) -
                             float(result["target_ms"])) <= 1,
        "producer_success": receipt["status"] == "PROCESS_EXITED" and
                            receipt["exit_code"] == 0 and
                            result["exit_code"] == 0 and
                            result["group_verified"] is True and
                            summary["targets_solved"] == 1 and
                            summary["targets_failed"] == 0,
    }
    return {
        "schema": "n83.fixed31-k1107-full-control-replay/v1",
        "status": "PASS_INDEPENDENT_FULL_REPLAY" if all(checks.values())
                  else "FAILED",
        "checks": checks,
        "source_sha256": base.SOURCE_SHA256,
        "binary_sha256": launch["binary_sha256"],
        "usable_points": len(points),
        "folded_columns": summary["orbit_columns"],
        "base_dump_sha256": receipt["base_dump_sha256"],
        "enumerated_set_sha256": set_sha,
        "recovered_scalar": str(scalar),
        "local_wall_seconds": receipt["wall_seconds"],
        "peak_child_rss_bytes": receipt["peak_child_rss_bytes"],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--base-dump", type=Path)
    args = parser.parse_args()
    output = args.output.resolve(strict=True)
    record = verify(output, args.base_dump.resolve(strict=True)
                    if args.base_dump else None)
    (output / "independent_replay.json").write_text(
        json.dumps(record, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": record["status"],
                      "checks": record["checks"]}, sort_keys=True))
    return 0 if record["status"] == "PASS_INDEPENDENT_FULL_REPLAY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
