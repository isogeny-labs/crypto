#!/usr/bin/env python3
"""Replay the retained primary-N83 Gray/hash hosted artifact, without a solver call."""

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
STUDY = HERE.parents[1]
ROOT = STUDY.parents[1]
PAIR = HERE.parent / "k600-gray-hash-paired-protocol-20261014"
LIVE = HERE.parent / "pilot-pair-live-s3-20261014"
sys.path.insert(0, str(PAIR))
import preflight  # noqa: E402
import receipt_audit  # noqa: E402


RUN_ID = 38983406418
ARTIFACT_ID = 12083608289
JOB_ID = 117249241144
HEAD_SHA = "8dd40601e80ae8e632229c6b4b633d4f21925513"
SOURCE_SHA = "cb98e85d1c1c1afdd5000bc70eea00969ee86c6c"
ZIP_SHA256 = "fcc3066d3168a902becb3f746d58b905f8ad3d953041c3765fb05f767ad76915"
PLAN_SHA256 = "fbd3fb943bc0f181797d106c41ee9961b8d70768b5c65a5584002e3c1b5e488f"
PREFIX = "n83-hosted-pair/"
ARCHIVES = "n83-archives/"
PINNED_AUDIT_PATHS = (
    "research/koblitz_n83_factor_base_sweep_20261008/verification/k600-gray-hash-paired-protocol-20261014/preflight.py",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/k600-gray-hash-paired-protocol-20261014/receipt_audit.py",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/pilot-pair-live-s3-20261014/audit.py",
    "research/koblitz_n83_factor_base_sweep_20261008/k600_mixed_orbit_supervisor.py",
    "research/koblitz_n83_factor_base_sweep_20261008/verification/paired-capacity-screen-20261013/screen.py",
)


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_bytes(commit: str, path: str) -> bytes:
    return subprocess.run(["git", "show", f"{commit}:{path}"], cwd=ROOT,
                          check=True, capture_output=True).stdout


def git_tree(commit: str, path: str) -> str:
    return subprocess.run(["git", "rev-parse", f"{commit}:{path}"], cwd=ROOT,
                          check=True, capture_output=True, text=True).stdout.strip()


def provider(provider_jobs: bytes, metadata: bytes, zip_size: int) -> dict:
    jobs = json.loads(provider_jobs)
    require(jobs.get("total_count") == 1 and len(jobs.get("jobs", [])) == 1,
            "provider job count changed")
    job = jobs["jobs"][0]
    for key, expected in (("id", JOB_ID), ("run_id", RUN_ID),
                          ("run_attempt", 1), ("head_sha", HEAD_SHA),
                          ("name", "paired-first-trial"), ("status", "completed"),
                          ("conclusion", "success")):
        require(job.get(key) == expected, f"provider job {key} changed")
    steps = {row["name"]: row for row in job.get("steps", [])}
    for name in ("Regenerate and hash the three frozen public factor bases",
                 "Run separately cold bounded Gray and hash arms",
                 "Preserve setup, raw stages and audited outcomes"):
        require(steps.get(name, {}).get("conclusion") == "success", f"provider step {name} failed")
    artifact = json.loads(metadata)
    for key, expected in (("id", ARTIFACT_ID),
                          ("name", f"n83-hosted-pair-{RUN_ID}-1"),
                          ("expired", False), ("size_in_bytes", zip_size)):
        require(artifact.get(key) == expected, f"provider artifact {key} changed")
    workflow = artifact.get("workflow_run", {})
    require(workflow.get("id") == RUN_ID and workflow.get("head_sha") == HEAD_SHA,
            "artifact workflow identity changed")
    return {"job_id": JOB_ID, "job_conclusion": job["conclusion"],
            "artifact_id": ARTIFACT_ID, "artifact_expires_at": artifact.get("expires_at"),
            "provider_jobs_sha256": sha256(provider_jobs),
            "artifact_metadata_sha256": sha256(metadata)}


def member_bytes(archive: ZipFile) -> tuple[dict[str, bytes], dict[str, str]]:
    values: dict[str, bytes] = {}
    digests: dict[str, str] = {}
    for info in archive.infolist():
        name = info.filename
        path = PurePosixPath(name)
        mode = info.external_attr >> 16
        require(not info.is_dir() and not path.is_absolute() and ".." not in path.parts
                and name not in values and not stat.S_ISLNK(mode),
                f"unsafe or duplicate ZIP member: {name}")
        values[name] = archive.read(info)
        digests[name] = sha256(values[name])
    require(archive.testzip() is None, "ZIP CRC check failed")
    return values, digests


