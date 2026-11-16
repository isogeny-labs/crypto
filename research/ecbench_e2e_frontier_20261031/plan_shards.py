#!/usr/bin/env python3
"""Validate every queue-safe frontier shard with the pinned ECBench binary.

This runs ``ecbench plan`` only.  It performs no benchmark execution and writes
one receipt that binds every spec identity and expansion count.
"""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
MANIFEST = ROOT / "execution" / "shard-manifest.json"
OUT = ROOT / "execution" / "shard-preflight-plans.json"
REGISTRY = ROOT / "method-registry.json"
COVERAGE = ROOT / "coverage.json"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    manifest = json.loads(MANIFEST.read_text())
    coverage = json.loads(COVERAGE.read_text())
    binary = Path(os.environ.get("ECBENCH_BIN", "target/release/ecbench")).resolve()
    if not binary.is_file():
        raise SystemExit(f"missing ECBENCH_BIN: {binary}")
    actual_binary_sha = sha256(binary)
    expected_binary_sha = manifest["binary_sha256"]
    if actual_binary_sha != expected_binary_sha:
        raise SystemExit(
            f"binary SHA-256 mismatch: expected {expected_binary_sha}, got {actual_binary_sha}"
        )

    rows = []
    failures = []
    planned_method_ids: set[str] = set()
    planned_solver_ids: set[str] = set()
    seen_specs: set[str] = set()
    for job in manifest["jobs"]:
        spec_rel = job["spec"]
        if spec_rel in seen_specs:
            failures.append({"spec": spec_rel, "error": "duplicate_manifest_spec"})
        seen_specs.add(spec_rel)
        spec = json.loads((ROOT / spec_rel).read_text())
        planned_method_ids.update(item["method"]["id"] for item in spec["arms"])
        planned_solver_ids.update(
            item["method"].get("params", {}).get("solver", "").split(":", 1)[0]
            for item in spec["arms"]
            if item["method"].get("params", {}).get("solver")
        )
        roles = {item["role"] for item in spec["arms"]}
        if "reference" not in roles or "control" not in roles:
            failures.append({"spec": spec_rel, "error": "missing_reference_or_aa_control"})
        measurement = spec["measurement"]
        expected_measurement = {
            "rounds": manifest["measured_rounds"],
            "warmup": manifest["warmups"],
            "order": manifest["order"],
            "isolation_required": manifest["isolation_required"],
        }
        observed_measurement = {key: measurement.get(key) for key in expected_measurement}
        if observed_measurement != expected_measurement:
            failures.append(
                {
                    "spec": spec_rel,
                    "error": "campaign_measurement_mismatch",
                    "expected": expected_measurement,
                    "actual": observed_measurement,
                }
            )
        if job["whole_job_process_timeout_ceiling_seconds"] > manifest[
            "whole_job_process_timeout_budget_seconds"
        ]:
            failures.append({"spec": spec_rel, "error": "process_timeout_budget_exceeded"})

    required_current = {
        item["id"] for item in coverage["methods"] if item["disposition"] == "current_primary"
    }
    required_solvers = {item["id"] for item in coverage["solver_backends"]}
    missing_current = sorted(required_current - planned_method_ids)
    missing_solvers = sorted(required_solvers - planned_solver_ids)
    if missing_current:
        failures.append({"error": "missing_current_primary_methods", "ids": missing_current})
    if missing_solvers:
        failures.append({"error": "missing_solver_backends", "ids": missing_solvers})

    for index, job in enumerate(manifest["jobs"], 1):
        spec_path = ROOT / job["spec"]
        try:
            completed = subprocess.run(
                [str(binary), "plan", "--spec", str(spec_path), "--json"],
                check=False,
                capture_output=True,
                text=True,
                timeout=300,
            )
        except subprocess.TimeoutExpired as exc:
            failures.append({"spec": job["spec"], "error": "plan_timeout_300s"})
            print(f"[{index:02d}/{len(manifest['jobs'])}] TIMEOUT {job['spec']}", flush=True)
            continue
        if completed.returncode != 0:
            error = completed.stderr.strip() or completed.stdout.strip() or "plan failed"
            failures.append(
                {"spec": job["spec"], "returncode": completed.returncode, "error": error}
            )
            print(f"[{index:02d}/{len(manifest['jobs'])}] FAIL {job['spec']}: {error}", flush=True)
            continue
        plan = json.loads(completed.stdout)
        actual_executions = (
            plan["executions"]
            if isinstance(plan["executions"], int)
            else len(plan["executions"])
        )
        expected_executions = job["planned_executions"]
        if actual_executions != expected_executions:
            failures.append(
                {
                    "spec": job["spec"],
                    "error": "execution_count_mismatch",
                    "expected": expected_executions,
                    "actual": actual_executions,
                }
            )
        row = {
            "spec": job["spec"],
            "spec_id": plan["spec_id"],
            "spec_sha256": plan["spec_sha256"],
            "file_sha256": sha256(spec_path),
            "executions": actual_executions,
            "arms": len(plan["arms"]),
            "workloads": len(plan["workloads"]),
            "process_timeout_seconds": job["process_timeout_seconds"],
            "whole_job_process_timeout_ceiling_seconds": job[
                "whole_job_process_timeout_ceiling_seconds"
            ],
        }
        if completed.stderr.strip():
            row["planner_stderr"] = completed.stderr.strip()
        rows.append(row)
        print(
            f"[{index:02d}/{len(manifest['jobs'])}] OK {job['spec']} "
            f"{plan['spec_id']} {actual_executions}",
            flush=True,
        )

    receipt = {
        "schema": "ecbench.frontier-shard-preflight-plans/v1",
        "date": "2026-10-31",
        "source_commit": manifest["source_commit"],
        "binary": str(binary),
        "binary_sha256": actual_binary_sha,
        "method_registry_sha256": sha256(REGISTRY),
        "coverage_sha256": sha256(COVERAGE),
        "command": "ecbench plan --spec SPEC --json",
        "benchmark_work_executed": False,
        "planned_specs": len(manifest["jobs"]),
        "validated_specs": len(rows),
        "total_executions": sum(row["executions"] for row in rows),
        "max_whole_job_process_timeout_ceiling_seconds": max(
            (row["whole_job_process_timeout_ceiling_seconds"] for row in rows),
            default=0,
        ),
        "failures": failures,
        "coverage": {
            "required_current_primary_methods": sorted(required_current),
            "planned_current_primary_methods": sorted(required_current & planned_method_ids),
            "missing_current_primary_methods": missing_current,
            "required_solver_backends": sorted(required_solvers),
            "planned_solver_backends": sorted(required_solvers & planned_solver_ids),
            "missing_solver_backends": missing_solvers,
            "all_planned_top_level_methods": sorted(planned_method_ids),
        },
        "status": "valid" if not failures else "invalid",
        "specs": rows,
    }
    OUT.write_text(json.dumps(receipt, indent=2) + "\n")
    print(
        json.dumps(
            {
                "status": receipt["status"],
                "validated_specs": receipt["validated_specs"],
                "planned_specs": receipt["planned_specs"],
                "total_executions": receipt["total_executions"],
                "failures": len(failures),
            },
            sort_keys=True,
        )
    )
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
