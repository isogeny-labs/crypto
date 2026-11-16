#!/usr/bin/env python3
"""Guarded hosted N83 Gray/hash pair with equal-domain canonical summands."""

from __future__ import annotations

import argparse
import hashlib
import json
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

PLAN_SHA256 = "7788514127328a78cf2b0f634003ddae3ff18a9ff0c0c8cd75b50fcc006152cd"
FULL_CHARGE_SECONDS = 7_500
S3_RECEIPT = HERE.parent / "hosted-m6-paired-run-39054543374/factor-bases-s3-check.json"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def write_new(path: Path, value: dict) -> None:
    with path.open("x") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")


def execute(archives: Path, worker: Path, output: Path, image: str,
            run_id: str, source_commit: str) -> dict:
    expected = plan.build()
    require(sha256(plan.SAVED) == PLAN_SHA256
            and plan.SAVED.read_text() == json.dumps(expected, sort_keys=True, indent=2) + "\n"
            and image == prior_run.IMAGE and run_id.isdecimal()
            and worker.is_file() and archives.is_dir()
            and prior_run.MANIFEST.is_file() and prior_run.CORPUS.is_file(),
            "frozen plan, image, worker or public input differs")
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                     text=True).strip()
    require(commit == source_commit and not subprocess.check_output(
        ["git", "status", "--porcelain"], cwd=ROOT, text=True),
        "hosted source commit or clean checkout differs")
    for arm in expected["arms"]:
        for kind in ("base", "pool"):
            archive = archives / f"{arm[f'{kind}_object_blake3']}.jsonl.gz"
            require(sha256(archive) == arm[f"{kind}_compressed_sha256"],
                    "regenerated factor-base archive differs")
    custody = json.loads(S3_RECEIPT.read_text())
    require(custody["schema"] == "n83.hosted-m6-s3-base-replay/v1"
            and custody["status"] == "PASS_3_FROZEN_S3_OBJECTS"
            and custody["source_plan_sha256"] == PLAN_SHA256
            and custody["selected_best_total_runtime"] is None
            and len(custody["objects"]) == 3,
            "prior S3 factor-base custody receipt differs")
    by_hash = {row["compressed_blake3"]: row for row in custody["objects"]}
    require(len(by_hash) == 3, "prior S3 custody object identities differ")
    for arm in expected["arms"]:
        for kind in ("base", "pool"):
            key = arm[f"{kind}_object_blake3"]
            archived = archives / f"{key}.jsonl.gz"
            row = by_hash[key]
            require(row["s3_uri"] == arm[f"{kind}_s3_uri"]
                    and row["compressed_sha256"] == sha256(archived)
                    and row["bytes"] == archived.stat().st_size,
                    "prior S3 custody row differs from regenerated archive")
    output.mkdir(parents=True, exist_ok=False)
    write_new(output / "config.json", {
        "schema": "n83.hosted-m6-canonical-pair-config/v1",
        "source_commit": commit,
        "plan_sha256": PLAN_SHA256,
        "protocol_sha256": sha256(HERE / "PROTOCOL.md"),
        "runner_sha256": sha256(Path(__file__)),
        "s3_custody_receipt_sha256": sha256(S3_RECEIPT),
        "worker_sha256": sha256(worker),
        "manifest_sha256": sha256(prior_run.MANIFEST),
        "corpus_sha256": sha256(prior_run.CORPUS),
        "image": image, "run_id": run_id,
        "ordering": "equal_domain_runs",
        "conservative_full_charge_seconds": FULL_CHARGE_SECONDS,
        "selected_best_total_runtime": None,
    })
    started = time.monotonic()
    arms = []
    for arm in expected["arms"]:
        arm_dir = output / arm["role"]
        arm_dir.mkdir()
        imported = prior_run.container_stage("import", arm, archives, worker,
                                              arm_dir, image, run_id, 300)
        if imported["timed_out"] or imported["exit_code"] != 0 or imported["cleanup_exit_code"] != 0:
            arms.append({"role": arm["role"], "status": "PRODUCER_FAILURE_import",
                         "import": imported, "search": None})
            break
        try:
            verified = json.loads((arm_dir / "import.stdout").read_text())
            require(verified["schema"] == "n83.cross-stream-pilot-import/v1"
                    and verified["status"] == "PASS"
                    and verified["small_object_blake3"] == arm["base_object_blake3"]
                    and verified["pool_object_blake3"] == arm["pool_object_blake3"]
                    and verified["shared_signed_orbits"] == arm["shared_signed_orbits"]
                    and verified["residual_signed_orbits"] == arm["residual_signed_orbits"]
                    and verified["solver_search_executed"] is False,
                    "exact import receipt differs")
        except (OSError, KeyError, ValueError, json.JSONDecodeError):
            arms.append({"role": arm["role"], "status": "PRODUCER_FAILURE_import_receipt",
                         "import": imported, "search": None})
            break
        canonical = dict(arm)
        argv = list(arm["worker_search_argv"])
        require(argv[1] == "run-cross-pilot" and argv[9:14] ==
                ["2300", "20000", "64", "4", "2"],
                "prior worker call or caps differ")
        argv[1] = "run-cross-pilot-canonical"
        canonical["worker_search_argv"] = argv
        searched = prior_run.container_stage("search", canonical, archives,
                                              worker, arm_dir, image, run_id, 2_400)
        if searched["timed_out"]:
            status = "UNKNOWN_outer_wall_cap"
        elif searched["exit_code"] == 124:
            status = "UNKNOWN_worker_wall_cap"
        elif searched["exit_code"] != 0 or searched["cleanup_exit_code"] != 0:
            status = "PRODUCER_FAILURE_search"
        else:
            try:
                result = json.loads((arm_dir / "search/result.json").read_text())
                require(result["schema"] == "n83.cross-stream-pilot-partial-result/v2"
                        and result["ordering"] == "equal_domain_runs"
                        and result["base_object_blake3"] == arm["base_object_blake3"]
                        and result["pool_object_blake3"] == arm["pool_object_blake3"]
                        and result["residual_signed_orbits"] == arm["residual_signed_orbits"]
                        and result["status"] in {
                            "PASS_one_bounded_partial", "UNKNOWN_search_cap",
                            "UNKNOWN_preflight_cap", "PASS_one_trial_refuted",
                            "PRODUCER_FAILURE_invalid_model",
                            "PRODUCER_FAILURE_unclassified_solver_result",
                        }
                        and ((result["status"] == "PASS_one_bounded_partial") ==
                             (result["candidate"] is not None)),
                        "canonical result receipt differs")
                status = result["status"]
            except (OSError, KeyError, ValueError, json.JSONDecodeError):
                status = "PRODUCER_FAILURE_result_receipt"
        arms.append({"role": arm["role"], "status": status,
                     "import": imported, "search": searched})
        write_new(arm_dir / "outer.json", arms[-1])
        if status.startswith("PRODUCER_FAILURE"):
            break
    changed = (subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                       text=True).strip() != commit or
               bool(subprocess.check_output(["git", "status", "--porcelain"],
                                            cwd=ROOT, text=True)))
    status = ("PRODUCER_FAILURE_frozen_source_changed" if changed else
              "PRODUCER_FAILURE_arm" if any(a["status"].startswith("PRODUCER_FAILURE")
                                            for a in arms) else
              "BOUNDED_UNKNOWN_incomplete_pair" if len(arms) != 2 else
              "BOUNDED_SCREEN_complete_pair")
    result = {"schema": "n83.hosted-m6-canonical-pair-outer/v1",
              "status": status, "source_commit": commit, "run_id": run_id,
              "arms": arms,
              "elapsed_supervisor_wall_seconds_L0": time.monotonic() - started,
              "conservative_full_charge_seconds": FULL_CHARGE_SECONDS,
              "natural_partial_yield": None, "matrix_rank": None,
              "total_index_calculus_runtime_ms": None,
              "selected_best_total_runtime": None}
    write_new(output / "outer.json", result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archives", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--image", default=prior_run.IMAGE)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--source-commit", required=True)
    args = parser.parse_args()
    result = execute(args.archives.resolve(strict=True), args.worker.resolve(strict=True),
                     args.output, args.image, args.run_id, args.source_commit)
    print(json.dumps({"status": result["status"],
                      "arms": [arm["status"] for arm in result["arms"]]}, sort_keys=True))
    if result["status"].startswith("PRODUCER_FAILURE"):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
