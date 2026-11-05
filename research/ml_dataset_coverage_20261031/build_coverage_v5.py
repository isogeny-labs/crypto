#!/usr/bin/env python3
"""Extend immutable coverage v4 with the checked ordinary-binary n47 pilot."""

import copy
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
RESEARCH = HERE.parent
INVENTORY = RESEARCH / "ml_dataset_inventory_20261031"
CAMPAIGN = RESEARCH / "ml_dataset_binary47_m4_f6_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    old_path = HERE / "coverage-v4.json"
    prior_inventory_path = INVENTORY / "inventory-v4.json"
    new_inventory_path = INVENTORY / "inventory-v5.json"
    old = read(old_path)
    inventory = read(new_inventory_path)
    audit_path = CAMPAIGN / "AUDIT.json"
    plan_path = CAMPAIGN / "plan.json"
    costs_path = CAMPAIGN / "cost-observations.jsonl"
    sample_path = CAMPAIGN / "S3-READBACK-SAMPLE.json"
    audit, plan = read(audit_path), read(plan_path)
    require(old["schema"] == "ml-dataset.requested-versus-observed-coverage/v4" and
            old["inventory_sha256"] == sha(prior_inventory_path) and
            old["observed"]["retained_receipts"] == 168 and
            inventory["schema"] == "ml-dataset.local-campaign-inventory/v5" and
            inventory["prior_inventory_sha256"] == sha(prior_inventory_path) and
            inventory["totals"]["observed_receipts"] == 172,
            "prior coverage or new inventory binding")
    row = inventory["campaigns"][-1]
    require(row["campaign"] == CAMPAIGN.name and
            row["audit_sha256"] == sha(audit_path) and
            row["plan_file_sha256"]["plan.json"] == sha(plan_path) and
            row["cost_observations_sha256"] == sha(costs_path) and
            row["independent_s3_readback_sample_sha256"] == sha(sample_path) and
            row["observed_receipts"] == 4 and row["retained_f6_pair_domains"] == 2 and
            row["official_export_flow"]["completed"] == 3 and
            row["official_export_flow"]["not_completed"] == 1 and
            audit["retained"] == 4 and audit["artifacts_rehashed"] == 128 and
            plan["families"] == ["binary"] and plan["field_sizes"] == [47] and
            plan["arities"] == [4], "binary47 campaign binding")
    methods = {"f6": 0, "enumerate": 0}
    twists, statuses = set(), {"verified": 0, "timeout": 0}
    for index in range(4):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody = checkpoint["custody"]
        native = custody["native_status"]
        spec = native["cell_spec"]
        require(checkpoint["index"] == custody["index"] == audit["rows"][index]["index"] == index and
                custody["archive_sha256"] == audit["rows"][index]["archive_sha256"] and
                custody["plan_file_sha256"] == sha(plan_path) and
                native["verified_and_exported"] and native["method_artifact_verified"] and
                checkpoint["published"]["archive_fresh_get"] and
                checkpoint["published"]["pointer_fresh_get"],
                f"binary47 receipt {index}")
        require(spec == {"solver": plan["solvers"][index % 2],
                         "twist": bool((index // 2) % 2),
                         "generator_index": 0, "key_index": 0},
                f"binary47 axis {index}")
        methods[spec["solver"]] += 1
        twists.add(spec["twist"])
        statuses[native["native_status"]] += 1
    require(methods == {"f6": 2, "enumerate": 2} and twists == {False, True} and
            statuses == audit["native_status_counts"] == {"verified": 3, "timeout": 1},
            "binary47 denominators")
    result = copy.deepcopy(old)
    result["schema"] = "ml-dataset.requested-versus-observed-coverage/v5"
    result["prior_coverage_sha256"] = sha(old_path)
    result["inventory_sha256"] = sha(new_inventory_path)
    result["binary47_independent_archive_audit_sha256"] = sha(audit_path)
    result["binary47_independent_s3_readback_sample_sha256"] = sha(sample_path)
    observed = result["observed"]
    observed["retained_receipts"] += 4
    require(observed["explicit_twists_by_family"]["binary"] == [False, True],
            "prior binary twist coverage")
    observed["native_status_counts_where_retained"]["verified"] += 3
    observed["native_status_counts_where_retained"]["timeout"] += 1
    for grid_row in observed["family_method_grid"]:
        if grid_row["family"] == "binary" and grid_row["method"] in methods:
            grid_row["observed_receipts"] += methods[grid_row["method"]]
    observed["campaigns"].append({"name": CAMPAIGN.name, "receipts": 4,
        "method_receipts": dict(sorted(methods.items())),
        "twist_not_explicit_in_checkpoint_receipts": 0})
    result["unobserved_axes"]["n_47_to_61"] = (
        "n=47 has sixteen Koblitz and four ordinary-binary retained cells; "
        "n=61 prime has four; n=48..60 have none")
    result["interpretation"].append(
        "The four ordinary-binary n47 m=4 receipts retain two F6 pair domains and two bounded enumeration traces across both twists; the twisted enumeration timeout is censored. None independently recovered a scalar, and scoped PMU observations are not whole-worker instruction labels.")
    require(sum(r["observed_receipts"] for r in observed["family_method_grid"]) == 172 and
            sum(r["receipts"] for r in observed["campaigns"]) == 172 and
            observed["independently_solved_ecdlp_cells"] ==
            inventory["totals"]["independently_solved_ecdlp_cells"] == 93 and
            observed["field_sizes"] == [23, 47, 61] and
            result["unobserved_axes"]["methods"] == ["pkm"],
            "coverage totals")
    target = HERE / "coverage-v5.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("coverage-v5.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps({"retained": 172, "sizes": observed["field_sizes"],
                      "unobserved_methods": result["unobserved_axes"]["methods"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
