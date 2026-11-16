#!/usr/bin/env python3
"""Append every same-curve ordered D18/legacy-base pair to the N83 ledger.

The suffix enumerates recipes lazily. Source dispositions are not runs.
"""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path

import exact_d18_frontier_v5 as v5
import paired_frontier_v3 as v3
import paired_frontier_v4 as v4


ROOT = Path(__file__).resolve().parent
OUTPUT = ROOT / "d18-mixed-frontier-v6.json"
V5_SHA256 = "0cc44a910d51d5078b0f0d9a6e352ca2ae4e8b0cfd5f97a2adedcf9f2dcfb3a5"
BASE_SHA256 = "7526b6218dfe0b074368901f3869ddf4a978cb0bacffe8654056056700f8a972"
BASE_COUNT = 660
PAIR_COUNT = 2 * BASE_COUNT + 1
RESIDUALS = (0, 1, 2)
EXACT_U = ("public_x_hash", 2026100801, "orbit_columns", 16627, "signed_frobenius")


def primary_indices(base: dict) -> list[int]:
    indices = [i for i, row in enumerate(base["base_specs"]) if row["a"] == 0]
    v3.require(len(indices) == BASE_COUNT, "primary legacy base count changed")
    return indices


def source_class(row: dict) -> str:
    if row["parameter_unit"] != "orbit_columns":
        return "binary_dimension_mixed_domain_unimplemented"
    if row["closure"] != "signed_frobenius":
        return "closure_incompatible"
    key = (row["family"], row["seed"], row["parameter_unit"],
           row["parameter"], row["closure"])
    if key == EXACT_U:
        return "exact_u_d18_pair_import_available_mixed_domain_unimplemented"
    return "exact_legacy_archive_pair_unavailable"


def disposition(row: dict, solver: dict, residual: int) -> str:
    if residual == 0:
        return "no_residual_pair_axis_redundant"
    if solver["max_large_primes"] != residual:
        return "large_prime_count_differs_from_residual_shape"
    if solver["summands"] not in (5, 6) or solver["summands"] - residual < 3:
        return "mixed_orbit_sum_shape_unavailable"
    if solver["backend"] != "sat_cdcl":
        return f"paired_backend_adapter_unavailable:{solver['backend']}"
    if solver["symmetry"] != v3.EXACT_PAIR_SYMMETRY:
        return "exact_first_pair_symmetry_adapter_unavailable"
    if solver["split_bits"] != 0:
        return "paired_split_adapter_unavailable"
    if solver["enumeration"] != "binary":
        return "paired_gray_enumeration_adapter_unavailable"
    if solver["threads"] != 1:
        return "paired_thread_setting_unmeasured"
    if solver["linear_algebra"] != "modular_biguint":
        return "paired_full_width_linear_algebra_required"
    return source_class(row)


def pair(base: dict, pair_index: int) -> dict:
    v3.require(0 <= pair_index < PAIR_COUNT, "D18 mixed pair index outside suffix")
    if pair_index == 2 * BASE_COUNT:
        return {"curve_a": 0, "left": dict(v5.BASE), "right": dict(v5.BASE),
                "orientation": "D18_self_redundant"}
    legacy_slot, orientation = divmod(pair_index, 2)
    index = primary_indices(base)[legacy_slot]
    legacy = dict(base["base_specs"][index], parent_base_index=index)
    d18 = dict(v5.BASE)
    return {"curve_a": 0, "left": d18 if orientation == 0 else legacy,
            "right": legacy if orientation == 0 else d18,
            "orientation": "D18_first" if orientation == 0 else "D18_second"}


