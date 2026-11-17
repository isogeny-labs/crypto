#!/usr/bin/env python3
"""Extend immutable coverage v6 with checked ordinary-binary n23 arity cells."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESEARCH = HERE.parent
INVENTORY = RESEARCH / "ml_dataset_inventory_20261031"
CAMPAIGN = RESEARCH / "ml_dataset_binary23_m3_15_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    old_path = HERE / "coverage-v6.json"
    prior_inventory_path = INVENTORY / "inventory-v6.json"
    new_inventory_path = INVENTORY / "inventory-v7.json"
    old, inventory = read(old_path), read(new_inventory_path)
    audit_path = CAMPAIGN / "AUDIT.json"
    sample_path = CAMPAIGN / "S3-READBACK-SAMPLE.json"
    pointer_path = CAMPAIGN / ".artifacts.json"
    plan_path = CAMPAIGN / "plan.json"
    audit, plan = read(audit_path), read(plan_path)
    require(old["schema"] == "ml-dataset.requested-versus-observed-coverage/v6" and
            old["inventory_sha256"] == sha(prior_inventory_path) and
            old["observed"]["retained_receipts"] == 198 and
            inventory["schema"] == "ml-dataset.local-campaign-inventory/v7" and
            inventory["prior_inventory_sha256"] == sha(prior_inventory_path) and
            inventory["totals"]["observed_receipts"] == 224 and
            inventory["totals"]["independently_solved_ecdlp_cells"] == 93,
            "prior snapshots and new inventory binding")
    row = inventory["campaigns"][-1]
    require(row["campaign"] == CAMPAIGN.name and
            row["audit_sha256"] == sha(audit_path) and
            row["plan_file_sha256"]["plan.json"] == sha(plan_path) and
            row["independent_s3_readback_sample_sha256"] == sha(sample_path) and
            row["cost_artifact_pointer_sha256"] == sha(pointer_path) and
            row["observed_receipts"] == 26 and row["archive_bytes"] == 7_001_623 and
            row["official_export_flow"]["completed"] == 26 and
            row["official_export_flow"]["not_completed"] == 0 and
            row["native_status_counts"] == {"verified": 6, "budget_exhausted": 20} and
            row["whole_worker_instruction_labels"] == 0 and
            audit["retained"] == 26 and audit["artifacts_rehashed"] == 832 and
            plan["families"] == ["binary"] and plan["field_sizes"] == [23] and
            plan["arities"] == list(range(3, 16)), "campaign row binding")
    for index in range(26):
        receipt = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        native = receipt["custody"]["native_status"]
        require(receipt["index"] == audit["rows"][index]["index"] == index and
                receipt["custody"]["archive_sha256"] == audit["rows"][index]["archive_sha256"] and
                receipt["custody"]["plan_file_sha256"] == sha(plan_path) and
                receipt["published"]["archive_fresh_get"] and
                receipt["published"]["pointer_fresh_get"] and
                native["verified_and_exported"] and
                native["cell_spec"] == {"m": 3 + index // 2, "solver": "enumerate",
                                        "twist": bool(index % 2), "generator_index": 0,
                                        "key_index": 0}, f"campaign receipt {index}")
    result = copy.deepcopy(old)
    result["schema"] = "ml-dataset.requested-versus-observed-coverage/v7"
    result["prior_coverage_sha256"] = sha(old_path)
    result["inventory_sha256"] = sha(new_inventory_path)
    result["binary23_arity_independent_archive_audit_sha256"] = sha(audit_path)
    result["binary23_arity_independent_s3_readback_sample_sha256"] = sha(sample_path)
    result["binary23_arity_cost_artifact_pointer_sha256"] = sha(pointer_path)
    observed = result["observed"]
    observed["retained_receipts"] += 26
    observed["native_status_counts_where_retained"]["verified"] += 6
    observed["native_status_counts_where_retained"]["budget_exhausted"] += 20
    for grid_row in observed["family_method_grid"]:
        if grid_row["family"] == "binary" and grid_row["method"] == "enumerate":
            grid_row["observed_receipts"] += 26
    observed["campaigns"].append({"name": CAMPAIGN.name, "receipts": 26,
        "method_receipts": {"enumerate": 26},
        "twist_not_explicit_in_checkpoint_receipts": 0})
    result["interpretation"].append(
        "The 26 ordinary-binary n23 enumeration receipts span m=3..15 on both twists with one fixed public core per twist. Six m=3..5 native searches finished as verified method outcomes; twenty m=6..15 searches stopped at the fixed work budget. All 26 cells completed verification and strict export, including the budget-limited outcomes. No new cell independently recovered a scalar, and no whole-worker instruction-to-solve label was verified.")
    require(sum(r["observed_receipts"] for r in observed["family_method_grid"]) == 224 and
            sum(r["receipts"] for r in observed["campaigns"]) == 224 and
            observed["independently_solved_ecdlp_cells"] == 93 and
            observed["field_sizes"] == [23, 47, 61] and
            result["unobserved_axes"]["methods"] == ["pkm"] and
            result["unobserved_axes"]["arities"] == [] and
            len(inventory["campaigns"]) == 10,
            "coverage totals and preserved requested denominator")
    target = HERE / "coverage-v7.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("coverage-v7.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps({"retained": 224, "arities": observed["arities"],
                      "unobserved_methods": result["unobserved_axes"]["methods"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
