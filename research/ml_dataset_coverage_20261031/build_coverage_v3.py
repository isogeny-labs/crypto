#!/usr/bin/env python3
"""Extend the checked coverage ledger with the retained prime61 pilot."""

import copy
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESEARCH = HERE.parent
INVENTORY_DIR = RESEARCH / "ml_dataset_inventory_20261031"
CAMPAIGN = RESEARCH / "ml_dataset_prime61_root_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    old_path = HERE / "coverage-v2.json"
    old = read(old_path)
    old_inventory = read(INVENTORY_DIR / "inventory-v2.json")
    new_inventory_path = INVENTORY_DIR / "inventory-v3.json"
    inventory = read(new_inventory_path)
    audit = read(CAMPAIGN / "AUDIT.json")
    require(old["schema"] == "ml-dataset.requested-versus-observed-coverage/v2" and
            old["inventory_sha256"] == sha(INVENTORY_DIR / "inventory-v2.json") and
            old_inventory["totals"]["observed_receipts"] == 148, "prior coverage binding")
    require(inventory["schema"] == "ml-dataset.local-campaign-inventory/v3" and
            inventory["prior_inventory_sha256"] == sha(INVENTORY_DIR / "inventory-v2.json") and
            inventory["totals"]["observed_receipts"] == 152 and
            len(inventory["campaigns"]) == len(old["observed"]["campaigns"]) + 1,
            "new inventory binding")
    row = inventory["campaigns"][-1]
    require(row["campaign"] == CAMPAIGN.name and row["audit_sha256"] == sha(CAMPAIGN / "AUDIT.json") and
            row["plan_file_sha256"]["plan.json"] == sha(CAMPAIGN / "plan.json") and
            row["specific_prime_modulus"] == audit["prime_modulus"] == "2305843009213693951",
            "prime61 campaign binding")
    method_counts = {"root": 0, "enumerate": 0}
    twists = set()
    statuses = {}
    for index in range(4):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody = checkpoint["custody"]
        native = custody["native_status"]
        spec = native["cell_spec"]
        audit_row = audit["rows"][index]
        require(checkpoint["index"] == custody["index"] == audit_row["index"] == index and
                custody["archive_sha256"] == audit_row["archive_sha256"] and
                custody["s3_uri"] == audit_row["s3_uri"] and
                custody["plan_file_sha256"] == sha(CAMPAIGN / "plan.json") and
                native["verified_and_exported"] and native["mersenne_modulus_verified"] and
                checkpoint["published"]["archive_fresh_get"] and
                checkpoint["published"]["pointer_fresh_get"], f"prime61 receipt {index}")
        require(spec["solver"] == audit_row["method"] and
                spec["twist"] == audit_row["twist"] and
                native["native_status"] == audit_row["native_status"],
                f"prime61 axes {index}")
        method_counts[spec["solver"]] += 1
        twists.add(spec["twist"])
        status = native["native_status"]
        statuses[status] = statuses.get(status, 0) + 1
    require(method_counts == {"root": 2, "enumerate": 2} and twists == {False, True} and
            statuses == {"timeout": 2, "verified": 2}, "prime61 axis counts")
    output = copy.deepcopy(old)
    output["schema"] = "ml-dataset.requested-versus-observed-coverage/v3"
    output["prior_coverage_sha256"] = sha(old_path)
    output["inventory_sha256"] = sha(new_inventory_path)
    observed = output["observed"]
    observed["retained_receipts"] = 152
    observed["field_sizes"] = sorted(set(observed["field_sizes"]) | {61})
    observed["methods"] = sorted(set(observed["methods"]) | set(method_counts))
    observed["explicit_twists_by_family"]["prime"] = [False, True]
    for status, count in statuses.items():
        observed["native_status_counts_where_retained"][status] = (
            observed["native_status_counts_where_retained"].get(status, 0) + count)
    for grid_row in observed["family_method_grid"]:
        if grid_row["family"] == "prime" and grid_row["method"] in method_counts:
            grid_row["observed_receipts"] += method_counts[grid_row["method"]]
    observed["campaigns"].append({"name": CAMPAIGN.name, "receipts": 4,
        "method_receipts": dict(sorted(method_counts.items())),
        "twist_not_explicit_in_checkpoint_receipts": 0,
        "specific_prime_modulus": audit["prime_modulus"]})
    output["unobserved_axes"]["field_sizes"] = sorted(
        set(output["requested"]["field_sizes"]) - set(observed["field_sizes"]))
    output["unobserved_axes"]["methods"] = sorted(
        set(output["requested"]["methods"]) - set(observed["methods"]))
    output["unobserved_axes"]["specific_prime_modulus_2^61-1"] = "four retained cells"
    output["unobserved_axes"]["n_47_to_61"] = "n=61 prime has four retained cells; n=47..60 have none"
    output["prime61_independent_archive_audit_sha256"] = sha(CAMPAIGN / "AUDIT.json")
    output["prime61_independent_s3_readback_sample_sha256"] = sha(CAMPAIGN / "S3-READBACK-SAMPLE.json")
    output["interpretation"].append(
        "The four prime61 receipts add two bounded root timeouts and two verified enumeration artifacts across both twists; none recovered a scalar.")
    require(sum(r["observed_receipts"] for r in observed["family_method_grid"]) == 152 and
            sum(r["receipts"] for r in observed["campaigns"]) == 152 and
            observed["independently_solved_ecdlp_cells"] == inventory["totals"]["independently_solved_ecdlp_cells"] == 93,
            "coverage totals")
    target = HERE / "coverage-v3.json"
    data = (json.dumps(output, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("coverage-v3.json differs; review before replacing")
    target.write_bytes(data)
    print(json.dumps({"retained": 152, "sizes": observed["field_sizes"],
                      "unobserved_methods": output["unobserved_axes"]["methods"]}, sort_keys=True))


if __name__ == "__main__":
    main()
