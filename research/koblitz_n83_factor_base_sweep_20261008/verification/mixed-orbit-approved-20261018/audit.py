#!/usr/bin/env python3
"""Independently check the approved N83 mixed-orbit replication receipts."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import subprocess

import blake3


HERE = Path(__file__).resolve().parent
CHECKOUT = HERE.parents[3]
SOURCE_COMMIT = "d5d3ec3a72cb801812e75a60776b5768e6f218be"
IMAGE = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
BINARY_SHA256 = "68d03989b8f2a4c3bec9c7fc52af42a8f2ddbc1da8cadd4b2d9fede8199ddca2"
PRIOR_AUDIT_SHA256 = "76a8bf74e46feb0ef4fbe650395f39733f4ff95d31c35f8829cb506a9ad04c7d"
BASES = {
    "small": ("76e1cbe4ea5a9d26e5b24a2d27f9401271dc471846529190a0416d8e09837a92",
              "eb342b574e244e1e67e8e78c805f33ee88edcdb4f3a14d4114e6f413beede380", 5956114),
    "envelope": ("82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851",
                 "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a", 10313647),
}


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha(path: Path) -> str:
    state = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            state.update(chunk)
    return state.hexdigest()


def b3(path: Path) -> str:
    state = blake3.blake3()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            state.update(chunk)
    return state.hexdigest()


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def option(command: list[str], name: str) -> str:
    return command[command.index(name) + 1]


def stage(evidence: Path, receipt: dict, name: str, wall: int, cpus: str,
          memory: str) -> str:
    require(receipt["name"] == name and receipt["exit_code"] == 0
            and receipt["timed_out"] is False
            and receipt["launch_error"] is None
            and receipt["cleanup_verified"] is True
            and receipt["cleanup_error"] is None,
            f"{name} stage did not complete and clean up")
    state, command = receipt["container_state"], receipt["command"]
    require(state["ExitCode"] == 0 and state["OOMKilled"] is False
            and state["Status"] == "exited" and state["Running"] is False
            and math.isfinite(receipt["wall_elapsed_seconds_L0"])
            and 0 < receipt["wall_elapsed_seconds_L0"] <= wall + 30,
            f"{name} container exit or wall differs")
    require(option(command, "--pull") == "never"
            and option(command, "--platform") == "linux/arm64"
            and option(command, "--network") == "none"
            and option(command, "--cpus") == cpus
            and option(command, "--memory") == memory
            and option(command, "--memory-swap") == memory
            and option(command, "--cap-drop") == "ALL"
            and IMAGE in command,
            f"{name} command limits differ")
    if name != "build":
        require("--read-only" in command and option(command, "--pids-limit") == "64"
                and option(command, "--security-opt") == "no-new-privileges"
                and all(any(f"dst={mount},readonly" in part for part in command)
                        for mount in ("/worker", "/small.jsonl.gz", "/envelope.jsonl.gz")),
                f"{name} runtime mount limits differ")
    for suffix in ("stdout", "stderr"):
        require(sha(evidence / name_dir(name) / f"{name}.{suffix}")
                == receipt[f"{suffix}_sha256"], f"{name} {suffix} hash differs")
    return option(command, "--name")


def name_dir(name: str) -> str:
    return "build" if name == "build" else "run"


def live(evidence: Path, filename: str, name: str, cpus: int, memory: int,
         read_only: bool, mounts: dict[str, bool]) -> None:
    item = read(evidence / filename)[0]
    host = item["HostConfig"]
    require(item["Name"] == "/" + name and item["State"]["Running"] is True
            and item["Image"] == IMAGE and host["Memory"] == memory
            and host["MemorySwap"] == memory
            and host["NanoCpus"] == cpus * 1_000_000_000
            and host["NetworkMode"] == "none"
            and host["ReadonlyRootfs"] is read_only
            and host["CapDrop"] == ["ALL"]
            and host["PidsLimit"] == (256 if cpus == 4 else 64),
            f"{name} live cgroup inspection differs")
    actual = {item["Destination"]: item["RW"] for item in item["Mounts"]}
    require(actual == mounts, f"{name} live mounts differ")


def audit(evidence: Path, small: Path, envelope: Path) -> dict:
    auth = read(evidence / "authorization.json")
    require(sha(evidence / "authorization.json") == sha(HERE / "authorization.json")
            and auth == {
                "schema": "n83.additional-local-budget/v2",
                "study": "koblitz_n83_factor_base_sweep_20261008",
                "purpose": "mixed_orbit_two_residual_first_gate",
                "prior_total_charged_seconds": "12600",
                "prior_import_audit_sha256": PRIOR_AUDIT_SHA256,
                "additional_authorized_seconds": "2400",
                "build_dir": "/Volumes/SSD990/llm/tmp/n83-first-mixed-orbit-20261018-01/build",
                "output_dir": "/Volumes/SSD990/llm/tmp/n83-first-mixed-orbit-20261018-01/run",
                "user_approval_reference": "2026-10-18 current chat approval, response annotation 1",
            }, "run-specific authorization differs")
    input_receipt = read(evidence / "inputs/inputs-receipt.json")
    prefix = ("s3://crypto-autoresearcher/factor-bases/icv1/etc/"
              "koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/")
    require(input_receipt["status"] == "PASS_FRESH_S3_BYTES"
            and input_receipt["selected_best_total_runtime"] is None,
            "fresh S3 receipt differs")
    for label, path in (("small", small), ("envelope", envelope)):
        digest, expected_sha, size = BASES[label]
        head = read(evidence / f"inputs/{label}-head.json")
        got = read(evidence / f"inputs/{label}-get.json")
        retained = input_receipt["objects"][label]
        require(path.stat().st_size == head["ContentLength"] == got["ContentLength"] == size
                and sha(path) == expected_sha and b3(path) == digest
                and head["ETag"] == got["ETag"] == retained["head_etag"]
                == retained["get_etag"]
                and head["Metadata"]["blake3"] == digest
                and head["ServerSideEncryption"] == "AES256"
                and retained["sha256"] == expected_sha
                and retained["blake3"] == digest
                and retained["s3_uri"] == prefix + digest + ".jsonl.gz"
                and retained["bytes"] == size,
                f"{label} S3 bytes or HEAD/GET receipt differs")

    build = read(evidence / "build/build.json")
    binary = evidence / "build/worker"
    require(build["schema"] == "n83.mixed-orbit-linux-build/v1"
            and build["status"] == "PASS_build_only"
            and build["source_commit"] == SOURCE_COMMIT
            and build["authorization_sha256"] == sha(evidence / "authorization.json")
            and build["container_image_id"] == IMAGE
            and build["binary_sha256"] == sha(binary) == BINARY_SHA256
            and binary.open("rb").read(4) == b"\x7fELF"
            and build["conservative_charge_seconds"] == 600
            and 0 < build["elapsed_seconds_L0"] <= 600
            and build["retained_n83_import_executed"] is False
            and build["retained_n83_search_executed"] is False,
            "frozen Linux build differs")
    build_name = stage(evidence, build["stage"], "build", 450, "4", "8192m")
    live(evidence, "build-live-inspect.json", build_name, 4, 8 * 1024**3,
         False, {"/cargo/registry": False, "/src": False, "/target": True})
    require(len(build["source_sha256"]) == 12, "frozen source set differs")
    for relative, expected in build["source_sha256"].items():
        blob = subprocess.check_output(["git", "show", f"{SOURCE_COMMIT}:{relative}"],
                                       cwd=CHECKOUT)
        require(hashlib.sha256(blob).hexdigest() == expected,
                f"source blob {relative} differs")

    config, outer, budget = (read(evidence / f"run/{name}.json")
                             for name in ("config", "outer", "budget"))
    require(config["schema"] == "n83.mixed-orbit-supervisor-config/v1"
            and config["source_commit"] == outer["source_commit"] == SOURCE_COMMIT
            and config["authorization_sha256"] == outer["authorization_sha256"]
                == budget["authorization_sha256"] == sha(evidence / "authorization.json")
            and config["container_image_id"] == IMAGE
            and config["build_cap_seconds"] == 600
            and config["verify_cap_seconds"] == 360
            and config["search_cap_seconds"] == 1200
            and config["worker_wall_seconds"] == 1150
            and config["cleanup_reserve_seconds"] == 240
            and config["conservative_total_charge_seconds"] == 2400
            and config["max_trials"] == 1
            and config["conflicts_per_trial"] == 5000
            and config["max_models_per_trial"] == 32
            and config["small_summands"] == 3
            and config["residual_summands"] == 2
            and config["total_index_calculus_runtime_ms"] is None,
            "run configuration differs")
    expected_inputs = {
        "authorization.json": sha(evidence / "authorization.json"),
        "build.json": sha(evidence / "build/build.json"),
        "worker": BINARY_SHA256,
        "small.jsonl.gz": BASES["small"][1],
        "envelope.jsonl.gz": BASES["envelope"][1],
    }
    for path, digest in config["input_sha256"].items():
        match = next((value for suffix, value in expected_inputs.items()
                      if path.endswith("/" + suffix)), None)
        if match is None:
            match = build["source_sha256"].get(str(Path(path).relative_to(CHECKOUT)))
        require(digest == match, f"frozen input {path} differs")
    require(len(config["input_sha256"]) == len(expected_inputs) + 12,
            "frozen input count differs")
    require(outer["schema"] == "n83.mixed-orbit-supervisor-outer/v1"
            and outer["status"] == "UNKNOWN_search_cap"
            and outer["config_sha256"] == sha(evidence / "run/config.json")
            and outer["import_passed"] is True
            and outer["event_replay_error"] is None
            and outer["frozen_input_changed"] is False
            and 0 < outer["elapsed_supervisor_seconds_L0"] <= 1800
            and outer["total_index_calculus_runtime_ms"] is None
            and outer["selected_best_total_runtime"] is None
            and read(evidence / "run-launch.stdout") == outer
            and (evidence / "run-launch.stderr").stat().st_size == 0,
            "outer run receipt differs")
    require([s["name"] for s in outer["stages"]] == ["verify", "search"],
            "run stage sequence differs")
    verify_name = stage(evidence, outer["stages"][0], "verify", 360, "1", "4096m")
    search_name = stage(evidence, outer["stages"][1], "search", 1200, "1", "4096m")
    live(evidence, "search-live-inspect.json", search_name, 1, 4 * 1024**3,
         True, {"/corpus.json": False, "/out": True, "/worker": False,
                "/small.jsonl.gz": False, "/envelope.jsonl.gz": False,
                "/small-manifest.json": False, "/envelope-manifest.json": False})
    imported = read(evidence / "run/verify.stdout")
    require(imported == {
        "schema": "n83.mixed-orbit-import/v1", "status": "PASS",
        "curve_slug": "icv1-f2m83-tm6151469093347-debefd74",
        "small_object_blake3": BASES["small"][0],
        "envelope_object_blake3": BASES["envelope"][0],
        "small_points": 196212, "small_signed_orbits": 1182,
        "residual_points": 143756, "residual_signed_orbits": 866,
        "solver_search_executed": False,
    }, "exact S3-backed import differs")
    worker_config = read(evidence / "run/run/config.json")
    result = read(evidence / "run/run/result.json")
    events = (evidence / "run/run/events.jsonl").read_text().splitlines()
    require(len(events) == 1, "trial event count differs")
    event = json.loads(events[0])
    require(worker_config["schema"] == "n83.mixed-orbit-partial-config/v1"
            and worker_config["budget_seconds"] == 1150
            and worker_config["max_trials"] == 1
            and worker_config["conflict_budget_per_trial"] == 5000
            and worker_config["max_models_per_trial"] == 32
            and worker_config["small_summands"] == 3
            and worker_config["residual_summands"] == 2
            and worker_config["order_first_pair"] is True
            and worker_config["max_variables"] == 150000
            and worker_config["max_domain_clauses"] == 12000000
            and worker_config["memory_cgroup_limit_bytes"] == 4 * 1024**3
            and worker_config["memory_cgroup_swap_limit_bytes"] == 0,
            "worker config differs")
    require(result["schema"] == "n83.mixed-orbit-partial-result/v1"
            and result["status"] == "UNKNOWN_search_cap"
            and result["trials_attempted"] == result["solver_exhausted_trials"] == 1
            and result["solver_calls"] == 1
            and result["solver_conflicts"] == 5000
            and all(result[k] == 0 for k in (
                "solver_models", "invalid_models", "group_rejected_models",
                "graph_accepted_candidates", "graph_pivot_rows", "graph_full_from_partials",
                "graph_capped_rows", "completed_rows"))
            and result["matrix_rank"] is None
            and 0 < result["memory_cgroup_peak_bytes"] <= 4 * 1024**3
            and result["total_index_calculus_runtime_ms"] is None
            and result["selected_best_total_runtime"] is None,
            "worker search receipt differs")
    require(event["trial"] == 0
            and event["a"] == "1833078656608906180176350"
            and event["b"] == "163089659398503559150141"
            and event["candidate"] is None and event["graph"] is None
            and event["search"]["solver_calls"] == 1
            and event["search"]["conflicts"] == 5000
            and event["search"]["models"] == 0
            and event["search"]["exhausted"] is True
            and event["search"]["refuted"] is False
            and event["search"]["peak_variables"] == 101636
            and event["search"]["domain_clauses"] == 465738
            and event["search"]["peak_clauses"] == 714072
            and event["search"]["peak_xors"] == 4980
            and 0 < event["elapsed_process_wall_ms"] <= 1200 * 1000,
            "trial event ledger differs")
    require(budget["schema"] == "n83.mixed-orbit-supervisor-budget/v2"
            and budget["prior_total_charged_seconds"] == "12600"
            and budget["prior_import_audit_sha256"] == PRIOR_AUDIT_SHA256
            and budget["additional_authorized_seconds"] == "2400"
            and budget["build_conservative_charge_seconds"] == 600
            and budget["run_conservative_charge_seconds"] == 1800
            and budget["conservative_charge_seconds"] == 2400
            and budget["authorized_full_gate_seconds"] == 2400
            and budget["remaining_additional_seconds"] == "0"
            and budget["round_status"] == outer["status"]
            and budget["selected_best_total_runtime"] is None,
            "round charge differs")
    cleanup = read(evidence / "cleanup.json")
    require(cleanup["status"] == "PASS_NO_RETAINED_CONTAINERS"
            and [row["container_name"] for row in cleanup["checks"]]
            == [build_name, verify_name, search_name]
            and all(row["remaining_ids"] == [] for row in cleanup["checks"]),
            "cleanup receipt differs")
    for name in (build_name, verify_name, search_name):
        remaining = subprocess.check_output(
            ["docker", "ps", "-aq", "--filter", f"name=^{name}$"], text=True)
        require(not remaining.strip(), f"{name} still exists")
    return {
        "schema": "n83.mixed-orbit-approved-20261018-audit/v1",
        "status": "PASS_VALIDATED_CENSORED",
        "source_commit": SOURCE_COMMIT,
        "binary_sha256": BINARY_SHA256,
        "import_status": imported["status"],
        "search_status": outer["status"],
        "solver_calls": result["solver_calls"],
        "solver_conflicts": result["solver_conflicts"],
        "solver_models": result["solver_models"],
        "conservative_round_charge_seconds": budget["conservative_charge_seconds"],
        "historical_protocol_baseline_seconds": budget["prior_total_charged_seconds"],
        "current_cumulative_charge_seconds": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evidence", type=Path, default=HERE / "evidence")
    parser.add_argument("--small-object", type=Path, required=True)
    parser.add_argument("--envelope-object", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.evidence.resolve(strict=True), args.small_object.resolve(strict=True),
                   args.envelope_object.resolve(strict=True))
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
