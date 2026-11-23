#!/usr/bin/env python3
"""Check frozen points, dynamic arm order and complete event accounting."""

import json
from pathlib import Path
from tempfile import TemporaryDirectory

from research.n83_fresh_cold_binary_retest_20261021 import run
from research.n83_fresh_cold_binary_retest_20261021.test_arms import files
from research.n83_online_finalists_20261018.test_select import fixture


def main():
    hashes, lines = run.fixture()
    assert len(lines) == 16 and len({line for line in lines}) == 16
    with TemporaryDirectory() as temporary:
        directory = Path(temporary)
        arms, choice = run.choose(*files(directory, fixture()))
        mixed = fixture()
        for point in mixed["points"]:
            for index, arm in enumerate(point["arms"]):
                arm["cold_process_ms"] = 70_000 + 10_000 * index
        five_arms, _ = run.choose(*files(directory, mixed))
    assert len(arms) == 4 and len(choice["ordered_candidate_ids"]) == 4
    assert len(five_arms) == 5
    events = [run.schedule(stratum, arms) for stratum in range(2)]
    five_events = [run.schedule(stratum, five_arms) for stratum in range(2)]
    assert all(len(part) == 8 * len(arms) * 8 for part in events)
    assert sum(map(len, five_events)) == 640
    assert {row["point_index"] for part in events for row in part} == set(range(16))
    for part in events:
        for index in sorted({row["point_index"] for row in part}):
            rows = [row for row in part if row["point_index"] == index]
            for arm in arms:
                pair = [row for row in rows if row["candidate"] == arm[0]]
                assert [row["kind"] for row in pair].count("ic") == 1
                assert [row["kind"] for row in pair].count("rho") == 1
                assert [row["kind"] for row in pair].count("calibration") == 6
    print(json.dumps({"status": "PASS_FINAL_EXTENDED_PREFLIGHT",
                      "points": len(lines), "arms": len(arms),
                      "events": sum(map(len, events)),
                      "five_arm_events": sum(map(len, five_events)),
                      "frozen_input_groups": len(hashes)}, sort_keys=True))


if __name__ == "__main__":
    main()
