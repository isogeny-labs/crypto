"""Independent N83 group replay for the FP2 four-byte ID31 K1107 control."""

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

from research.n83_fourbyte_fp2_20261027 import full_k1107
from research.n83_indexed5_20261013 import analyze as group


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE_DUMP_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
EXPECTED_SCALAR = 3640978018804724


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def one_json(path):
    lines = path.read_text().splitlines()
    if len(lines) != 1:
        raise ValueError(f"expected one JSON row in {path}")
    return json.loads(lines[0])


def executed_source_sha256(commit):
    if not isinstance(commit, str) or len(commit) != 40 or any(
            digit not in "0123456789abcdef" for digit in commit):
        raise ValueError("invalid executed source commit")
    source = subprocess.check_output(
        ["git", "show", f"{commit}:examples/koblitz_orbit_dlp_fast.rs"],
        cwd=ROOT)
    return hashlib.sha256(source).hexdigest()


def verify(output):
    launch = json.loads((output / "launch.json").read_text())
    receipt = json.loads((output / "receipt.json").read_text())
    checks = {
        "source_hash": launch["source_sha256"] == executed_source_sha256(
            launch["source_git_head"]),
        "binary_hash": launch["binary_sha256"] ==
            sha256(output / "executed_binary"),
        "target_hash": launch["target_sha256"] ==
            sha256(HERE / "target_k1107.jsonl"),
        "public_fixture": (HERE / "target_k1107.jsonl").read_bytes() ==
            (ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl")
            .read_bytes().splitlines(keepends=True)[0],
        "flags": launch["flags"] == full_k1107.FLAGS,
        "command": len(launch["command"]) == 5 and
            Path(launch["command"][0]).name == "executed_binary" and
            launch["command"][1] == "construct:83:1:1107" and
            Path(launch["command"][2]).name == "target_k1107.jsonl" and
            launch["command"][3] == "29" and
            Path(launch["command"][4]).name == "targets_result.jsonl",
        "answer_sidecar": int(one_json(
            ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
        )["scalar"]) == EXPECTED_SCALAR,
        "receipt_hashes": all(
            digest == (sha256(output / name) if (output / name).exists() else None)
            for name, digest in receipt["files_sha256"].items()),
    }
    if receipt["status"] != "PROCESS_EXITED" or receipt["exit_code"] != 0:
        return {"schema": "n83.fourbyte-fp2-full-k1107-replay/v1",
                "status": "BOUNDED_UNKNOWN_" + receipt["status"],
                "checks": checks, "process_wall_seconds":
                receipt["process_wall_seconds"]}
    summary = one_json(output / "stdout.jsonl")
    target = one_json(output / "targets_result.jsonl")
    dump = one_json(output / "base_dump.jsonl")
    points = [group.point_from_json(point) for point in
              dump["factor_base_point_coordinates"]]
    public = group.point_from_json(one_json(HERE / "target_k1107.jsonl"))
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
        "base_identity": sha256(output / "base_dump.jsonl") == BASE_DUMP_SHA256 and
            len(points) == 183762 and dump["base_hash"] == summary["base_hash"],
        "index_layout": summary["root_table_layout"] ==
            "indexed83_4byte_exact_id31_fp2" and
            summary["root_table_slot_bytes"] == 4 and
            summary["root_table_indexed4_id31_enabled"] is True and
            summary["root_table_indexed4_fp2_enabled"] is True and
            summary["root_fp2_bytes"] > 0 and
            summary["indexed5_split_at"] == 553 and
            summary["primary_table_entries"] > 0 and
            summary["overflow_table_entries"] > 0,
        "rank": summary["rank"] == summary["orbit_columns"] == 1107,
        "target": group.point_from_json(target["target"]) == public and
            group.point_from_json(target["published_q"]) == public,
        "scalar": scalar == EXPECTED_SCALAR and
            group.multiply(group.G, scalar) == public and
            group.multiply(public, group.R) is None,
        "four_point_relation": relation == public,
        "online_phases": all(isinstance(value, (int, float)) and
            math.isfinite(float(value)) and value >= 0 for value in phases) and
            abs(math.fsum(map(float, phases)) - float(target["target_ms"])) <= 1,
        "producer_success": target["group_verified"] is True and
            target["exit_code"] == 0 and summary["targets_solved"] == 1 and
            summary["targets_failed"] == 0,
    })
    return {"schema": "n83.fourbyte-fp2-full-k1107-replay/v1",
            "status": "PASS_INDEPENDENT_FULL_REPLAY" if all(checks.values())
                      else "FAILED",
            "checks": checks, "recovered_scalar": str(scalar),
            "process_wall_seconds": receipt["process_wall_seconds"],
            "online_ms": target["target_ms"],
            "root_table_allocated_bytes": summary["root_table_allocated_bytes"]}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    output = args.output.resolve(strict=True)
    result = verify(output)
    (output / "independent_replay.json").write_text(
        json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"status": result["status"], "output": str(output)},
                     sort_keys=True))
    return 0 if result["status"] == "PASS_INDEPENDENT_FULL_REPLAY" else 1


if __name__ == "__main__":
    raise SystemExit(main())
