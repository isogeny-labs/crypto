"""Independent group and phase replay for paired K1107 index layouts."""

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

from research.n83_fourbyte_pair_20261027.run import (
    BINARY_SHA256, FLAGS, ROOT, SOURCE_SHA256, TARGET, TARGET_SHA256, sha256,
    write_json,
)
from research.n83_indexed5_20261013 import analyze as group

BASE_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
SCALAR = 3640978018804724


def one_json(path):
    lines = path.read_text().splitlines()
    if len(lines) != 1:
        raise ValueError(f"expected one JSON row: {path}")
    return json.loads(lines[0])


def verify_arm(path, layout, sequence, schedule):
    launch = json.loads((path / "launch.json").read_text())
    receipt = json.loads((path / "receipt.json").read_text())
    expected_flags = dict(FLAGS)
    if layout == "plain":
        expected_flags.pop("KIC_INDEXED4_FP2")
    source = subprocess.check_output([
        "git", "show", f"{launch['source_git_head']}:examples/koblitz_orbit_dlp_fast.rs"
    ], cwd=ROOT)
    checks = {
        "identity": launch["source_git_head"] == schedule["source_git_head"] and
            launch["source_sha256"] == SOURCE_SHA256 and
            hashlib.sha256(source).hexdigest() == SOURCE_SHA256 and
            launch["binary_sha256"] == BINARY_SHA256 and
            sha256(path / "executed_binary") == BINARY_SHA256 and
            launch["target_sha256"] == schedule["target_sha256"] ==
            TARGET_SHA256 == sha256(TARGET),
        "flags": launch["flags"] == expected_flags and
            launch["layout"] == receipt["layout"] == layout and
            launch["sequence"] == receipt["sequence"] == sequence,
        "command": len(launch["command"]) == 5 and
            Path(launch["command"][0]).name == "executed_binary" and
            launch["command"][1] == "construct:83:1:1107" and
            Path(launch["command"][2]).name == "target_k1107.jsonl" and
            launch["command"][3] == "29" and
            Path(launch["command"][4]).name == "targets_result.jsonl",
        "receipt": receipt["status"] == "PROCESS_EXITED" and
            receipt["exit_code"] == 0 and all(
                digest == (sha256(path / name) if (path / name).exists() else None)
                for name, digest in receipt["files_sha256"].items()),
    }
    if not checks["receipt"]:
        return {"status": "BOUNDED_UNKNOWN", "checks": checks,
                "layout": layout, "sequence": sequence}
    summary = one_json(path / "stdout.jsonl")
    target = one_json(path / "targets_result.jsonl")
    dump = one_json(path / "base_dump.jsonl")
    points = [group.point_from_json(point) for point in
              dump["factor_base_point_coordinates"]]
    public = group.point_from_json(one_json(TARGET))
    scalar = int(target["recovered_scalar"])
    relation = None
    indices = target.get("point_indices")
    if isinstance(indices, list) and len(indices) == 4 and all(
            isinstance(index, int) and 0 <= index < len(points)
            for index in indices):
        for index in indices:
            relation = group.add(relation, points[index])
    phases = [target.get(key) for key in (
        "target_query_ms", "target_pdp_ms", "target_relation_check_ms",
        "target_descent_ms", "target_recovery_check_ms")]
    checks.update({
        "base": sha256(path / "base_dump.jsonl") == BASE_SHA256 and
            len(points) == 183762 and dump["base_hash"] == summary["base_hash"],
        "layout": summary["root_table_layout"] == (
            "indexed83_4byte_exact_id31_fp2" if layout == "fp2"
            else "indexed83_4byte_exact_id31") and
            summary["root_table_indexed4_fp2_enabled"] == (layout == "fp2") and
            summary["root_table_slot_bytes"] == 4 and
            summary["root_table_indexed4_id31_enabled"] is True,
        "rank": summary["rank"] == summary["orbit_columns"] == 1107,
        "answer": group.point_from_json(target["target"]) == public and
            group.point_from_json(target["published_q"]) == public and
            scalar == SCALAR and group.multiply(group.G, scalar) == public and
            group.multiply(public, group.R) is None and relation == public and
            target["group_verified"] is True and target["exit_code"] == 0,
        "phases": all(isinstance(value, (int, float)) and
            math.isfinite(float(value)) and value >= 0 for value in phases) and
            abs(math.fsum(map(float, phases)) - float(target["target_ms"])) <= 1,
    })
    return {"status": "PASS_INDEPENDENT_FULL_REPLAY" if all(checks.values())
            else "FAILED", "checks": checks, "layout": layout,
            "sequence": sequence, "process_wall_seconds":
            receipt["process_wall_seconds"], "index_build_ms":
            summary["timing_ms"]["index_build"], "rank_stage_ms":
            summary["timing_ms"]["rank_stage"],
            "online_ms": target["target_ms"], "peak_rss_bytes":
            summary["peak_rss_bytes"], "root_fp2_bytes":
            summary["root_fp2_bytes"], "base_hash": summary["base_hash"]}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve(strict=True)
    schedule = json.loads((output / "schedule.json").read_text())
    if (schedule["schema"] != "n83.fourbyte-paired-k1107-schedule/v1" or
            schedule["host_tier"] != "shared_local_diagnostic"):
        raise ValueError("pair schedule differs from frozen protocol")
    layouts = schedule["order"].split("-")
    arms = [verify_arm(output / f"{sequence}_{layout}", layout, sequence,
                       schedule) for sequence, layout in enumerate(layouts)]
    by_layout = {arm["layout"]: arm for arm in arms}
    passed = all(arm["status"] == "PASS_INDEPENDENT_FULL_REPLAY"
                 for arm in arms)
    if passed:
        passed = (by_layout["plain"]["base_hash"] ==
                  by_layout["fp2"]["base_hash"] and
                  by_layout["plain"]["root_fp2_bytes"] == 0 and
                  by_layout["fp2"]["root_fp2_bytes"] > 0)
    result = {"schema": "n83.fourbyte-paired-k1107-replay/v1",
              "status": "PASS_PAIRED_FULL_REPLAY" if passed else "FAILED",
              "host_tier": "shared_local_diagnostic", "arms": arms}
    if passed:
        result["plain_over_fp2_index_build"] = (
            by_layout["plain"]["index_build_ms"] /
            by_layout["fp2"]["index_build_ms"])
        result["plain_over_fp2_process_wall"] = (
            by_layout["plain"]["process_wall_seconds"] /
            by_layout["fp2"]["process_wall_seconds"])
    write_json(output / "independent_replay.json", result)
    print(json.dumps({"status": result["status"], "output": str(output)}))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
