#!/usr/bin/env python3
"""Append the completed paired-method campaign to the immutable v1 inventory."""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_binary_methods_20261031"


def read(path):
    return json.loads(path.read_text())


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    previous = read(HERE / "inventory.json")
    assert previous["schema"] == "ml-dataset.local-campaign-inventory/v1"
    assert previous["totals"]["observed_receipts"] == 100
    audit = read(CAMPAIGN / "AUDIT.json")
    plan = read(CAMPAIGN / "plan.json")
    export = read(CAMPAIGN / "experiment-export.json")
    producer = read(CAMPAIGN / "experiment-export-producer-receipt.json")
    checkpoints = sorted((CAMPAIGN / "checkpoints").glob("shard-*.json"))
    assert len(checkpoints) == 48
    assert audit["retained_rows"] == audit["unique_run_ids"] == 48
    assert audit["experiment_export_sha256"] == producer["export_sha256"] == sha256(CAMPAIGN / "experiment-export.json")
    assert audit["protocol_sha256"] == sha256(CAMPAIGN / "protocol.json") == plan["protocol_sha256"]
    assert producer["official_checker_exit_code"] == 0
    assert export["flow"] == producer["flow"]
    assert export["flow"]["attempted"] == export["flow"]["planned"] == 48
    archive_bytes = 0
    solves = 0
    for index, path in enumerate(checkpoints):
        receipt = read(path)
        assert path.name == f"shard-{index:02d}.json"
        assert receipt["index"] == receipt["custody"]["index"] == index
        assert receipt["custody"]["protocol_file_sha256"] == plan["protocol_sha256"]
        assert receipt["published"]["archive_fresh_get"] and receipt["published"]["pointer_fresh_get"]
        assert receipt["published"]["archive_sha256"] == receipt["custody"]["archive_sha256"]
        native = receipt["custody"]["native_status"]
        assert native["verified_and_exported"]
        archive_bytes += receipt["custody"]["archive_bytes"]
        solves += native["independently_solved_ecdlp_cells"]
    assert solves == audit["independent_ecdlp_solves"] == 13
    row = {
        "campaign": CAMPAIGN.name,
        "family": plan["families"],
        "field_sizes": plan["field_sizes"],
        "arities": plan["arities"],
        "methods": plan["solvers"],
        "twists_planned": plan["twists"],
        "planned_cells": 48,
        "observed_receipts": 48,
        "native_verified_and_exported": 48,
        "independently_solved_ecdlp_cells": solves,
        "bounded_root_certificates": 0,
        "retained_f6_pair_domains": 0,
        "producer_s3_readback_receipts": 48,
        "archive_bytes": archive_bytes,
        "audit_sha256": sha256(CAMPAIGN / "AUDIT.json"),
        "plan_file_sha256": {"plan.json": sha256(CAMPAIGN / "plan.json")},
        "protocol_file_sha256": {"protocol.json": sha256(CAMPAIGN / "protocol.json")},
        "official_experiment_export_checker_receipted": True,
        "official_export_flow": export["flow"],
        "native_status_counts": audit["native_status_counts"],
    }
    totals = dict(previous["totals"])
    totals["archive_bytes"] += archive_bytes
    totals["observed_receipts"] += 48
    totals["independently_solved_ecdlp_cells"] += solves
    totals["official_experiment_exports_checker_receipted"] += 1
    totals["official_export_completed_rows"] += export["flow"]["completed"]
    totals["official_export_censored_rows"] += export["flow"]["not_completed"]
    result = {
        "schema": "ml-dataset.local-campaign-inventory/v2",
        "scope": "five named local synthetic campaigns; producer readback receipts plus one independent final-cell S3 GET/hash",
        "prior_inventory_sha256": sha256(HERE / "inventory.json"),
        "default_plan_method_cells": previous["default_plan_method_cells"],
        "campaigns": previous["campaigns"] + [row],
        "totals": totals,
    }
    destination = HERE / "inventory-v2.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if destination.exists() and destination.read_bytes() != data:
        raise ValueError("inventory-v2.json differs; review before replacing")
    destination.write_bytes(data)
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()
