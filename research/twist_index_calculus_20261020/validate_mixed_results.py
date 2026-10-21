#!/usr/bin/env python3
"""Independent correctness audit of the Rust experiment's sealed NDJSON.

This is a verifier, not a measurement producer. It uses its own prime-field
group law to recompute every reported target scalar and checks the accounting
and exact-census invariants.
"""

import json
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
MAIN = HERE / "mixed_component_results.ndjson"
STRICT = HERE / "mixed_full_rank_control.ndjson"
CURVES = {
    101: (3, 17, 107, 97),
    211: (1, 1, 223, 201),
    307: (1, 6, 331, 285),
    509: (1, 17, 503, 517),
    1009: (1, 14, 1013, 1007),
    2003: (2, 15, 1933, 2075),
}
PHASES = (
    "target_query_ns",
    "target_PDP_ns",
    "target_relation_check_ns",
    "target_descent_ns",
    "target_recovery_check_ns",
)
ARMS = ("native-B", "native-trace", "mixed-rect", "mixed-unique")


def read(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def add(p, q, prime, a):
    if p is None:
        return q
    if q is None:
        return p
    x1, y1 = p
    x2, y2 = q
    if x1 == x2 and (y1 + y2) % prime == 0:
        return None
    if p == q:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, prime) % prime
    else:
        slope = (y2 - y1) * pow((x2 - x1) % prime, -1, prime) % prime
    x3 = (slope * slope - x1 - x2) % prime
    y3 = (slope * (x1 - x3) - y1) % prime
    return (x3, y3)


def mul(k, p, prime, a):
    out = None
    while k:
        if k & 1:
            out = add(out, p, prime, a)
        p = add(p, p, prime, a)
        k >>= 1
    return out


def check():
    main = read(MAIN)
    strict = read(STRICT)
    kinds = defaultdict(list)
    for row in main:
        kinds[row["kind"]].append(row)
    assert {k: len(v) for k, v in kinds.items()} == {
        "metadata": 1,
        "curve": 6,
        "extension_order": 6,
        "color_space": 6,
        "census": 72,
        "dominance": 36,
        "solve": 216,
        "rho": 54,
    }
    curve_rows = {row["p"]: row for row in kinds["curve"]}
    assert set(curve_rows) == set(CURVES)
    for p, curve in curve_rows.items():
        a, b, r, nt = CURVES[p]
        assert (curve["a"], curve["b"], curve["r"], curve["twist_order"]) == (
            a, b, r, nt
        )
        generator = (curve["generator_x"], curve["generator_y"])
        assert generator != (0, 0)
        assert mul(r, generator, p, a) is None
        assert (generator[1] * generator[1] - generator[0] ** 3
                - a * generator[0] - b) % p == 0
    for row in kinds["extension_order"]:
        p = row["p"]
        assert row["enumerated_x"] == p * p
        assert row["order"] == row["order_product"] == CURVES[p][2] * CURVES[p][3]
    for row in kinds["color_space"]:
        assert (row["zero_color_vectors"], row["rank"], row["codimension"]) == (
            2, 1, 2
        )
    for row in kinds["census"]:
        p, b = row["p"], row["base_size"]
        assert b in (12, 24, 36) and row["targets"] == CURVES[p][2] - 1
        assert 0 <= row["covered"] <= row["targets"]
        assert row["support_0"] == 0
        assert sum(row[f"support_{j}"] for j in (0, 1, 2)) + row[
            "support_3plus"
        ] == row["unique_trace_rows"]
        columns = b // 3 if row["arm"] in ("native-trace", "mixed-rect") else b
        trace_rank = b - 2 if row["arm"] == "mixed-unique" else columns
        raw_rank = b - 2 if row["arm"].startswith("mixed") else columns
        assert row["trace_rank"] == trace_rank
        assert row["raw_rank"] == raw_rank
    for row in kinds["dominance"]:
        assert row["violations"] == 0
        assert row["shared_rows"] + row["shorter_native_rows"] == row["mixed_rows"]
        if row["arm"] == "mixed-unique":
            assert row["shorter_native_rows"] == 0
    assert sum(row["mixed_rows"] for row in kinds["dominance"]
               if row["arm"] == "mixed-unique") == 27536
    assert sum(row["shared_rows"] for row in kinds["dominance"]
               if row["arm"] == "mixed-rect") == 12412
    assert sum(row["shorter_native_rows"] for row in kinds["dominance"]
               if row["arm"] == "mixed-rect") == 288
    groups = defaultdict(dict)
    for row in kinds["solve"]:
        p, b, seed = row["p"], row["base_size"], row["seed"]
        groups[(p, b, seed)][row["arm"]] = row
        assert row["correct"] is True
        assert row["rank_goal"] == row["trace_rank"]
        assert row["unique_traces"] == row["trace_columns"]
        assert row["online_relation_checks"] >= 1
        assert sum(row[k] for k in PHASES) == row["online_ns"]
        assert row["total_ns"] >= sum(
            row[k] for k in ("base_ns", "table_ns", "relation_ns",
                             "linear_ns", "online_ns")
        )
        a, bb, r, _ = CURVES[p]
        target = (row["target_x"], row["target_y"])
        generator = (
            curve_rows[p]["generator_x"],
            curve_rows[p]["generator_y"],
        )
        assert (target[1] * target[1] - target[0] ** 3
                - a * target[0] - bb) % p == 0
        assert mul(row["recovered"], generator, p, a) == target
        assert 0 <= row["recovered"] < r
        if row["arm"] == "mixed-unique":
            assert row["trace_rank"] == b - 2
            assert row["full_log_solution"] is False
        else:
            assert row["trace_rank"] == row["trace_columns"]
            assert row["full_log_solution"] is True
    assert len(groups) == 54
    for key, arm_rows in groups.items():
        assert set(arm_rows) == set(ARMS)
        scalars = {v["recovered"] for v in arm_rows.values()}
        targets = {(v["target_x"], v["target_y"]) for v in arm_rows.values()}
        assert len(scalars) == len(targets) == 1
    rho = {(row["p"], row["base_size"], row["seed"]): row
           for row in kinds["rho"]}
    assert set(rho) == set(groups)
    for key, row in rho.items():
        p, _, _ = key
        target = next(iter(groups[key].values()))
        assert (row["target_x"], row["target_y"], row["recovered"]) == (
            target["target_x"], target["target_y"], target["recovered"]
        )
        generator = (
            curve_rows[p]["generator_x"],
            curve_rows[p]["generator_y"],
        )
        assert mul(row["recovered"], generator, p, CURVES[p][0]) == (
            row["target_x"], row["target_y"]
        )
    strict_solves = [row for row in strict if row["kind"] == "solve"]
    assert len(strict_solves) == 216
    assert sum(row["correct"] for row in strict_solves) == 162
    for row in strict_solves:
        if row["arm"] == "mixed-unique":
            assert row["recovered"] is None
            assert row["trace_rank"] == row["raw_columns"] - 2
        else:
            assert row["correct"] is True
    return len(kinds["census"]), len(kinds["solve"]), len(kinds["rho"])


if __name__ == "__main__":
    census, solves, rho = check()
    print(f"VALID: {census} exhaustive censuses, {solves} IC solves, "
          f"{rho} matched rho solves, 54 retained full-rank failures")
