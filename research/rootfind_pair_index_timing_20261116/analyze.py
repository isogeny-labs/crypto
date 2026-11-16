#!/usr/bin/env python3
"""Derive descriptive fixed-stream index timings from one saved raw run."""

import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ARMS = ("hash", "hash-aa", "flat-direct-u32", "paged", "inline")
EXPECTED = {
    13: (17, 33, 272, 261, 11, 2112),
    19: (70, 139, 4830, 4800, 30, 8896),
    23: (138, 275, 18906, 18842, 64, 17600),
}
BASE_SHA256 = {
    13: "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
    19: "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
    23: "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def median(rows: list[dict], key: str) -> int:
    return int(statistics.median(row[key] for row in rows))


def main() -> None:
    raw = (HERE / "run.stdout").read_text().splitlines()
    cases = [
        json.loads(line.split("=", 1)[1])
        for line in raw
        if line.startswith("PAIR_INDEX_CASE_JSON=")
    ]
    rows = [
        json.loads(line.split("=", 1)[1])
        for line in raw
        if line.startswith("PAIR_INDEX_TIMING_JSON=")
    ]
    assert [case["n"] for case in cases] == [13, 19, 23]
    assert len(rows) == 3 * len(ARMS) * 101
    assert any(line.startswith("test result: ok. 1 passed; 0 failed") for line in raw)
    assert "exit_code=0" in (HERE / "run.meta").read_text().splitlines()
    groups = defaultdict(list)
    for row in rows:
        assert row["schema"] == "rootfind-pair-index-timing/v1"
        assert row["n"] in EXPECTED and row["arm"] in ARMS
        assert row["combined_ns"] == row["build_ns"] + row["probe_ns"]
        assert row["build_ns"] > 0 and row["probe_ns"] > 0
        groups[row["n"], row["arm"]].append(row)

    cells = []
    for case in cases:
        n = case["n"]
        assert case["schema"] == "rootfind-pair-index-case/v1"
        assert case["base_sha256"] == BASE_SHA256[n]
        actual = tuple(
            case[k]
            for k in (
                "abscissae",
                "points",
                "pair_attempts",
                "first_keys",
                "checked_collisions",
                "probes",
            )
        )
        assert actual == EXPECTED[n], (n, actual)
        summary = {}
        for arm in ARMS:
            arm_rows = groups[n, arm]
            assert len(arm_rows) == 101
            assert sorted(row["repetition"] for row in arm_rows) == list(range(101))
            assert all(row["checksum"] == case["expected_checksum"] for row in arm_rows)
            assert all(row["pair_sha256"] == case["pair_sha256"] for row in arm_rows)
            assert all(row["probe_sha256"] == case["probe_sha256"] for row in arm_rows)
            summary[arm] = {
                "repetitions": 101,
                "build_median_ns": median(arm_rows, "build_ns"),
                "probe_median_ns": median(arm_rows, "probe_ns"),
                "combined_median_ns": median(arm_rows, "combined_ns"),
                "combined_min_ns": min(row["combined_ns"] for row in arm_rows),
                "combined_max_ns": max(row["combined_ns"] for row in arm_rows),
            }
        hash_total = summary["hash"]["combined_median_ns"]
        aa_total = summary["hash-aa"]["combined_median_ns"]
        aa_difference = abs(aa_total - hash_total)
        for arm in ARMS[2:]:
            total = summary[arm]["combined_median_ns"]
            summary[arm]["combined_ratio_to_hash"] = total / hash_total
            summary[arm]["passes_component_rule"] = total < hash_total - aa_difference
        cells.append(
            {
                "n": n,
                "curve_id": case["curve_id"],
                "base_sha256": case["base_sha256"],
                "pair_attempts": case["pair_attempts"],
                "first_keys": case["first_keys"],
                "checked_collisions": case["checked_collisions"],
                "probes": case["probes"],
                "pair_sha256": case["pair_sha256"],
                "probe_sha256": case["probe_sha256"],
                "expected_checksum": case["expected_checksum"],
                "hash_aa_combined_difference_ns": aa_difference,
                "arms": summary,
            }
        )
    result = {
        "schema": "rootfind-pair-index-timing-result/v1",
        "status": "completed_l0_component_timer",
        "run_id": "run-20261116T141419Z",
        "protocol_commit": "46faae79f8",
        "amendment_commit": "71d5af799b",
        "source_commit": "1b91f79a5c",
        "executable_sha256": "4f8e218be771a8cf58cc1005c9aa49561ae85701d3846f5af0f266ca862ec7ff",
        "raw_sha256": {
            "stdout": sha(HERE / "run.stdout"),
            "stderr": sha(HERE / "run.stderr"),
            "meta": sha(HERE / "run.meta"),
        },
        "unit": "nanoseconds measured with Instant on one L0 macOS process",
        "timed_scope": "fresh index allocation/insertion plus fixed precomputed key lookup",
        "omitted": [
            "elliptic-curve pair sums",
            "target and probe-key generation",
            "relation rank and linear algebra",
            "verification",
            "process startup",
            "per-arm peak RSS",
        ],
        "cells": cells,
        "independent_replay": False,
    }
    (HERE / "RESULTS.json").write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
