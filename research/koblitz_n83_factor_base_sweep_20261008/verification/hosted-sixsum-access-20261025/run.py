#!/usr/bin/env python3
"""Run a frozen, separately cold 4+2 Gray/hash access pilot on Linux ARM."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import time
import uuid

import audit as six_audit
import protocol as six_protocol


HERE = Path(__file__).resolve().parent
STAGES = ("build", "import", "search")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")


def pinned_modules(source: Path):
    paired = source / "research/koblitz_n83_factor_base_sweep_20261008/verification/k600-gray-hash-paired-protocol-20261014"
    sys.path.insert(0, str(paired))
    import preflight  # noqa: PLC0415
    import receipt_audit  # noqa: PLC0415
    import supervisor  # noqa: PLC0415
    return preflight, receipt_audit, supervisor


def checked_protocol(source: Path, archives: Path, preflight) -> dict:
    six_protocol.require(sha256(six_protocol.OUTPUT) ==
                         hashlib.sha256((json.dumps(six_protocol.make(), indent=2,
                                                    sort_keys=True) + "\n").encode()).hexdigest(),
                         "six-summand protocol replay changed")
    protocol = json.loads(six_protocol.OUTPUT.read_text())
    six_protocol.require(protocol["source_commit"] == six_protocol.SOURCE_COMMIT,
                         "source commit changed")
    six_protocol.require(sha256(preflight.PLAN) == protocol["parent_plan_sha256"],
                         "parent plan bytes changed")
    parent = preflight.make_plan(archives)
    encoded = (json.dumps(parent, indent=2, sort_keys=True) + "\n").encode()
    six_protocol.require(preflight.PLAN.read_bytes() == encoded,
                         "parent plan replay changed")
    six_protocol.require(parent["curve_slug"] == protocol["curve_slug"] and
                         parent["public_target"] == protocol["public_target"] and
                         [arm["role"] for arm in parent["arms"]] ==
                         [arm["role"] for arm in protocol["arms"]],
                         "parent identity changed")
    for old, arm in zip(parent["arms"], protocol["arms"]):
        for key in ("role", "base_object_blake3", "pool_object_blake3",
                    "base_compressed_sha256", "pool_compressed_sha256",
                    "shared_signed_orbits", "residual_signed_orbits", "worker_verify_argv"):
            six_protocol.require(old[key] == arm[key], f"parent {key} changed")
    return protocol


def image_identity(reference: str) -> tuple[str, dict]:
    row = json.loads(subprocess.run(
        ["docker", "image", "inspect", reference, "--format", "{{json .}}"],
        capture_output=True, text=True, timeout=15, check=True).stdout)
    if row.get("Os") != "linux" or row.get("Architecture") != "arm64":
        raise ValueError("pinned image is not Linux arm64")
    image_id = row.get("Id")
    if not isinstance(image_id, str) or not image_id.startswith("sha256:"):
        raise ValueError("Docker image ID missing")
    return image_id, {"reference": reference, "image_id": image_id,
                      "repo_digests": row.get("RepoDigests", []),
                      "os": row["Os"], "architecture": row["Architecture"]}


def cgroup_probe(image_id: str) -> dict:
    command = ["docker", "run", "--rm", "--pull", "never", "--network", "none",
               "--cpus", "1", "--memory", "4096m", "--memory-swap", "4096m",
               image_id, "sh", "-c",
               "cat /sys/fs/cgroup/memory.max /sys/fs/cgroup/memory.swap.max"]
    result = subprocess.run(command, capture_output=True, text=True,
                            timeout=30, check=True)
    if result.stdout.splitlines() != ["4294967296", "0"]:
        raise ValueError("hosted Linux cgroup does not provide 4 GiB and zero swap")
    return {"command": command, "stdout": result.stdout,
            "stderr_sha256": hashlib.sha256(result.stderr.encode()).hexdigest()}


def stage_issue(receipt: dict, name: str, run_dir: Path, worker_wall: int) -> str | None:
    if not receipt.get("cleanup_verified"):
        return "PRODUCER_FAILURE_cleanup"
    if receipt.get("timed_out"):
        return "UNKNOWN_wall_cap"
    state = receipt.get("container_state")
    if isinstance(state, dict) and state.get("OOMKilled") is True:
        return "UNKNOWN_resource_cap"
    if receipt.get("exit_code") == 124 and name == "search":
        try:
            cap = json.loads((run_dir / "search/cap.json").read_text())
        except (OSError, UnicodeError, json.JSONDecodeError):
            cap = None
        if (isinstance(cap, dict) and cap.get("schema") == "n83.cross-stream-pilot-partial-cap/v1"
                and cap.get("status") == "UNKNOWN_budget"
                and cap.get("budget_seconds") == worker_wall
                and type(cap.get("elapsed_process_wall_ms")) in (int, float)
                and cap["elapsed_process_wall_ms"] >= 0):
            return "UNKNOWN_worker_wall_cap"
    if receipt.get("exit_code") in (134, 137):
        return "UNKNOWN_resource_cap"
    if (receipt.get("launch_error") or receipt.get("exit_code") != 0
            or not isinstance(state, dict) or state.get("ExitCode") != 0):
        return "PRODUCER_FAILURE_worker_exit"
    return None


def bounded_stage_command(supervisor, name: str, arm: dict, source: Path,
                          archives: Path, build: Path, run_dir: Path,
                          binary: Path, registry: Path, image_id: str,
                          token: str, build_cpus: int):
    command, output, container = supervisor.stage_command(
        name, arm, source, archives, build, run_dir, binary, registry,
        image_id, token)
    if name == "build":
        if command.count("--cpus") != 1:
            raise ValueError("pinned build CPU option changed")
        index = command.index("--cpus") + 1
        if command[index] != "4" or build_cpus != 2:
            raise ValueError("pinned build CPU envelope changed")
        command[index] = str(build_cpus)
    return command, output, container


def checked_preparation(path: Path, source_commit: str,
                        archive_sha256: dict[str, str], cap: int) -> dict:
    row = json.loads(path.read_text())
    by_name = {Path(name).name: digest for name, digest in archive_sha256.items()}
    if (row.get("schema") != "n83.hosted-regenerated-archives/v1"
            or row.get("status") != "PASS_3_FROZEN_OBJECTS"
            or row.get("source_commit") != source_commit
            or row.get("workflow_dispatch_head_sha") != os.environ.get("GITHUB_SHA")
            or row.get("archive_sha256") != by_name
            or row.get("pilot_completed_base_count") != 54):
        raise ValueError("hosted preparation receipt does not bind frozen inputs")
    seconds = row.get("hosted_preparation_wall_seconds_L0")
    if type(seconds) not in (int, float) or not 0 <= seconds <= cap:
        raise ValueError("hosted preparation exceeded reserved setup envelope")
    return row


def run(source: Path, archives: Path, output: Path, scratch: Path,
        preparation_receipt: Path | None = None) -> dict:
    source = source.resolve(strict=True)
    archives = archives.resolve(strict=True)
    output, scratch = output.resolve(), scratch.resolve()
    if (output.exists() or scratch.exists() or output.is_relative_to(source)
            or scratch.is_relative_to(source) or output.is_relative_to(HERE.parents[3])
            or scratch.is_relative_to(HERE.parents[3])):
        raise ValueError("output and scratch must be fresh paths outside both checkouts")
    if platform.system() != "Linux" or platform.machine() not in ("aarch64", "arm64"):
        raise ValueError("hosted gate requires Linux arm64")
    preflight, pinned, supervisor = pinned_modules(source)
    if preflight.ROOT.resolve() != source:
        raise ValueError("pinned module source checkout mismatch")
    protocol = checked_protocol(source, archives, preflight)
    commit = supervisor.PRIOR.clean_commit(source)
    if commit != protocol["source_commit"]:
        raise ValueError("pinned source commit changed")
    protocol_commit = supervisor.PRIOR.clean_commit(HERE.parents[3])
    sources = supervisor.source_hashes(source)
    protocol_sources = {str(path.relative_to(HERE.parents[3])): sha256(path)
                        for path in (HERE / "protocol.py", HERE / "protocol.json",
                                     HERE / "audit.py", HERE / "run.py",
                                     HERE.parents[3] / ".github/workflows/n83-hosted-paired-gate.yml")}
    archive_hashes = {str(archives / f"{digest}.jsonl.gz"):
                      sha256(archives / f"{digest}.jsonl.gz")
                      for digest in (preflight.BASE, *preflight.POOLS.values())}
    for arm in protocol["arms"]:
        for digest, expected in ((arm["base_object_blake3"], arm["base_compressed_sha256"]),
                                 (arm["pool_object_blake3"], arm["pool_compressed_sha256"])):
            if archive_hashes[str(archives / f"{digest}.jsonl.gz")] != expected:
                raise ValueError("compressed factor-base bytes changed")
    image_id, image = image_identity(protocol["container_image"])
    cgroup = cgroup_probe(image_id)
    registry = Path(os.environ.get("CARGO_HOME", str(Path.home() / ".cargo"))) / "registry"
    if not registry.is_dir():
        raise ValueError("offline Cargo registry missing")
    if preparation_receipt is None:
        raise ValueError("hosted preparation receipt required")
    preparation_receipt = preparation_receipt.resolve(strict=True)
    preparation = checked_preparation(
        preparation_receipt, commit, archive_hashes,
        protocol["conservative_preparation_charge_seconds"])
    preparation_hash = sha256(preparation_receipt)
    started = time.monotonic()
    deadline = started + protocol["conservative_pair_charge_seconds"]
    output.mkdir(parents=True)
    scratch.mkdir(parents=True)
    write_json(output / "config.json", {
        "schema": "n83.hosted-sixsum-access-config/v1",
        "source_commit": commit, "protocol_commit": protocol_commit,
        "source_sha256": sources, "protocol_source_sha256": protocol_sources,
        "protocol_sha256": sha256(six_protocol.OUTPUT),
        "archive_sha256": archive_hashes,
        "preparation_receipt_sha256": preparation_hash,
        "preparation": preparation,
        "hosted_runner": os.environ.get("RUNNER_NAME"),
        "github_run_id": os.environ.get("GITHUB_RUN_ID"),
        "image": image, "cgroup_probe": cgroup,
        "conservative_pair_charge_seconds": protocol["conservative_pair_charge_seconds"],
        "conservative_full_charge_seconds": protocol["conservative_full_charge_seconds"],
        "selected_best_total_runtime": None,
    })

    def frozen_changed() -> bool:
        try:
            return (supervisor.PRIOR.clean_commit(source) != commit
                    or supervisor.PRIOR.clean_commit(HERE.parents[3]) != protocol_commit
                    or supervisor.source_hashes(source) != sources
                    or any(sha256(HERE.parents[3] / path) != digest
                           for path, digest in protocol_sources.items())
                    or any(sha256(Path(path)) != digest for path, digest in archive_hashes.items()))
        except (OSError, ValueError):
            return True

    arms = []
    for arm in protocol["arms"]:
        arm_out = output / arm["role"]
        run_dir = arm_out / "run"
        build = scratch / arm["role"] / "build"
        (build / "target").mkdir(parents=True)
        run_dir.mkdir(parents=True)
        binary = arm_out / "worker"
        token = uuid.uuid4().hex[:16]
        stages = []
        status, replay_error = "UNKNOWN_before_stage", None
        for name in STAGES:
            if time.monotonic() + protocol["stage_wall_caps_seconds"][name] + protocol["conservative_stage_charges_seconds"]["cleanup"] > deadline:
                status = "UNKNOWN_budget_before_stage"
                break
            if frozen_changed():
                status = "PRODUCER_FAILURE_frozen_input_changed"
                break
            command, stage_output, container = bounded_stage_command(
                supervisor,
                name, arm, source, archives, build, run_dir, binary, registry,
                image_id, token, protocol["build_cpus"])
            receipt = supervisor.PRIOR.stage(
                name=name, command=command,
                wall=protocol["stage_wall_caps_seconds"][name],
                output=stage_output, container_name=container)
            stages.append(receipt)
            if name == "build":
                for suffix in ("stdout", "stderr"):
                    log = build / f"build.{suffix}"
                    if log.is_file():
                        shutil.copyfile(log, arm_out / log.name)
            write_json(arm_out / f"{name}.stage.json", receipt)
            issue = stage_issue(receipt, name, run_dir, protocol["worker_wall_seconds"])
            if issue:
                status = issue
                break
            try:
                if name == "build":
                    artifact = build / "target/release/examples/koblitz_n83_mixed_orbit_worker"
                    if not supervisor.PRIOR.is_linux_elf(artifact):
                        raise ValueError("built worker is not Linux ELF")
                    shutil.copyfile(artifact, binary)
                    binary.chmod(0o755)
                    status = "PASS_build_only"
                elif name == "import":
                    pinned.audit_verify(json.loads((run_dir / "import.stdout").read_text()), arm)
                    pinned.domains(archives / f"{arm['base_object_blake3']}.jsonl.gz",
                                   archives / f"{arm['pool_object_blake3']}.jsonl.gz",
                                   arm["shared_signed_orbits"])
                    status = "PASS_import_only"
                else:
                    base, residual = pinned.domains(
                        archives / f"{arm['base_object_blake3']}.jsonl.gz",
                        archives / f"{arm['pool_object_blake3']}.jsonl.gz",
                        arm["shared_signed_orbits"])
                    status = six_audit.search(
                        json.loads((run_dir / "search/config.json").read_text()),
                        json.loads((run_dir / "search/result.json").read_text()),
                        arm, base, residual, pinned, protocol)
            except (OSError, ValueError, TypeError, KeyError, json.JSONDecodeError) as error:
                status = "PRODUCER_FAILURE_receipt_replay"
                replay_error = f"{type(error).__name__}: {error}"
                break
        outer = {
            "schema": "n83.hosted-sixsum-access-arm/v1", "role": arm["role"],
            "status": status, "stages": stages,
            "binary_sha256": sha256(binary) if binary.is_file() else None,
            "replay_error": replay_error,
            "conservative_charge_seconds": sum(protocol["conservative_stage_charges_seconds"].values()),
            "graph_rows": None, "matrix_rank": None,
            "total_index_calculus_runtime_ms": None,
        }
        write_json(arm_out / "outer.json", outer)
        arms.append(outer)
        if status.startswith("PRODUCER_FAILURE"):
            break
    elapsed = time.monotonic() - started
    changed = frozen_changed()
    status = ("PRODUCER_FAILURE_frozen_input_changed" if changed else
              "PRODUCER_FAILURE_budget_overshoot" if elapsed > protocol["conservative_pair_charge_seconds"] else
              "PRODUCER_FAILURE_arm" if any(a["status"].startswith("PRODUCER_FAILURE") for a in arms) else
              "BOUNDED_UNKNOWN_incomplete_pair" if len(arms) != 2 or any(
                  a["status"] in ("UNKNOWN_before_stage", "UNKNOWN_budget_before_stage") for a in arms) else
              "BOUNDED_PARTIAL_sixsum_access_first_trial")
    result = {
        "schema": "n83.hosted-sixsum-access-outer/v1", "status": status,
        "arms": arms, "elapsed_supervisor_seconds_L0": elapsed,
        "elapsed_preparation_seconds_L0": preparation["hosted_preparation_wall_seconds_L0"],
        "elapsed_combined_seconds_L0": preparation["hosted_preparation_wall_seconds_L0"] + elapsed,
        "conservative_charge_seconds": protocol["conservative_preparation_charge_seconds"]
        + len(arms) * sum(protocol["conservative_stage_charges_seconds"].values()),
        "frozen_input_changed": changed,
        "new_n83_solver_search_executed": any(
            any(stage["name"] == "search" and stage["launch_error"] is None
                and stage["container_state"] is not None for stage in arm["stages"])
            for arm in arms),
        "selected_best_total_runtime": None,
    }
    write_json(output / "outer.json", result)
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--archives", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--scratch", type=Path, required=True)
    parser.add_argument("--preparation-receipt", type=Path)
    args = parser.parse_args()
    result = run(args.source, args.archives, args.output, args.scratch,
                 args.preparation_receipt)
    print(json.dumps(result, sort_keys=True))
    return 1 if result["status"].startswith("PRODUCER_FAILURE") else 0


if __name__ == "__main__":
    raise SystemExit(main())