def check_sources(config: dict) -> None:
    require(config.get("source_commit") == SOURCE_SHA
            and config.get("plan_sha256") == PLAN_SHA256,
            "frozen source or plan identity changed")
    require(sha256((PAIR / "plan.json").read_bytes()) == PLAN_SHA256,
            "local paired plan changed")
    sources = {**config["source_sha256"], **config["hosted_source_sha256"]}
    require(sources["src:git-tree"] == git_tree(SOURCE_SHA, "src"),
            "pinned Rust source tree changed")
    for path, expected in sources.items():
        if path == "src:git-tree":
            continue
        require(sha256(git_bytes(SOURCE_SHA, path)) == expected,
                f"pinned source blob changed: {path}")
    for path in PINNED_AUDIT_PATHS:
        require(sha256((ROOT / path).read_bytes()) == sources[path],
                f"local replay implementation differs from pinned source: {path}")


def s3_custody(directory: Path, archive_bytes: dict[str, bytes]) -> dict:
    objects = {}
    for digest, expected in archive_bytes.items():
        name = f"{digest}.jsonl.gz"
        data = (directory / f"s3-{name}").read_bytes()
        head = record_from_file(HERE / f"s3-head-{digest}.json")
        get = record_from_file(HERE / f"s3-get-{digest}.json")
        require(data == expected, f"S3 downloaded bytes differ from hosted artifact: {name}")
        require(head.get("ContentLength") == get.get("ContentLength") == len(data)
                and head.get("ETag") == get.get("ETag")
                and get.get("Metadata", {}).get("blake3") == digest,
                f"S3 HEAD/GET identity changed: {name}")
        objects[name] = {
            "s3_uri": "s3://crypto-autoresearcher/factor-bases/icv1/etc/"
                      "koblitz_n83_factor_base_sweep_20261008/a0/objects/" + name,
            "compressed_sha256": sha256(data), "content_length": len(data),
            "etag": head["ETag"], "last_modified": head["LastModified"],
            "head_response_sha256": sha256((HERE / f"s3-head-{digest}.json").read_bytes()),
            "get_response_sha256": sha256((HERE / f"s3-get-{digest}.json").read_bytes()),
            "downloaded_byte_identity": True,
        }
    return {"status": "PASS_FRESH_S3_DOWNLOADED_BYTE_IDENTITY", "objects": objects}


