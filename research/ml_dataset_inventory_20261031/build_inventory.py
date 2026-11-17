#!/usr/bin/env python3
"""Recompute a bounded custody inventory from immutable local campaign receipts.

This is an inventory of observed local campaigns, not a generator or a claim
that the planned default matrix has been executed. It never contacts S3; the
readback fields are producer receipts and are labelled as such in the output.
"""

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CAMPAIGNS = (
    "ml_dataset_local_campaign_20261031",
    "ml_dataset_koblitz_root_bsgs_20261031",
    "ml_dataset_koblitz_m4_f6_bsgs_mnt_20261031",
    "ml_dataset_binary_root_bsgs_20261031",
)
DEFAULT_PLANNED_CELLS = 24 * 14 * 3 * 10 * 16 * 8 * 64 * 2


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(test, message):
    if not test:
        raise ValueError(message)


def inspect(name):
    base = ROOT / "research" / name
    audit_path = base / "AUDIT-01.json"
    audit = json.loads(audit_path.read_text())
    plan_paths = sorted(base.glob("plan*.json"))
    protocol_paths = sorted(base.glob("protocol*.json"))
    plans = {sha256(path): path.name for path in plan_paths}
    protocols = {sha256(path): path.name for path in protocol_paths}
    plan = json.loads((base / "plan.json").read_text())
    planned = audit["planned_cells"]
    checkpoints = sorted((base / "checkpoints").glob("shard-*.json"))
    require(len(checkpoints) == planned, f"{name}: expected {planned} receipts")
    indices = set()
    solved = root_certificates = f6_domains = archive_bytes = 0
    for path in checkpoints:
        receipt = json.loads(path.read_text())
        index = receipt["index"]
        require(path.name == f"shard-{index:02d}.json", f"{name}: filename/index mismatch")
        require(index not in indices, f"{name}: duplicate index {index}")
        indices.add(index)
        custody = receipt["custody"]
        published = receipt["published"]
        native = custody["native_status"]
        require(custody["index"] == index, f"{name}: custody/index mismatch")
        require(custody["plan_file_sha256"] in plans, f"{name}: unknown plan binding")
        require(custody["protocol_file_sha256"] in protocols, f"{name}: unknown protocol binding")
        require(native["verified_and_exported"], f"{name}: native verification/export failed")
        require(native["artifact_hashes_verified"], f"{name}: native hashes failed")
        require(published["archive_fresh_get"] and published["pointer_fresh_get"],
                f"{name}: producer readback missing")
        require(published["archive_sha256"] == custody["archive_sha256"],
                f"{name}: published archive hash differs")
        archive_bytes += custody["archive_bytes"]
        solved += native["independently_solved_ecdlp_cells"]
        root_certificates += int(native.get("root_certificate_promoted", False))
        f6_domains += int(native.get("f6_pair_domain_promoted", False))
    require(indices == set(range(planned)), f"{name}: missing or out-of-range indices")
    require(not audit["broader_matrix_claim"], f"{name}: audit overclaims broader matrix")
    require(archive_bytes == audit.get("archive_total_bytes", audit.get("total_archive_bytes")),
            f"{name}: archive byte total mismatch")
    export_path = base / "experiment-export.json"
    producer_path = base / "experiment-export-producer-receipt.json"
    export_checker_receipted = False
    export_flow = None
    if export_path.exists():
        require(producer_path.exists(), f"{name}: export lacks producer receipt")
        producer = json.loads(producer_path.read_text())
        require(producer["export_sha256"] == sha256(export_path),
                f"{name}: export digest mismatch")
        checker = producer["official_checker"]
        require(checker["exit_code"] == 0 and "(0 finding(s))" in checker["stdout"],
                f"{name}: official checker receipt failed")
        export_flow = producer["flow"]
        require(export_flow["planned"] == planned and
                export_flow["attempted"] == planned and
                export_flow["completed"] + export_flow["not_completed"] == planned,
                f"{name}: official export flow mismatch")
        export_checker_receipted = True
    return {
        "campaign": name,
        "family": plan["families"],
        "field_sizes": plan["field_sizes"],
        "arities": plan["arities"],
        "methods": plan["solvers"],
        "twists_planned": plan["twists"],
        "planned_cells": planned,
        "observed_receipts": len(checkpoints),
        "native_verified_and_exported": len(checkpoints),
        "independently_solved_ecdlp_cells": solved,
        "bounded_root_certificates": root_certificates,
        "retained_f6_pair_domains": f6_domains,
        "producer_s3_readback_receipts": len(checkpoints),
        "archive_bytes": archive_bytes,
        "audit_sha256": sha256(audit_path),
        "plan_file_sha256": {path.name: sha256(path) for path in plan_paths},
        "protocol_file_sha256": {path.name: sha256(path) for path in protocol_paths},
        "official_experiment_export_checker_receipted": export_checker_receipted,
        "official_export_flow": export_flow,
    }


def main():
    rows = [inspect(name) for name in CAMPAIGNS]
    output = {
        "schema": "ml-dataset.local-campaign-inventory/v1",
        "scope": "four named local synthetic campaigns; producer readback receipts, not a live S3 audit",
        "default_plan_method_cells": DEFAULT_PLANNED_CELLS,
        "campaigns": rows,
        "totals": {
            "observed_receipts": sum(row["observed_receipts"] for row in rows),
            "independently_solved_ecdlp_cells": sum(row["independently_solved_ecdlp_cells"] for row in rows),
            "bounded_root_certificates": sum(row["bounded_root_certificates"] for row in rows),
            "retained_f6_pair_domains": sum(row["retained_f6_pair_domains"] for row in rows),
            "archive_bytes": sum(row["archive_bytes"] for row in rows),
            "official_experiment_exports_checker_receipted": sum(
                row["official_experiment_export_checker_receipted"] for row in rows),
            "official_export_completed_rows": sum(
                row["official_export_flow"]["completed"]
                for row in rows if row["official_export_flow"] is not None),
            "official_export_censored_rows": sum(
                row["official_export_flow"]["not_completed"]
                for row in rows if row["official_export_flow"] is not None),
        },
    }
    destination = Path(__file__).with_name("inventory.json")
    data = json.dumps(output, indent=2, sort_keys=True).encode() + b"\n"
    if destination.exists() and destination.read_bytes() != data:
        raise ValueError("inventory.json differs; review before replacing immutable inventory")
    destination.write_bytes(data)
    print(f"{destination}: {output['totals']}")


if __name__ == "__main__":
    main()
