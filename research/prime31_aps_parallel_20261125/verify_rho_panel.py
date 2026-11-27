#!/usr/bin/env python3
"""Independently check the development shared-table rho panel's custody."""

import hashlib
import json
import sys
from pathlib import Path


def main(panel_path: Path, archive_path: Path) -> None:
    raw = panel_path.read_bytes()
    panel = json.loads(raw)
    archive = json.loads(archive_path.read_bytes())
    assert panel["workers"] == 4
    assert len(panel["cases"]) == len(archive["cases"]) == 48
    total_steps = total_wall = total_cpu = 0
    fold_mode = panel["cases"][0]["report"]["fold_negation"]
    for index, (case, old) in enumerate(zip(panel["cases"], archive["cases"])):
        assert case["case"] == old["case"] == index
        assert "error" not in case
        report = case["report"]
        assert report["public_Q"] == old["public_Q"]
        assert report["known_answer_in_solver"] is False
        assert report["complete_target_scalar_recovered"] is True
        assert report["log"] == old["caller_known_scalar_verifier_only"]
        assert report["workers"] == 4
        assert report["lanes_per_worker"] == 16
        assert report["fold_negation"] is fold_mode
        workers = report["worker_reports"]
        assert len(workers) == 4
        assert [item["worker"] for item in workers] == list(range(4))
        steps = sum(item["walk_steps"] for item in workers)
        assert steps == report["walk_steps"] <= report["step_cap"]
        assert all(item["walk_steps"] % 16 == 0 for item in workers)
        assert report["distinguished_records"] <= sum(
            item["distinguished_visits"] for item in workers
        )
        total_steps += steps
        total_wall += report["cold_wall_ns"]
        total_cpu += report.get("process_cpu_ns", 0)
    print(json.dumps({
        "standing": "development rho correctness and accounting check",
        "cases": 48,
        "checked_logs": 48,
        "walk_steps": total_steps,
        "fold_negation": fold_mode,
        "summed_cold_wall_ns_diagnostic": total_wall,
        "summed_process_cpu_ns_diagnostic": total_cpu,
        "panel_sha256": hashlib.sha256(raw).hexdigest(),
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: verify_rho_panel.py PANEL_JSON ARCHIVED_48Q_JSON")
    main(Path(sys.argv[1]), Path(sys.argv[2]))
