#!/usr/bin/env python3
"""Create the pre-data ordinary-binary n47 four-cell protocol draft."""
import copy
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
TEMPLATE = HERE.parent / "ml_dataset_koblitz47_m4_f6_enum_20261031" / "protocol.draft.json"
SEED = 2026103105
ROOT = "/mnt/spot-codex-state/tmp/ml-dataset-binary47-m4-f6-enum-20261031"


def rename(value):
    if isinstance(value, str):
        for old, new in [("KOBLITZ47", "BINARY47"), ("Koblitz47", "Binary47"),
                         ("Koblitz n47", "ordinary binary n47"), ("Koblitz models", "ordinary binary models"),
                         ("Koblitz", "ordinary binary"), ("koblitz47", "binary47"),
                         ("koblitz", "binary")]:
            value = value.replace(old, new)
        return value.replace("16-row", "4-row").replace("16 fixed", "4 fixed").replace("all 16", "all 4").replace("All 16", "All 4").replace("16 cells", "4 cells").replace("16 indices", "4 indices").replace("indices0..15", "indices0..3").replace("indices 0..15", "indices 0..3").replace("0..15 inclusive", "0..3 inclusive").replace("Four generator/key", "One generator/key").replace("four generator/key", "one generator/key").replace("two generators, two keys", "one generator, one key").replace("two generators and two keys", "one generator and one key").replace("g0k0, g0k1, g1k0, g1k1", "g0k0")
    if isinstance(value, list):
        return [rename(v) for v in value]
    if isinstance(value, dict):
        return {k: rename(v) for k, v in value.items()}
    return value


def main():
    p = rename(copy.deepcopy(json.loads(TEMPLATE.read_text())))
    p["protocol_id"] = "ML-DATASET-BINARY47-M4-F6-ENUM-20261031"
    p["study_dir"] = "research/ml_dataset_binary47_m4_f6_enum_20261031"
    p["authors"] = ["ML dataset ordinary-binary n47 pilot"]
    p["question"] = "Can four fixed ordinary-binary n47 m4 F6/enumeration cells on both twists retain exact construction and method outcomes with durable custody?"
    for arm in p["arms"]:
        arm["description"] = "One fixed generator/key instance on the declared ordinary-binary n47 m4 twist and method; one attempt and no timing comparison."
        arm["parameters"].update({"family": "binary", "generators": 1, "keys": 1, "seed": SEED,
                                   "shard_count": 4, "shard_indices": "native mixed-radix indices 0..3"})
    p["sample_size"]["instances"] = 1
    p["sample_size"]["power"]["note"] = "One generator/key instance per method-by-twist arm; four fixed cells. Exact accounting only, no inferential effect or power claim."
    p["sample_size"]["preflight_runtime"].update({"estimated_total_seconds": 90.0, "fixed_seconds": 10.0,
        "per_sample_seconds": 80.0, "measured_on": "Planning allowance only; four cells have a hard 360s generate-only outer bound. Exact ordinary-binary point-count construction may exhaust the 30s cell cap; finalization and custody are separately bounded."})
    p["randomization"]["instance_sampling"]["rule"] = "Fixed seed and sole g0k0 instance. F6 and enumeration share public input within each twist; both twists and all four outcomes are retained."
    p["randomization"]["seed_derivation"]["seeds"] = [SEED]
    p["variable_ledger"]["comparisons"][0]["threshold"] = "All four indices accounted; method labels require native replay and durable labels require fresh S3 readback."
    for f in p["variable_ledger"]["factors"]:
        if f["name"] == "curve-and-method":
            f["how"] = "Ordinary binary n47 m4, F6/enumeration, one generator and key, both twists; construction point-count status retained."
    p["stopping_rules"][0]["rule"] = "All four plan indices 0..3, at most one new native attempt each; retain every outcome without replacement."
    p["analysis_plan"]["outputs"] = ["4-row official export", "native status counts", "artifact and scalar labels", "archive audit and report"]
    p["invalidation_rules"].append("Point-count construction failure or timeout remains a counted cell and is never replaced by another curve seed.")
    p["inadmissible"].append("assuming a nontrivial Frobenius orbit for a generic ordinary-binary model without the recorded coefficient-field degree and physical transport")
    p["literature"] = [p["literature"][0], {
        "claim": "Characteristic-two summation-polynomial and Weil-descent scope",
        "provenance": "retrieved", "ref": "https://eprint.iacr.org/2013/596.pdf",
        "verified_by": "Shantz and Teske explicitly study E/F_(2^n) and distinguish the heuristic algebraic method from exact finite-domain observations; this pilot makes only exact software claims."}, {
        "claim": "Native ordinary-binary construction and geometric four-sum capability",
        "provenance": "internal", "ref": "src/ml_dataset/curves.rs:566-623,687-748; src/ml_dataset/solvers.rs:330-338,2440-2513",
        "verified_by": "Source inspection; n47 per-cell point-count construction remains an observed outcome."}]
    ext = p["extensions"].pop("ml_dataset_koblitz47_m4_f6_enum")
    ext["plan_axes"].update({"families": ["binary"], "generators": 1, "keys": 1, "seed": SEED})
    ext.update({"schema": "ml-dataset.binary47-m4-f6-enum-pilot/v1", "planned_cells": 4,
                "shard_count": 4, "selected_shards": "0..3 inclusive", "output_root": ROOT,
                "s3_logical_name_namespace": "restricted/ml-dataset-binary47-m4-f6-enum-20261031",
                "label_scope": "bounded geometric F6/enumeration artifacts; scalar only after independent replay; coefficient-field degree checked per twist"})
    ext["export_mapping"].update({"instance_ids": ["0"], "rows_expected": 4})
    p["extensions"]["ml_dataset_binary47_m4_f6_enum"] = ext
    p["freeze"] = {"status": "draft", "protocol_sha256": None,
                   "commit": None, "frozen_at": None, "filed_before_affected_data": False}
    out = HERE / "protocol.draft.json"
    out.write_text(json.dumps(p, sort_keys=True, indent=2) + "\n")
    print(out)


if __name__ == "__main__":
    main()
