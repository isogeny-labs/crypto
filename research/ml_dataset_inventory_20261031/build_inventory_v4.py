#!/usr/bin/env python3
"""Append the checked Koblitz47 campaign to the immutable v3 inventory."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_koblitz47_m4_f6_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    prior_path = HERE / "inventory-v3.json"
    prior = read(prior_path)
    audit = read(CAMPAIGN / "AUDIT.json")
    plan = read(CAMPAIGN / "plan.json")
    export = read(CAMPAIGN / "experiment-export.json")
    receipt = read(CAMPAIGN / "experiment-export-producer-receipt.json")
    sample = read(CAMPAIGN / "S3-READBACK-SAMPLE.json")
    require(prior["schema"] == "ml-dataset.local-campaign-inventory/v3" and
            prior["totals"]["observed_receipts"] == 152 and
            plan["protocol_sha256"] == sha(CAMPAIGN / "protocol.json"),
            "prior inventory or protocol binding")
    require(plan["field_sizes"] == [47] and plan["arities"] == [4] and
            plan["families"] == ["koblitz"] and plan["solvers"] == ["f6", "enumerate"] and
            plan["generators"] == plan["keys"] == 2 and plan["twists"] is True,
            "unexpected Koblitz47 plan")
    require(audit["schema"] == "ml-dataset.koblitz47-m4-f6-enum-independent-audit/v1" and
            audit["retained"] == audit["planned"] == 16 and
            audit["artifacts_rehashed"] == 512 and
            audit["native_status_counts"] == {"verified": 16} and
            audit["independent_ecdlp_solves"] == 0 and
            audit["protocol_file_sha256"] == sha(CAMPAIGN / "protocol.json") and
            audit["plan_file_sha256"] == sha(CAMPAIGN / "plan.json"),
            "independent audit binding")
    require(receipt["export_sha256"] == sha(CAMPAIGN / "experiment-export.json") and
            receipt["official_checker_exit_code"] == 0 and
            receipt["flow"] == export["flow"] and
            export["flow"]["planned"] == export["flow"]["attempted"] ==
            export["flow"]["completed"] == 16 and
            export["flow"]["not_completed"] == 0, "official export binding")
    require(sample["sample_indices"] == [0, 15] and
            all(x["fresh_get_hash_matched"] for x in sample["samples"]),
            "independent S3 sample")
    costs = [json.loads(line) for line in (CAMPAIGN / "cost-observations.jsonl").read_text().splitlines()]
    require(len(costs) == 16 and all(len(x["phase_observations"]) == 4 and
            not any(p["training_cost_eligible"] for p in x["phase_observations"]) for x in costs),
            "cost projection scope")
    methods, twists = {"f6": 0, "enumerate": 0}, set()
    archive_bytes = 0
    for index in range(16):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        spec = custody["native_status"]["cell_spec"]
        require(checkpoint["index"] == custody["index"] == index and
                custody["plan_file_sha256"] == sha(CAMPAIGN / "plan.json") and
                custody["protocol_file_sha256"] == sha(CAMPAIGN / "protocol.json") and
                custody["native_status"]["verified_and_exported"] and
                custody["native_status"]["method_artifact_verified"] and
                published["archive_fresh_get"] and published["pointer_fresh_get"] and
                published["archive_sha256"] == custody["archive_sha256"] and
                audit["rows"][index]["archive_sha256"] == custody["archive_sha256"] and
                costs[index]["index"] == index and
                costs[index]["archive_sha256"] == custody["archive_sha256"],
                f"Koblitz47 checkpoint {index}")
        require(spec == {"solver": plan["solvers"][index % 2],
                         "twist": bool((index // 2) % 2),
                         "key_index": (index // 4) % 2,
                         "generator_index": (index // 8) % 2}, f"Koblitz47 axis {index}")
        methods[spec["solver"]] += 1
        twists.add(spec["twist"])
        archive_bytes += custody["archive_bytes"]
    require(methods == {"f6": 8, "enumerate": 8} and twists == {False, True},
            "method or twist denominator")
    row = {"campaign": CAMPAIGN.name, "family": ["koblitz"], "field_sizes": [47],
           "arities": [4], "methods": plan["solvers"], "twists_planned": True,
           "planned_cells": 16, "observed_receipts": 16,
           "native_verified_and_exported": 16,
           "independently_solved_ecdlp_cells": 0,
           "bounded_root_certificates": 0, "retained_f6_pair_domains": 8,
           "producer_s3_readback_receipts": 16, "archive_bytes": archive_bytes,
           "audit_sha256": sha(CAMPAIGN / "AUDIT.json"),
           "cost_observations_sha256": sha(CAMPAIGN / "cost-observations.jsonl"),
           "plan_file_sha256": {"plan.json": sha(CAMPAIGN / "plan.json")},
           "protocol_file_sha256": {"protocol.json": sha(CAMPAIGN / "protocol.json")},
           "official_experiment_export_checker_receipted": True,
           "official_export_flow": export["flow"],
           "native_status_counts": audit["native_status_counts"],
           "independent_s3_readback_sample_sha256": sha(CAMPAIGN / "S3-READBACK-SAMPLE.json"),
           "whole_worker_instruction_labels": 0}
    totals = dict(prior["totals"])
    totals["archive_bytes"] += archive_bytes
    totals["observed_receipts"] += 16
    totals["retained_f6_pair_domains"] += 8
    totals["official_experiment_exports_checker_receipted"] += 1
    totals["official_export_completed_rows"] += 16
    result = {"schema": "ml-dataset.local-campaign-inventory/v4",
              "scope": "seven named local synthetic campaigns with producer S3 readback receipts",
              "prior_inventory_sha256": sha(prior_path),
              "default_plan_method_cells": prior["default_plan_method_cells"],
              "campaigns": prior["campaigns"] + [row], "totals": totals}
    target = HERE / "inventory-v4.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("inventory-v4.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()