def run(zip_path: Path, jobs_path: Path, metadata_path: Path, s3_dir: Path) -> dict:
    zip_data = zip_path.read_bytes()
    require(sha256(zip_data) == ZIP_SHA256, "provider ZIP SHA-256 changed")
    provenance = provider(jobs_path.read_bytes(), metadata_path.read_bytes(), len(zip_data))
    with ZipFile(zip_path) as archive:
        files, digests = member_bytes(archive)

    def record(name: str) -> dict:
        require(name in files, f"ZIP member missing: {name}")
        value = json.loads(files[name])
        require(isinstance(value, dict), f"ZIP JSON object required: {name}")
        return value

    config = record(PREFIX + "config.json")
    outer = record(PREFIX + "outer.json")
    regeneration = record("n83-setup/regeneration.json")
    check_sources(config)
    require(regeneration.get("source_commit") == SOURCE_SHA
            and regeneration.get("workflow_dispatch_head_sha") == HEAD_SHA
            and regeneration.get("status") == "PASS_3_FROZEN_OBJECTS",
            "archive regeneration identity changed")
    require(config.get("github_run_id") == str(RUN_ID)
            and config.get("selected_best_total_runtime") is None,
            "hosted config run or selection changed")
    require(outer.get("status") == "BOUNDED_PARTIAL_hosted_first_trial"
            and outer.get("selected_best_total_runtime") is None
            and outer.get("conservative_charge_seconds") == 4800
            and outer.get("conservative_charge_including_preparation_seconds_L0") == 5014
            and outer.get("new_n83_solver_search_executed") is True,
            "hosted outer status or charge changed")
    plan = record_from_file(PAIR / "plan.json")
    require([arm["role"] for arm in outer["arms"]] ==
            [arm["role"] for arm in plan["arms"]], "hosted arm order changed")

    import blake3  # noqa: F401 - required by both independent archive and search replay
    live = preflight.load_module("n83_hosted_replay_live_audit", LIVE / "audit.py")
    archive_hashes = {}
    archive_bytes = {}
    with TemporaryDirectory(prefix="n83-hosted-pair-audit-") as temporary:
        archive_dir = Path(temporary)
        for digest in (preflight.BASE, *preflight.POOLS.values()):
            name = f"{digest}.jsonl.gz"
            member = ARCHIVES + name
            require(member in files, f"archive missing: {name}")
            data = files[member]
            archive_bytes[digest] = data
            archive_hashes[name] = sha256(data)
            require(regeneration["archive_sha256"].get(name) == archive_hashes[name],
                    f"regenerated archive hash changed: {name}")
            require(archive_hashes[name] in config["input_sha256"].values(),
                    f"worker input archive hash changed: {name}")
            (archive_dir / name).write_bytes(data)
        require(live.audit(archive_dir) == record_from_file(LIVE / "audit.json"),
                "independent archive and orbit audit changed")

        arm_receipts = []
        for arm, saved in zip(plan["arms"], outer["arms"], strict=True):
            role = arm["role"]
            folder = PREFIX + role + "/"
            require(saved["status"] == "UNKNOWN_search_cap"
                    and saved["replay_error"] is None
                    and saved["total_index_calculus_runtime_ms"] is None,
                    f"{role} disposition changed")
            require(digests[folder + "worker"] == saved["binary_sha256"],
                    f"{role} binary hash changed")
            require([stage["name"] for stage in saved["stages"]] ==
                    ["build", "import", "search"], f"{role} stage order changed")
            for stage in saved["stages"]:
                name = stage["name"]
                require(stage["exit_code"] == 0 and stage["cleanup_verified"] is True
                        and stage["timed_out"] is False
                        and stage["container_state"]["ExitCode"] == 0
                        and stage["container_state"]["OOMKilled"] is False,
                        f"{role} {name} resource or producer failure")
                prefix = folder + ("" if name == "build" else "run/")
                for suffix in ("stdout", "stderr"):
                    member = prefix + name + "." + suffix
                    require(digests[member] == stage[suffix + "_sha256"],
                            f"{role} {name} {suffix} hash changed")
            receipt_audit.audit_verify(record(folder + "run/import.stdout"), arm)
            base, residual = receipt_audit.domains(
                archive_dir / f"{arm['base_object_blake3']}.jsonl.gz",
                archive_dir / f"{arm['pool_object_blake3']}.jsonl.gz",
                arm["shared_signed_orbits"])
            result = record(folder + "run/search/result.json")
            status = receipt_audit.audit_search(
                record(folder + "run/search/config.json"), result, arm, base, residual)
            stats = result["search"]
            require(status == "UNKNOWN_search_cap" and stats["conflicts"] == 5000
                    and stats["models"] == 0 and result["candidate"] is None,
                    f"{role} bounded search changed")
            arm_receipts.append({"role": role, "status": status,
                                 "base_points": len(base), "residual_points": len(residual),
                                 "conflicts": stats["conflicts"], "models": stats["models"],
                                 "worker_wall_ms_L0": result["worker_wall_ms_L0"],
                                 "search_cgroup_peak_bytes": result["memory_cgroup_peak_bytes"],
                                 "binary_sha256": saved["binary_sha256"]})

    return {"schema": "n83.hosted-pair-independent-replay/v1",
            "status": "PASS_INDEPENDENT_REPLAY_CENSORED",
            "run_id": RUN_ID, "run_attempt": 1, "dispatch_head_sha": HEAD_SHA,
            "source_commit": SOURCE_SHA, "plan_sha256": PLAN_SHA256,
            "artifact_zip_sha256": ZIP_SHA256, "artifact_size_bytes": len(zip_data),
            "provider": provenance, "member_sha256": digests,
            "archive_sha256": archive_hashes,
            "s3_custody": s3_custody(s3_dir, archive_bytes), "arms": arm_receipts,
            "conservative_charge_seconds": 4800,
            "conservative_charge_including_preparation_seconds_L0": 5014,
            "selected_best_total_runtime": None}


def record_from_file(path: Path) -> dict:
    return json.loads(path.read_text())


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact-zip", type=Path, required=True)
    parser.add_argument("--provider-jobs", type=Path, required=True)
    parser.add_argument("--artifact-metadata", type=Path, required=True)
    parser.add_argument("--s3-dir", type=Path, required=True,
                        help="directory holding fresh s3-<BLAKE3>.jsonl.gz downloads")
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    result = run(args.artifact_zip, args.provider_jobs, args.artifact_metadata,
                 args.s3_dir)
    encoded = (json.dumps(result, indent=2, sort_keys=True) + "\n").encode()
    destination = HERE / "receipt.json"
    if args.write:
        with destination.open("xb") as output:
            output.write(encoded)
    else:
        require(destination.read_bytes() == encoded, "saved replay receipt changed")
    print(json.dumps({"status": result["status"], "arms": result["arms"],
                      "selected_best_total_runtime": None}, sort_keys=True))


if __name__ == "__main__":
    main()
