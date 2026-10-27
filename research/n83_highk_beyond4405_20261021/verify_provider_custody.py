#!/usr/bin/env python3
"""Admit a downloaded Tier 1 archive only after confirmed Pod deletion."""

import argparse
from datetime import timedelta
import hashlib
import json
from pathlib import Path
import tarfile

from research.n83_highk_beyond4405_20261021 import run, runpod_runner
from research.n83_highk_beyond4405_20261021 import verify_holdout
from research.n83_highk_beyond4405_20261021 import watch_runpod as watch


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def validate(allocation_path: Path, archive_dir: Path) -> dict:
    result = {
        "schema": "n83.fixed31-provider-custody/v1",
        "status": "BOUNDED_UNKNOWN",
        "final_primary_status": "BOUNDED_UNKNOWN",
        "final_cold_cost_status": "BOUNDED_UNKNOWN",
        "checks": {},
        "scientific_checks": {},
    }
    receipt = runpod_runner.read_allocation(allocation_path)
    pod_id = receipt["pod_id"]
    allocation_hash = run.digest(allocation_path)
    try:
        started = read(archive_dir / "watchdog_started.json")
        ready = read(archive_dir / "watchdog_ready.json")
        pod_guard = read(archive_dir / "pod_deadline_guard.json")
        transfer = read(archive_dir / "watchdog_transfer.json")
        absent = read(archive_dir / "provider_post_terminate.json")
        embedded = read(archive_dir / "results" / "runpod_allocation.json")
        timestamps = [runpod_runner.timestamp(value["recorded_utc"])
                      for value in (started, ready, pod_guard, transfer, absent)]
        created = runpod_runner.timestamp(receipt["created_utc"])
        deadline = runpod_runner.timestamp(receipt["stop_after_utc"])
        retire_at = deadline - timedelta(minutes=5)
        result["checks"] = {
            "allocation_bound": (
                embedded == receipt and
                all(value.get("pod_id") == pod_id for value in
                    (started, ready, transfer, absent)) and
                started.get("allocation_sha256") == allocation_hash and
                ready.get("allocation_sha256") == allocation_hash and
                started.get("schema") == "n83.fixed31-watchdog-start/v1" and
                ready.get("schema") == "n83.fixed31-watchdog-ready/v1" and
                runpod_runner.timestamp(started["provider_deadline_utc"]) ==
                deadline and
                runpod_runner.timestamp(started["retire_at_utc"]) ==
                retire_at and
                runpod_runner.timestamp(ready["retire_at_utc"]) ==
                retire_at),
            "chronology_and_margin": (
                created <= timestamps[0] <= timestamps[1] <= timestamps[2]
                <= timestamps[3] <= timestamps[4] <= retire_at),
            "pod_guard_armed": (
                pod_guard.get("schema") == "n83.fixed31-pod-deadline-guard/v1"
                and pod_guard.get("status") == "ARMED"
                and pod_guard.get("pod_id") == pod_id
                and pod_guard.get("allocation_sha256") == allocation_hash
                and runpod_runner.timestamp(pod_guard["stop_after_utc"]) ==
                deadline),
            "transfer_and_absence": (
                transfer.get("reason") == "PRODUCER_DONE" and
                transfer.get("spool_succeeded") is True and
                transfer.get("archive_ready") is True and
                transfer.get("error_type") is None and
                absent.get("pod_present") is False and
                not (archive_dir / "provider_termination_error.json").exists()),
            "archive_sha256": watch.archive_ready(archive_dir),
        }
        if result["checks"]["archive_sha256"]:
            with tarfile.open(archive_dir / "raw_results.tar.gz", "r:gz") as tar:
                members = {item.name: item for item in tar.getmembers()}
                required = ("watchdog_ready.json", "pod_deadline_guard.json",
                            "results/source.json",
                            "results/host.json", "results/runpod_allocation.json")
                if all(name in members and members[name].isfile()
                       for name in required):
                    content = tar.extractfile(members["watchdog_ready.json"]).read()
                    guard_content = tar.extractfile(
                        members["pod_deadline_guard.json"]).read()
                    archived_receipt = tar.extractfile(
                        members["results/runpod_allocation.json"]).read()
                    result["checks"]["archive_identity"] = (
                        hashlib.sha256(content).digest() ==
                        hashlib.sha256((archive_dir / "watchdog_ready.json")
                                       .read_bytes()).digest() and
                        hashlib.sha256(guard_content).digest() ==
                        hashlib.sha256((archive_dir / "pod_deadline_guard.json")
                                       .read_bytes()).digest() and
                        json.loads(archived_receipt) == receipt)
                else:
                    result["checks"]["archive_identity"] = False
                if all(name in members and members[name].isfile() for name in
                       ("results/screen_analysis.json",
                        "results/holdout/analysis.json")):
                    screen = json.load(tar.extractfile(
                        members["results/screen_analysis.json"]))
                    holdout = json.load(tar.extractfile(
                        members["results/holdout/analysis.json"]))
                    candidate = holdout.get("candidate_id")
                    rows = holdout.get("rows")
                    reported_cold = holdout.get("cold_cost_selector")
                    cold_replay = None
                    if (isinstance(candidate, str) and isinstance(rows, list)
                            and all(isinstance(row, dict) for row in rows)):
                        try:
                            cold_replay = verify_holdout.cross_family_cold_select(
                                rows, candidate)
                        except (KeyError, ValueError, TypeError, OverflowError,
                                ZeroDivisionError):
                            pass
                    result["scientific_checks"]["cold_selector_replay"] = (
                        cold_replay is not None and
                        cold_replay == reported_cold)
                    result["archived_scientific_status"] = {
                        "screen": screen.get("status"),
                        "primary": holdout.get("primary_status"),
                        "cross_family_selector": holdout.get(
                            "cross_family_selector", {}).get("status"),
                        "cold_cost_selector": (
                            cold_replay.get("status") if
                            result["scientific_checks"]["cold_selector_replay"]
                            else None),
                    }
        else:
            result["checks"]["archive_identity"] = False
    except (OSError, KeyError, ValueError, TypeError, AttributeError,
            tarfile.TarError) as error:
        result["error_type"] = type(error).__name__
    if result["checks"] and all(result["checks"].values()):
        result["status"] = "PASS_PROVIDER_CUSTODY"
        scientific = result.get("archived_scientific_status", {})
        if (scientific.get("screen") == "SCREEN_ADMITTED" and
                scientific.get("primary") ==
                "PASS_VERIFIED_CROSS_FAMILY_TIER1"):
            result["final_primary_status"] = (
                "PASS_VERIFIED_CROSS_FAMILY_TIER1")
            if (result["scientific_checks"].get("cold_selector_replay") and
                    scientific.get("cold_cost_selector") == "SELECTED"):
                result["final_cold_cost_status"] = "SELECTED_TWO_FAMILIES_TIER1"
    result["pod_id"] = pod_id
    result["allocation_sha256"] = allocation_hash
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", required=True, type=Path)
    parser.add_argument("--archive-dir", required=True, type=Path)
    args = parser.parse_args()
    result = validate(args.allocation.resolve(), args.archive_dir.resolve())
    output = args.archive_dir / "provider_custody_validation.json"
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": result["status"],
                      "final_primary_status": result["final_primary_status"],
                      "final_cold_cost_status": result["final_cold_cost_status"],
                      "scientific_checks": result["scientific_checks"],
                      "checks": result["checks"]}, sort_keys=True))
    return 0 if result["status"] == "PASS_PROVIDER_CUSTODY" else 2


if __name__ == "__main__":
    raise SystemExit(main())
