#!/usr/bin/env python3
"""Extend checked coverage with the Koblitz47 m=4 F6/enumeration campaign."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESEARCH = HERE.parent
INVENTORY = RESEARCH / "ml_dataset_inventory_20261031"
CAMPAIGN = RESEARCH / "ml_dataset_koblitz47_m4_f6_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    old_path = HERE / "coverage-v3.json"
    new_inventory_path = INVENTORY / "inventory-v4.json"
    old = read(old_path)
    inventory = read(new_inventory_path)
    audit = read(CAMPAIGN / "AUDIT.json")
    plan = read(CAMPAIGN / "plan.json")
    require(old["schema"] == "ml-dataset.requested-versus-observed-coverage/v3" and
            old["inventory_sha256"] == sha(INVENTORY / "inventory-v3.json") and
            old["observed"]["retained_receipts"] == 152 and
            inventory["schema"] == "ml-dataset.local-campaign-inventory/v4" and
            inventory["prior_inventory_sha256"] == sha(INVENTORY / "inventory-v3.json") and
            inventory["totals"]["observed_receipts"] == 168,
            "prior coverage or new inventory binding")
    row = inventory["campaigns"][-1]
    require(row["campaign"] == CAMPAIGN.name and
            row["audit_sha256"] == sha(CAMPAIGN / "AUDIT.json") and
            row["plan_file_sha256"]["plan.json"] == sha(CAMPAIGN / "plan.json") and
            row["retained_f6_pair_domains"] == 8 and
            audit["retained"] == 16 and audit["artifacts_rehashed"] == 512 and
            plan["field_sizes"] == [47] and plan["arities"] == [4],
            "Koblitz47 campaign binding")
    methods = {"f6": 0, "enumerate": 0}
    twists = set()
    statuses = {}
    for index in range(16):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody = checkpoint["custody"]
        native = custody["native_status"]
        spec = native["cell_spec"]
        require(checkpoint["index"] == custody["index"] == audit["rows"][index]["index"] == index and
                custody["archive_sha256"] == audit["rows"][index]["archive_sha256"] and
                custody["plan_file_sha256"] == sha(CAMPAIGN / "plan.json") and
                native["verified_and_exported"] and native["method_artifact_verified"] and
                checkpoint["published"]["archive_fresh_get"] and
                checkpoint["published"]["pointer_fresh_get"], f"Koblitz47 receipt {index}")
        require(spec == {"solver": plan["solvers"][index % 2],
                         "twist": bool((index // 2) % 2),
                         "key_index": (index // 4) % 2,
                         "generator_index": (index // 8) % 2},
                f"Koblitz47 axis {index}")
        methods[spec["solver"]] += 1
        twists.add(spec["twist"])
        status = native["native_status"]
        statuses[status] = statuses.get(status, 0) + 1
    require(methods == {"f6": 8, "enumerate": 8} and
            twists == {False, True} and statuses == {"verified": 16},
            "Koblitz47 denominators")
    result = copy.deepcopy(old)
    result["schema"] = "ml-dataset.requested-versus-observed-coverage/v4"
    result["prior_coverage_sha256"] = sha(old_path)
    result["inventory_sha256"] = sha(new_inventory_path)
    result["koblitz47_independent_archive_audit_sha256"] = sha(CAMPAIGN / "AUDIT.json")
    result["koblitz47_independent_s3_readback_sample_sha256"] = sha(CAMPAIGN / "S3-READBACK-SAMPLE.json")
    observed = result["observed"]
    observed["retained_receipts"] += 16
    observed["field_sizes"] = sorted(set(observed["field_sizes"]) | {47})
    observed["native_status_counts_where_retained"]["verified"] += 16
    for grid_row in observed["family_method_grid"]:
        if grid_row["family"] == "koblitz" and grid_row["method"] in methods:
            grid_row["observed_receipts"] += methods[grid_row["method"]]
    observed["campaigns"].append({"name": CAMPAIGN.name, "receipts": 16,
        "method_receipts": dict(sorted(methods.items())),
        "twist_not_explicit_in_checkpoint_receipts": 0})
    result["unobserved_axes"]["field_sizes"] = sorted(
        set(result["requested"]["field_sizes"]) - set(observed["field_sizes"]))
    result["unobserved_axes"]["n_47_to_61"] = (
        "n=47 Koblitz has sixteen retained cells and n=61 prime has four; n=48..60 have none")
    result["interpretation"].append(
        "The sixteen Koblitz47 m=4 receipts retain eight F6 pair-domain and eight bounded enumeration artifacts across both twists; none independently recovered a scalar. F6 here is a pair-domain method, not a Groebner F6 implementation.")
    require(sum(r["observed_receipts"] for r in observed["family_method_grid"]) == 168 and
            sum(r["receipts"] for r in observed["campaigns"]) == 168 and
            observed["independently_solved_ecdlp_cells"] == inventory["totals"]["independently_solved_ecdlp_cells"] == 93,
            "coverage totals")
    target = HERE / "coverage-v4.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("coverage-v4.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps({"retained": 168, "sizes": observed["field_sizes"],
                      "unobserved_methods": result["unobserved_axes"]["methods"]}, sort_keys=True))


if __name__ == "__main__":
    main()
