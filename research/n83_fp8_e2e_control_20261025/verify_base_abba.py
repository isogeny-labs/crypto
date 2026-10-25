#!/usr/bin/env python3
"""Independently replay the same-policy K1111/K1115 full-process comparison."""

import argparse
import json
import math
from pathlib import Path

from research.n83_fp8_e2e_control_20261025 import run, run_base_abba as control
from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")


def read(path):
    return json.loads(Path(path).read_text())


def lines(path):
    return [json.loads(line) for line in Path(path).read_text().splitlines() if line]


def verify(output):
    output = Path(output).resolve(strict=True)
    receipt = read(output / "summary.json")
    binary = output / "koblitz_orbit_dlp_fast"
    if (receipt.get("schema") != "n83.fp8-base-abba/v1" or
            receipt.get("source_sha256") != run.sha(run.SOURCE) or
            run.sha(run.SOURCE) != run.SOURCE_SHA256 or
            receipt.get("binary_sha256") != run.sha(binary) or
            receipt.get("binary_sha256") !=
            "a768b71f8059f0cf9049213ebc5bd2ed7ff9eccc3ab4947cff822093b58ef706" or
            receipt.get("target_sha256") != run.sha(fixture.TARGETS) or
            run.sha(fixture.TARGETS) != fixture.TARGETS_SHA256 or
            receipt.get("answer_sha256") != run.sha(fixture.ANSWERS) or
            run.sha(fixture.ANSWERS) != fixture.ANSWERS_SHA256 or
            tuple(receipt.get("order", ())) != control.ORDER or
            receipt.get("base_hashes") != {str(k): v for k, v in control.BASE_HASHES.items()} or
            receipt.get("base_manifest_sha256") !=
            {str(k): v for k, v in control.BASE_MANIFEST_SHA256.items()} or
            any(run.sha(control.base_manifest_path(k)) != control.BASE_MANIFEST_SHA256[k]
                for k in control.BASE_HASHES) or
            receipt.get("base_flags") != run.FLAGS or
            receipt.get("cap_seconds_per_child") != run.CAP_SECONDS or
            receipt.get("rss_cap_bytes") != run.RSS_CAP_BYTES):
        raise ValueError("frozen source, binary, input or protocol differs")
    scalar = int(read(fixture.ANSWERS)["scalar"])
    point = group.point_from_json(read(fixture.TARGETS))
    if (not group.on_curve(point) or group.multiply(point, group.R) is not None or
            group.multiply(group.G, scalar) != point):
        raise ValueError("public point scalar replay failed")
    checked = []
    for index, child in enumerate(receipt.get("rows", ())):
        if index >= len(control.ORDER):
            raise ValueError("extra child")
        k = control.ORDER[index]
        folder = output / f"{index}-k{k}"
        launch = read(folder / "launch.json")
        flags = dict(run.FLAGS)
        flags["KIC_INDEXED4_FINGERPRINT8"] = "1"
        command = launch.get("command")
        if not isinstance(command, list) or len(command) != 5:
            raise ValueError("incomplete child command")
        recorded_root = Path(command[0]).parent
        relative_target = fixture.TARGETS.relative_to(run.ROOT)
        if (not recorded_root.is_absolute() or recorded_root.name != output.name or
                command[0] != str(recorded_root / binary.name) or
                command[1] != f"construct:83:1:{k}" or
                not Path(command[2]).is_absolute() or
                Path(command[2]).parts[-len(relative_target.parts):] != relative_target.parts or
                command[3] != "29" or
                command[4] != str(recorded_root / f"{index}-k{k}/target.jsonl") or
                launch.get("flags") != flags or
                child.get("index") != index or child.get("k") != k or
                child.get("stdout_sha256") != run.sha(folder / "stdout.jsonl") or
                child.get("stderr_sha256") != run.sha(folder / "stderr.log") or
                child.get("target_output_sha256") != (
                    run.sha(folder / "target.jsonl") if (folder / "target.jsonl").exists() else None)):
            raise ValueError(f"child {index} launch or raw hash differs")
        row = {"index": index, "k": k, "status": child["status"]}
        if child["status"] == "PROCESS_EXITED":
            summaries = [x for x in lines(folder / "stdout.jsonl")
                         if x.get("kind") == "compact_orbit_dlp_summary"]
            targets = [x for x in lines(folder / "target.jsonl")
                       if x.get("kind") == "compact_orbit_dlp_target"]
            if len(summaries) != 1 or len(targets) != 1:
                raise ValueError(f"child {index} missing exact summary or target")
            summary, target = summaries[0], targets[0]
            phases = [target.get(name) for name in PHASES]
            if (child.get("exit_code") != 0 or
                    not isinstance(child.get("peak_sampled_rss_bytes"), int) or
                    child["peak_sampled_rss_bytes"] > run.RSS_CAP_BYTES or
                    summary.get("n") != 83 or summary.get("a") != 1 or
                    summary.get("orbit_columns") != k or
                    summary.get("factor_base_points") != 166 * k or
                    summary.get("base_hash") != control.BASE_HASHES[k] or
                    summary.get("rank") != k or summary.get("targets_solved") != 1 or
                    summary.get("root_fingerprint_sidecar_bytes", 0) <= 0 or
                    target.get("group_verified") is not True or
                    target.get("recovered_scalar") != scalar or
                    target.get("target") != list(map(str, point)) or
                    any(not isinstance(value, (int, float)) or
                        not math.isfinite(value) or value < 0 for value in phases) or
                    abs(sum(phases) - target.get("target_phase_sum_ms", -1)) > 0.01 or
                    abs(sum(phases) - target.get("target_ms", -1)) > 1.0):
                raise ValueError(f"child {index} base, rank, scalar or phase check failed")
            row.update({"base_hash": summary["base_hash"],
                        "factor_base_points": summary["factor_base_points"],
                        "root_table_entries": summary["root_table_entries"],
                        "rank": summary["rank"], "scalar": scalar,
                        "index_ms": summary["timing_ms"]["index_build"],
                        "process_ms": summary["timing_ms"]["process_total"],
                        "online_ms": target["target_ms"]})
        checked.append(row)
    complete = len(checked) == 4 and all(
        row["status"] == "PROCESS_EXITED" for row in checked)
    result = {"schema": "n83.fp8-base-abba-replay/v1",
              "status": "PASS_INDEPENDENT_REPLAY" if complete else "BOUNDED_INCOMPLETE",
              "rows": checked}
    (output / "independent_replay.json").write_text(
        json.dumps(result, sort_keys=True, indent=2) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = verify(args.output)
    print(result["status"])
    raise SystemExit(0 if result["status"] == "PASS_INDEPENDENT_REPLAY" else 1)
