#!/usr/bin/env python3
"""One source-pinned, exposed-target N83 canonical 4+2 SAT cap rung."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
PRIOR = HERE.parent / "hosted-m6-paired-protocol-20261017"
sys.path.insert(0, str(PRIOR))
import plan  # noqa: E402
import run as prior_run  # noqa: E402


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_new(path: Path, value: dict) -> None:
    with path.open("x") as sink:
        json.dump(value, sink, sort_keys=True, indent=2)
        sink.write("\n")


def preflight(inputs: Path, arm_name: str, image: str) -> tuple[dict, dict, str]:
    dispatch = json.loads((HERE / "dispatch.json").read_text())
    require(dispatch["schema"] == "n83.hosted-m6-highcap-dispatch/v1"
            and dispatch["selected_best_total_runtime"] is None
            and dispatch["arms"] == ["gray_pool", "hash_pool"]
            and arm_name in dispatch["arms"]
            and image == prior_run.IMAGE,
            "high-cap dispatch, arm or image differs")
    source_hashes = {
        plan.SAVED: dispatch["frozen_plan_sha256"],
        prior_run.MANIFEST: dispatch["manifest_sha256"],
        prior_run.CORPUS: dispatch["corpus_sha256"],
        PRIOR / "run.py": dispatch["prior_runner_sha256"],
        PRIOR / "plan.py": dispatch["prior_plan_program_sha256"],
    }
    require(all(sha256(path) == digest for path, digest in source_hashes.items()),
            "frozen source, manifest or corpus differs")
    expected = plan.build()
    require(plan.SAVED.read_text() == json.dumps(expected, indent=2, sort_keys=True) + "\n",
            "frozen plan no longer replays")
    arm = next(row for row in expected["arms"] if row["role"] == arm_name)
    receipt = json.loads((inputs / "input.json").read_text())
    require(receipt["schema"] == "n83.hosted-m6-highcap-input/v1"
            and receipt["status"] == "PASS_EXACT_ARCHIVED_INPUTS"
            and receipt["provider_artifact_id"] == dispatch["provider_artifact_id"]
            and receipt["provider_zip_sha256"] == dispatch["provider_zip_sha256"]
            and receipt["dispatch_sha256"] == sha256(HERE / "dispatch.json"),
            "archived input receipt differs")
    worker = inputs / "worker"
    require(sha256(worker) == dispatch["worker_sha256"],
            "archived worker differs")
    for row in expected["arms"]:
        for kind in ("base", "pool"):
            key = row[f"{kind}_object_blake3"]
            archive = inputs / f"{key}.jsonl.gz"
            require(dispatch["source_archive_sha256"][key] == sha256(archive)
                    and row[f"{kind}_compressed_sha256"] == sha256(archive),
                    "archived factor base differs")
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                     text=True).strip()
    require(not subprocess.check_output(["git", "status", "--porcelain"],
                                        cwd=ROOT, text=True),
            "hosted source checkout is dirty")
    return dispatch, arm, commit


def execute(inputs: Path, output: Path, arm_name: str, image: str,
            run_id: str) -> dict:
    dispatch, arm, commit = preflight(inputs, arm_name, image)
    require(run_id.isdecimal(), "run ID differs")
    output.mkdir(parents=True, exist_ok=False)
    write_new(output / "config.json", {
        "schema": "n83.hosted-m6-highcap-config/v1",
        "status": "STARTED",
        "source_commit": commit,
        "provider_source_commit": dispatch["provider_source_commit"],
        "provider_artifact_id": dispatch["provider_artifact_id"],
        "dispatch_sha256": sha256(HERE / "dispatch.json"),
        "worker_sha256": dispatch["worker_sha256"],
        "arm": arm_name,
        "image": image,
        "runner_name": os.environ.get("RUNNER_NAME"),
        "run_id": run_id,
        "run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "conservative_full_charge_seconds": dispatch["job_cap_seconds"],
        "selected_best_total_runtime": None,
    })
    started = time.monotonic()
    imported = prior_run.container_stage("import", arm, inputs,
                                         inputs / "worker", output, image,
                                         run_id, dispatch["import_cap_seconds"])
    search = None
    worker_status = None
    if (imported["timed_out"] or imported["exit_code"] != 0
            or imported["cleanup_exit_code"] != 0):
        status = "PRODUCER_FAILURE_import"
    else:
        try:
            checked = json.loads((output / "import.stdout").read_text())
            require(checked["schema"] == "n83.cross-stream-pilot-import/v1"
                    and checked["status"] == "PASS"
                    and checked["small_object_blake3"] == arm["base_object_blake3"]
                    and checked["pool_object_blake3"] == arm["pool_object_blake3"]
                    and checked["shared_signed_orbits"] == arm["shared_signed_orbits"]
                    and checked["residual_signed_orbits"] == arm["residual_signed_orbits"]
                    and checked["solver_search_executed"] is False,
                    "exact import differs")
            search_arm = dict(arm)
            argv = list(arm["worker_search_argv"])
            require(argv[1] == "run-cross-pilot"
                    and argv[9:14] == ["2300", "20000", "64", "4", "2"],
                    "prior search command differs")
            argv[1] = "run-cross-pilot-canonical"
            argv[9] = str(dispatch["worker_wall_cap_seconds"])
            argv[10] = str(dispatch["conflict_cap"])
            search_arm["worker_search_argv"] = argv
            search = prior_run.container_stage(
                "search", search_arm, inputs, inputs / "worker", output,
                image, run_id, dispatch["outer_search_cap_seconds"])
            if search["timed_out"]:
                status = "UNKNOWN_outer_wall_cap"
            elif search["exit_code"] == 124:
                status = "UNKNOWN_worker_wall_cap"
            elif search["exit_code"] != 0 or search["cleanup_exit_code"] != 0:
                status = "PRODUCER_FAILURE_search"
            else:
                result = json.loads((output / "search/result.json").read_text())
                require(result["schema"] == "n83.cross-stream-pilot-partial-result/v2"
                        and result["ordering"] == "equal_domain_runs"
                        and result["base_object_blake3"] == arm["base_object_blake3"]
                        and result["pool_object_blake3"] == arm["pool_object_blake3"]
                        and result["status"] in {
                            "PASS_one_bounded_partial", "UNKNOWN_search_cap",
                            "UNKNOWN_preflight_cap", "PASS_one_trial_refuted",
                            "PRODUCER_FAILURE_invalid_model",
                            "PRODUCER_FAILURE_unclassified_solver_result",
                        }, "worker result differs")
                worker_status = result["status"]
                status = ("OBSERVED_MODEL_REQUIRES_INDEPENDENT_REPLAY"
                          if worker_status == "PASS_one_bounded_partial"
                          else worker_status)
        except (OSError, KeyError, ValueError, json.JSONDecodeError) as error:
            status = "PRODUCER_FAILURE_receipt"
            (output / "receipt-error.txt").write_text(f"{type(error).__name__}: {error}\n")
    outer = {
        "schema": "n83.hosted-m6-highcap-outer/v1",
        "status": status,
        "worker_status": worker_status,
        "arm": arm_name,
        "source_commit": commit,
        "run_id": run_id,
        "import": imported,
        "search": search,
        "elapsed_supervisor_wall_seconds_L0": time.monotonic() - started,
        "conservative_full_charge_seconds": dispatch["job_cap_seconds"],
        "natural_partial_yield": None,
        "matrix_rank": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }
    write_new(output / "outer.json", outer)
    return outer


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--arm", choices=["gray_pool", "hash_pool"], required=True)
    parser.add_argument("--image", default=prior_run.IMAGE)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        dispatch, arm, commit = preflight(args.inputs, args.arm, args.image)
        print(json.dumps({"status": "PASS_HIGHCAP_PREFLIGHT", "arm": arm["role"],
                          "source_commit": commit,
                          "provider_artifact_id": dispatch["provider_artifact_id"]},
                         sort_keys=True))
        return
    result = execute(args.inputs, args.output, args.arm, args.image, args.run_id)
    print(json.dumps({"status": result["status"], "arm": args.arm}, sort_keys=True))
    if result["status"].startswith("PRODUCER_FAILURE"):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