def case(design: dict, ordinal: int) -> dict:
    v3.require(0 <= ordinal < design["total_tuple_count"], "v6 case outside ledger")
    if ordinal < design["v5_tuple_count_preserved"]:
        return v5.case(json.loads(v5.OUTPUT.read_text()), ordinal)
    inside = ordinal - design["v5_tuple_count_preserved"]
    pair_index, inside = divmod(inside, len(RESIDUALS) * design["solver_tuple_count"])
    residual_index, solver_index = divmod(inside, design["solver_tuple_count"])
    base = json.loads(v3.PARENT.read_text())
    spec = pair(base, pair_index)
    legacy = (None if spec["orientation"] == "D18_self_redundant" else
              spec["right"] if spec["orientation"] == "D18_first" else spec["left"])
    solver = v3.solver_from_index(solver_index, json.loads(v4.PARENT.read_text()))
    residual = RESIDUALS[residual_index]
    return {
        "schema": "n83.factor-base-d18-mixed-case/v6",
        "ordinal": ordinal, "pair_index": pair_index,
        "residual_index": residual_index, "solver_index": solver_index,
        "pair": spec, "residual_summands": residual,
        "base_summands": solver["summands"] - residual,
        "solver": solver,
        "conditional_unordered_point_multisets": None,
        "exact_overlap_measured": False,
        "disposition": ("D18_self_pair_redundant" if legacy is None
                        else disposition(legacy, solver, residual)),
        "executed_complete_pipeline": False,
        "candidate_id": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }


def build() -> dict:
    v3.require(v3.sha256(v5.OUTPUT) == V5_SHA256, "v5 parent bytes changed")
    v3.require(v3.sha256(v3.PARENT) == BASE_SHA256, "legacy base catalog changed")
    parent = json.loads(v5.OUTPUT.read_text())
    base = json.loads(v3.PARENT.read_text())
    solver_parent = json.loads(v4.PARENT.read_text())
    v3.require(parent["schema"] == "n83.factor-base-design/v5-exact-d18"
               and parent["total_tuple_count"] == 26939329920
               and parent["solver_tuple_count"] == 51840,
               "v5 parent dimensions changed")
    indices = primary_indices(base)
    classes = Counter(source_class(base["base_specs"][i]) for i in indices)
    v3.require(classes["exact_u_d18_pair_import_available_mixed_domain_unimplemented"] == 1,
               "exact uniform partner changed")
    counts: Counter = Counter()
    representatives = {source_class(base["base_specs"][i]): base["base_specs"][i]
                       for i in indices}
    for category, multiplicity in classes.items():
        legacy = representatives[category]
        for solver_index in range(parent["solver_tuple_count"]):
            solver = v3.solver_from_index(solver_index, solver_parent)
            for residual in RESIDUALS:
                counts[disposition(legacy, solver, residual)] += 2 * multiplicity
    counts["D18_self_pair_redundant"] += len(RESIDUALS) * parent["solver_tuple_count"]
    added = PAIR_COUNT * len(RESIDUALS) * parent["solver_tuple_count"]
    v3.require(sum(counts.values()) == added, "mixed D18 disposition count changed")
    return {
        "schema": "n83.factor-base-design/v6-d18-mixed",
        "study": parent["study"],
        "parent_v5_sha256": V5_SHA256,
        "legacy_base_catalog_sha256": BASE_SHA256,
        "v5_tuple_count_preserved": parent["total_tuple_count"],
        "primary_legacy_base_count": len(indices),
        "ordered_mixed_pair_count": PAIR_COUNT,
        "source_class_legacy_base_counts": dict(sorted(classes.items())),
        "residual_summands_axis": list(RESIDUALS),
        "solver_tuple_count": parent["solver_tuple_count"],
        "added_tuple_count": added,
        "total_tuple_count": parent["total_tuple_count"] + added,
        "disposition_counts": dict(sorted(counts.items())),
        "coverage": "v5 ordinal prefix preserved; both orders of exact D18 with each of 660 primary legacy bases plus the redundant D18 self-pair, residual 0/1/2, and every frozen solver tuple",
        "measurement_scope": "source availability only; D18 mixed-orbit domain and conditional overlap unmeasured",
        "candidate_id": None,
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--write", action="store_true")
    group.add_argument("--check", action="store_true")
    group.add_argument("--case", type=int)
    args = parser.parse_args()
    generated = v3.canonical_bytes(build())
    if args.case is not None:
        v3.require(OUTPUT.read_bytes() == generated, "saved v6 design differs")
        print(json.dumps(case(json.loads(OUTPUT.read_text()), args.case),
                         sort_keys=True, indent=2))
        return
    if args.check:
        v3.require(OUTPUT.read_bytes() == generated, "saved v6 design differs")
    elif OUTPUT.exists():
        v3.require(OUTPUT.read_bytes() == generated, "refusing changed v6 output")
    else:
        OUTPUT.write_bytes(generated)
    print(json.dumps({"status": "PASS", "sha256": hashlib.sha256(generated).hexdigest()}))


if __name__ == "__main__":
    main()
