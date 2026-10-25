#!/usr/bin/env python3
"""Independent source, output, phase, and group replay of fp8 ABBA runs."""

import argparse
import json
import math
from pathlib import Path

from research.n83_fp8_e2e_control_20261025 import run
from research.n83_indexed5_20261013 import analyze as group
from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


PHASES = ("target_query_ms", "target_pdp_ms", "target_relation_check_ms",
          "target_descent_ms", "target_recovery_check_ms")


def read(path):
    return json.loads(Path(path).read_text())


def json_lines(path):
    return [json.loads(line) for line in Path(path).read_text().splitlines() if line]


def verify(output):
    output = Path(output).resolve(strict=True)
    receipt = read(output / "summary.json")
    binary = output / "koblitz_orbit_dlp_fast"
    if (receipt.get("schema") != "n83.fp8-full-abba/v1" or
            receipt.get("source_sha256") != run.SOURCE_SHA256 or
            run.sha(run.SOURCE) != run.SOURCE_SHA256 or
            receipt.get("binary_sha256") != run.sha(binary) or
            receipt.get("target_sha256") != run.sha(fixture.TARGETS) != fixture.TARGETS_SHA256 or
            receipt.get("answer_sha256") != run.sha(fixture.ANSWERS) != fixture.ANSWERS_SHA256 or
            tuple(receipt.get("order", ())) != run.ORDER or
            receipt.get("base_flags") != run.FLAGS or
            receipt.get("k") != 1107 or
            receipt.get("cap_seconds_per_child") != run.CAP_SECONDS or
            receipt.get("rss_cap_bytes") != run.RSS_CAP_BYTES):
        raise ValueError("frozen source, fixture, executable, or protocol differs")
    point = group.point_from_json(read(fixture.TARGETS))
    scalar = int(read(fixture.ANSWERS)["scalar"])
    if (not group.on_curve(point) or group.multiply(point, group.R) is not None or
            group.multiply(group.G, scalar) != point):
        raise ValueError("public point does not replay")
    rows = receipt.get("rows", [])
    checked = []
    for index, row in enumerate(rows):
        arm = run.ORDER[index]
        folder = output / f"{index}-{arm}"
        flags = dict(run.FLAGS)
        if arm == "fp8":
            flags["KIC_INDEXED4_FINGERPRINT8"] = "1"
        command = [str(binary), "construct:83:1:1107", str(fixture.TARGETS),
                   "29", str(folder / "target.jsonl")]
        launch = read(folder / "launch.json")
        if (row.get("index") != index or row.get("arm") != arm or
                launch.get("flags") != flags or launch.get("command") != command or
                row.get("stdout_sha256") != run.sha(folder / "stdout.jsonl") or
                row.get("stderr_sha256") != run.sha(folder / "stderr.log") or
                row.get("target_output_sha256") != (
                    run.sha(folder / "target.jsonl") if (folder / "target.jsonl").exists() else None) or
                not isinstance(row.get("wall_seconds"), (float, int)) or
                row["wall_seconds"] <= 0 or
                not isinstance(row.get("peak_sampled_rss_bytes"), int)):
            raise ValueError(f"child {index} launch or raw hashes differ")
        outcome = {"index": index, "arm": arm, "status": row["status"]}
        if row["status"] == "PROCESS_EXITED":
            summaries = [item for item in json_lines(folder / "stdout.jsonl")
                         if item.get("kind") == "compact_orbit_dlp_summary"]
            targets = [item for item in json_lines(folder / "target.jsonl")
                       if item.get("kind") == "compact_orbit_dlp_target"]
            if len(summaries) != 1 or len(targets) != 1:
                raise ValueError(f"child {index} lacks one summary and point")
            summary, target = summaries[0], targets[0]
            phases = [target.get(key) for key in PHASES]
            if (row.get("exit_code") != 0 or
                    row["peak_sampled_rss_bytes"] > run.RSS_CAP_BYTES or
                    summary.get("n") != 83 or summary.get("a") != 1 or
                    summary.get("orbit_columns") != 1107 or
                    summary.get("rank") != 1107 or
                    summary.get("targets_solved") != 1 or
                    target.get("group_verified") is not True or
                    target.get("recovered_scalar") != scalar or
                    target.get("target") != list(map(str, point)) or
                    any(not isinstance(p, (int, float)) or not math.isfinite(p) or p < 0
                        for p in phases) or
                    abs(sum(phases) - target.get("target_phase_sum_ms", -1)) > 0.01 or
                    abs(sum(phases) - target.get("target_ms", -1)) > 1.0):
                raise ValueError(f"child {index} correctness or timing differs")
            outcome.update({"base_hash": summary["base_hash"],
                            "factor_base_points": summary["factor_base_points"],
                            "regular_states": summary["regular_states"],
                            "root_table_entries": summary["root_table_entries"],
                            "rank": summary["rank"], "online_ms": target["target_ms"],
                            "process_ms": summary["timing_ms"]["process_total"],
                            "index_ms": summary["timing_ms"]["index_build"],
                            "recovered_scalar": scalar})
        checked.append(outcome)
    complete = len(checked) == len(run.ORDER) and all(
        item["status"] == "PROCESS_EXITED" for item in checked)
    parity = complete and all(len({item[field] for item in checked}) == 1 for field in
                              ("base_hash", "factor_base_points", "regular_states",
                               "root_table_entries", "rank", "recovered_scalar"))
    result = {"schema": "n83.fp8-full-abba-replay/v1",
              "status": "PASS_EXACT_FULL_PROCESS_PARITY" if parity else "BOUNDED_INCOMPLETE",
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
    raise SystemExit(0 if result["status"] == "PASS_EXACT_FULL_PROCESS_PARITY" else 1)
