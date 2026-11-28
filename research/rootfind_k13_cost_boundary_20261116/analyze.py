#!/usr/bin/env python3
"""Recompute K13 clock boundaries from one pinned archived JSONL object."""

import argparse
import collections
import hashlib
import json
from pathlib import Path

EXPECTED_SHA256 = "dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44"
SESSION = "ECBS1hc322a24b6610"
ARMS = (
    "rho-strong", "matrix-f4", "matrix-f5", "inherited-f4",
    "f6-ic", "rootfind-m3", "batch-m3", "batch-m3-aa",
)


def audit(path: Path) -> dict:
    raw = path.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == EXPECTED_SHA256, "input digest"
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == 48 and {r["session_id"] for r in rows} == {SESSION}
    assert {r["arm"] for r in rows} == set(ARMS)
    assert {(r["workload"]["target_seed"], r["workload"]["target_index"]) for r in rows} == {
        (202611160511, 0), (202611160511, 1)
    }
    assert all(r["workload"]["curve_spec"] == {"kind": "koblitz", "a": 0, "n": 13} for r in rows)
    assert all(r["outcome"]["status"] == "verified" and r["outcome"]["matches_target"] for r in rows)
    assert all(r["work"]["complete"] and not r["work"]["unpriced"] and not r["work"]["known_uncounted"] for r in rows)
    assert sum(r["warmup"] for r in rows) == 16

    cells = collections.defaultdict(dict)
    for r in rows:
        if r["warmup"]:
            assert r["round"] == 0
            continue
        assert r["round"] in (1, 2)
        key = (r["workload"]["workload_sha256"], r["round"])
        assert r["arm"] not in cells[key], "duplicate arm in cell"
        cells[key][r["arm"]] = r
    assert len(cells) == 4
    for cell in cells.values():
        assert set(cell) == set(ARMS), "incomplete matched cell"
        assert len({tuple(r["workload"]["target"]) for r in cell.values()}) == 1
        assert cell["rho-strong"]["factor_base"] is None
        assert len({r["factor_base"]["points_sha256"] for name, r in cell.items() if name != "rho-strong"}) == 1
        assert cell["batch-m3"]["work"]["bitops"] == cell["batch-m3-aa"]["work"]["bitops"]

    arms = {}
    for name in ARMS:
        selected = [cell[name] for cell in cells.values()]
        solve = [r["time"]["solve_wall_ns"] for r in selected]
        process = [r["time"]["process_wall_ns"] for r in selected]
        assert all(isinstance(x, int) and x > 0 for x in solve + process)
        assert all(p >= s for p, s in zip(process, solve))
        phases = collections.Counter()
        for r in selected:
            for phase in r["work"]["phases"]:
                phases[phase["name"]] += int(phase["bitops"])
        bitops = sum(int(r["work"]["bitops"]) for r in selected)
        assert sum(phases.values()) == bitops
        arms[name] = {
            "measured_rows": 4,
            "solve_wall_ns_sum": sum(solve),
            "process_wall_ns_sum": sum(process),
            "outside_solve_ns_sum": sum(p - s for p, s in zip(process, solve)),
            "solve_wall_ns_min": min(solve),
            "solve_wall_ns_max": max(solve),
            "bitops_sum": str(bitops),
            "bitops_by_phase": dict(sorted(phases.items())),
        }

    return {
        "schema": "rootfind-k13-cost-boundary/v1",
        "input_sha256": EXPECTED_SHA256,
        "session_id": SESSION,
        "source_commit": "8f4cc824c87a236aafd28aa390c744e355a08e45",
        "scope": "archived_l0_two_targets_two_measured_rounds_posthoc",
        "records_total": 48,
        "warmups": 16,
        "measured_records": 32,
        "matched_cells": 4,
        "arms": arms,
        "independent_clock_attestation": False,
        "instruction_audit_run": False,
        "new_trial_run": False,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--records", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    args.out.write_text(json.dumps(audit(args.records), indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
