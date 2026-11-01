#!/usr/bin/env python3
"""Independently recompute the N19 standardized price tables from exact counts."""

import csv
import json
import subprocess
from collections import Counter, defaultdict
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
OUT = HERE / "results/analysis"
BITS = (16, 20, 24, 28, 32, 36, 40)
CELLS = {(s, i, l) for s in (0.0, 0.5, 1.0) for i in (1.0, 10.0, 100.0, 1000.0)
         for l in (0.0, 0.1, 1.0)}


def read_jsonl(path):
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def read_csv(path):
    with path.open(newline="") as stream:
        return list(csv.DictReader(stream))


def pval(count, cell, unit):
    sq, inv, native = cell
    denom = unit[0] + sq * unit[1] + inv * unit[2]
    assert denom > 0
    return (count["m"] + sq * count["s"] + inv * count["i"]) / denom + native * count["logical"]


def replay_source(spec, target, scalar):
    """A second source-curve implementation, independent of analyze.py."""
    p = spec["p"]
    assert spec["n"] == 2 and spec["modulus"] == 1 and p % 4 == 3

    def pair(v):
        return (v % p, v // p)

    def plus(a, b):
        return ((a[0] + b[0]) % p, (a[1] + b[1]) % p)

    def minus(a, b):
        return ((a[0] - b[0]) % p, (a[1] - b[1]) % p)

    def times(a, b):
        return ((a[0] * b[0] - a[1] * b[1]) % p,
                (a[0] * b[1] + a[1] * b[0]) % p)

    def divide(a, b):
        factor = pow((b[0] ** 2 + b[1] ** 2) % p, p - 2, p)
        return times(a, (b[0] * factor % p, -b[1] * factor % p))

    def step(left, right):
        if left is None:
            return right
        if right is None:
            return left
        x, y = left
        xx, yy = right
        if x == xx and plus(y, yy) == (0, 0):
            return None
        slope = (divide(plus(times((3, 0), times(x, x)), pair(spec["a"])),
                        times((2, 0), y)) if left == right else
                 divide(minus(yy, y), minus(xx, x)))
        x3 = minus(minus(times(slope, slope), x), xx)
        y3 = minus(times(slope, minus(x, x3)), y)
        return (x3, y3)

    base = (pair(spec["gx"]), pair(spec["gy"]))
    want = tuple(pair(int(value, 16)) for value in target)
    acc = None
    while scalar:
        if scalar & 1:
            acc = step(acc, base)
        base = step(base, base)
        scalar >>= 1
    return acc == want


def archive_rows(bits):
    archive = ROOT / "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst"
    member = f"sessions_projective_chart_v2/order-genus-{bits}/records.jsonl"
    data = subprocess.check_output(["tar", "--zstd", "-xOf", str(archive), member])
    out = {}
    for row in map(json.loads, data.splitlines()):
        if row["arm"] == "rho_neg":
            ident = row["workload"]["workload_id"]
            assert ident not in out
            out[ident] = row["workload"]
    assert len(out) == 180
    return out


def close(actual, expected):
    return abs(actual - expected) <= 2e-9 * max(1.0, abs(expected))


def main():
    records = read_jsonl(OUT / "cost-records.jsonl")
    replay = read_jsonl(OUT / "replay.jsonl")
    assert len(records) == len(replay) == 1260
    by_id = {}
    for row in records:
        assert row["schema"] == "kummer-n19-cost-record/v1"
        ident = row["workload_id"]
        assert ident not in by_id
        by_id[ident] = row
        assert row["bits"] in BITS
        assert row["rho_verified"] is True
        assert row["baseline_verified"] is True
        assert row["fallback_failures"] <= row["fallback_steps"]
        assert row["transition_validation_enabled"] is False
    assert Counter(x["bits"] for x in records) == {bits: 180 for bits in BITS}
    assert {x["workload_id"] for x in replay} == set(by_id)
    archived = {ident: workload for bits in BITS for ident, workload in archive_rows(bits).items()}
    assert set(archived) == set(by_id)
    for row in replay:
        source = by_id[row["workload_id"]]
        workload = archived[row["workload_id"]]
        assert row["bits"] == source["bits"]
        assert row["rho_scalar_replayed"] is True
        assert replay_source(workload["curve_spec"], workload["target"], row["rho_recovered"])
        assert row["candidate_status"] == source["candidate_status"]
        assert row["candidate_scalar_replayed"] == source["candidate_verified"]
        if source["candidate_verified"]:
            assert row["candidate_recovered"] == row["rho_recovered"]
            assert replay_source(workload["curve_spec"], workload["target"],
                                 row["candidate_recovered"])
    target = read_csv(OUT / "target-price.csv")
    assert len(target) == 1260 * 36
    grouped = defaultdict(list)
    seen = set()
    for row in target:
        assert row["schema"] == "kummer-n19-target-cost/v1"
        ident = row["workload_id"]
        assert ident in by_id
        source = by_id[ident]
        cell = tuple(float(row[k]) for k in ("S_per_M", "I_per_M", "native_GAE_per_logical"))
        assert cell in CELLS
        key = (ident, cell)
        assert key not in seen
        seen.add(key)
        assert int(row["bits"]) == source["bits"]
        assert int(row["p"]) == source["p"]
        assert int(row["target_index"]) == source["target_index"]
        assert row["candidate_status"] == source["candidate_status"]
        assert row["rho_verified"] == "True"
        assert row["candidate_verified"] == str(source["candidate_verified"])
        unit = source["source_add_unit"]
        online = pval(source["candidate_online"], cell, unit)
        cold = pval(source["candidate_cold"], cell, unit)
        baseline = pval(source["baseline_cold"], cell, unit)
        rho = pval(source["rho"], cell, unit)
        assert rho > 0 and baseline > 0 and cold >= online
        for field, value in (("candidate_online_GAE", online),
                             ("candidate_cold_GAE", cold), ("baseline_cold_GAE", baseline),
                             ("rho_GAE", rho)):
            assert close(float(row[field]), value), (ident, cell, field)
        assert row["baseline_verified"] == "True"
        assert close(float(row["baseline_cold_over_rho"]), baseline/rho)
        if source["candidate_verified"]:
            assert close(float(row["candidate_online_over_rho"]), online/rho)
            assert close(float(row["candidate_cold_over_rho"]), cold/rho)
            assert close(float(row["candidate_cold_over_baseline"]), cold/baseline)
            assert row["candidate_win"] == str(cold < rho)
            assert row["candidate_loss"] == str(cold >= rho)
        else:
            assert row["candidate_online_over_rho"] == ""
            assert row["candidate_cold_over_rho"] == ""
            assert row["candidate_cold_over_baseline"] == ""
            assert row["candidate_win"] == row["candidate_loss"] == "False"
        grouped[cell].append((source["bits"], cold, baseline, rho,
                              source["candidate_verified"], cold < rho))
    assert len(grouped) == 36 and all(len(x) == 1260 for x in grouped.values())
    grid = read_csv(OUT / "price-grid.csv")
    assert len(grid) == 36 * 8
    grid_seen = set()
    for row in grid:
        assert row["schema"] == "kummer-n19-price-grid/v1"
        cell = tuple(float(row[k]) for k in ("S_per_M", "I_per_M", "native_GAE_per_logical"))
        assert cell in CELLS
        slice_name = row["slice"]
        assert slice_name == "all" or slice_name in {f"bits{bits}" for bits in BITS}
        key = (slice_name, cell)
        assert key not in grid_seen
        grid_seen.add(key)
        subset = [x for x in grouped[cell] if slice_name == "all" or x[0] == int(slice_name[4:])]
        verified = [x for x in subset if x[4]]
        assert int(row["targets"]) == len(subset)
        assert int(row["verified"]) == len(verified)
        assert int(row["failures"]) == int(row["null_ratios"]) == len(subset)-len(verified)
        assert int(row["candidate_wins"]) == sum(x[4] and x[5] for x in subset)
        assert int(row["candidate_losses"]) == sum(x[4] and not x[5] for x in subset)
        assert close(float(row["candidate_cold_GAE"]), sum(x[1] for x in subset))
        assert close(float(row["baseline_cold_GAE"]), sum(x[2] for x in subset))
        assert close(float(row["rho_GAE"]), sum(x[3] for x in subset))
        assert close(float(row["baseline_cold_over_rho"]),
                     sum(x[2] for x in subset)/sum(x[3] for x in subset))
        if len(verified) == len(subset):
            assert close(float(row["candidate_cold_over_rho"]),
                         sum(x[1] for x in verified)/sum(x[3] for x in verified))
            assert close(float(row["candidate_cold_over_baseline"]),
                         sum(x[1] for x in verified)/sum(x[2] for x in verified))
        else:
            assert row["candidate_cold_over_rho"] == ""
            assert row["candidate_cold_over_baseline"] == ""
    summary = json.loads((OUT / "summary.json").read_text())
    assert summary["targets"] == 1260 and summary["price_cells"] == 36
    assert summary["slice_price_rows"] == 288
    assert summary["rho_verified"] == 1260
    assert summary["baseline_verified"] == 1260
    assert summary["candidate_verified"] == sum(x["candidate_verified"] for x in records)
    assert summary["status"] == dict(Counter(x["candidate_status"] for x in records))
    print(json.dumps({"schema": "kummer-n19-independent-validator/v1", "valid": True,
                      "targets": 1260, "target_price_rows": len(target),
                      "price_grid_rows": len(grid), "candidate_verified": summary["candidate_verified"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
