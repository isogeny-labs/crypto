#!/usr/bin/env python3
"""Append the checked ordinary-binary n47 pilot to immutable inventory v4."""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_binary47_m4_f6_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    prior_path = HERE / "inventory-v4.json"
    prior = read(prior_path)
    plan_path = CAMPAIGN / "plan.json"
    protocol_path = CAMPAIGN / "protocol.json"
    audit_path = CAMPAIGN / "AUDIT.json"
    export_path = CAMPAIGN / "experiment-export.json"
    costs_path = CAMPAIGN / "cost-observations.jsonl"
    sample_path = CAMPAIGN / "S3-READBACK-SAMPLE.json"
    plan, audit = read(plan_path), read(audit_path)
    export = read(export_path)
    producer = read(CAMPAIGN / "experiment-export-producer-receipt.json")
    sample = read(sample_path)
    costs = [json.loads(line) for line in costs_path.read_text().splitlines()]
    require(prior["schema"] == "ml-dataset.local-campaign-inventory/v4" and
            prior["totals"]["observed_receipts"] == 168 and
            prior["campaigns"][-1]["campaign"] == "ml_dataset_koblitz47_m4_f6_enum_20261031",
            "prior inventory binding")
    require(plan["schema"] == "crypto.ml-dataset.plan/v1" and
            plan["families"] == ["binary"] and plan["field_sizes"] == [47] and
            plan["arities"] == [4] and plan["solvers"] == ["f6", "enumerate"] and
            plan["curve_seeds"] == plan["generators"] == plan["keys"] == 1 and
            plan["twists"] is True and plan["protocol_sha256"] == sha(protocol_path),
            "binary47 plan or protocol binding")
    require(audit["schema"] == "ml-dataset.binary47-m4-f6-enum-independent-audit/v1" and
            audit["retained"] == audit["planned"] == 4 and
            audit["artifacts_rehashed"] == 128 and
            audit["native_status_counts"] == {"verified": 3, "timeout": 1} and
            audit["twist_counts"] == {"untwisted": 2, "twisted": 2} and
            audit["independent_ecdlp_solves"] == 0 and
            audit["plan_file_sha256"] == sha(plan_path) and
            audit["protocol_file_sha256"] == sha(protocol_path) and
            audit["export_sha256"] == sha(export_path), "independent audit binding")
    flow = export["flow"]
    require(producer["export_sha256"] == sha(export_path) and
            producer["official_checker_exit_code"] == 0 and
            "(0 finding(s))" in producer["official_checker_stdout"] and
            producer["flow"] == flow and
            flow["planned"] == flow["attempted"] == 4 and
            flow["completed"] == 3 and flow["not_completed"] == 1 and
            flow["excluded"] == [] and flow["interrupted"] == 0,
            "official export binding")
    require(sample["schema"] == "ml-dataset.binary47-independent-s3-sample/v1" and
            len(sample["rows"]) == 2 and
            [row["index"] for row in sample["rows"]] == [0, 3] and
            all(row["fresh_get_sha256_match"] for row in sample["rows"]),
            "independent S3 sample")
    require(len(costs) == 4 and all(len(row["phase_observations"]) == 4 and
            not any(phase["training_cost_eligible"] or
                    phase["whole_worker_coverage_verified"] for phase in row["phase_observations"])
            for row in costs), "scoped cost observations")
    methods = {"f6": 0, "enumerate": 0}
    statuses = {"verified": 0, "timeout": 0}
    archive_bytes = 0
    f6_domains = 0
    for index in range(4):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        native = custody["native_status"]
        spec = native["cell_spec"]
        expected = {"solver": plan["solvers"][index % 2],
                    "twist": bool((index // 2) % 2),
                    "generator_index": 0, "key_index": 0}
        require(checkpoint["index"] == custody["index"] ==
                audit["rows"][index]["index"] == costs[index]["index"] == index and
                spec == expected and
                audit["rows"][index]["method"] == costs[index]["method"] == spec["solver"] and
                audit["rows"][index]["twist"] == costs[index]["twist"] == spec["twist"] and
                custody["plan_file_sha256"] == sha(plan_path) and
                custody["protocol_file_sha256"] == sha(protocol_path) and
                custody["archive_sha256"] == published["archive_sha256"] ==
                audit["rows"][index]["archive_sha256"] == costs[index]["archive_sha256"] and
                published["archive_fresh_get"] and published["pointer_fresh_get"] and
                native["verified_and_exported"] and native["artifact_hashes_verified"] and
                native["method_artifact_verified"] and
                native["independently_solved_ecdlp_cells"] == 0 and
                native["ecdlp_solve_promoted"] is False,
                f"binary47 checkpoint {index}")
        role = ("independently-reconstructed-native-pair-domain" if spec["solver"] == "f6"
                else "all-visited-pdp-transitions")
        require(role in native["method_artifact_roles"] and
                native["native_status"] == ("timeout" if index == 3 else "verified"),
                f"binary47 method or censoring {index}")
        methods[spec["solver"]] += 1
        statuses[native["native_status"]] += 1
        f6_domains += int(spec["solver"] == "f6")
        archive_bytes += custody["archive_bytes"]
    for sample_row in sample["rows"]:
        index = sample_row["index"]
        require(sample_row["archive_sha256"] == audit["rows"][index]["archive_sha256"] and
                sample_row["s3_uri"] == audit["rows"][index]["s3_uri"],
                f"binary47 independent readback {index}")
    require(methods == {"f6": 2, "enumerate": 2} and
            statuses == audit["native_status_counts"] and f6_domains == 2 and
            archive_bytes == 1_015_585, "binary47 denominators")
    row = {"campaign": CAMPAIGN.name, "family": ["binary"], "field_sizes": [47],
           "arities": [4], "methods": plan["solvers"], "twists_planned": True,
           "planned_cells": 4, "observed_receipts": 4,
           "native_verified_and_exported": 4,
           "independently_solved_ecdlp_cells": 0,
           "bounded_root_certificates": 0, "retained_f6_pair_domains": 2,
           "producer_s3_readback_receipts": 4, "archive_bytes": archive_bytes,
           "audit_sha256": sha(audit_path),
           "cost_observations_sha256": sha(costs_path),
           "plan_file_sha256": {"plan.json": sha(plan_path)},
           "protocol_file_sha256": {"protocol.json": sha(protocol_path)},
           "official_experiment_export_checker_receipted": True,
           "official_export_flow": flow, "native_status_counts": statuses,
           "independent_s3_readback_sample_sha256": sha(sample_path),
           "whole_worker_instruction_labels": 0}
    totals = dict(prior["totals"])
    for key, addition in (("archive_bytes", archive_bytes), ("observed_receipts", 4),
                          ("retained_f6_pair_domains", 2),
                          ("official_experiment_exports_checker_receipted", 1),
                          ("official_export_completed_rows", 3),
                          ("official_export_censored_rows", 1)):
        totals[key] += addition
    result = {"schema": "ml-dataset.local-campaign-inventory/v5",
              "scope": "eight named local synthetic campaigns with producer S3 readback receipts",
              "prior_inventory_sha256": sha(prior_path),
              "default_plan_method_cells": prior["default_plan_method_cells"],
              "campaigns": prior["campaigns"] + [row], "totals": totals}
    target = HERE / "inventory-v5.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("inventory-v5.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()
