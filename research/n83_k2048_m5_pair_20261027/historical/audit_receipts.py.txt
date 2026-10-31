#!/usr/bin/env python3
"""Check identities, pairing, order and guard receipts for the frozen panel."""

import argparse
import json
from pathlib import Path


CELLS = [
    ("u-0-preflight", "U", 0, "preflight"),
    ("d-0-preflight", "D", 0, "preflight"),
    ("u-0-search", "U", 0, "search"),
    ("d-0-search", "D", 0, "search"),
    ("d-1-search", "D", 1, "search"),
    ("u-1-search", "U", 1, "search"),
]
OBJECTS = {
    "U": "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a",
    "D": "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7",
}
CORPUS = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd"
BINARY = "5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06"
SOURCE = "a70610fb78a193bece7c387a72594dbad0f233b277ec7430fbb59213fd51f12a"
TARGETS = {
    0: ["1834913121178197242856238", "8797964190732846213981377"],
    1: ["5809312440517013698248638", "3568442427236746854022671"],
}


def audit(directory):
    if not __debug__:
        raise RuntimeError("receipt audit requires Python assertions enabled")
    rows = []
    previous_end = None
    for cell, arm, fixture, mode in CELLS:
        guard = json.loads((directory / f"{cell}-guard.json").read_text())
        worker = json.loads((directory / f"{cell}.json").read_text())
        assert guard["schema"] == "n83.k2048-m5-guard/v1"
        assert worker["schema"] == "n83.k2048-m5-model/v1"
        assert guard["cell"] == cell and guard["status"] == "PASS"
        assert worker["arm"] == arm and worker["fixture_index"] == fixture
        assert worker["mode"] == mode
        assert worker["target_point"] == TARGETS[fixture]
        assert worker["curve_id"] == "EC1N83Ckb0h252dacc102a4"
        assert worker["columns"] == 2048 and worker["points"] == 339968
        assert worker["representative_x_count"] == 2048
        assert guard["object_sha256"] == worker["object_sha256"] == OBJECTS[arm]
        assert guard["corpus_sha256"] == worker["corpus_sha256"] == CORPUS
        assert guard["binary_sha256"] == worker["binary_sha256"] == BINARY
        assert worker["source_blake3"] == SOURCE
        assert guard["actual_memory_limit_bytes"] == 4 * 1024**3
        assert guard["actual_nano_cpus"] == 1_000_000_000
        assert guard["cgroup_memory_peak_bytes"] <= 4 * 1024**3
        assert guard["cgroup_memory_peak_bytes"] >= worker["memory_cgroup_peak_bytes"]
        assert guard["network"] == "none" and not guard["outer_watchdog_fired"]
        assert not guard["oom_killed"] and guard["exit_code"] == 0
        assert worker["sat_variables"] == 111478
        assert worker["sat_clauses"] >= 1140557
        assert worker["sat_xor_rows"] == 4980
        assert worker["group_relation_verified"] is False
        assert worker["rank"] is None and worker["candidate_id"] is None
        assert worker["online_ms"] is None
        if mode == "preflight":
            assert worker["root_status"] == worker["solve_status"] == "SKIPPED"
            assert worker["conflicts"] == 0
            assert worker["sat_clauses"] == 1140557
        else:
            assert worker["root_status"] == "Fixpoint"
            assert worker["solve_status"] in ("Unknown", "Sat", "Unsat")
            assert worker["conflicts"] <= 5000
            if worker["solve_status"] == "Sat":
                assert worker["model_verified"] is True
        if previous_end is not None:
            assert guard["started_utc"] >= previous_end, "cells overlap or run out of order"
        previous_end = guard["ended_utc"]
        rows.append({
            "cell": cell,
            "solve_status": worker["solve_status"],
            "conflicts": worker["conflicts"],
            "model_ms": worker["model_ms"],
            "root_ms": worker["root_ms"],
            "search_ms": worker["search_ms"],
            "process_wall_ms": worker["process_wall_ms"],
            "peak_bytes": guard["cgroup_memory_peak_bytes"],
        })
    return {
        "schema": "n83.k2048-m5-panel-audit/v1",
        "status": "PASS",
        "cells": rows,
        "single_target_online_speedup": None,
        "factor_base_selection": None,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.input)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
