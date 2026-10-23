#!/usr/bin/env python3
"""Materialize canonical one-target workload and IC1 run IDs from admitted evidence."""

import argparse
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode("utf-8")


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def render(value):
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False)
            + "\n").encode("utf-8")


def materialize():
    analysis_path = RESULTS / "analysis.json"
    analysis = json.loads(analysis_path.read_text())
    fixture = json.loads((HERE / "fixture_manifest.json").read_text())
    host = json.loads((RESULTS / "host.json").read_text())
    source = json.loads((RESULTS / "source.json").read_text())
    custody = json.loads((RESULTS / "custody.json").read_text())
    if (host["git_sha"] != custody["hosted_run_head_sha"] or
            source["targets_sha256"] != fixture["targets_sha256"] or
            source["rho_seed_base"] != 2026102100 or
            custody["analysis_sha256"] != sha256(analysis_path)):
        raise ValueError("host, source, fixture or custody differs")
    if (analysis["status"] != "COLD_WINNER_CONFIRMED_WITHIN_PAIR" or
            analysis["selection_admitted"] is not True or
            len(analysis["point_rows"]) != 16 or
            analysis["ic_arm_count"] != 64 or
            analysis["rho_arm_count"] != 16):
        raise ValueError("only the admitted 16-point result can be materialized")
    manifests = {}
    for k in (1103, 1107):
        path = HERE / f"candidate_k{k}_root11.json"
        manifest = json.loads(path.read_text())
        if sha256(path) != analysis["candidate_manifests"][str(k)]["manifest_sha256"]:
            raise ValueError(f"K{k} candidate manifest differs")
        if manifest["candidate_id"] != analysis["candidate_manifests"][str(k)]["candidate_id"]:
            raise ValueError(f"K{k} candidate ID differs")
        manifests[k] = manifest
    curve = manifests[1107]["curve"]
    if (curve["curve_id"] != manifests[1103]["curve"]["curve_id"] or
            curve["r"] != manifests[1103]["curve"]["r"]):
        raise ValueError("candidate curves differ")
    workloads, rows = [], []
    for point in sorted(analysis["point_rows"], key=lambda row: row["point_index"]):
        index = point["point_index"]
        if index != len(workloads) + 1 or len(point["arms"]) != 4:
            raise ValueError("missing or repeated point/arm")
        workload = {
            "cache_policy": "empty_factor_base_root_index_and_log_caches",
            "curve_id": curve["curve_id"],
            "input_law": {
                "domain": fixture["domain"],
                "point_index_zero_based": index - 1,
                "scalar_rule": "1+LE(SHA256(domain||u32le(i))) mod (r-1)",
                "targets_sha256": fixture["targets_sha256"],
            },
            "subgroup_order": curve["r"],
            "target_count": 1,
            "target_point": point["public_point"],
            "temperature": "cold",
        }
        workload_id = hashlib.sha256(canonical(workload)).hexdigest()[:12]
        workloads.append({"point_index": index, "workload_id": workload_id,
                          "canonical_record": workload})
        for arm in point["arms"]:
            k = arm["k"]
            suffix = arm["arm"].rsplit("_", 1)[-1]
            if k not in manifests or suffix not in ("a", "b"):
                raise ValueError("unexpected candidate or A/B arm")
            if arm["status"] != "PASS_INDEPENDENT_REPLAY":
                raise ValueError("arm did not pass independent replay")
            run_number = 1 if suffix == "a" else 2
            candidate_id = manifests[k]["candidate_id"]
            rows.append({
                "candidate_id": candidate_id,
                "workload_id": workload_id,
                "run_id": f"{candidate_id}W{workload_id}R{run_number}",
                "run_number": run_number,
                "point_index": index,
                "producer_arm": arm["arm"],
                "hosted_run_id": custody["hosted_run_id"],
                "artifact": {"sha256": custody["raw_zip_sha256"],
                             "s3_uri": custody["s3_uri"]},
                "host": {"hardware_tier": analysis["hardware_tier"],
                         "provider": host["provider"],
                         "instance_id": host["instance_id"],
                         "cpu_model": host["cpu_model"],
                         "visible_affinity": host["visible_affinity"],
                         "memory_kib": host["mem_total_kib"]},
                "limits": {"ic_seconds": 480, "rho_seconds": 120},
                "seeds": {"rank": 29,
                          "rho": source["rho_seed_base"] + index},
                "executed_source": {"ic_sha256": source["source_sha256"],
                                    "ic_binary_sha256": source["binary_sha256"],
                                    "rho_binary_sha256": source["rho_binary_sha256"]},
                "source_analysis_sha256": sha256(analysis_path),
                "status": arm["status"],
                "correctness": {
                    "base_hash": arm["base_hash"],
                    "actual_factor_base_points": arm["actual_factor_base_points"],
                    "effective_columns": arm["effective_columns"],
                    "rho_status": point["rho"]["status"],
                    "resource_status": arm["resources"]["status"],
                    "aa_status": point["aa_control"][f"k{k}"]["status"],
                },
                "measurements_ms": {
                    "one_target_online": arm["one_target_online_ms"],
                    "online_phases": arm["online_phases_ms"],
                    "cold_process": arm["cold_process_ms_exploratory"],
                    "setup": arm["setup_ms"],
                    "rank_stage": arm["rank_stage_ms"],
                    "target_stages": arm["targets_total_ms"],
                    "cold_unattributed": arm["cold_unattributed_ms"],
                    "same_point_rho_online": point["rho"]["online_ms"],
                },
                "workload_role": "one_public_target_fresh_process",
            })
    if len(rows) != 64 or len({row["run_id"] for row in rows}) != 64:
        raise ValueError("IC1 run IDs are not unique and complete")
    return ({"schema": "n83.ic1-single-target-workloads/v1", "rows": workloads},
            {"schema": "n83.ic1-run-rows/v1", "rows": rows})


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    workloads, rows = materialize()
    for name, value in (("workloads.json", workloads), ("rows.json", rows)):
        path = RESULTS / name
        data = render(value)
        if args.check:
            if not path.exists() or path.read_bytes() != data:
                raise ValueError(f"{path} differs from materialized evidence")
        else:
            path.write_bytes(data)
    print(json.dumps({"status": "PASS", "workloads": 16, "runs": 64},
                     sort_keys=True))


if __name__ == "__main__":
    main()
