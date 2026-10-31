#!/usr/bin/env python3
"""Append the exact D18 primary base to the frozen N83 ordinal sweep.

This is a lazy design ledger, not an execution or performance result.
"""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path

import paired_frontier_v3 as v3
import paired_frontier_v4 as v4


ROOT = Path(__file__).resolve().parent
EVIDENCE = ROOT.parent / "n83_d18_subgroup_orbit_census_20261018"
OUTPUT = ROOT / "exact-d18-frontier-v5.json"
V4_SHA256 = "8e512b8dbc6577093f8e31f4afb625894cbdedd9f9f3e87dc4f36a7df52344c2"
RECEIPTS = {
    "export_manifest": (EVIDENCE / "export-v1/manifest.json", "2575cea6b16cfdaa02531c79ca4797d976eb861def3182983c3048ce75dea045"),
    "export_replay": (EVIDENCE / "export-v1/replay.json", "1f3e5cad62b9deb93cfa255e02e1ae1490faab53b625f2e01f40ea7b2aee7e6d"),
    "s3_custody": (EVIDENCE / "export-v1/s3-custody.json", "6b467e6e30d2af0f641fa503ebccf1f998fb6745b95abf8a6f5174e604d67f63"),
    "exact_import": (EVIDENCE / "import-v1/import.json", "a47f386dc8ef548830e29ffd6e0d1ce21ea228626fcbe11af66f95a94bb90164"),
    "m4_natural_result": (EVIDENCE / "natural-v1/NATURAL_RESULT.md", "1b93939a1bfc65a96f71985b9334b01f1bcbdcb32769c380eba0e82afc8ed068"),
}
BASE = {
    "curve_id": "icv1-f2m83-tm6151469093347-debefd74",
    "curve_a": 0,
    "family": "direct_subgroup_x_d18",
    "closure": "signed_frobenius",
    "parameter_unit": "orbit_columns",
    "parameter": 16627,
    "point_count": 2760082,
    "archive_blake3": "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
    "point_set_blake3": "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0",
}


def disposition(solver: dict) -> str:
    """First source/execution gate that applies; never infer a runtime winner."""
    if solver["backend"] != "sat_cdcl":
        return f"backend_unavailable:{solver['backend']}"
    if solver["symmetry"] != "signed_frobenius":
        return "symmetry_adapter_unavailable"
    if solver["split_bits"] != 0:
        return "splitting_adapter_unavailable"
    if solver["enumeration"] != "binary":
        return "gray_enumeration_adapter_unavailable"
    if solver["max_large_primes"] != 0:
        return "large_prime_graph_unavailable"
    if solver["threads"] != 1:
        return "thread_setting_unmeasured"
    if solver["linear_algebra"] != "modular_biguint":
        return "full_width_linear_algebra_required"
    if solver["summands"] == 4:
        return "bounded_m4_screening_censored_full_pipeline_unmeasured"
    if solver["summands"] == 5:
        return "m5_source_gate_available_screening_pending"
    return "summand_shape_unavailable"


def case(design: dict, ordinal: int) -> dict:
    v3.require(0 <= ordinal < design["total_tuple_count"], "v5 case outside ledger")
    parent = json.loads(v4.OUTPUT.read_text())
    if ordinal < design["v4_tuple_count_preserved"]:
        return v4.case(parent, json.loads(v4.PARENT.read_text()), ordinal)
    index = ordinal - design["v4_tuple_count_preserved"]
    solver = v3.solver_from_index(index, json.loads(v4.PARENT.read_text()))
    return {
        "schema": "n83.factor-base-exact-d18-case/v5",
        "ordinal": ordinal,
        "solver_index": index,
        "base": BASE,
        "solver": solver,
        "disposition": disposition(solver),
        "executed_complete_pipeline": False,
        "candidate_id": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }


def build() -> dict:
    v3.require(v3.sha256(v4.OUTPUT) == V4_SHA256, "v4 parent bytes changed")
    for path, expected in RECEIPTS.values():
        v3.require(v3.sha256(path) == expected, f"frozen evidence changed: {path}")
    parent = json.loads(v4.OUTPUT.read_text())
    v3.require(parent["schema"] == "n83.factor-base-design/v4-cross-stream-frontier"
               and parent["total_tuple_count"] == 26939278080
               and parent["solver_tuple_count"] == 51840,
               "v4 dimensions changed")
    solver_parent = json.loads(v4.PARENT.read_text())
    counts = Counter(disposition(v3.solver_from_index(i, solver_parent))
                     for i in range(parent["solver_tuple_count"]))
    v3.require(sum(counts.values()) == 51840, "D18 solver count changed")
    return {
        "schema": "n83.factor-base-design/v5-exact-d18",
        "study": parent["study"],
        "parent_v4_sha256": V4_SHA256,
        "evidence_sha256": {key: digest for key, (_, digest) in RECEIPTS.items()},
        "v4_tuple_count_preserved": parent["total_tuple_count"],
        "added_base": BASE,
        "solver_tuple_count": parent["solver_tuple_count"],
        "added_tuple_count": parent["solver_tuple_count"],
        "total_tuple_count": parent["total_tuple_count"] + parent["solver_tuple_count"],
        "disposition_counts": dict(sorted(counts.items())),
        "coverage": "v4 ordinal prefix preserved; one exact D18 primary base crossed with every frozen v3 solver tuple",
        "measurement_scope": "source and bounded-screening status only; no full-runtime selection",
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
    data = v3.canonical_bytes(build())
    if args.case is not None:
        v3.require(OUTPUT.read_bytes() == data, "saved v5 design differs")
        print(json.dumps(case(json.loads(OUTPUT.read_text()), args.case), sort_keys=True, indent=2))
        return
    if args.check:
        v3.require(OUTPUT.read_bytes() == data, "saved v5 design differs")
    elif OUTPUT.exists():
        v3.require(OUTPUT.read_bytes() == data, "refusing changed v5 output")
    else:
        OUTPUT.write_bytes(data)
    print(json.dumps({"status": "PASS", "sha256": hashlib.sha256(data).hexdigest()}))


if __name__ == "__main__":
    main()
