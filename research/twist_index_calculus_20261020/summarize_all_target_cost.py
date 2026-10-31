#!/usr/bin/env python3
"""Independently audit the exhaustive cold one-target mixed IC sweep."""

import gzip
import hashlib
import json
import math
import statistics
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
RAW = HERE / "all_target_cost.ndjson.gz"
CURVES = {
    101: (3, 17, 107, 97),
    211: (1, 1, 223, 201),
    307: (1, 6, 331, 285),
    509: (1, 17, 503, 517),
    1009: (1, 14, 1013, 1007),
    2003: (2, 15, 1933, 2075),
}
ARMS = ("native-B", "native-trace", "mixed-rect", "mixed-unique")
PHASES = (
    "target_query_ns", "target_PDP_ns", "target_relation_check_ns",
    "target_descent_ns", "target_recovery_check_ns",
)


def add(left, right, p, a):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if left == right:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow((x2 - x1) % p, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def rank_quantile(values, q):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(q * len(ordered)) - 1)]


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as data:
        for chunk in iter(lambda: data.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def audit():
    expected = {}
    seen = set()
    counts = Counter()
    cells = defaultdict(lambda: defaultdict(list))
    metadata_seen = False
    with gzip.open(RAW, "rt") as data:
        for line in data:
            row = json.loads(line)
            kind = row["kind"]
            counts[kind] += 1
            if kind == "metadata":
                assert not metadata_seen
                assert row["schema"] == "mixed-extension-ic/all-target-cost/v1"
                assert row["target_schedule"] == "every_nonzero_scalar"
                metadata_seen = True
            elif kind == "curve":
                p = row["p"]
                a, b, r, nt = CURVES[p]
                assert (row["a"], row["b"], row["r"],
                        row["twist_order"]) == (a, b, r, nt)
                generator = (row["generator_x"], row["generator_y"])
                assert (generator[1] ** 2 - generator[0] ** 3
                        - a * generator[0] - b) % p == 0
                points = {}
                point = None
                for k in range(1, r):
                    point = add(point, generator, p, a)
                    assert point is not None and point not in points.values()
                    points[k] = point
                assert add(point, generator, p, a) is None
                expected[p] = points
            elif kind == "extension_order":
                p = row["p"]
                assert row["enumerated_x"] == p * p
                assert row["order"] == row["order_product"] == (
                    CURVES[p][2] * CURVES[p][3]
                )
            elif kind == "color_space":
                assert (row["zero_color_vectors"], row["rank"],
                        row["codimension"]) == (2, 1, 2)
            elif kind in ("solve", "rho"):
                p, b, k = row["p"], row["base_size"], row["seed"]
                assert b in (12, 24, 36) and k in expected[p]
                assert (row["target_x"], row["target_y"]) == expected[p][k]
                assert row["recovered"] == k
                arm = row["arm"] if kind == "solve" else "rho"
                assert arm in (*ARMS, "rho")
                key = (p, b, k, arm)
                assert key not in seen, key
                seen.add(key)
                cells[(p, b, arm)]["total_ns"].append(row["total_ns"])
                if kind == "solve":
                    columns = b // 3 if arm in ("native-trace", "mixed-rect") else b
                    rank = b - 2 if arm == "mixed-unique" else columns
                    assert row["correct"] is True
                    assert row["policy"] == "row-span"
                    assert (row["trace_columns"], row["unique_traces"],
                            row["trace_rank"], row["rank_goal"]) == (
                                columns, columns, rank, rank
                            )
                    assert row["raw_rank"] == (
                        b - 2 if arm.startswith("mixed") else columns
                    )
                    assert row["full_log_solution"] == (arm != "mixed-unique")
                    assert row["online_relation_checks"] >= 1
                    assert 1 <= row["trials"] <= CURVES[p][2] - 1
                    assert 1 <= row["online_queries"] <= CURVES[p][2]
                    assert sum(row[f] for f in PHASES) == row["online_ns"]
                    assert row["total_ns"] >= sum(row[f] for f in (
                        "base_ns", "table_ns", "relation_ns", "linear_ns",
                        "online_ns",
                    ))
                    cells[(p, b, arm)]["online_ns"].append(row["online_ns"])
                    cells[(p, b, arm)]["online_queries"].append(
                        row["online_queries"]
                    )
                    cells[(p, b, arm)]["group_adds"].append(sum(
                        row[f] for f in (
                            "setup_e_add", "setup_t_add", "relation_e_add",
                            "relation_t_add", "online_e_add", "online_t_add",
                        )
                    ))
            else:
                raise AssertionError(kind)
    assert metadata_seen and set(expected) == set(CURVES)
    assert counts == Counter({
        "metadata": 1, "curve": 6, "extension_order": 6,
        "color_space": 6, "solve": 49248, "rho": 12312,
    }), counts
    for p in CURVES:
        for b in (12, 24, 36):
            for arm in (*ARMS, "rho"):
                assert len(cells[(p, b, arm)]["total_ns"]) == CURVES[p][2] - 1
    summary = []
    for p in CURVES:
        for b in (12, 24, 36):
            arms = {}
            for arm in (*ARMS, "rho"):
                metrics = cells[(p, b, arm)]
                arms[arm] = {
                    "median_total_us": round(statistics.median(
                        metrics["total_ns"]) / 1000, 3),
                    "p05_total_us": round(rank_quantile(
                        metrics["total_ns"], 0.05) / 1000, 3),
                    "p95_total_us": round(rank_quantile(
                        metrics["total_ns"], 0.95) / 1000, 3),
                }
                if arm != "rho":
                    arms[arm].update({
                        "median_online_us": round(statistics.median(
                            metrics["online_ns"]) / 1000, 3),
                        "median_online_queries": statistics.median(
                            metrics["online_queries"]),
                        "max_online_queries": max(metrics["online_queries"]),
                        "median_group_adds": statistics.median(
                            metrics["group_adds"]),
                    })
            summary.append({"p": p, "B": b,
                            "targets_per_arm": CURVES[p][2] - 1, "arms": arms})
    return {"schema": "mixed-all-target-cost-summary/v1",
            "raw_sha256": digest(RAW),
            "source_sha256": digest(HERE / "mixed_extension_ic.rs"),
            "record_counts": dict(counts), "rows": summary}


if __name__ == "__main__":
    print(json.dumps(audit(), indent=2))
