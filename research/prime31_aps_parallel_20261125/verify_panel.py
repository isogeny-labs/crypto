#!/usr/bin/env python3
"""Check the development APS row-panel against the archived public targets."""

import hashlib
import json
import sys
from pathlib import Path

N = 2_147_446_831


def main(panel_path: Path, archive_path: Path) -> None:
    panel_bytes = panel_path.read_bytes()
    archive_bytes = archive_path.read_bytes()
    panel = json.loads(panel_bytes)
    archive = json.loads(archive_bytes)
    cases = panel["cases"]
    assert panel["workers"] == 4
    assert len(cases) == len(archive["cases"]) == 48
    serial_pairs = parallel_pairs = 0
    for index, (case, old) in enumerate(zip(cases, archive["cases"])):
        assert case["case"] == old["case"] == index
        assert "error" not in case
        report = case["report"]
        assert report["public_Q"] == old["public_Q"]
        assert report["serial_witness"] == report["parallel_witness"]
        witness = report["parallel_witness"]
        assert witness["status"] == "verified"
        k = old["caller_known_scalar_verifier_only"]
        assert witness["log"] == k
        assert (witness["alpha"] + witness["beta"] * k) % N == 0
        workers = report["worker_reports"]
        assert len(workers) == 4
        assert [w["left_rows_assigned"] for w in workers] == [
            [0, 96], [96, 192], [192, 288], [288, 384]
        ]
        assert sum(w["s3_pairs"] for w in workers) == report[
            "parallel_s3_pairs_including_speculation"
        ]
        for worker, item in enumerate(workers):
            assert item["worker"] == worker
            assert item["left_rows_started"] + item["left_rows_not_started"] == 96
            assert item["s3_pairs"] == item["left_rows_started"] * 384
        serial_pairs += report["serial_s3_pairs"]
        parallel_pairs += report["parallel_s3_pairs_including_speculation"]
    print(json.dumps({
        "standing": "development parity and accounting check, not performance",
        "cases": len(cases),
        "checked_witnesses": len(cases),
        "serial_s3_pairs": serial_pairs,
        "parallel_s3_pairs": parallel_pairs,
        "parallel_over_serial_s3_pairs": parallel_pairs / serial_pairs,
        "panel_sha256": hashlib.sha256(panel_bytes).hexdigest(),
        "archive_sha256": hashlib.sha256(archive_bytes).hexdigest(),
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: verify_panel.py PANEL_JSON ARCHIVED_48Q_JSON")
    main(Path(sys.argv[1]), Path(sys.argv[2]))
