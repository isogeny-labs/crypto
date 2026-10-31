#!/usr/bin/env python3
"""Append the reviewed Koblitz n23 m3..15 enumeration campaign to v5."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_koblitz23_m3_15_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    prior_path = HERE / "inventory-v5.json"
    prior = read(prior_path)
    plan_path = CAMPAIGN / "plan.json"
    protocol_path = CAMPAIGN / "protocol.json"
    audit_path = CAMPAIGN / "AUDIT.json"
    export_path = CAMPAIGN / "experiment-export.json"
    costs_path = CAMPAIGN / "cost-observations.jsonl"
    sample_path = CAMPAIGN / "S3-READBACK-SAMPLE.json"
    custody18_path = CAMPAIGN / "S3-INTERRUPTED-CUSTODY-18.json"
    plan, audit, export = read(plan_path), read(audit_path), read(export_path)
    producer, sample, custody18 = (read(CAMPAIGN / "experiment-export-producer-receipt.json"),
                                   read(sample_path), read(custody18_path))
    costs = [json.loads(line) for line in costs_path.read_text().splitlines()]
    require(prior["schema"] == "ml-dataset.local-campaign-inventory/v5" and
            prior["totals"]["observed_receipts"] == 172 and
            prior["campaigns"][-1]["campaign"] == "ml_dataset_binary47_m4_f6_enum_20261031",
            "prior inventory binding")
    require(plan["schema"] == "crypto.ml-dataset.plan/v1" and
            plan["families"] == ["koblitz"] and plan["field_sizes"] == [23] and
            plan["arities"] == list(range(3, 16)) and
            plan["solvers"] == ["enumerate"] and
            plan["curve_seeds"] == plan["generators"] == plan["keys"] == 1 and
            plan["twists"] is True and plan["factor_base_size"] == 8 and
            plan["protocol_sha256"] == sha(protocol_path), "frozen campaign axes")
    require(audit["schema"] == "ml-dataset.koblitz23-m3-15-enum-independent-audit/v1" and
            audit["planned"] == audit["retained"] == len(audit["rows"]) == 26 and
            audit["artifacts_rehashed"] == 685 and
            audit["native_status_counts"] == {"verified": 4, "timeout": 22} and
            audit["twist_counts"] == {"untwisted": 13, "twisted": 13} and
            audit["independent_ecdlp_solves"] == 0 and
            audit["archive_bytes_total"] == 8_849_802 and
            audit["plan_file_sha256"] == sha(plan_path) and
            audit["protocol_file_sha256"] == sha(protocol_path) and
            audit["export_sha256"] == sha(export_path), "independent audit binding")
    flow = export["flow"]
    require(producer["export_sha256"] == sha(export_path) and
            producer["official_checker_exit_code"] == 0 and
            "(0 finding(s))" in producer["official_checker_stdout"] and
            producer["flow"] == flow and
            flow["planned"] == flow["attempted"] == 26 and
            flow["completed"] == 4 and flow["not_completed"] == 22 and
            flow["excluded"] == [] and flow["interrupted"] == flow["resumed"] == 0,
            "official export binding")
    require(len(costs) == 26 and
            sum(len(row["phase_observations"]) for row in costs) == 82 and
            all(not phase["training_cost_eligible"] and
                    not phase["whole_worker_coverage_verified"] and
                    phase["whole_worker_retired_instructions"] is None
                    for row in costs for phase in row["phase_observations"]),
            "cost label scope")
    require(sample["schema"] == "ml-dataset.koblitz23-separate-s3-readback/v1" and
            [row["index"] for row in sample["rows"]] == [0, 25] and
            all(row["fresh_get_hash_and_size_match"] for row in sample["rows"]),
            "independent S3 sample")
    require(custody18["index"] == 18 and custody18["new_native_attempts"] == 0 and
            custody18["new_s3_puts"] == 0 and
            {row["role"] for row in custody18["objects"]} == {"archive", "pointer"} and
            all(row["fresh_get_byte_identical"] for row in custody18["objects"]),
            "interrupted custody reconciliation")
    archive_bytes = 0
    statuses = {"verified": 0, "timeout": 0}
    public_cores = {False: set(), True: set()}
    for index in range(26):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        native = custody["native_status"]
        spec = {"m": 3 + index // 2, "solver": "enumerate", "twist": bool(index % 2),
                "generator_index": 0, "key_index": 0}
        row = audit["rows"][index]
        require(checkpoint["index"] == custody["index"] == row["index"] ==
                costs[index]["index"] == index and
                native["cell_spec"] == spec and row["m"] == spec["m"] and
                row["method"] == costs[index]["method"] == "enumerate" and
                row["twist"] == costs[index]["twist"] == spec["twist"] and
                custody["plan_file_sha256"] == sha(plan_path) and
                custody["protocol_file_sha256"] == sha(protocol_path) and
                custody["archive_sha256"] == published["archive_sha256"] ==
                row["archive_sha256"] == costs[index]["archive_sha256"] and
                custody["archive_bytes"] == row["archive_bytes"] and
                custody["s3_uri"] == row["s3_uri"] and
                custody["fresh_get_byte_comparison"] and
                published["archive_fresh_get"] and published["pointer_fresh_get"] and
                native["verified_and_exported"] and native["artifact_hashes_verified"] and
                native["independently_solved_ecdlp_cells"] == 0 and
                native["ecdlp_solve_promoted"] is False and
                native["native_status"] == row["native_status"] ==
                ("verified" if index < 4 else "timeout"),
                f"campaign checkpoint {index}")
        if index < 4:
            require(native["method_artifact_verified"] and
                    "all-visited-pdp-transitions" in native["method_artifact_roles"],
                    f"complete trace {index}")
        else:
            require(not native["method_artifact_verified"], f"censored trace {index}")
        public_cores[spec["twist"]].add(row["public_core_sha256"])
        statuses[native["native_status"]] += 1
        archive_bytes += custody["archive_bytes"]
    require(statuses == audit["native_status_counts"] and
            archive_bytes == audit["archive_bytes_total"] and
            all(len(cores) == 1 for cores in public_cores.values()) and
            public_cores[False] != public_cores[True], "campaign denominators or public pairing")
    for sample_row in sample["rows"]:
        row = audit["rows"][sample_row["index"]]
        require(sample_row["archive_sha256"] == row["archive_sha256"] and
                sample_row["archive_bytes"] == row["archive_bytes"] and
                sample_row["s3_uri"] == row["s3_uri"], "sample identity")
    require(next(x for x in custody18["objects"] if x["role"] == "archive")["sha256"] ==
            audit["rows"][18]["archive_sha256"], "interrupted archive identity")
    campaign_row = {"campaign": CAMPAIGN.name, "family": ["koblitz"],
                    "field_sizes": [23], "arities": list(range(3, 16)),
                    "methods": ["enumerate"], "twists_planned": True,
                    "planned_cells": 26, "observed_receipts": 26,
                    "native_verified_and_exported": 26,
                    "independently_solved_ecdlp_cells": 0,
                    "bounded_root_certificates": 0, "retained_f6_pair_domains": 0,
                    "producer_s3_readback_receipts": 26,
                    "archive_bytes": archive_bytes, "audit_sha256": sha(audit_path),
                    "cost_observations_sha256": sha(costs_path),
                    "plan_file_sha256": {"plan.json": sha(plan_path)},
                    "protocol_file_sha256": {"protocol.json": sha(protocol_path)},
                    "official_experiment_export_checker_receipted": True,
                    "official_export_flow": flow, "native_status_counts": statuses,
                    "independent_s3_readback_sample_sha256": sha(sample_path),
                    "interrupted_custody_reconciliation_sha256": sha(custody18_path),
                    "whole_worker_instruction_labels": 0}
    totals = dict(prior["totals"])
    for key, addition in (("archive_bytes", archive_bytes), ("observed_receipts", 26),
                          ("official_experiment_exports_checker_receipted", 1),
                          ("official_export_completed_rows", 4),
                          ("official_export_censored_rows", 22)):
        totals[key] += addition
    result = {"schema": "ml-dataset.local-campaign-inventory/v6",
              "scope": "nine named local synthetic campaigns with producer S3 readback receipts",
              "prior_inventory_sha256": sha(prior_path),
              "default_plan_method_cells": prior["default_plan_method_cells"],
              "campaigns": prior["campaigns"] + [campaign_row], "totals": totals}
    target = HERE / "inventory-v6.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("inventory-v6.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()
