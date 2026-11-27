#!/usr/bin/env python3
"""Check the bounded 31-bit APS queue's public results and work ledger."""

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
    wall = cpu = pairs = cancelled = 0
    for index, (case, old) in enumerate(zip(panel["cases"], archive["cases"])):
        assert case["case"] == old["case"] == index
        assert "error" not in case
        report = case["report"]
        assert report["public_Q"] == old["public_Q"]
        assert report["known_answer_in_solver"] is False
        assert report["complete_target_scalar_recovered"] is True
        witness = report["solution"]
        assert report["public_Q"] == serial["cases"][index]["report"]["public_Q"]
        assert witness == serial["cases"][index]["report"]["serial_witness"]
        k = old["caller_known_scalar_verifier_only"]
        assert witness["log"] == k
        assert (witness["alpha"] + witness["beta"] * k) % N == 0
        assert report["shared_preparation_s3_sqrts"] == 768
        assert report["workers"] == 4
        workers = report["worker_reports"]
        assert len(workers) == 4
        assert [worker["worker"] for worker in workers] == list(range(4))
        queue = report["queue"]
        assert queue["capacity_cells"] == 8
        assert queue["chunk_rows"] == 16
        assert queue["generated_cells"] == 24
        assert queue["admitted_cells"] == 24
        assert queue["queued_cells"] == 24
        assert queue["dequeued_cells"] == 24
        assert queue["never_started_cells"] == 0
        cells = queue["cells"]
        assert [cell["id"] for cell in cells] == list(range(24))
        assert [cell["rows"] for cell in cells] == [
            [16 * j, 16 * (j + 1)] for j in range(24)
        ]
        assert sum(worker["cells_received"] for worker in workers) == 24
        assert sum(worker["s3_pairs"] for worker in workers) == report[
            "parallel_s3_pairs_including_speculation"
        ]
        assert sum(cell["s3_pairs"] for cell in cells) == report[
            "parallel_s3_pairs_including_speculation"
        ]
        statuses = [cell["status"] for cell in cells]
        assert all(status in {"completed", "witness", "cancelled_unstarted",
                              "cancelled_partial"} for status in statuses)
        assert sum(status.startswith("cancelled") for status in statuses) == queue[
            "cancelled_cells"
        ]
        assert sum(status in {"completed", "witness"} for status in statuses) == queue[
            "completed_or_witness_cells"
        ]
        for cell in cells:
            assert 0 <= cell["rows_started"] <= 16
            assert cell["s3_pairs"] == cell["rows_started"] * 384
        assert report["cold_wall_ns"] > 0
        assert report["process_cpu_ns"] > 0
        wall += report["cold_wall_ns"]
        cpu += report["process_cpu_ns"]
        pairs += report["parallel_s3_pairs_including_speculation"]
        cancelled += queue["cancelled_cells"]
    print(json.dumps({
        "standing": "development queue correctness and accounting check",
        "cases": 48,
        "checked_witnesses": 48,
        "identical_serial_witnesses": 48,
        "s3_pairs": pairs,
        "cancelled_cells": cancelled,
        "summed_cold_wall_ns_diagnostic": wall,
        "summed_process_cpu_ns_diagnostic": cpu,
        "panel_sha256": hashlib.sha256(raw).hexdigest(),
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 4:
        raise SystemExit("usage: verify_queued_panel.py PANEL_JSON ARCHIVE_JSON SERIAL_PANEL_JSON")
    main(Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]))
