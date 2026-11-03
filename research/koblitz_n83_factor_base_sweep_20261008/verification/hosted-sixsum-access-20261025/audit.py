#!/usr/bin/env python3
"""Fail-closed 4+2 search receipt replay using the pinned source auditor."""

from __future__ import annotations

import blake3


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def search(config: object, result: object, arm: dict, base_points: list,
           residual: set, pinned, protocol: dict) -> str:
    require(isinstance(config, dict) and isinstance(result, dict), "search receipts missing")
    preflight, prior = pinned.preflight, pinned.PRIOR
    expected_config = {
        "schema": "n83.cross-stream-pilot-partial-config/v1",
        "study": "koblitz_n83_factor_base_sweep_20261008",
        "base_object_blake3": arm["base_object_blake3"],
        "pool_object_blake3": arm["pool_object_blake3"],
        "pilot_manifest_blake3": blake3.blake3(
            preflight.PINNED_SHA256["manifest"][0].read_bytes()).hexdigest(),
        "source_blake3": blake3.blake3(
            preflight.PINNED_SHA256["worker_source"][0].read_bytes()).hexdigest(),
        "public_corpus_path": "/corpus.json",
        "public_corpus_canonical_json_blake3": prior.CORPUS_BLAKE3,
        "budget_seconds": protocol["worker_wall_seconds"],
        "conflict_budget": protocol["conflict_budget"],
        "max_models": protocol["max_models"],
        "base_summands": 4, "residual_summands": 2,
        "order_first_pair": True, "max_variables": 150000,
        "max_domain_clauses": 12000000,
        "memory_cgroup_limit_bytes": protocol["search_memory_bytes"],
        "memory_cgroup_swap_limit_bytes": protocol["search_swap_bytes"],
    }
    for key, value in expected_config.items():
        require(type(config.get(key)) is type(value) and config[key] == value,
                f"search config {key} changed")
    expected_result = {
        "schema": "n83.cross-stream-pilot-partial-result/v1",
        "study": "koblitz_n83_factor_base_sweep_20261008",
        "base_object_blake3": arm["base_object_blake3"],
        "pool_object_blake3": arm["pool_object_blake3"],
        "base_policy": "public_x_sequential", "base_seed": 2026100802,
        "pool_policy": "public_x_gray_prefix" if arm["role"] == "gray_pool" else "public_x_hash",
        "pool_seed": 2026100801,
        "public_corpus_canonical_json_blake3": prior.CORPUS_BLAKE3,
        "base_signed_orbits": 600, "pool_signed_orbits": 600,
        "shared_signed_orbits": arm["shared_signed_orbits"],
        "residual_signed_orbits": arm["residual_signed_orbits"],
        "trial": 0, "a": protocol["first_trial_a"],
        "b": protocol["first_trial_b"],
        "graph_rows": None, "matrix_rank": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }
    for key, value in expected_result.items():
        require(type(result.get(key)) is type(value) and result[key] == value,
                f"search result {key} changed")
    wall = 1000 * protocol["worker_wall_seconds"]
    require(prior.finite_nonnegative(result.get("base_import_elapsed_ms_L0"))
            and prior.finite_nonnegative(result.get("worker_wall_ms_L0"))
            and result["base_import_elapsed_ms_L0"] <= result["worker_wall_ms_L0"] <= wall,
            "worker runtime counters changed")
    peak = result.get("memory_cgroup_peak_bytes")
    require(type(peak) is int and 0 < peak <= protocol["search_memory_bytes"],
            "worker memory counter changed")
    stats = result.get("search")
    require(isinstance(stats, dict), "search counters missing")
    shape = arm["source_capacity"]
    require(shape == pinned.CAPACITY.requirements(600, 600 + arm["residual_signed_orbits"], 6, 2),
            "capacity source changed")
    require(stats.get("domain_clauses") == shape["domain_clauses"]
            and type(stats.get("peak_variables")) is int
            and 0 <= stats["peak_variables"] <= shape["total_variables"]
            and type(stats.get("peak_xors")) is int
            and 0 <= stats["peak_xors"] <= shape["xor_rows"]
            and type(stats.get("peak_clauses")) is int
            and stats["peak_clauses"] >= 0,
            "search capacity counters changed")
    for key, cap in (("conflicts", protocol["conflict_budget"]), ("models", 256),
                     ("invalid_models", 256), ("group_rejected_models", 256),
                     ("solver_calls", 8), ("patterns_attempted", 8)):
        value = stats.get(key)
        require(type(value) is int and 0 <= value <= cap, f"search {key} cap changed")
    require(type(stats.get("exhausted")) is bool and type(stats.get("refuted")) is bool,
            "search disposition counters missing")
    status = result.get("status")
    require(status in {"PASS_one_bounded_partial", "PASS_one_trial_refuted",
                       "UNKNOWN_search_cap", "UNKNOWN_preflight_cap",
                       "PRODUCER_FAILURE_invalid_model"}, "search status unclassified")
    candidate = result.get("candidate")
    require((candidate is not None) == (status == "PASS_one_bounded_partial"),
            "candidate/status mismatch")
    require((stats["invalid_models"] > 0) == (status == "PRODUCER_FAILURE_invalid_model"),
            "invalid-model disposition mismatch")
    if status == "UNKNOWN_search_cap":
        require(stats["exhausted"], "search cap lacks exhausted counter")
    elif status == "UNKNOWN_preflight_cap":
        require(stats["exhausted"] and stats["patterns_attempted"] == 0
                and stats["solver_calls"] == 0, "preflight cap has solver calls")
    elif status == "PASS_one_trial_refuted":
        require(stats["refuted"] and not stats["exhausted"], "refutation counter changed")
    if candidate is not None:
        require(isinstance(candidate, dict) and candidate.get("source_id") == 1
                and (candidate.get("a"), candidate.get("b")) ==
                (protocol["first_trial_a"], protocol["first_trial_b"]),
                "candidate source changed")
        indices = candidate.get("base_point_indices")
        points_raw = candidate.get("residual_points")
        require(isinstance(indices, list) and isinstance(points_raw, list),
                "candidate domain arrays missing")
        points = [prior.parse_point(point) for point in points_raw]
        require(len(indices) == 4
                and all(type(index) is int and 0 <= index < len(base_points) for index in indices)
                and len(points) == 2 and all(point in residual for point in points),
                "candidate outside exact domains")
        require(prior.replay_equation(candidate["a"], candidate["b"],
                                      [base_points[index] for index in indices],
                                      points, tuple(map(int, protocol["public_target"]))),
                "candidate group equation fails independent replay")
    return status
