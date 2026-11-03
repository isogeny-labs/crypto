#!/usr/bin/env python3
"""Audit the rational-only control against every mixed-unique target."""

import gzip
import hashlib
import json
import statistics
from collections import Counter, defaultdict
from pathlib import Path

from summarize_all_target_cost import CURVES, add

HERE = Path(__file__).resolve().parent
RAW = HERE / "native_mimic_control.ndjson.gz"
PHASES = (
    "target_query_ns", "target_PDP_ns", "target_relation_check_ns",
    "target_descent_ns", "target_recovery_check_ns",
)
E_CALLS = ("setup_e_add", "relation_e_add", "online_e_add")
T_CALLS = ("setup_t_add", "relation_t_add", "online_t_add")
SAME_RELATION = (
    "trials", "raw_hits", "unique_raw_rows", "unique_trace_rows",
    "raw_rank", "trace_rank", "rank_goal", "online_queries",
    "online_relation_checks", "unspanned_hits", "recovered",
    "correct", "relation_probes", "online_probes", "online_row_ops",
)


def sha256(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def audit():
    count = Counter()
    generators = {}
    points = {}
    census = defaultdict(dict)
    equality = set()
    pending = {}
    summary = defaultdict(lambda: defaultdict(list))
    paired = 0
    for line in gzip.open(RAW, "rt"):
        row = json.loads(line)
        kind = row["kind"]
        count[kind] += 1
        if kind == "metadata":
            assert row["schema"] == "mixed-extension-ic/native-mimic/v1"
            assert row["target_schedule"] == "every_nonzero_scalar"
        elif kind == "curve":
            p = row["p"]
            a, b, r, nt = CURVES[p]
            assert (row["a"], row["b"], row["r"],
                    row["twist_order"]) == (a, b, r, nt)
            generator = (row["generator_x"], row["generator_y"])
            generators[p] = generator
            point = None
            pts = {}
            for k in range(1, r):
                point = add(point, generator, p, a)
                assert point is not None
                pts[k] = point
            assert add(point, generator, p, a) is None
            points[p] = pts
        elif kind == "extension_order":
            p = row["p"]
            assert row["enumerated_x"] == p * p
            assert row["order"] == CURVES[p][2] * CURVES[p][3]
        elif kind == "color_space":
            assert (row["zero_color_vectors"], row["rank"],
                    row["codimension"]) == (2, 1, 2)
        elif kind == "census":
            key = (row["p"], row["base_size"])
            assert row["arm"] in ("native-mimic", "mixed-unique")
            census[key][row["arm"]] = row
            assert row["targets"] == CURVES[row["p"]][2] - 1
            assert row["trace_rank"] == row["base_size"] - 2
        elif kind == "mimic_equality":
            assert row["equal"] is True
            equality.add((row["p"], row["base_size"]))
        elif kind == "solve":
            p, b, k = row["p"], row["base_size"], row["seed"]
            assert b in (12, 24, 36) and k in points[p]
            assert (row["target_x"], row["target_y"]) == points[p][k]
            assert row["recovered"] == k and row["correct"] is True
            assert row["policy"] == "row-span"
            assert row["raw_columns"] == row["trace_columns"] == b
            assert row["unique_traces"] == b
            assert row["rank_goal"] == row["raw_rank"] == row["trace_rank"] == b - 2
            assert row["full_log_solution"] is False
            assert sum(row[x] for x in PHASES) == row["online_ns"]
            assert row["total_ns"] >= sum(row[x] for x in (
                "base_ns", "table_ns", "relation_ns", "linear_ns",
                "online_ns",
            ))
            arm = row["arm"]
            assert arm in ("native-mimic", "mixed-unique")
            key = (p, b, k)
            assert arm not in pending.setdefault(key, {})
            pending[key][arm] = row
            summary[(p, b)][arm + "_total_us"].append(row["total_ns"] / 1000)
            summary[(p, b)][arm + "_e_add"].append(sum(row[x] for x in E_CALLS))
            summary[(p, b)][arm + "_t_add"].append(sum(row[x] for x in T_CALLS))
            if len(pending[key]) == 2:
                m = pending[key]["native-mimic"]
                u = pending[key]["mixed-unique"]
                assert all(m[field] == u[field] for field in SAME_RELATION), key
                assert all(m[field] == u[field] for field in E_CALLS), key
                assert all(m[field] == 0 for field in T_CALLS), key
                assert sum(u[field] for field in T_CALLS) > 0, key
                paired += 1
                del pending[key]
        else:
            raise AssertionError(kind)
    assert not pending and len(equality) == len(census) == 18
    assert count == Counter({
        "metadata": 1, "curve": 6, "extension_order": 6,
        "color_space": 6, "census": 36, "mimic_equality": 18,
        "solve": 24624,
    }), count
    assert paired == 12312
    rows = []
    for p in CURVES:
        for b in (12, 24, 36):
            c = census[(p, b)]
            assert set(c) == {"native-mimic", "mixed-unique"}
            for field in ("covered", "raw_hits", "unique_raw_rows",
                          "unique_trace_rows", "raw_rank", "trace_rank"):
                assert c["native-mimic"][field] == c["mixed-unique"][field]
            s = summary[(p, b)]
            assert len(s["native-mimic_total_us"]) == CURVES[p][2] - 1
            assert len(s["mixed-unique_total_us"]) == CURVES[p][2] - 1
            rows.append({
                "p": p, "B": b, "targets": CURVES[p][2] - 1,
                "relation_rows": c["native-mimic"]["unique_trace_rows"],
                "covered": c["native-mimic"]["covered"],
                "native_mimic_median_total_us": round(statistics.median(
                    s["native-mimic_total_us"]), 3),
                "mixed_unique_median_total_us": round(statistics.median(
                    s["mixed-unique_total_us"]), 3),
                "median_e_add_both": statistics.median(
                    s["native-mimic_e_add"]),
                "median_mixed_t_add": statistics.median(
                    s["mixed-unique_t_add"]),
            })
    return {"schema": "mixed-native-mimic-audit/v1",
            "raw_sha256": sha256(RAW),
            "source_sha256": sha256(HERE / "mixed_extension_ic.rs"),
            "record_counts": dict(count), "paired_targets": paired,
            "same_relation_and_E_calls": paired,
            "rows": rows}


if __name__ == "__main__":
    print(json.dumps(audit(), indent=2))
