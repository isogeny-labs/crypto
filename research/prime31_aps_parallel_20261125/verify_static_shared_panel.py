#!/usr/bin/env python3
"""Check the shared-preparation static worker control against serial APS."""

import hashlib
import json
import sys
from pathlib import Path

N = 2_147_446_831


def main(panel_path: Path, archive_path: Path, serial_path: Path) -> None:
    raw = panel_path.read_bytes()
    panel = json.loads(raw)
    archive = json.loads(archive_path.read_bytes())
    serial = json.loads(serial_path.read_bytes())
    assert panel["workers"] == 4
    assert len(panel["cases"]) == len(archive["cases"]) == len(serial["cases"]) == 48
    wall = cpu = pairs = 0
    for index, (case, old, baseline) in enumerate(zip(
            panel["cases"], archive["cases"], serial["cases"])):
        assert case["case"] == old["case"] == baseline["case"] == index
        assert "error" not in case
        report = case["report"]
        assert report["public_Q"] == old["public_Q"] == baseline["report"]["public_Q"]
        assert report["known_answer_in_solver"] is False
        assert report["complete_target_scalar_recovered"] is True
        witness = report["solution"]
        k = old["caller_known_scalar_verifier_only"]
        assert witness == baseline["report"]["serial_witness"]
        assert witness["log"] == k
        assert (witness["alpha"] + witness["beta"] * k) % N == 0
        assert report["shared_preparation_s3_sqrts"] == 768
        workers = report["worker_reports"]
        assert [worker["left_rows_assigned"] for worker in workers] == [
            [0, 96], [96, 192], [192, 288], [288, 384]
        ]
        assert sum(worker["s3_pairs"] for worker in workers) == report[
            "parallel_s3_pairs_including_speculation"
        ]
        for worker in workers:
            assert worker["left_rows_started"] + worker["left_rows_not_started"] == 96
            assert worker["s3_pairs"] == worker["left_rows_started"] * 384
        assert report["cold_wall_ns"] > 0 and report["process_cpu_ns"] > 0
        wall += report["cold_wall_ns"]
        cpu += report["process_cpu_ns"]
        pairs += report["parallel_s3_pairs_including_speculation"]
    print(json.dumps({
        "standing": "development static APS correctness and accounting check",
        "cases": 48,
        "identical_serial_witnesses": 48,
        "s3_pairs": pairs,
        "summed_cold_wall_ns_diagnostic": wall,
        "summed_process_cpu_ns_diagnostic": cpu,
        "panel_sha256": hashlib.sha256(raw).hexdigest(),
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 4:
        raise SystemExit("usage: verify_static_shared_panel.py PANEL ARCHIVE SERIAL_PANEL")
    main(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]))
