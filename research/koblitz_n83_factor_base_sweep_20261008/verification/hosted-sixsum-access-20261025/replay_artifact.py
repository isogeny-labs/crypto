#!/usr/bin/env python3
"""Independently replay a retained hosted 4+2 artifact; run no solver."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import stat
import subprocess
import sys
from tempfile import TemporaryDirectory
from zipfile import ZipFile


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
PAIR = HERE.parent / "k600-gray-hash-paired-protocol-20261014"
LIVE = HERE.parent / "pilot-pair-live-s3-20261014"
sys.path.insert(0, str(PAIR))
import preflight  # noqa: E402
import receipt_audit as pinned  # noqa: E402
import audit as six_audit  # noqa: E402


RUN_ID = 40150464971
DISPATCH_SHA = "b62addef868b7954c42ce04b897f6b2384040229"
SOURCE_SHA = "cb98e85d1c1c1afdd5000bc70eea00969ee86c6c"
PROTOCOL_SHA256 = "72fb323bac60813cccd6e28198a40926043ae0c1a5baf6dfe6579990b74e4841"
STUDY = "research/koblitz_n83_factor_base_sweep_20261008"
PROTOCOL_PATH = STUDY + "/verification/hosted-sixsum-access-20261025/protocol.json"
PREFIX = "n83-hosted-sixsum/"
ARCHIVES = "n83-archives/"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def option(command: list, flag: str) -> object:
    try:
        return command[command.index(flag) + 1]
    except (ValueError, IndexError):
        return None


def git_bytes(commit: str, path: str) -> bytes:
    return subprocess.run(["git", "show", f"{commit}:{path}"], cwd=ROOT,
                          check=True, capture_output=True).stdout


def git_tree(commit: str, path: str) -> str:
    return subprocess.run(["git", "rev-parse", f"{commit}:{path}"], cwd=ROOT,
                          check=True, capture_output=True, text=True).stdout.strip()


def members(path: Path) -> tuple[dict[str, bytes], dict[str, str], int, str]:
    data = path.read_bytes()
    values: dict[str, bytes] = {}
    digests: dict[str, str] = {}
    with ZipFile(path) as archive:
        for info in archive.infolist():
            name = info.filename
            parts = PurePosixPath(name)
            mode = info.external_attr >> 16
            require(not info.is_dir() and not parts.is_absolute()
                    and ".." not in parts.parts and name not in values
                    and not stat.S_ISLNK(mode),
                    f"unsafe or duplicate ZIP member: {name}")
            values[name] = archive.read(info)
            digests[name] = sha256(values[name])
        require(archive.testzip() is None, "ZIP CRC check failed")
    return values, digests, len(data), sha256(data)


def provider(jobs_path: Path, metadata_path: Path,
             zip_size: int, zip_hash: str) -> dict:
    jobs_bytes, metadata_bytes = jobs_path.read_bytes(), metadata_path.read_bytes()
    jobs, artifact = json.loads(jobs_bytes), json.loads(metadata_bytes)
    rows = jobs.get("jobs", [])
    require(isinstance(rows, list) and jobs.get("total_count") == len(rows),
            "provider job count changed")
    selected = [row for row in rows if row.get("name") == "six-summand-access"]
    require(len(selected) == 1, "provider six-summand job missing or duplicated")
    job = selected[0]
    for key, value in (("run_id", RUN_ID), ("run_attempt", 1),
                       ("head_sha", DISPATCH_SHA), ("status", "completed"),
                       ("conclusion", "success")):
        require(job.get(key) == value, f"provider job {key} changed")
    steps = {row["name"]: row for row in job.get("steps", [])}
    for name in ("Regenerate and hash frozen public factor bases",
                 "Run separately cold bounded 4+2 Gray and hash arms",
                 "Preserve preparation, raw stages and audited outcomes"):
        require(steps.get(name, {}).get("conclusion") == "success",
                f"provider step {name} failed")
    for key, value in (("name", f"n83-hosted-sixsum-{RUN_ID}-1"),
                       ("expired", False), ("size_in_bytes", zip_size),
                       ("digest", f"sha256:{zip_hash}")):
        require(artifact.get(key) == value, f"provider artifact {key} changed")
    workflow = artifact.get("workflow_run", {})
    require(workflow.get("id") == RUN_ID and workflow.get("head_sha") == DISPATCH_SHA,
            "provider artifact workflow identity changed")
    require(type(artifact.get("id")) is int and type(job.get("id")) is int,
            "provider IDs missing")
    return {"job_id": job["id"], "job_conclusion": job["conclusion"],
            "artifact_id": artifact["id"],
            "artifact_expires_at": artifact.get("expires_at"),
            "provider_jobs_sha256": sha256(jobs_bytes),
            "artifact_metadata_sha256": sha256(metadata_bytes)}


def check_sources(config: dict) -> dict:
    require(config.get("source_commit") == SOURCE_SHA
            and config.get("protocol_commit") == DISPATCH_SHA
            and config.get("protocol_sha256") == PROTOCOL_SHA256,
            "run source or protocol identity changed")
    protocol_bytes = git_bytes(DISPATCH_SHA, PROTOCOL_PATH)
    require(sha256(protocol_bytes) == PROTOCOL_SHA256
            and sha256(HERE.joinpath("protocol.json").read_bytes()) == PROTOCOL_SHA256,
            "dispatch protocol bytes changed")
    protocol = json.loads(protocol_bytes)
    require(protocol["schema"] == "n83.hosted-sixsum-access-protocol/v2"
            and protocol["source_commit"] == SOURCE_SHA,
            "dispatch protocol source changed")
    source_hashes = config["source_sha256"]
    require(source_hashes.get("src:git-tree") == git_tree(SOURCE_SHA, "src"),
            "pinned Rust source tree changed")
    for path, expected in source_hashes.items():
        if path != "src:git-tree":
            require(sha256(git_bytes(SOURCE_SHA, path)) == expected,
                    f"pinned source blob changed: {path}")
    for path, expected in config["protocol_source_sha256"].items():
        require(sha256(git_bytes(DISPATCH_SHA, path)) == expected,
                f"dispatch protocol blob changed: {path}")
    for path in ("examples/koblitz_n83_mixed_orbit_worker.rs",
                 STUDY + "/verification/k600-gray-hash-paired-protocol-20261014/preflight.py",
                 STUDY + "/verification/k600-gray-hash-paired-protocol-20261014/receipt_audit.py",
                 STUDY + "/verification/pilot-pair-live-s3-20261014/audit.py",
                 STUDY + "/verification/paired-capacity-screen-20261013/screen.py",
                 STUDY + "/k600_mixed_orbit_supervisor.py"):
        require(sha256((ROOT / path).read_bytes()) == source_hashes[path],
                f"local independent replay dependency differs from pinned source: {path}")
    local_audit = STUDY + "/verification/hosted-sixsum-access-20261025/audit.py"
    require(sha256((ROOT / local_audit).read_bytes()) ==
            config["protocol_source_sha256"][local_audit],
            "local six-summand receipt auditor differs from dispatch source")
    return protocol


def replay(zip_path: Path, jobs_path: Path, metadata_path: Path,
           s3_dir: Path) -> dict:
    files, digests, zip_size, zip_hash = members(zip_path)
    provenance = provider(jobs_path, metadata_path, zip_size, zip_hash)

    def record(name: str) -> dict:
        require(name in files, f"ZIP member missing: {name}")
        value = json.loads(files[name])
        require(isinstance(value, dict), f"ZIP JSON object required: {name}")
        return value

    config = record(PREFIX + "config.json")
    outer = record(PREFIX + "outer.json")
    preparation = record("n83-sixsum-setup/regeneration.json")
    protocol = check_sources(config)
    require(config.get("github_run_id") == str(RUN_ID)
            and config.get("selected_best_total_runtime") is None,
            "hosted config run or selection changed")
    require(config.get("image", {}).get("reference") == protocol["container_image"]
            and config["image"].get("architecture") == "arm64"
            and config["image"].get("os") == "linux"
            and config.get("cgroup_probe", {}).get("stdout") == "4294967296\n0\n",
            "image or zero-swap cgroup identity changed")
    require(preparation == config.get("preparation")
            and sha256(files["n83-sixsum-setup/regeneration.json"])
            == config.get("preparation_receipt_sha256")
            and preparation.get("source_commit") == SOURCE_SHA
            and preparation.get("workflow_dispatch_head_sha") == DISPATCH_SHA
            and preparation.get("status") == "PASS_3_FROZEN_OBJECTS"
            and preparation.get("pilot_completed_base_count") == 54,
            "preparation receipt or source changed")
    seconds = preparation.get("hosted_preparation_wall_seconds_L0")
    require(type(seconds) in (int, float)
            and 0 <= seconds <= protocol["conservative_preparation_charge_seconds"],
            "preparation charge outside envelope")
    require(outer.get("schema") == "n83.hosted-sixsum-access-outer/v1"
            and outer.get("selected_best_total_runtime") is None
            and outer.get("frozen_input_changed") is False
            and outer.get("elapsed_preparation_seconds_L0") == seconds
            and outer.get("conservative_charge_seconds") ==
            protocol["conservative_full_charge_seconds"],
            "outer disposition or charge changed")
    require(len(outer.get("arms", [])) == 2, "hosted pair incomplete")
    archive_hashes = {}
    s3_custody = {}
    with TemporaryDirectory(prefix="n83-sixsum-replay-") as temp:
        directory = Path(temp)
        for digest in (preflight.BASE, *preflight.POOLS.values()):
            name = f"{digest}.jsonl.gz"
            member = ARCHIVES + name
            require(member in files, f"archive member missing: {name}")
            data = files[member]
            archive_hashes[name] = sha256(data)
            require(preparation["archive_sha256"].get(name) == archive_hashes[name]
                    and archive_hashes[name] in config["archive_sha256"].values(),
                    f"regenerated archive hash changed: {name}")
            s3_data = (s3_dir / f"s3-{name}").read_bytes()
            require(s3_data == data, f"fresh S3 bytes differ from artifact: {name}")
            head_bytes = (s3_dir / f"s3-head-{digest}.json").read_bytes()
            head = json.loads(head_bytes)
            require(head.get("ContentLength") == len(data)
                    and head.get("Metadata", {}).get("blake3") == digest
                    and isinstance(head.get("ETag"), str),
                    f"fresh S3 HEAD identity changed: {name}")
            s3_custody[name] = {
                "s3_uri": "s3://crypto-autoresearcher/factor-bases/icv1/etc/"
                          "koblitz_n83_factor_base_sweep_20261008/a0/objects/" + name,
                "compressed_sha256": archive_hashes[name],
                "content_length": len(data), "etag": head["ETag"],
                "head_response_sha256": sha256(head_bytes),
                "downloaded_byte_identity": True,
            }
            (directory / name).write_bytes(data)
        live = preflight.load_module("n83_sixsum_replay_live_audit", LIVE / "audit.py")
        require(live.audit(directory) == json.loads((LIVE / "audit.json").read_text()),
                "independent archive orbit replay changed")

        arms = []
        for arm, saved in zip(protocol["arms"], outer["arms"], strict=True):
            role = arm["role"]
            folder = PREFIX + role + "/"
            require(saved.get("role") == role and saved.get("replay_error") is None
                    and saved.get("total_index_calculus_runtime_ms") is None,
                    f"{role} arm identity or outcome changed")
            require(digests.get(folder + "worker") == saved.get("binary_sha256"),
                    f"{role} binary bytes changed")
            stages = saved.get("stages", [])
            require([stage.get("name") for stage in stages] ==
                    ["build", "import", "search"], f"{role} stage order changed")
            for stage in stages:
                name = stage["name"]
                require(record(folder + f"{name}.stage.json") == stage,
                        f"{role} stage receipt changed")
                state = stage.get("container_state")
                command = stage.get("command")
                require(isinstance(state, dict) and isinstance(command, list)
                        and bool(command) and command[0] == "docker"
                        and option(command, "--network") == "none"
                        and option(command, "--cpus") ==
                        ("2" if name == "build" else "1")
                        and option(command, "--memory") ==
                        ("8192m" if name == "build" else "4096m")
                        and option(command, "--memory-swap") ==
                        ("8192m" if name == "build" else "4096m"),
                        f"{role} {name} resource command changed")
                if name != "build":
                    argv = arm["worker_verify_argv"] if name == "import" else arm["worker_search_argv"]
                    require(command[-len(argv):] == argv,
                            f"{role} {name} worker arguments changed")
                require(stage.get("exit_code") == 0
                        and stage.get("cleanup_verified") is True
                        and stage.get("timed_out") is False
                        and stage.get("launch_error") is None
                        and stage.get("cleanup_error") is None
                        and state.get("ExitCode") == 0
                        and state.get("OOMKilled") is False,
                        f"{role} {name} resource or producer failure")
                log_prefix = folder + ("" if name == "build" else "run/")
                for suffix in ("stdout", "stderr"):
                    require(digests.get(log_prefix + f"{name}.{suffix}") ==
                            stage.get(f"{suffix}_sha256"),
                            f"{role} {name} {suffix} hash changed")
            pinned.audit_verify(record(folder + "run/import.stdout"), arm)
            base, residual = pinned.domains(
                directory / f"{arm['base_object_blake3']}.jsonl.gz",
                directory / f"{arm['pool_object_blake3']}.jsonl.gz",
                arm["shared_signed_orbits"])
            result = record(folder + "run/search/result.json")
            worker_status = six_audit.search(
                record(folder + "run/search/config.json"), result,
                arm, base, residual, pinned, protocol)
            require(saved["status"] == worker_status,
                    f"{role} worker status differs from independent replay")
            stats = result["search"]
            arms.append({"role": role, "status": worker_status,
                         "base_points": len(base), "residual_points": len(residual),
                         "conflicts": stats["conflicts"], "models": stats["models"],
                         "worker_wall_ms_L0": result["worker_wall_ms_L0"],
                         "search_cgroup_peak_bytes": result["memory_cgroup_peak_bytes"],
                         "binary_sha256": saved["binary_sha256"]})
    require(outer["new_n83_solver_search_executed"] is True,
            "hosted search did not execute")
    return {
        "schema": "n83.hosted-sixsum-independent-replay/v1",
        "status": "PASS_INDEPENDENT_REPLAY_BOUNDED",
        "run_id": RUN_ID, "dispatch_head_sha": DISPATCH_SHA,
        "source_commit": SOURCE_SHA, "protocol_sha256": PROTOCOL_SHA256,
        "artifact_zip_sha256": zip_hash, "artifact_size_bytes": zip_size,
        "provider": provenance, "member_sha256": digests,
        "archive_sha256": archive_hashes,
        "s3_custody": s3_custody,
        "arms": arms,
        "conservative_charge_seconds": outer["conservative_charge_seconds"],
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact-zip", type=Path, required=True)
    parser.add_argument("--provider-jobs", type=Path, required=True)
    parser.add_argument("--artifact-metadata", type=Path, required=True)
    parser.add_argument("--s3-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = replay(args.artifact_zip, args.provider_jobs,
                    args.artifact_metadata, args.s3_dir)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps({"status": result["status"], "run_id": RUN_ID,
                      "artifact_zip_sha256": result["artifact_zip_sha256"]},
                     sort_keys=True))


if __name__ == "__main__":
    main()
