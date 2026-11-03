#!/usr/bin/env python3
"""Append the prime61 pilot to the immutable five-campaign inventory."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_prime61_root_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    previous_path = HERE / "inventory-v2.json"
    previous = read(previous_path)
    require(previous["schema"] == "ml-dataset.local-campaign-inventory/v2" and
            previous["totals"]["observed_receipts"] == 148, "unexpected prior inventory")
    plan = read(CAMPAIGN / "plan.json")
    audit = read(CAMPAIGN / "AUDIT.json")
    export = read(CAMPAIGN / "experiment-export.json")
    producer = read(CAMPAIGN / "experiment-export-producer-receipt.json")
    sample = read(CAMPAIGN / "S3-READBACK-SAMPLE.json")
    require(plan["families"] == ["prime"] and plan["field_sizes"] == [61] and
            plan["arities"] == [2] and plan["solvers"] == ["root", "enumerate"] and
            plan["twists"] is True, "unexpected plan axes")
    require(plan["protocol_sha256"] == sha(CAMPAIGN / "protocol.json") ==
            audit["protocol_file_sha256"], "protocol binding")
    require(sha(CAMPAIGN / "plan.json") == audit["plan_file_sha256"], "plan binding")
    require(sha(CAMPAIGN / "experiment-export.json") ==
            producer["export_sha256"] == audit["export_sha256"], "export binding")
    require(producer["official_checker_exit_code"] == 0 and
            export["flow"] == producer["flow"] and
            export["flow"]["planned"] == export["flow"]["attempted"] == 4 and
            export["flow"]["completed"] == export["flow"]["not_completed"] == 2,
            "official export flow")
    require(audit["prime_modulus"] == "2305843009213693951" and
            audit["retained"] == 4 and audit["independent_ecdlp_solves"] == 0 and
            audit["native_status_counts"] == {"timeout": 2, "verified": 2},
            "audit result")
    require(sample["sample_indices"] == [0, 3] and all(
            row["fresh_get_hash_matched"] for row in sample["samples"]), "independent S3 sample")
    archive_bytes = 0
    method_counts = {"root": 0, "enumerate": 0}
    for index in range(4):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody = checkpoint["custody"]
        native = custody["native_status"]
        spec = native["cell_spec"]
        require(checkpoint["index"] == custody["index"] == index and
                custody["plan_file_sha256"] == sha(CAMPAIGN / "plan.json") and
                custody["protocol_file_sha256"] == sha(CAMPAIGN / "protocol.json") and
                native["verified_and_exported"] and native["mersenne_modulus_verified"] and
                checkpoint["published"]["archive_fresh_get"] and
                checkpoint["published"]["pointer_fresh_get"] and
                checkpoint["published"]["archive_sha256"] == custody["archive_sha256"],
                f"checkpoint {index} binding")
        require(spec["solver"] == ("root" if index % 2 == 0 else "enumerate") and
                spec["twist"] is (index >= 2), f"checkpoint {index} axis")
        require(audit["rows"][index]["archive_sha256"] == custody["archive_sha256"] and
                audit["rows"][index]["s3_uri"] == custody["s3_uri"],
                f"checkpoint {index} audit binding")
        method_counts[spec["solver"]] += 1
        archive_bytes += custody["archive_bytes"]
    row = {"campaign": CAMPAIGN.name, "family": ["prime"], "field_sizes": [61],
           "arities": [2], "methods": plan["solvers"], "twists_planned": True,
           "planned_cells": 4, "observed_receipts": 4, "native_verified_and_exported": 4,
           "independently_solved_ecdlp_cells": 0, "bounded_root_certificates": 0,
           "retained_f6_pair_domains": 0, "producer_s3_readback_receipts": 4,
           "archive_bytes": archive_bytes, "audit_sha256": sha(CAMPAIGN / "AUDIT.json"),
           "plan_file_sha256": {"plan.json": sha(CAMPAIGN / "plan.json")},
           "protocol_file_sha256": {"protocol.json": sha(CAMPAIGN / "protocol.json")},
           "official_experiment_export_checker_receipted": True,
           "official_export_flow": export["flow"],
           "native_status_counts": audit["native_status_counts"],
           "specific_prime_modulus": audit["prime_modulus"],
           "independent_s3_readback_sample_sha256": sha(CAMPAIGN / "S3-READBACK-SAMPLE.json")}
    totals = dict(previous["totals"])
    totals["archive_bytes"] += archive_bytes
    totals["observed_receipts"] += 4
    totals["official_experiment_exports_checker_receipted"] += 1
    totals["official_export_completed_rows"] += 2
    totals["official_export_censored_rows"] += 2
    result = {"schema": "ml-dataset.local-campaign-inventory/v3",
              "scope": "six named local synthetic campaigns with producer S3 readback receipts",
              "prior_inventory_sha256": sha(previous_path),
              "default_plan_method_cells": previous["default_plan_method_cells"],
              "campaigns": previous["campaigns"] + [row], "totals": totals}
    target = HERE / "inventory-v3.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("inventory-v3.json differs; review before replacing")
    target.write_bytes(data)
    print(json.dumps({"totals": totals, "method_counts": method_counts}, sort_keys=True))


if __name__ == "__main__":
    main()
