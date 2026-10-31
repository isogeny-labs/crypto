#!/usr/bin/env python3
"""Build the fixed pre-data prime61 administrative pilot protocol from a checked template."""

import copy
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
TEMPLATE = HERE.parent / "ml_dataset_binary_methods_20261031" / "protocol.draft.json"
METHODS = ("root", "enumerate")
SEED = 2026103103
OUTPUT_ROOT = "/mnt/spot-codex-state/tmp/ml-dataset-prime61-root-enum-20261031"


def main():
    p = copy.deepcopy(json.loads(TEMPLATE.read_text()))
    p["protocol_id"] = "ML-DATASET-PRIME61-ROOT-ENUM-20261031"
    p["study_dir"] = "research/ml_dataset_prime61_root_enum_20261031"
    p["question"] = ("Can four fixed prime-field n61 m2 root/enumerate cells across both twists "
                     "retain exact native outcomes, specific modulus identity, and durable custody?")
    p["claims"] = [
        {"id": "fixed-population-accounting", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "All four fixed plan indices have at most one native attempt; censored, failed, unsupported and interrupted cells remain in the denominator."},
        {"id": "mersenne-modulus-identity", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "A constructed prime curve is labelled as the requested n61 case only after its public artifact records p=2^61-1 and native verification passes."},
        {"id": "native-method-evidence", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "Root and enumerate artifacts are retained at their actual bounded native verification scope; neither a timeout nor a partial domain is promoted to a negative PDP result."},
        {"id": "ecdlp-recovery", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "An ECDLP solve is labelled only if a scalar was independently recovered without the construction witness and replayed on the declared generator and target."},
        {"id": "custody", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "Each terminal shard retains its stage receipts and restricted archive with S3 PUT and fresh GET/hash, or its exact failed custody stage."},
        {"id": "excluded-inference", "class": "out_of_scope", "decided_by": [],
         "text": "No method speed ranking, exhaustive n61 search, general PDP negative, complete root execution cost, instruction count, exact fall degree, or broader curve-matrix conclusion is inferred."},
    ]
    source_arm = p["arms"][0]
    arms = []
    for method in METHODS:
        for twist in (False, True):
            arm = copy.deepcopy(source_arm)
            arm["id"] = f"prime61-{method}-{'twisted' if twist else 'untwisted'}"
            arm["description"] = (f"One fixed prime p=2^61-1 n61 m2 {method} cell, "
                                  f"twist={str(twist).lower()}, generator index0 and key index0; "
                                  "both methods of this twist share the same public input.")
            q = arm["parameters"]
            q.update({"family": "prime", "field_size_bits": 61, "solver": method,
                      "twists": twist, "generators": 1, "keys": 1, "seed": SEED,
                      "shard_count": 4, "shard_indices": "mixed-radix selected indices 0..3"})
            arms.append(arm)
    p["arms"] = arms
    p["sample_size"]["instances"] = 1
    p["sample_size"]["power"]["note"] = ("One public instance per twist shared by two methods; "
        "four fixed cells, exact administrative accounting only, no inferential effect or power claim.")
    p["sample_size"]["preflight_runtime"].update({"estimated_total_seconds": 90.0,
        "fixed_seconds": 10.0, "per_sample_seconds": 80.0,
        "measured_on": "Planning allowance: four cells each under a 90s outer cap; not a measured runtime estimate."})
    p["randomization"]["arm_order_reason"] = ("Deterministic mixed-radix order pairs root and "
        "enumerate on each public twist input; no timing-effect comparison is made.")
    p["randomization"]["instance_sampling"]["rule"] = ("One fixed generator/key draw on each "
        "twist. Root and enumerate share the public curve, generator and target within a twist; "
        "opposite twists are distinct curves. All four outcomes are retained.")
    p["randomization"]["seed_derivation"].update({"seeds": [SEED],
        "note": "Native plan seed fixed before data; curve seed index0."})
    p["variable_ledger"]["comparisons"][0].update({"arms": [a["id"] for a in arms],
        "threshold": "All four indices accounted; specific modulus and method claims require native replay, and durability requires fresh S3 archive readback."})
    p["variable_ledger"]["factors"] = [
        {"name": "instrument", "treatment": "fixed", "how": "Pinned clean source c41bec15, binary SHA b8e0881e, preflight and per-attempt hash checks."},
        {"name": "curve-and-method", "treatment": "crossed", "how": "Prime n61 m2, one curve seed, root/enumerate and both twists; one shared generator/key draw per twist."},
        {"name": "threads", "treatment": "fixed", "how": "RAYON_NUM_THREADS=1 and one generate command at a time."},
        {"name": "host-and-contention", "treatment": "measured", "how": "Shared host conditions retained as context; no timing-effect inference."},
        {"name": "timeout", "treatment": "fixed", "how": "30s inner worker and 90s outer generate cap per shard."},
        {"name": "analyst-selection", "treatment": "fixed", "how": "All four frozen indices retained regardless of outcome."},
        {"name": "output-filesystem", "treatment": "fixed", "how": "One predeclared local XFS root, 8GiB free-space floor, restricted S3 custody per cell."},
    ]
    p["stopping_rules"][0]["rule"] = "All four plan indices0..3, one native attempt each; retain every outcome and no replacement."
    p["stopping_rules"][1]["rule"] = ("Each generate has30s inner and90s outer cap; finite finalization and S3 stage caps. "
        "Stop on failed custody and resume only finalization; require at least8GiB XFS free before a new attempt. Overall campaign2h ceiling.")
    p["decision_rule"]["statistic"] = ("Exact counts by method, twist, modulus identity and native terminal status; "
        "verified artifact, independent scalar recovery and S3 readback-complete counts")
    p["decision_rule"]["text"] = ("Account all four fixed cells. Promote only exact verified method and modulus claims. "
        "Retain all timeout, unsupported, partial and failed outcomes. Durable requires archive PUT and fresh GET/hash match.")
    p["invalidation_rules"][0] = "Pinned binary, plan or protocol digest changes invalidate new admission."
    p["inadmissible"][-1] = "claiming complete root execution cost or an exhaustive n61 negative from bounded traces"
    p["controls"]["positive"]["not_applicable_reason"] = ("No detector sensitivity or comparative "
        "claim; this is fixed-cell administrative coverage and native replay. A completed root is not an ECDLP control.")
    p["literature"][1] = {"claim": "Prime n61 modulus selection and bounded root capability",
        "provenance": "internal", "ref": "src/ml_dataset/curves.rs:630-641; crypto dataset capabilities",
        "verified_by": "source inspection and pinned native capabilities; native modulus and certificate still checked per cell"}
    p["literature"][2] = {"claim": "Dataset construction, leakage boundaries and S3 custody contract",
        "provenance": "internal", "ref": "docs/ml-dataset/DESIGN.md; docs/ml-dataset/OPERATIONS.md",
        "verified_by": "retained repository contract and prior local campaign receipts"}
    p["extensions"] = {"ml_dataset_prime61_root_enum": {
        "schema": "ml-dataset.prime61-root-enum-pilot/v1", "source_commit": source_arm["instrument"]["commit"],
        "binary_sha256": source_arm["instrument"]["binary_sha256"],
        "plan_axes": {"n": [61], "m": [2], "families": ["prime"], "solvers": list(METHODS),
            "curve_seeds": 1, "generators": 1, "keys": 1, "twists": [False, True],
            "seed": SEED, "factor_base_size": 8, "degree_bound": 3, "work_limit": 20000},
        "expected_prime_modulus": "2305843009213693951", "planned_cells": 4,
        "shard_count": 4, "selected_shards": "0..3 inclusive",
        "output_root": OUTPUT_ROOT, "output_filesystem": "xfs",
        "s3_content_addressed_prefix": "s3://setec-astronomy-artifacts/store/sha256",
        "s3_logical_name_namespace": "restricted/ml-dataset-prime61-root-enum-20261031",
        "label_scope": "bounded root/enumerate native evidence; scalar only on independent replay",
        "no_cloud_worker_submission": True,
        "export_mapping": {"arms": "solver by twist, four arms", "instance_ids": ["g0k0"],
            "rows_expected": 4, "twist_identity_note": "Methods share input within twist; twists are distinct curves."}}}
    p["freeze"] = {"status": "draft", "protocol_sha256": None,
        "commit": None, "frozen_at": None, "filed_before_affected_data": False}
    target = HERE / "protocol.draft.json"
    encoded = (json.dumps(p, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != encoded:
        raise ValueError("protocol draft differs; do not overwrite without review")
    target.write_bytes(encoded)
    print(f"{target}: {len(arms)} arms")


if __name__ == "__main__":
    main()
