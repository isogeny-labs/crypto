#!/usr/bin/env python3
"""Audit all frozen ECCp-131 benchmark cells and their retained receipts."""
import hashlib
import json
import statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUNS = HERE / "runs"
ORDER = [(0, "tower"), (0, "small_a"), (0, "small_b"),
         (1, "small_a"), (1, "tower"), (1, "small_b"),
         (2, "small_a"), (2, "small_b"), (2, "tower")]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    driver = json.loads((RUNS / "bench-driver.json").read_text())
    assert [(x["rep"], x["arm"]) for x in driver] == ORDER
    assert all(x["exit_code"] == 0 for x in driver)
    source_paths = {
        "replay.json": RUNS / "replay.json",
        "screen.py": HERE.parent / "eccp131_structured_ic_20261022" / "screen.py",
        "curve.rs": HERE.parents[1] / "src/ecc/curve.rs",
        "params.rs": HERE.parents[1] / "src/bin/ic/params.rs",
    }
    expected_sources = {k: sha(v) for k, v in source_paths.items()}
    scalar_hashes = set()
    base_hashes = {arm: set() for _, arm in ORDER}
    rows = []
    for rep, arm in ORDER:
        stem = f"bench-r{rep}-{arm}"
        data = json.loads((RUNS / (stem + ".json")).read_text())
        receipt = json.loads((RUNS / (stem + ".receipt.json")).read_text())
        assert data["schema"] == "eccp131-equal-column-screen-bench/v1"
        assert (data["rep"], data["arm"]) == (rep, arm)
        assert data["source_sha256"] == expected_sources
        assert (data["columns"], data["seed"], data["setup_candidates"], data["target_candidates"]) == (115, 131, 256, 256)
        assert receipt["exit_code"] == 0 and receipt["stop_reason"] is None
        assert receipt["wall_limit_seconds"] == 30 and receipt["rss_limit_mib"] == 2048
        assert receipt["wall_seconds"] < 30 and receipt["max_rss_kib_sampled"] < 2048 * 1024
        assert not (RUNS / (stem + ".stderr")).read_text()
        result = data["result"]
        assert len(result["base"]) == 115 and len(result["planted_controls"]) == 8
        assert result["distinct_pair_points"] == 26450
        assert not result["setup_relations"] and not result["target_relations"]
        assert result["setup_rank"] == result["combined_rank"] == 0
        assert result["target_scalar"] is None
        assert data["ec_add_calls"] == 270564
        scalar_hashes.add(data["candidate_scalar_sha256"])
        base_hashes[arm].add(result["base_sha256"])
        rows.append({"rep": rep, "arm": arm, "screen_seconds": data["screen_seconds"],
                     "base_prepare_seconds": data["base_prepare_seconds"],
                     "process_seconds": receipt["wall_seconds"],
                     "max_rss_kib_sampled": receipt["max_rss_kib_sampled"]})
    assert len(scalar_hashes) == 1
    assert all(len(v) == 1 for v in base_hashes.values())
    assert base_hashes["small_a"] == base_hashes["small_b"]
    assert base_hashes["tower"] != base_hashes["small_a"]
    summaries = {}
    for arm in ("tower", "small_a", "small_b"):
        cells = [row for row in rows if row["arm"] == arm]
        summaries[arm] = {field: {"median": statistics.median(row[field] for row in cells),
                                  "min": min(row[field] for row in cells),
                                  "max": max(row[field] for row in cells)}
                          for field in ("screen_seconds", "base_prepare_seconds", "process_seconds", "max_rss_kib_sampled")}
    print(json.dumps({"status": "pass", "cells": len(rows), "candidate_scalar_sha256": next(iter(scalar_hashes)),
                      "base_sha256": {k: next(iter(v)) for k, v in base_hashes.items()},
                      "source_sha256": expected_sources, "rows": rows, "summaries": summaries}, indent=2))


if __name__ == "__main__":
    main()
