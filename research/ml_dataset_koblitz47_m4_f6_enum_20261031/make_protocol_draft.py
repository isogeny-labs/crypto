#!/usr/bin/env python3
"""Construct the pre-data Koblitz n47 m4 F6/enumeration protocol."""

import copy
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
TEMPLATE = HERE.parent / "ml_dataset_koblitz_m4_f6_bsgs_mnt_20261031" / "protocol.draft.json"
SEED = 2026103104
ROOT = "/mnt/spot-codex-state/tmp/ml-dataset-koblitz47-m4-f6-enum-20261031"
SOLVERS = ("f6", "enumerate")


def main():
    p = copy.deepcopy(json.loads(TEMPLATE.read_text()))
    p["amendments"] = []
    p["protocol_id"] = "ML-DATASET-KOBLITZ47-M4-F6-ENUM-20261031"
    p["study_dir"] = "research/ml_dataset_koblitz47_m4_f6_enum_20261031"
    p["authors"] = ["ML dataset Koblitz47 F6 and enumeration campaign"]
    p["question"] = ("Can all 16 fixed Koblitz n47 m4 F6/enumeration cells across two "
                     "generators, two keys and both twists retain exact native outcomes and durable custody?")
    p["claims"] = [
        {"id": "fixed-population-accounting", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "All 16 fixed plan indices have at most one native attempt; censored, failed, unsupported and interrupted cells remain in the denominator."},
        {"id": "f6-four-sum-capture", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "A completed F6WidePairIndex m4 cell retains a hash-checked bounded signed pair table; any positive PDP witness requires independent group replay. The adapter is geometric four-sum, not general Groebner F6."},
        {"id": "enumeration-capture", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "A completed enumeration cell retains its counted visited transitions and bounded finite-domain outcome; a budget stop does not certify no decomposition."},
        {"id": "scalar-label", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "An ECDLP scalar label is promoted only after native independent recovery and generator-target group replay."},
        {"id": "custody", "class": "exact_computation",
         "decided_by": ["single-arm-receipt"],
         "text": "Each attempted shard retains native verify, report and strict export statuses, a restricted archive and fresh S3 archive and pointer readback receipts; failures remain counted."},
    ]
    p["controls"]["positive"]["not_applicable_reason"] = (
        "Any positive PDP witness or independently recovered ECDLP scalar requires native mathematical replay; this fixed-population pilot makes no detector-sensitivity claim.")
    source_arm = p["arms"][0]
    arms = []
    for twist in (False, True):
        for solver in SOLVERS:
            arm = copy.deepcopy(source_arm)
            arm["id"] = f"koblitz47-m4-{solver}-{'twisted' if twist else 'untwisted'}"
            arm["description"] = (f"Four fixed generator/key instances for Koblitz n47 m4 {solver} "
                                  f"on the {'twisted' if twist else 'untwisted'} curve; "
                                  "one attempt per shard and no timing comparison.")
            arm["parameters"].update({"family": "koblitz", "field_size_bits": 47,
                "arity": 4, "solver": solver, "twists": twist, "generators": 2,
                "keys": 2, "curve_seeds": 1, "seed": SEED, "shard_count": 16,
                "shard_indices": "native mixed-radix indices 0..15"})
            arms.append(arm)
    p["arms"] = arms
    p["sample_size"]["instances"] = 4
    p["sample_size"]["power"]["note"] = ("Four generator/key instance IDs per solver-by-twist arm; "
        "16 fixed native cells. Exact accounting only, no inferential effect or power claim.")
    p["sample_size"]["preflight_runtime"].update({"estimated_total_seconds": 90.0,
        "fixed_seconds": 10.0, "per_sample_seconds": 80.0,
        "measured_on": "Per-cell planning allowance under a 90s outer cap; 16 cells imply at most 1440s generate-only, not measured timing. Finalization and custody have separate caps."})
    p["randomization"]["arm_order_reason"] = ("Fixed mixed-radix order pairs F6 and "
        "enumeration on each public input; no timing-effect comparison.")
    p["randomization"]["instance_sampling"]["rule"] = (
        "Fixed seed and lexicographic generator/key indices g0k0, g0k1, g1k0, g1k1. "
        "Each method pair shares one public input within a twist; all 16 outcomes are retained.")
    p["randomization"]["seed_derivation"].update({"seeds": [SEED],
        "note": "Native plan seed fixed before data; one curve seed index0."})
    p["variable_ledger"]["comparisons"][0].update({"arms": [a["id"] for a in arms],
        "threshold": "All 16 indices accounted; method labels require native replay and durable labels require S3 fresh readback."})
    p["variable_ledger"]["factors"] = [
        {"name": "instrument", "treatment": "fixed", "how": "Pinned clean source c41bec15, binary SHA b8e0881e, per-attempt hash checks."},
        {"name": "curve-and-method", "treatment": "crossed", "how": "Koblitz n47 m4, F6/enumeration, two generators, two keys, both twists."},
        {"name": "threads", "treatment": "fixed", "how": "RAYON_NUM_THREADS=1 and one native command at a time."},
        {"name": "host-and-contention", "treatment": "measured", "how": "Shared host context retained; no timing-effect inference."},
        {"name": "timeout", "treatment": "fixed", "how": "30s inner cell and 90s outer runner cap."},
        {"name": "analyst-selection", "treatment": "fixed", "how": "All 16 frozen indices retained regardless of outcome."},
        {"name": "filesystem", "treatment": "fixed", "how": "XFS output with 8GiB free-space floor and restricted S3 custody."},
    ]
    p["stopping_rules"][0]["rule"] = "All 16 plan indices0..15, at most one new native attempt each; retain every outcome without replacement."
    p["stopping_rules"][1]["rule"] = ("Each generate has30s inner and90s outer cap; finite finalization and S3 stage caps. "
        "Stop on failed custody and resume finalization only; require at least8GiB XFS free before new attempt. Overall campaign3h ceiling.")
    p["decision_rule"]["statistic"] = ("Exact counts by method, twist, generator/key and native status; "
        "verified F6 pair domains, enumeration traces, independently replayed scalar labels and S3 custody")
    p["decision_rule"]["text"] = ("Account all 16 fixed cells. Promote only verified method and scalar labels. "
        "Retain all timeouts, unsupported, partial and construction failures. Durable requires fresh S3 readback/hash match.")
    p["metrics"]["primary"] = [p["metrics"]["primary"][0]]
    p["metrics"]["primary"].append({"id": "method-artifact-verified", "name": "Verified method artifact",
        "definition": "Native verify passes and the retained manifest contains the declared F6 pair-domain or counted enumeration transition artifact.",
        "unit": "Boolean", "kind": "exact", "aggregation": "per_row", "resolution": "1",
        "source": "native cell manifest and dataset verify receipt", "uncertainty": "none_exact"})
    p["metrics"]["primary"].append({"id": "durable-cell", "name": "Durable cell",
        "definition": "Restricted archive and custody pointer pass S3 PUT and fresh GET byte-equality/hash checks.",
        "unit": "Boolean", "kind": "exact", "aggregation": "per_row", "resolution": "1",
        "source": "content-addressed S3 archive and pointer receipts", "uncertainty": "none_exact"})
    p["analysis_plan"].update({"inputs": ["frozen plan/protocol", "native runner receipts",
        "cell manifests", "Git custody checkpoints", "S3 readback receipts"],
        "outputs": ["16-row official export", "native status counts", "artifact and scalar labels", "archive audit and report"]})
    p["invalidation_rules"] = ["Pinned binary, plan or protocol digest changes invalidate new admission.",
        "Existing attempt marker or live worker blocks another attempt for that shard.",
        "Failed native verify prevents method or scalar promotion.",
        "Missing fresh S3 GET/hash match prevents durability promotion."]
    p["inadmissible"] = ["replacing a seed or shard after failure", "rerunning an attempted shard",
        "dropping failed, timed-out or interrupted attempts", "using restricted witnesses as public ML features",
        "promoting missing S3 readback as durable", "claiming F6 is a general Groebner algorithm",
        "promoting a bounded negative as an exhaustive no-decomposition result",
        "claiming timing advantage or exact fall degree from this fixed population"]
    p["literature"] = [p["literature"][0], {"claim": "Native Koblitz n47 m4 F6/enumeration capability",
        "provenance": "internal", "ref": "src/ml_dataset/solvers.rs:330-338; src/ml_dataset/curves.rs:687-730",
        "verified_by": "source inspection and pinned native capabilities; per-cell construction still checked"}]
    p["extensions"] = {"ml_dataset_koblitz47_m4_f6_enum": {
        "schema": "ml-dataset.koblitz47-m4-f6-enum-pilot/v1", "source_commit": source_arm["instrument"]["commit"],
        "binary_sha256": source_arm["instrument"]["binary_sha256"],
        "plan_axes": {"n": [47], "m": [4], "families": ["koblitz"], "solvers": list(SOLVERS),
            "curve_seeds": 1, "generators": 2, "keys": 2, "twists": [False, True], "seed": SEED,
            "factor_base_size": 8, "degree_bound": 3, "work_limit": 20000},
        "planned_cells": 16, "shard_count": 16, "selected_shards": "0..15 inclusive",
        "output_root": ROOT, "output_filesystem": "xfs",
        "s3_content_addressed_prefix": "s3://setec-astronomy-artifacts/store/sha256",
        "s3_logical_name_namespace": "restricted/ml-dataset-koblitz47-m4-f6-enum-20261031",
        "label_scope": "bounded F6/enumeration artifacts; scalar only after independent replay",
        "no_cloud_worker_submission": True,
        "export_mapping": {"arms": "method by twist", "instance_ids": ["0", "1", "2", "3"],
            "rows_expected": 16, "twist_identity_note": "Methods share public input within twist/generator/key; opposite twists are distinct."}}}
    p["freeze"] = {"status": "draft", "protocol_sha256": None,
        "commit": None, "frozen_at": None, "filed_before_affected_data": False}
    target = HERE / "protocol.draft.json"
    data = (json.dumps(p, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("existing protocol draft differs")
    target.write_bytes(data)
    print(f"{target}: {len(arms)} arms, 16 fixed cells")


if __name__ == "__main__":
    main()
