#!/usr/bin/env python3
"""Check raw APS/rho development replays against caller-only answers."""

import hashlib
import json
import sys
from pathlib import Path

N = 2_147_446_831


def main(archive_path: Path, paths: list[Path]) -> None:
    assert len(paths) == 4
    archive = json.loads(archive_path.read_bytes())
    results = []
    for path, method in zip(paths, ("aps", "rho", "rho", "aps")):
        raw = path.read_bytes()
        panel = json.loads(raw)
        cases = panel["cases"]
        assert panel["workers"] == 4
        assert len(cases) == len(archive["cases"]) == 48
        wall = cpu = work = 0
        for index, (case, old) in enumerate(zip(cases, archive["cases"])):
            assert case["case"] == old["case"] == index
            assert "error" not in case
            row = case["report"]
            assert row["public_Q"] == old["public_Q"]
            assert row["known_answer_in_solver"] is False
            assert row["complete_target_scalar_recovered"] is True
            assert row["workers"] == 4
            assert len(row["worker_reports"]) == 4
            assert [item["worker"] for item in row["worker_reports"]] == list(range(4))
            k = old["caller_known_scalar_verifier_only"]
            if method == "aps":
                witness = row["solution"]
                assert witness["log"] == k
                assert (witness["alpha"] + witness["beta"] * k) % N == 0
                units = row["parallel_s3_pairs_including_speculation"]
                assert units == sum(item["s3_pairs"] for item in row["worker_reports"])
                assert [item["left_rows_assigned"] for item in row["worker_reports"]] == [
                    [0, 96], [96, 192], [192, 288], [288, 384]
                ]
            else:
                assert row["log"] == k
                assert row["fold_negation"] is True
                assert row["lanes_per_worker"] == 16
                units = row["walk_steps"]
                assert units == sum(item["walk_steps"] for item in row["worker_reports"])
                assert units <= row["step_cap"]
            assert row["cold_wall_ns"] > 0
            assert row["process_cpu_ns"] > 0
            wall += row["cold_wall_ns"]
            cpu += row["process_cpu_ns"]
            work += units
        results.append({
            "file": path.name,
            "method": method,
            "cases": 48,
            "summed_cold_wall_ns_diagnostic": wall,
            "summed_process_cpu_ns_diagnostic": cpu,
            "work_units": work,
            "sha256": hashlib.sha256(raw).hexdigest(),
        })
    print(json.dumps({"standing": "unisolated development replay", "arms": results},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 6:
        raise SystemExit("usage: verify_abba.py ARCHIVE A1 R1 R2 A2")
    main(Path(sys.argv[1]), [Path(arg) for arg in sys.argv[2:]])
