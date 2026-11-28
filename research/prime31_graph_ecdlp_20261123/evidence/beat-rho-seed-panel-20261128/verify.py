#!/usr/bin/env python3
"""Independently replay retained process output and summarize the frozen panel."""
import gzip
import hashlib
import json
import statistics
from pathlib import Path

from run_panel import G, N, Q, SEEDS, mul

HERE = Path(__file__).resolve().parent
OUT = HERE / "results"


def main():
    rows = [json.loads(line) for line in (OUT / "records.jsonl").read_text().splitlines()]
    assert len(rows) == 17
    assert mul(N, G) is None
    assert all(row["error"] is None and row["wait_status"] == 0 for row in rows)
    by_key = {(row["round"], row["arm"], row["aa"]): row for row in rows}
    assert len(by_key) == 17
    assert [(row["round"], row["arm"]) for row in rows[:16]] == [
        (i, arm) for i in range(8)
        for arm in (("ic", "rho") if i % 2 == 0 else ("rho", "ic"))
    ]
    raw = {}
    for row in rows:
        blob = gzip.open(OUT / row["stdout_gzip"], "rb").read()
        assert hashlib.sha256(blob).hexdigest() == row["stdout_sha256"]
        report = json.loads(blob)
        assert tuple(report["public_target"]) == Q
        scalar = report["recovered_scalar"]
        assert scalar == row["recovered_scalar"] == 987_654_321
        assert mul(scalar, G) == Q
        assert row["wall_ns"] > 0 and row["max_rss_kb"] > 0
        if row["arm"] == "ic":
            assert report["recovered_scalar_replay"] is True
            assert report["probe_additions"] == row["ic_probes"] == 10_068_344
            assert len(report["relation_rows"]) == row["ic_rows"] == 707
            assert len(report["queries"]) == row["ic_queries"] == 11
        else:
            assert report["verified"] is True and report["seed"] == row["seed"]
            for key, alias in (("steps", "rho_steps"), ("walks", "rho_walks"),
                               ("group_adds", "rho_adds"), ("cycles", "rho_cycles"),
                               ("distinguished_points", "rho_distinguished_points")):
                assert report[key] == row[alias]
        raw[(row["round"], row["arm"], row["aa"])] = report
    for i, seed in enumerate(SEEDS):
        assert by_key[i, "rho", False]["seed"] == seed
    aa = raw[8, "rho", True]
    reference = raw[0, "rho", False]
    for key in ("seed", "recovered_scalar", "steps", "group_adds", "walks",
                "cycles", "distinguished_points", "stored_distinguished_points"):
        assert aa[key] == reference[key], key
    ic = [by_key[i, "ic", False] for i in range(8)]
    rho = [by_key[i, "rho", False] for i in range(8)]
    wall_ratios = [x["wall_ns"] / y["wall_ns"] for x, y in zip(ic, rho)]
    cpu_ratios = [(x["cpu_user_s"] + x["cpu_sys_s"]) /
                  (y["cpu_user_s"] + y["cpu_sys_s"]) for x, y in zip(ic, rho)]
    summary = {
        "all_17_processes_verified": True,
        "rho_aa_trajectory_identical": True,
        "ic_median_wall_ms": statistics.median(x["wall_ns"] for x in ic) / 1e6,
        "rho_median_wall_ms": statistics.median(x["wall_ns"] for x in rho) / 1e6,
        "paired_ic_over_rho_wall": wall_ratios,
        "paired_wall_median": statistics.median(wall_ratios),
        "paired_wall_min": min(wall_ratios),
        "paired_wall_max": max(wall_ratios),
        "rho_faster_pairs": sum(value > 1 for value in wall_ratios),
        "paired_cpu_median": statistics.median(cpu_ratios),
        "rho_steps": [x["rho_steps"] for x in rho],
    }
    (OUT / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
