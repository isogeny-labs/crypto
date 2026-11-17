#!/usr/bin/env python3
"""Build the pre-data natural ordinary-binary n23 arity protocol."""
import copy
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
TEMPLATE = HERE.parent / "ml_dataset_binary47_m4_f6_enum_20261031" / "protocol.draft.json"
ARITIES = list(range(3, 16))
SEED = 2026103107
ROOT = "/mnt/spot-codex-state/tmp/ml-dataset-binary23-m3-15-enum-20261031"


def main():
    p = copy.deepcopy(json.loads(TEMPLATE.read_text()))
    p["protocol_id"] = "ML-DATASET-BINARY23-M3-15-ENUM-20261031"
    p["study_dir"] = "research/ml_dataset_binary23_m3_15_enum_20261031"
    p["authors"] = ["ML dataset ordinary-binary n23 arity campaign"]
    p["question"] = "Can all 26 fixed natural ordinary-binary n23 m3..15 enumeration cells across both twists retain bounded native outcomes and durable custody?"
    p["claims"] = [c for c in p["claims"] if c["id"] != "f6-four-sum-capture"]
    for c in p["claims"]:
        if c["id"] == "fixed-population-accounting":
            c["text"] = "All 26 fixed plan indices have at most one native attempt; censored, failed, unsupported and interrupted cells remain in the denominator. This is the ordinary-binary 26-cell block of a separately planned 78-cell three-family n23 arity portfolio."
        elif c["id"] == "enumeration-capture":
            c["text"] = "A completed enumeration cell retains its counted visited transitions under the m-specific bounded finite-domain outcome; a budget stop or timeout does not certify no decomposition."
    p["arms"] = [a for a in p["arms"] if a["parameters"]["solver"] == "enumerate"]
    for arm in p["arms"]:
        twist = bool(arm["parameters"]["twists"])
        arm["id"] = f"binary23-m3-15-enumerate-{'twisted' if twist else 'untwisted'}"
        arm["description"] = f"Thirteen fixed arities m3..15 on the {'twisted' if twist else 'untwisted'} ordinary-binary n23 curve; one generator/key, one attempt per cell, no timing comparison."
        arm["parameters"].update({"family": "binary", "field_size_bits": 23, "arity": ARITIES,
            "solver": "enumerate", "twists": twist, "generators": 1, "keys": 1,
            "curve_seeds": 1, "seed": SEED, "factor_base_size": 3,
            "work_limit": 4096, "seconds_per_case": 60,
            "outer_generate_timeout_seconds": 150, "shard_count": 26,
            "shard_indices": "m3..15 each with untwisted then twisted"})
    p["sample_size"]["instances"] = 13
    p["sample_size"]["power"]["note"] = "Thirteen m values per twist arm; 26 fixed native cells. Exact accounting only, no inferential effect or power claim."
    p["sample_size"]["preflight_runtime"].update({"estimated_total_seconds": 150.0,
        "fixed_seconds": 10.0, "per_sample_seconds": 140.0,
        "measured_on": "Planning allowance only: 26 cells have a hard 3900s generate-only outer bound at 150s each. Native finalization and S3 custody have separate bounded stages, not performance timings."})
    p["randomization"]["arm_order_reason"] = "Native mixed-radix order pairs both twists at each m. No timing-effect comparison is made."
    p["randomization"]["instance_sampling"]["rule"] = "Fixed seed and one generator/key index. All m3..15 on both twists, exactly 26 cells; no replacement after timeout or failure. Arity labels are not independent public keys."
    p["randomization"]["seed_derivation"].update({"seeds": [SEED], "note": "Native plan seed fixed before data; one curve seed index0."})
    p["variable_ledger"]["comparisons"][0].update({"arms": [a["id"] for a in p["arms"]],
        "threshold": "All 26 indices accounted; each promoted enumeration artifact requires native verification and S3 fresh readback."})
    p["variable_ledger"]["factors"] = [
        {"name": "instrument", "treatment": "fixed", "how": "Pinned clean source c41bec15 and binary SHA b8e0881e, checked per attempt."},
        {"name": "curve-arity-twist", "treatment": "crossed", "how": "Ordinary binary n23, m3..15, both twists, one curve seed and one generator/key; retain point-count construction failures."},
        {"name": "threads", "treatment": "fixed", "how": "RAYON_NUM_THREADS=1 and one native command at a time."},
        {"name": "host-and-contention", "treatment": "measured", "how": "Shared host context retained; no timing-effect inference."},
        {"name": "timeout", "treatment": "fixed", "how": "60s inner cell and 150s outer runner cap; enumeration has 4096-transition work limit."},
        {"name": "analyst-selection", "treatment": "fixed", "how": "All 26 indices retained regardless of outcome."},
        {"name": "filesystem", "treatment": "fixed", "how": "XFS output with 8GiB free-space floor and restricted S3 custody."},
    ]
    p["stopping_rules"][0]["rule"] = "All 26 plan indices0..25, at most one native attempt each; retain every outcome without replacement."
    p["stopping_rules"][1]["rule"] = "Each generate has 60s inner and 150s outer cap; finite finalization and S3 stage caps. Stop on failed custody and resume finalization only; require 8GiB XFS free before new attempt. Overall campaign 8h ceiling."
    p["execution"]["timeout_seconds"] = 150
    p["decision_rule"]["statistic"] = "Exact counts by arity, twist and native status; verified enumeration transitions, independently replayed scalar labels and S3 custody"
    p["decision_rule"]["text"] = "Account all 26 fixed cells. Promote only verified method and scalar labels. Retain timeout, unsupported, partial and construction failures. Durable requires fresh S3 readback/hash match."
    p["decision_rule"]["verdict_vocabulary"] = ["verified_enumeration_and_durable", "verified_ecdlp_and_durable", "verified_custody_pending", "failed", "timed_out", "interrupted", "invalid_receipt", "unattempted"]
    p["measurement"]["record"] = ["enumeration transition verification status" if item == "F6 geometric finite-domain verification status" else item for item in p["measurement"]["record"]]
    for metric in p["metrics"]["primary"]:
        if metric["id"] == "method-artifact-verified":
            metric["definition"] = "Native verify passes and the retained manifest contains the counted enumeration transition artifact."
    p["analysis_plan"].update({"inputs": ["frozen plan/protocol", "native runner receipts", "cell manifests", "Git custody checkpoints", "S3 readback receipts"],
        "outputs": ["26-row official export", "native status counts by arity and twist", "artifact and scalar labels", "archive audit and report"]})
    p["invalidation_rules"] = ["Pinned binary, plan or protocol digest changes invalidate new admission.",
        "Existing attempt marker or live worker blocks another attempt for that shard.",
        "Failed native verify prevents method or scalar promotion.",
        "Missing fresh S3 GET/hash match prevents durability promotion.",
        "Point-count construction failure or timeout remains a counted cell and is never replaced by another curve seed."]
    p["inadmissible"] = ["replacing a seed or shard after failure", "rerunning an attempted shard",
        "dropping failed, timed-out or interrupted attempts", "using restricted witnesses as public ML features",
        "promoting missing S3 readback as durable", "promoting a bounded negative as an exhaustive no-decomposition result",
        "claiming timing advantage or exact fall degree from this fixed population",
        "treating shared public key across m as independent key draws",
        "assuming a nontrivial Frobenius orbit for ordinary-binary without recorded coefficient-field degree and physical transport",
        "combining a future planted-positive control with the natural 26-cell denominator"]
    p["literature"] = [p["literature"][0], p["literature"][1],
        {"claim": "Native ordinary-binary and enumeration capability", "provenance": "internal",
         "ref": "src/ml_dataset/curves.rs:687-885; src/ml_dataset/solvers.rs:746-838",
         "verified_by": "Pinned source and native capabilities; per-cell construction remains observed."}]
    ext = p["extensions"].pop("ml_dataset_binary47_m4_f6_enum")
    ext.update({"schema": "ml-dataset.binary23-m3-15-enum/v1", "plan_axes": {
        "n": [23], "m": ARITIES, "families": ["binary"], "solvers": ["enumerate"],
        "curve_seeds": 1, "generators": 1, "keys": 1, "twists": [False, True],
        "seed": SEED, "factor_base_size": 3, "degree_bound": 3, "work_limit": 4096},
        "planned_cells": 26, "shard_count": 26, "selected_shards": "0..25 inclusive",
        "output_root": ROOT, "s3_logical_name_namespace": "restricted/ml-dataset-binary23-m3-15-enum-20261031",
        "label_scope": "bounded enumeration transition artifacts; scalar only after independent replay; ordinary-binary coefficient-field degree checked per twist",
        "portfolio_scope": "One fixed 26-cell ordinary-binary block of a three-family 78-cell n23 m3..15 enumeration portfolio.",
        "export_mapping": {"arms": "twist", "instance_ids": [str(i) for i in range(13)],
            "instance_id_to_m": {str(i): m for i, m in enumerate(ARITIES)}, "rows_expected": 26,
            "twist_identity_note": "m varies on a seeded curve and generator/key; opposite twists are distinct. Shared keys across m are not independent."},
        "separate_positive_control_dependency": "Native CLI currently samples unconditioned scalar targets; planted-PDP construction requires versioned target-generation/schema mode and restricted planting labels. Such controls must have a separate protocol and denominator."})
    p["extensions"]["ml_dataset_binary23_m3_15_enum"] = ext
    p["freeze"] = {"status": "draft", "protocol_sha256": None, "commit": None,
                   "frozen_at": None, "filed_before_affected_data": False}
    (HERE / "protocol.draft.json").write_text(json.dumps(p, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
