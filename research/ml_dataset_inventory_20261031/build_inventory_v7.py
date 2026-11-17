#!/usr/bin/env python3
"""Append the audited ordinary-binary n23 arity block to immutable v6."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CAMPAIGN = HERE.parent / "ml_dataset_binary23_m3_15_enum_20261031"


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    prior_path = HERE / "inventory-v6.json"
    prior = read(prior_path)
    paths = {name: CAMPAIGN / name for name in (
        "plan.json", "protocol.json", "AUDIT.json", "experiment-export.json",
        "experiment-export-producer-receipt.json", "S3-READBACK-SAMPLE.json",
        ".artifacts.json")}
    plan, audit, export = (read(paths[name]) for name in
                           ("plan.json", "AUDIT.json", "experiment-export.json"))
    producer = read(paths["experiment-export-producer-receipt.json"])
    sample = read(paths["S3-READBACK-SAMPLE.json"])
    migration = read(paths[".artifacts.json"])
    cost = migration["files"]["cost-observations.jsonl"]
    require(prior["schema"] == "ml-dataset.local-campaign-inventory/v6" and
            prior["totals"] == {
                "archive_bytes": 33_359_314, "bounded_root_certificates": 16,
                "independently_solved_ecdlp_cells": 93, "observed_receipts": 198,
                "official_experiment_exports_checker_receipted": 8,
                "official_export_censored_rows": 31,
                "official_export_completed_rows": 103, "retained_f6_pair_domains": 12} and
            len(prior["campaigns"]) == 9, "immutable v6 denominator")
    require(plan["schema"] == "crypto.ml-dataset.plan/v1" and
            plan["families"] == ["binary"] and plan["field_sizes"] == [23] and
            plan["arities"] == list(range(3, 16)) and
            plan["solvers"] == ["enumerate"] and
            plan["curve_seeds"] == plan["generators"] == plan["keys"] == 1 and
            plan["twists"] is True and plan["factor_base_size"] == 3 and
            plan["protocol_sha256"] == sha(paths["protocol.json"]),
            "frozen campaign axes")
    require(audit["schema"] == "ml-dataset.binary23-m3-15-enum-independent-audit/v1" and
            audit["planned"] == audit["retained"] == len(audit["rows"]) == 26 and
            audit["artifacts_rehashed"] == 832 and
            audit["native_status_counts"] == {"verified": 6, "budget_exhausted": 20} and
            audit["twist_counts"] == {"untwisted": 13, "twisted": 13} and
            audit["independent_ecdlp_solves"] == 0 and
            audit["archive_bytes_total"] == 7_001_623 and
            audit["plan_file_sha256"] == sha(paths["plan.json"]) and
            audit["protocol_file_sha256"] == sha(paths["protocol.json"]) and
            audit["export_sha256"] == sha(paths["experiment-export.json"]),
            "independent archive audit binding")
    flow = export["flow"]
    require(producer["export_sha256"] == sha(paths["experiment-export.json"]) and
            producer["official_checker_exit_code"] == 0 and
            "(0 finding(s))" in producer["official_checker_stdout"] and
            producer["flow"] == flow and
            flow["planned"] == flow["attempted"] == flow["completed"] == 26 and
            flow["not_completed"] == flow["interrupted"] == flow["resumed"] == 0 and
            flow["excluded"] == [], "official export binding")
    require(migration["schema"] == "artifacts.manifest/v1" and
            set(migration["files"]) == {"cost-observations.jsonl"} and
            cost == {"sha256": "dd0bbc80af0713e8d8cc1586b9793cf35082f0dc16c95596e441cdc357cfb41c",
                     "bytes": 85780}, "migrated cost artifact pointer")
    cost_path = CAMPAIGN / "cost-observations.jsonl"
    if cost_path.exists():
        require(sha(cost_path) == cost["sha256"] and
                cost_path.stat().st_size == cost["bytes"], "hydrated cost artifact")
    require(sample["schema"] == "ml-dataset.binary23-m3-15-enum-s3-sample/v1" and
            [row["index"] for row in sample["samples"]] == [0, 25] and
            all(row["fresh_get_hash_match"] for row in sample["samples"]),
            "independent S3 sample")
    statuses = {"verified": 0, "budget_exhausted": 0}
    public_cores = {False: set(), True: set()}
    archive_bytes = 0
    for index in range(26):
        checkpoint = read(CAMPAIGN / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        native, row = custody["native_status"], audit["rows"][index]
        spec = {"m": 3 + index // 2, "solver": "enumerate", "twist": bool(index % 2),
                "generator_index": 0, "key_index": 0}
        require(checkpoint["index"] == custody["index"] == row["index"] == index and
                native["cell_spec"] == spec and row["m"] == spec["m"] and
                row["twist"] == spec["twist"] and row["method"] == "enumerate" and
                custody["plan_file_sha256"] == sha(paths["plan.json"]) and
                custody["protocol_file_sha256"] == sha(paths["protocol.json"]) and
                custody["archive_sha256"] == published["archive_sha256"] ==
                row["archive_sha256"] and custody["archive_bytes"] == row["archive_bytes"] and
                custody["s3_uri"] == row["s3_uri"] and
                custody["fresh_get_byte_comparison"] and
                published["archive_fresh_get"] and published["pointer_fresh_get"] and
                native["verified_and_exported"] and native["artifact_hashes_verified"] and
                native["method_artifact_verified"] and
                "all-visited-pdp-transitions" in native["method_artifact_roles"] and
                native["independently_solved_ecdlp_cells"] == 0 and
                native["ecdlp_solve_promoted"] is False and
                native["native_status"] == row["native_status"] ==
                ("verified" if index < 6 else "budget_exhausted"),
                f"campaign checkpoint {index}")
        statuses[native["native_status"]] += 1
        archive_bytes += custody["archive_bytes"]
        public_cores[spec["twist"]].add(row["public_core_sha256"])
    require(statuses == audit["native_status_counts"] and
            archive_bytes == audit["archive_bytes_total"] and
            all(len(cores) == 1 for cores in public_cores.values()) and
            public_cores[False] != public_cores[True], "paired public instances")
    for sample_row in sample["samples"]:
        row = audit["rows"][sample_row["index"]]
        require(sample_row["sha256"] == row["archive_sha256"] and
                sample_row["bytes"] == row["archive_bytes"] and
                sample_row["uri"] == row["s3_uri"], "sample identity")
    campaign_row = {"campaign": CAMPAIGN.name, "family": ["binary"],
                    "field_sizes": [23], "arities": list(range(3, 16)),
                    "methods": ["enumerate"], "twists_planned": True,
                    "planned_cells": 26, "observed_receipts": 26,
                    "native_verified_and_exported": 26,
                    "independently_solved_ecdlp_cells": 0,
                    "bounded_root_certificates": 0, "retained_f6_pair_domains": 0,
                    "producer_s3_readback_receipts": 26,
                    "archive_bytes": archive_bytes, "audit_sha256": sha(paths["AUDIT.json"]),
                    "cost_observations_sha256": cost["sha256"],
                    "cost_artifact_pointer_sha256": sha(paths[".artifacts.json"]),
                    "plan_file_sha256": {"plan.json": sha(paths["plan.json"])},
                    "protocol_file_sha256": {"protocol.json": sha(paths["protocol.json"])},
                    "official_experiment_export_checker_receipted": True,
                    "official_export_flow": flow, "native_status_counts": statuses,
                    "independent_s3_readback_sample_sha256": sha(paths["S3-READBACK-SAMPLE.json"]),
                    "whole_worker_instruction_labels": 0}
    totals = dict(prior["totals"])
    for key, addition in (("archive_bytes", archive_bytes), ("observed_receipts", 26),
                          ("official_experiment_exports_checker_receipted", 1),
                          ("official_export_completed_rows", 26)):
        totals[key] += addition
    result = {"schema": "ml-dataset.local-campaign-inventory/v7",
              "scope": "ten named local synthetic campaigns with producer S3 readback receipts",
              "prior_inventory_sha256": sha(prior_path),
              "default_plan_method_cells": prior["default_plan_method_cells"],
              "campaigns": prior["campaigns"] + [campaign_row], "totals": totals}
    target = HERE / "inventory-v7.json"
    data = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("inventory-v7.json differs; review source changes")
    target.write_bytes(data)
    print(json.dumps(totals, sort_keys=True))


if __name__ == "__main__":
    main()
