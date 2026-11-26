#!/usr/bin/env python3
"""Run the frozen RR9 full-eight cases and seal launch evidence.

This launcher is deliberately not a grader.  It preserves the producer's raw
bytes, resource observations, and exit status for cases 0, 1, and 2 in the
frozen order.  A separate implementation must decide algebraic correctness.
"""

from __future__ import annotations

import argparse
import ctypes
import datetime as dt
import fcntl
import hashlib
import json
import math
import os
import platform
import resource
import stat
import signal
import subprocess
import sys
import threading
import time
import uuid
from pathlib import Path
from typing import Any


SCHEMA = "prime31.rr9.full8-f4-suite-receipt.v1"
EXPECTED_PRODUCER_SCHEMA = "prime31.rr9.full8-f4-screen.v2"
EXPECTED_INPUT_SCHEMA = "prime31.rr9.full8-input-system.v1"
CASE_ORDER = (0, 1, 2)
OUTER_CAP_SECONDS = 210
OUTER_CAP_SCOPE = (
    "producer_commands_plus_process_group_drain_raw_endpoint_fsync_snapshot_"
    "hashing_strict_interface_and_resource_validation"
)
RAW_PRODUCER_JSON_MAX_BYTES = 134_217_728
STDERR_MAX_BYTES = 16_777_216
TIME_V_MAX_BYTES = 1_048_576
CUSTODY_MANIFEST_MAX_BYTES = 16_777_216
PROCESS_CONTAINMENT_GUARANTEE = "dedicated-job-cgroup"
PR_SET_CHILD_SUBREAPER = 36
PR_GET_CHILD_SUBREAPER = 37
F_ADD_SEALS = 1033
F_GET_SEALS = 1034
F_SEAL_SEAL = 0x0001
F_SEAL_SHRINK = 0x0002
F_SEAL_GROW = 0x0004
F_SEAL_WRITE = 0x0008
REQUIRED_MEMFD_SEALS = F_SEAL_SEAL | F_SEAL_SHRINK | F_SEAL_GROW | F_SEAL_WRITE
HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parents[2]
NATIVE_CRATE = HERE.parent / "native"
DEFAULT_PROTOCOL = HERE / "RR9_FULL8_F4_PROTOCOL_20261122.md"
DEFAULT_SOURCE = NATIVE_CRATE / "examples" / "prime31_rr9_full8_f4.rs"
DEFAULT_TIME = Path("/usr/bin/time")
DEFAULT_CARGO_LOCK = NATIVE_CRATE / "Cargo.lock"
DEFAULT_CARGO_MANIFEST = NATIVE_CRATE / "Cargo.toml"
FROZEN_CHILD_ENVIRONMENT = {
    "LC_ALL": "C",
    "LANG": "C",
    "TZ": "UTC",
    "OMP_NUM_THREADS": "1",
    "RAYON_NUM_THREADS": "1",
}
REQUIRED_TIME_V_FIELDS = (
    "user_seconds",
    "system_seconds",
    "elapsed_wall",
    "elapsed_wall_seconds",
    "maximum_resident_set_kib",
    "time_exit_status",
)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def capture_raw_artifact(
    path: Path, *, buffer_limit: int | None
) -> tuple[dict[str, Any], bytes | None, list[str]]:
    """Hash and optionally buffer one closed regular file from the same FD."""

    flags = (
        os.O_RDONLY
        | getattr(os, "O_NOFOLLOW", 0)
        | getattr(os, "O_NONBLOCK", 0)
    )
    try:
        descriptor = os.open(path, flags)
    except FileNotFoundError:
        return {"path": path.name, "present": False}, None, []
    except OSError as error:
        return (
            {"path": path.name, "present": False},
            None,
            [f"raw_artifact_open_failed:{type(error).__name__}"],
        )
    digest = hashlib.sha256()
    chunks: list[bytes] = []
    total = 0
    oversized = False
    reached_eof = False
    errors: list[str] = []
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode):
            return (
                {"path": path.name, "present": False},
                None,
                ["raw_artifact_not_regular"],
            )
        while True:
            if buffer_limit is None:
                read_size = 1024 * 1024
            else:
                remaining_probe = buffer_limit + 1 - total
                if remaining_probe <= 0:
                    break
                read_size = min(1024 * 1024, remaining_probe)
            chunk = os.read(descriptor, read_size)
            if not chunk:
                reached_eof = True
                break
            digest.update(chunk)
            total += len(chunk)
            if buffer_limit is not None and not oversized:
                if total <= buffer_limit:
                    chunks.append(chunk)
                else:
                    oversized = True
                    chunks.clear()
        after = os.fstat(descriptor)
    finally:
        os.close(descriptor)
    metadata_changed = (
        before.st_size != after.st_size
        or before.st_mtime_ns != after.st_mtime_ns
        or before.st_ctime_ns != after.st_ctime_ns
    )
    hash_complete = reached_eof and total == after.st_size and not metadata_changed
    if metadata_changed or (not oversized and not hash_complete):
        errors.append("raw_artifact_changed_during_capture")
    if buffer_limit is not None and (
        oversized or after.st_size > buffer_limit
    ):
        oversized = True
        errors.append(f"raw_artifact_exceeds_buffer_limit:{buffer_limit}")
    captured = {
        "path": path.name,
        "present": True,
        "bytes": after.st_size,
        "sha256": digest.hexdigest() if hash_complete else None,
        "hash_complete": hash_complete,
        "observed_prefix_bytes": total,
        "observed_prefix_sha256": digest.hexdigest(),
        "observed_bytes_kind": "exact" if hash_complete else "lower_bound",
        "device": before.st_dev,
        "inode": before.st_ino,
        "mode": stat.S_IMODE(before.st_mode),
    }
    buffered = (
        b"".join(chunks)
        if buffer_limit is not None and not oversized and hash_complete
        else None
    )
    return captured, buffered, errors


def bound_file(path: Path) -> dict[str, Any]:
    resolved = path.resolve(strict=True)
    if not resolved.is_file():
        raise ValueError(f"bound path is not a regular file: {resolved}")
    return {
        "path": str(resolved),
        "bytes": resolved.stat().st_size,
        "sha256": sha256_file(resolved),
    }


def stage_file(
    source: Path,
    destination: Path,
    expected: dict[str, Any],
    *,
    executable: bool,
) -> dict[str, Any]:
    """Copy one already-bound input into the private custody directory.

    The digest is computed from the same byte stream that is written.  A
    mismatch with the preflight binding is fatal, so replacing or rewriting a
    source between preflight and custody cannot silently change what runs.
    """

    resolved = source.resolve(strict=True)
    digest = hashlib.sha256()
    byte_count = 0
    with resolved.open("rb") as input_handle:
        before = os.fstat(input_handle.fileno())
        if not stat.S_ISREG(before.st_mode):
            raise ValueError(f"custody source is not a regular file: {resolved}")
        with destination.open("xb") as output_handle:
            while True:
                chunk = input_handle.read(1024 * 1024)
                if not chunk:
                    break
                digest.update(chunk)
                byte_count += len(chunk)
                output_handle.write(chunk)
            output_handle.flush()
            os.fsync(output_handle.fileno())
        after = os.fstat(input_handle.fileno())
    if byte_count != expected["bytes"] or digest.hexdigest() != expected["sha256"]:
        raise ValueError(f"bound input changed before custody copy: {resolved}")
    if before.st_size != after.st_size or before.st_mtime_ns != after.st_mtime_ns:
        raise ValueError(f"custody source changed while it was copied: {resolved}")
    destination.chmod(0o500 if executable else 0o400)
    staged_stat = destination.stat()
    staged = {
        "path": str(destination.resolve(strict=True)),
        "present": True,
        "bytes": byte_count,
        "sha256": digest.hexdigest(),
        "hash_complete": True,
        "device": staged_stat.st_dev,
        "inode": staged_stat.st_ino,
        "mode": stat.S_IMODE(staged_stat.st_mode),
    }
    return {
        "source_path": str(resolved),
        "staged": staged,
        "executable": executable,
    }


def stage_inputs(
    custody_dir: Path,
    paths: dict[str, Path],
    bindings: dict[str, dict[str, Any]],
) -> dict[str, dict[str, Any]]:
    custody_dir.mkdir(mode=0o700, exist_ok=False)
    names = {
        "binary": "producer",
        "protocol": "protocol.md",
        "source": "producer-source.rs",
        "verifier_source": "independent-verifier.py",
        "verifier_executable": "verifier-runtime",
        "time_binary": "gnu-time",
        "cargo_lock": "Cargo.lock",
        "cargo_manifest": "Cargo.toml",
    }
    result: dict[str, dict[str, Any]] = {}
    for key, filename in names.items():
        result[key] = stage_file(
            paths[key],
            custody_dir / filename,
            bindings[key],
            executable=key in {"binary", "verifier_executable", "time_binary"},
        )
    return result


def _write_all(descriptor: int, data: bytes) -> None:
    offset = 0
    while offset < len(data):
        written = os.write(descriptor, data[offset:])
        if written <= 0:
            raise OSError("short write to memfd")
        offset += written


def _hash_fd(descriptor: int, byte_count: int) -> str:
    digest = hashlib.sha256()
    offset = 0
    while offset < byte_count:
        chunk = os.pread(descriptor, min(1024 * 1024, byte_count - offset), offset)
        if not chunk:
            raise OSError("short read from memfd")
        digest.update(chunk)
        offset += len(chunk)
    return digest.hexdigest()


def create_sealed_memfd(
    name: str, custody_entry: dict[str, Any]
) -> tuple[int, dict[str, Any]]:
    if not hasattr(os, "memfd_create") or not hasattr(os, "MFD_ALLOW_SEALING"):
        raise RuntimeError("Linux memfd sealing support is unavailable")
    source = custody_entry["staged"]
    source_path = Path(source["path"])
    source_flags = (
        os.O_RDONLY
        | getattr(os, "O_NOFOLLOW", 0)
        | getattr(os, "O_NONBLOCK", 0)
    )
    source_fd = os.open(source_path, source_flags)
    memfd = -1
    try:
        source_before = os.fstat(source_fd)
        if not stat.S_ISREG(source_before.st_mode):
            raise ValueError(f"memfd source is not regular: {source_path}")
        memfd = os.memfd_create(
            f"rr9-{name}",
            os.MFD_ALLOW_SEALING,
        )
        digest = hashlib.sha256()
        copied = 0
        while True:
            chunk = os.read(source_fd, 1024 * 1024)
            if not chunk:
                break
            digest.update(chunk)
            copied += len(chunk)
            _write_all(memfd, chunk)
        source_after = os.fstat(source_fd)
        if (
            source_before.st_dev != source_after.st_dev
            or source_before.st_ino != source_after.st_ino
            or source_before.st_size != source_after.st_size
            or source_before.st_mtime_ns != source_after.st_mtime_ns
            or source_before.st_ctime_ns != source_after.st_ctime_ns
        ):
            raise ValueError(f"memfd source changed during copy: {source_path}")
        if copied != source["bytes"] or digest.hexdigest() != source["sha256"]:
            raise ValueError(f"memfd source binding mismatch: {source_path}")
        os.fchmod(memfd, 0o500)
        os.fsync(memfd)
        fcntl.fcntl(memfd, F_ADD_SEALS, REQUIRED_MEMFD_SEALS)
        applied_seals = fcntl.fcntl(memfd, F_GET_SEALS)
        if applied_seals != REQUIRED_MEMFD_SEALS:
            raise RuntimeError(
                f"incomplete memfd seals for {name}: {applied_seals:#x}"
            )
        observed = os.fstat(memfd)
        memfd_hash = _hash_fd(memfd, observed.st_size)
        if observed.st_size != copied or memfd_hash != digest.hexdigest():
            raise ValueError(f"sealed memfd bytes mismatch: {name}")
        receipt = {
            "name": name,
            "fd": memfd,
            "source_binding": dict(source),
            "device": observed.st_dev,
            "inode": observed.st_ino,
            "bytes": observed.st_size,
            "sha256": memfd_hash,
            "mode": stat.S_IMODE(observed.st_mode),
            "requested_seal_mask": REQUIRED_MEMFD_SEALS,
            "applied_seal_mask": applied_seals,
            "seal_names": [
                "F_SEAL_WRITE",
                "F_SEAL_GROW",
                "F_SEAL_SHRINK",
                "F_SEAL_SEAL",
            ],
            "proc_self_path": f"/proc/self/fd/{memfd}",
            "wrapper_owned_path": f"/proc/{os.getpid()}/fd/{memfd}",
            "held_open_through_seal": True,
        }
        return memfd, receipt
    except Exception:
        if memfd >= 0:
            os.close(memfd)
        raise
    finally:
        os.close(source_fd)


def create_execution_memfds(
    custody: dict[str, dict[str, Any]],
) -> tuple[dict[str, int], dict[str, dict[str, Any]]]:
    handles: dict[str, int] = {}
    receipts: dict[str, dict[str, Any]] = {}
    try:
        for key, name in (("binary", "producer"), ("time_binary", "gnu-time")):
            descriptor, receipt = create_sealed_memfd(name, custody[key])
            handles[key] = descriptor
            receipts[key] = receipt
    except Exception:
        for descriptor in handles.values():
            os.close(descriptor)
        raise
    return handles, receipts


def sealed_memfd_errors(
    memfd_receipts: dict[str, dict[str, Any]],
) -> list[str]:
    errors: list[str] = []
    for key in ("binary", "time_binary"):
        expected = memfd_receipts.get(key)
        if not isinstance(expected, dict):
            errors.append(f"{key}_memfd_receipt_missing")
            continue
        descriptor = expected.get("fd")
        if not isinstance(descriptor, int):
            errors.append(f"{key}_memfd_descriptor_missing")
            continue
        try:
            observed = os.fstat(descriptor)
            seals = fcntl.fcntl(descriptor, F_GET_SEALS)
        except OSError as error:
            errors.append(f"{key}_memfd_unavailable:{type(error).__name__}")
            continue
        if observed.st_dev != expected.get("device"):
            errors.append(f"{key}_memfd_device_mismatch")
        if observed.st_ino != expected.get("inode"):
            errors.append(f"{key}_memfd_inode_mismatch")
        if observed.st_size != expected.get("bytes"):
            errors.append(f"{key}_memfd_size_mismatch")
            continue
        if stat.S_IMODE(observed.st_mode) != expected.get("mode"):
            errors.append(f"{key}_memfd_mode_mismatch")
        if seals != REQUIRED_MEMFD_SEALS or seals != expected.get(
            "applied_seal_mask"
        ):
            errors.append(f"{key}_memfd_seal_mask_mismatch")
        try:
            observed_hash = _hash_fd(descriptor, observed.st_size)
        except OSError as error:
            errors.append(f"{key}_memfd_hash_failed:{type(error).__name__}")
            continue
        if observed_hash != expected.get("sha256"):
            errors.append(f"{key}_memfd_sha256_mismatch")
    return errors


def memfd_proc_path_errors(
    memfd_receipts: dict[str, dict[str, Any]],
) -> list[str]:
    errors: list[str] = []
    for key, expected in memfd_receipts.items():
        for path_field in ("proc_self_path", "wrapper_owned_path"):
            path_value = expected.get(path_field)
            if not isinstance(path_value, str):
                errors.append(f"{key}_{path_field}_missing")
                continue
            try:
                descriptor = os.open(path_value, os.O_RDONLY)
            except OSError as error:
                errors.append(
                    f"{key}_{path_field}_unavailable:{type(error).__name__}"
                )
                continue
            try:
                observed = os.fstat(descriptor)
            finally:
                os.close(descriptor)
            if (
                observed.st_dev != expected.get("device")
                or observed.st_ino != expected.get("inode")
                or observed.st_size != expected.get("bytes")
            ):
                errors.append(f"{key}_{path_field}_binding_mismatch")
    return errors


def probe_memfd_child_access(
    memfd_receipts: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    """Prove a child can execute/open the sealed-FD paths used by the suite.

    The probe executes a wrapper-owned sealed script memfd, rather than the
    scientific producer, then opens and hashes the exact producer and GNU-time
    memfds from that child.  This exercises the relevant procfs/Yama boundary
    before the producer-suite clock begins without consuming a scientific case.
    """

    if not hasattr(os, "memfd_create") or not hasattr(os, "MFD_ALLOW_SEALING"):
        raise RuntimeError("Linux memfd sealing support is unavailable")
    python_path = str(Path(sys.executable).resolve(strict=True))
    probe_source = (
        f"#!{python_path}\n"
        "import hashlib,json,os,sys\n"
        "if len(sys.argv)==2:\n"
        " pid=os.fork()\n"
        " if pid:\n"
        "  _,status=os.waitpid(pid,0)\n"
        "  raise SystemExit(os.waitstatus_to_exitcode(status))\n"
        " os.execve(sys.argv[0],[sys.argv[0],sys.argv[1],'grandchild'],dict(os.environ))\n"
        "expected=json.loads(sys.argv[1])\n"
        "observed={}\n"
        "for name,item in expected.items():\n"
        " fd=os.open(item['path'],os.O_RDONLY)\n"
        " try:\n"
        "  st=os.fstat(fd); h=hashlib.sha256()\n"
        "  while True:\n"
        "   chunk=os.read(fd,1048576)\n"
        "   if not chunk: break\n"
        "   h.update(chunk)\n"
        "  observed[name]={'device':st.st_dev,'inode':st.st_ino,"
        "'bytes':st.st_size,'sha256':h.hexdigest(),"
        "'executable_access':os.access(item['path'],os.X_OK)}\n"
        " finally:\n"
        "  os.close(fd)\n"
        "print(json.dumps(observed,separators=(',',':'),sort_keys=True))\n"
    ).encode("utf-8")
    probe_fd = os.memfd_create("rr9-procfs-exec-probe", os.MFD_ALLOW_SEALING)
    try:
        _write_all(probe_fd, probe_source)
        os.fchmod(probe_fd, 0o500)
        os.fsync(probe_fd)
        fcntl.fcntl(probe_fd, F_ADD_SEALS, REQUIRED_MEMFD_SEALS)
        applied_seals = fcntl.fcntl(probe_fd, F_GET_SEALS)
        if applied_seals != REQUIRED_MEMFD_SEALS:
            raise RuntimeError("incomplete procfs probe memfd seals")
        probe_stat = os.fstat(probe_fd)
        probe_path = f"/proc/{os.getpid()}/fd/{probe_fd}"
        expected = {
            "producer": {
                "path": memfd_receipts["binary"]["wrapper_owned_path"],
                "device": memfd_receipts["binary"]["device"],
                "inode": memfd_receipts["binary"]["inode"],
                "bytes": memfd_receipts["binary"]["bytes"],
                "sha256": memfd_receipts["binary"]["sha256"],
            },
            "time_binary": {
                "path": memfd_receipts["time_binary"]["proc_self_path"],
                "device": memfd_receipts["time_binary"]["device"],
                "inode": memfd_receipts["time_binary"]["inode"],
                "bytes": memfd_receipts["time_binary"]["bytes"],
                "sha256": memfd_receipts["time_binary"]["sha256"],
            },
        }
        completed = subprocess.run(
            [
                probe_path,
                json.dumps(expected, separators=(",", ":"), sort_keys=True),
            ],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=dict(FROZEN_CHILD_ENVIRONMENT),
            pass_fds=(memfd_receipts["time_binary"]["fd"],),
            check=False,
            timeout=5,
        )
        if len(completed.stdout) > 65_536 or len(completed.stderr) > 65_536:
            raise RuntimeError("sealed memfd child-access probe output exceeded cap")
        if completed.returncode != 0:
            raise RuntimeError(
                "sealed memfd child-access probe failed: "
                + completed.stderr.decode("utf-8", errors="replace")
            )
        try:
            observed = strict_json_loads(completed.stdout.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError, ValueError) as error:
            raise RuntimeError("invalid sealed memfd child-access receipt") from error
        if not isinstance(observed, dict):
            raise RuntimeError("invalid sealed memfd child-access receipt")
        errors: list[str] = []
        for name, item in expected.items():
            row = observed.get(name)
            if not isinstance(row, dict):
                errors.append(f"{name}_observation_missing")
                continue
            for field in ("device", "inode", "bytes", "sha256"):
                if row.get(field) != item[field]:
                    errors.append(f"{name}_{field}_mismatch")
            if row.get("executable_access") is not True:
                errors.append(f"{name}_execute_permission_missing")
        if errors:
            raise RuntimeError(
                "sealed memfd child-access probe mismatch: " + ",".join(errors)
            )
        return {
            "scientific_producer_invoked": False,
            "probe_executed_from_wrapper_owned_proc_path": probe_path,
            "probe_grandchild_reexec_from_wrapper_owned_proc_path": True,
            "child_environment": dict(FROZEN_CHILD_ENVIRONMENT),
            "exit_code": completed.returncode,
            "stdout_sha256": hashlib.sha256(completed.stdout).hexdigest(),
            "stderr_sha256": hashlib.sha256(completed.stderr).hexdigest(),
            "observations": observed,
            "probe_memfd": {
                "device": probe_stat.st_dev,
                "inode": probe_stat.st_ino,
                "bytes": probe_stat.st_size,
                "sha256": hashlib.sha256(probe_source).hexdigest(),
                "mode": stat.S_IMODE(probe_stat.st_mode),
                "seal_mask": applied_seals,
            },
        }
    finally:
        os.close(probe_fd)


def custody_recheck(
    custody: dict[str, dict[str, Any]], keys: tuple[str, ...] | None = None
) -> tuple[dict[str, dict[str, Any]], list[str]]:
    observations: dict[str, dict[str, Any]] = {}
    errors: list[str] = []
    selected = tuple(custody) if keys is None else keys
    for key in selected:
        expected = custody[key]["staged"]
        staged_path = Path(expected["path"])
        expected_bytes = expected.get("bytes")
        if type(expected_bytes) is not int or expected_bytes < 0:
            errors.append(f"{key}_custody_expected_size_invalid")
            continue
        observed, _, capture_errors = capture_raw_artifact(
            staged_path, buffer_limit=expected_bytes
        )
        observed["path"] = expected["path"]
        observations[key] = observed
        errors.extend(
            f"{key}_custody_{capture_error}"
            for capture_error in capture_errors
        )
        if not observed.get("present", False):
            errors.append(f"{key}_custody_missing_or_unreadable")
            continue
        if observed.get("mode", 0) & 0o222:
            errors.append(f"{key}_custody_writable")
        if observed.get("mode") != expected.get("mode"):
            errors.append(f"{key}_custody_mode_mismatch")
        if observed.get("device") != expected.get("device"):
            errors.append(f"{key}_custody_device_mismatch")
        if observed.get("inode") != expected.get("inode"):
            errors.append(f"{key}_custody_inode_mismatch")
        if observed.get("bytes") != expected_bytes:
            errors.append(f"{key}_custody_size_mismatch")
        if (
            not observed.get("hash_complete", False)
            or observed.get("sha256") != expected.get("sha256")
        ):
            errors.append(f"{key}_custody_sha256_mismatch")
    return observations, errors


def custody_errors(
    custody: dict[str, dict[str, Any]], keys: tuple[str, ...] | None = None
) -> list[str]:
    return custody_recheck(custody, keys)[1]


def artifact(path: Path) -> dict[str, Any]:
    observed, _, _ = capture_raw_artifact(path, buffer_limit=None)
    return observed


def artifact_matches(left: dict[str, Any], right: dict[str, Any]) -> bool:
    return all(
        left.get(key) == right.get(key)
        for key in (
            "present",
            "bytes",
            "sha256",
            "hash_complete",
            "device",
            "inode",
        )
    )


def raw_artifact_errors(
    receipt: dict[str, Any], output_dir: Path
) -> list[str]:
    errors: list[str] = []
    for row in receipt.get("cases", []):
        case = row.get("case")
        for field, suffix, cap_bytes in (
            ("stdout", "stdout", RAW_PRODUCER_JSON_MAX_BYTES),
            ("stderr", "stderr", STDERR_MAX_BYTES),
            ("time_v", "time-v.txt", TIME_V_MAX_BYTES),
        ):
            expected = row.get(field)
            if not isinstance(expected, dict):
                errors.append(f"case_{case}_{field}_binding_missing")
                continue
            observed, _, capture_errors = capture_raw_artifact(
                output_dir / f"case-{case}.{suffix}", buffer_limit=cap_bytes
            )
            errors.extend(
                f"case_{case}_{field}_{error}" for error in capture_errors
            )
            if not artifact_matches(expected, observed):
                errors.append(f"case_{case}_{field}_changed_after_capture")
    return errors


def capture_case_raw_artifacts(
    output_dir: Path, case: int
) -> tuple[dict[str, dict[str, Any]], list[str]]:
    artifacts: dict[str, dict[str, Any]] = {}
    errors: list[str] = []
    for field, suffix, cap_bytes in (
        ("stdout", "stdout", RAW_PRODUCER_JSON_MAX_BYTES),
        ("stderr", "stderr", STDERR_MAX_BYTES),
        ("time_v", "time-v.txt", TIME_V_MAX_BYTES),
    ):
        observed, _, capture_errors = capture_raw_artifact(
            output_dir / f"case-{case}.{suffix}", buffer_limit=cap_bytes
        )
        artifacts[field] = observed
        errors.extend(f"{field}:{error}" for error in capture_errors)
    return artifacts, errors


def stream_snapshot_binding_errors(
    states: dict[str, dict[str, Any]],
    artifacts: dict[str, dict[str, Any]],
) -> list[str]:
    errors: list[str] = []
    for name, state in states.items():
        observed = artifacts[name]
        if not observed.get("present", False) or not observed.get(
            "hash_complete", False
        ):
            errors.append(f"{name}_retained_artifact_incomplete")
            continue
        comparisons = (
            ("bytes", state.get("written_bytes"), observed.get("bytes")),
            (
                "sha256",
                state.get("retained_prefix_sha256"),
                observed.get("sha256"),
            ),
            ("device", state.get("retained_device"), observed.get("device")),
            ("inode", state.get("retained_inode"), observed.get("inode")),
        )
        for field, expected, actual in comparisons:
            if expected != actual:
                errors.append(f"{name}_retained_{field}_mismatch")
    return errors


def stream_state_parseable(state: dict[str, Any]) -> bool:
    return (
        state.get("observation_complete") is True
        and state.get("cap_exceeded") is False
        and state.get("drain_error") is None
        and state.get("retained_prefix") is False
    )


def bootstrap_binding(path: Path) -> dict[str, Any]:
    resolved = path.resolve(strict=True)
    observed, _, errors = capture_raw_artifact(resolved, buffer_limit=None)
    if errors or not observed.get("present", False):
        raise ValueError(
            f"bootstrap binding failed for {resolved}: {','.join(errors)}"
        )
    observed["path"] = str(resolved)
    return observed


def bootstrap_binding_recheck(
    bootstrap: dict[str, Any],
) -> tuple[dict[str, dict[str, Any]], list[str]]:
    observations: dict[str, dict[str, Any]] = {}
    errors: list[str] = []
    for name in ("launcher_source", "python_executable"):
        expected = bootstrap.get(name)
        if not isinstance(expected, dict) or not isinstance(expected.get("path"), str):
            errors.append(f"{name}_bootstrap_binding_missing")
            continue
        expected_bytes = expected.get("bytes")
        if type(expected_bytes) is not int or expected_bytes < 0:
            errors.append(f"{name}_bootstrap_expected_size_invalid")
            continue
        observed, _, capture_errors = capture_raw_artifact(
            Path(expected["path"]), buffer_limit=expected_bytes
        )
        observed["path"] = expected["path"]
        observations[name] = observed
        errors.extend(f"{name}_{error}" for error in capture_errors)
        if not artifact_matches(expected, observed):
            errors.append(f"{name}_bootstrap_binding_changed")
    return observations, errors


def bootstrap_binding_errors(bootstrap: dict[str, Any]) -> list[str]:
    return bootstrap_binding_recheck(bootstrap)[1]


def bounded_file_binding(path: Path, cap_bytes: int, label: str) -> dict[str, Any]:
    observed, _, errors = capture_raw_artifact(path, buffer_limit=cap_bytes)
    if errors or not observed.get("present", False) or not observed.get(
        "hash_complete", False
    ):
        raise ValueError(f"{label} binding failed: {','.join(errors)}")
    observed["path"] = str(path.resolve(strict=True))
    return observed


def bounded_file_binding_errors(
    expected: dict[str, Any], cap_bytes: int, label: str
) -> list[str]:
    path_value = expected.get("path")
    if not isinstance(path_value, str):
        return [f"{label}_binding_path_missing"]
    observed, _, capture_errors = capture_raw_artifact(
        Path(path_value), buffer_limit=cap_bytes
    )
    observed["path"] = path_value
    errors = [f"{label}_{error}" for error in capture_errors]
    if not artifact_matches(expected, observed):
        errors.append(f"{label}_binding_changed")
    return errors


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")


def git_text(repo: Path, *args: str) -> str | None:
    del repo, args
    return None


def git_succeeds(repo: Path, *args: str) -> bool | None:
    del repo, args
    return None


def git_path_state(repo: Path, path: Path) -> dict[str, Any]:
    try:
        relative = path.resolve().relative_to(repo.resolve())
    except ValueError:
        return {
            "inside_repository": False,
            "repo_relative_path": None,
            "tracked": None,
            "porcelain_v1": None,
            "clean": None,
        }
    status = git_text(
        repo,
        "status",
        "--porcelain=v1",
        "--untracked-files=all",
        "--",
        relative.as_posix(),
    )
    return {
        "inside_repository": True,
        "repo_relative_path": relative.as_posix(),
        "tracked": git_succeeds(
            repo, "ls-files", "--error-unmatch", "--", relative.as_posix()
        ),
        "porcelain_v1": None if status is None else status.splitlines(),
        "clean": None if status is None else status == "",
    }


def repository_receipt(repo: Path, paths: dict[str, Path]) -> dict[str, Any]:
    status = git_text(
        repo, "status", "--porcelain=v1", "--untracked-files=all"
    )
    return {
        "admission_bearing": False,
        "purpose": "informational_git_provenance_only",
        "available": False,
        "unavailable_reason": "git_not_invoked_by_custody_wrapper",
        "path": str(repo.resolve()),
        "head": git_text(repo, "rev-parse", "HEAD"),
        "worktree_porcelain_v1": None if status is None else status.splitlines(),
        "worktree_clean": None if status is None else status == "",
        "paths": {name: git_path_state(repo, path) for name, path in paths.items()},
    }


def first_line(path: Path) -> str | None:
    try:
        with path.open("r", encoding="utf-8", errors="replace") as handle:
            return handle.readline().strip()
    except OSError:
        return None


def environment_receipt(image_digest: str | None) -> dict[str, Any]:
    cpu_model = None
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8").splitlines():
            if line.lower().startswith("model name"):
                cpu_model = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    memory_limit = None
    for candidate in (
        Path("/sys/fs/cgroup/memory.max"),
        Path("/sys/fs/cgroup/memory/memory.limit_in_bytes"),
    ):
        value = first_line(candidate)
        if value is not None:
            memory_limit = {"path": str(candidate), "value": value}
            break
    return {
        "uname": list(platform.uname()),
        "python": sys.version.split()[0],
        "working_directory": str(Path.cwd().resolve()),
        "cpu_model": cpu_model,
        "logical_cpu_count": os.cpu_count(),
        "cgroup_memory_limit": memory_limit,
        "container_image_digest": image_digest,
        "child_environment": dict(FROZEN_CHILD_ENVIRONMENT),
        "inherited_environment_used_for_children": False,
    }


def enable_child_subreaper() -> dict[str, Any]:
    libc = ctypes.CDLL(None, use_errno=True)
    prctl = libc.prctl
    prctl.restype = ctypes.c_int
    if prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) != 0:
        error_number = ctypes.get_errno()
        raise OSError(error_number, os.strerror(error_number))
    observed = ctypes.c_int(0)
    if prctl(PR_GET_CHILD_SUBREAPER, ctypes.byref(observed), 0, 0, 0) != 0:
        error_number = ctypes.get_errno()
        raise OSError(error_number, os.strerror(error_number))
    if observed.value != 1:
        raise RuntimeError("PR_SET_CHILD_SUBREAPER was not retained")
    return {
        "prctl_set_option": PR_SET_CHILD_SUBREAPER,
        "prctl_get_option": PR_GET_CHILD_SUBREAPER,
        "enabled": True,
    }


def process_containment_receipt(guarantee: str) -> dict[str, Any]:
    if guarantee != PROCESS_CONTAINMENT_GUARANTEE:
        raise ValueError(
            "process containment guarantee must be "
            f"{PROCESS_CONTAINMENT_GUARANTEE!r}"
        )
    cgroup_path = Path("/proc/self/cgroup")
    cgroup_bytes = cgroup_path.read_bytes()
    pid_namespace = Path("/proc/self/ns/pid")
    namespace_stat = pid_namespace.stat()
    subreaper = enable_child_subreaper()
    return {
        "guarantee": guarantee,
        "local_defense": (
            "dedicated start_new_session process group; TERM then KILL after "
            "every timed-command leader reaches a terminal state"
        ),
        "setsid_escape_local_prevention": False,
        "external_eks_job_receipt_required_for_admission": True,
        "child_subreaper": subreaper,
        "proc_self_cgroup": {
            "path": str(cgroup_path),
            "utf8": cgroup_bytes.decode("utf-8"),
            "bytes": len(cgroup_bytes),
            "sha256": hashlib.sha256(cgroup_bytes).hexdigest(),
        },
        "pid_namespace": {
            "path": str(pid_namespace),
            "link": os.readlink(pid_namespace),
            "inode": namespace_stat.st_ino,
        },
        "wrapper_pid": os.getpid(),
        "wrapper_session_id": os.getsid(0),
        "wrapper_process_group_id": os.getpgrp(),
    }


def containment_stability_errors(
    expected: dict[str, Any], observed: dict[str, Any]
) -> list[str]:
    errors: list[str] = []
    scalar_fields = (
        "guarantee",
        "wrapper_pid",
        "wrapper_session_id",
        "wrapper_process_group_id",
    )
    for field in scalar_fields:
        if observed.get(field) != expected.get(field):
            errors.append(f"containment_{field}_mismatch")
    for field in ("proc_self_cgroup", "pid_namespace", "child_subreaper"):
        if observed.get(field) != expected.get(field):
            errors.append(f"containment_{field}_mismatch")
    return errors


def containment_recheck(
    expected: dict[str, Any],
) -> tuple[dict[str, Any], list[str]]:
    try:
        observed = process_containment_receipt(PROCESS_CONTAINMENT_GUARANTEE)
    except Exception as error:
        return (
            {
                "capture_error": {
                    "type": type(error).__name__,
                    "message": str(error),
                }
            },
            [f"containment_capture_failed:{type(error).__name__}"],
        )
    return observed, containment_stability_errors(expected, observed)


def validate_inputs(
    binary: Path,
    protocol: Path,
    source: Path,
    verifier_source: Path,
    verifier_executable: Path,
    time_binary: Path,
    cargo_lock: Path,
    cargo_manifest: Path,
) -> dict[str, Any]:
    bindings = {
        "binary": bound_file(binary),
        "protocol": bound_file(protocol),
        "source": bound_file(source),
        "verifier_source": bound_file(verifier_source),
        "verifier_executable": bound_file(verifier_executable),
        "launcher": bound_file(Path(__file__)),
        "time_binary": bound_file(time_binary),
        "cargo_lock": bound_file(cargo_lock),
        "cargo_manifest": bound_file(cargo_manifest),
    }
    if not os.access(binary.resolve(), os.X_OK):
        raise ValueError(f"producer binary is not executable: {binary}")
    if not os.access(time_binary.resolve(), os.X_OK):
        raise ValueError(f"time binary is not executable: {time_binary}")
    if not os.access(verifier_executable.resolve(), os.X_OK):
        raise ValueError(
            f"verifier executable is not executable: {verifier_executable}"
        )
    return bindings


def input_paths(
    binary: Path,
    protocol: Path,
    source: Path,
    verifier_source: Path,
    verifier_executable: Path,
    time_binary: Path,
    cargo_lock: Path,
    cargo_manifest: Path,
) -> dict[str, Path]:
    return {
        "binary": binary,
        "protocol": protocol,
        "source": source,
        "verifier_source": verifier_source,
        "verifier_executable": verifier_executable,
        "launcher": Path(__file__),
        "time_binary": time_binary,
        "cargo_lock": cargo_lock,
        "cargo_manifest": cargo_manifest,
    }


def followup_links() -> dict[str, Any]:
    empty_artifact = {"path": None, "sha256": None}
    return {
        "independent_full8_verifier": {
            "verifier_source": dict(empty_artifact),
            "verifier_executable": dict(empty_artifact),
            "receipt": dict(empty_artifact),
        },
        "same_host_d9_positive": {"receipt": dict(empty_artifact)},
        "same_host_d9_negative_1": {"receipt": dict(empty_artifact)},
        "same_host_d9_negative_2": {"receipt": dict(empty_artifact)},
        "same_host_128_chart_signed_control": {"receipt": dict(empty_artifact)},
        "external_eks_job_containment": {"receipt": dict(empty_artifact)},
    }


def plan(
    binary: Path,
    protocol: Path,
    source: Path,
    verifier_source: Path,
    verifier_executable: Path,
    time_binary: Path,
    cargo_lock: Path,
    cargo_manifest: Path,
    repo: Path,
    image_digest: str | None,
    process_containment_guarantee: str | None,
) -> dict[str, Any]:
    if not image_digest:
        raise ValueError("container image digest is required")
    if process_containment_guarantee is None:
        raise ValueError("process containment guarantee is required")
    containment = process_containment_receipt(process_containment_guarantee)
    launcher_bootstrap = {
        "trust_model": "bootstrap_trusted_not_staged",
        "external_image_and_eks_corroboration_required": True,
        "launcher_source": bootstrap_binding(Path(__file__)),
        "python_executable": bootstrap_binding(Path(sys.executable)),
    }
    bindings = validate_inputs(
        binary,
        protocol,
        source,
        verifier_source,
        verifier_executable,
        time_binary,
        cargo_lock,
        cargo_manifest,
    )
    return {
        "schema": SCHEMA,
        "mode": "dry_run",
        "case_order": list(CASE_ORDER),
        "outer_cap_seconds": OUTER_CAP_SECONDS,
        "outer_cap_scope": OUTER_CAP_SCOPE,
        "receipt_sealing_excluded_from_producer_cap": True,
        "bindings": bindings,
        "repository": repository_receipt(
            repo,
            {
                "binary": binary,
                "protocol": protocol,
                "source": source,
                "verifier_source": verifier_source,
                "verifier_executable": verifier_executable,
                "launcher": Path(__file__),
                "time_binary": time_binary,
                "cargo_lock": cargo_lock,
                "cargo_manifest": cargo_manifest,
            },
        ),
        "commands": [
            [
                "<sealed-gnu-time-memfd:/proc/self/fd/N>",
                "-v",
                "-o",
                "<capped-time-v-pipe:/proc/self/fd/N>",
                "<sealed-producer-memfd:/proc/WRAPPER_PID/fd/M>",
                str(case),
            ]
            for case in CASE_ORDER
        ],
        "producer_invoked": False,
        "planned_container_image_digest": image_digest,
        "sealed_memfd_execution_required": True,
        "child_environment": dict(FROZEN_CHILD_ENVIRONMENT),
        "process_containment": containment,
        "launcher_bootstrap": launcher_bootstrap,
        "scientific_evaluation_performed": False,
        "comparison_admissible": False,
        "required_followup_links": followup_links(),
    }


def kill_group_at_outer_deadline(
    process: subprocess.Popen[bytes],
) -> dict[str, Any]:
    """Kill the timed group and observe its leader without a blocking wait."""

    kill_sent = False
    try:
        os.killpg(process.pid, signal.SIGKILL)
        kill_sent = True
    except ProcessLookupError:
        pass
    return_code = process.poll()
    return {
        "kill_sent": kill_sent,
        "leader_terminal_observed": return_code is not None,
        "leader_return_code": return_code,
        "observation_method": "subprocess_poll_waitpid_wnohang",
        "blocking_wait_used": False,
        "observed_monotonic_ns": time.monotonic_ns(),
    }


def process_group_exists(process_group_id: int) -> bool:
    try:
        os.killpg(process_group_id, 0)
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    return True


def reap_process_group(
    process_group_id: int,
    deadline_monotonic_ns: int | None = None,
) -> list[dict[str, int]]:
    reaped: list[dict[str, int]] = []
    while len(reaped) < 4096:
        try:
            child_pid, status = os.waitpid(-process_group_id, os.WNOHANG)
        except ChildProcessError:
            break
        if child_pid == 0:
            break
        reaped.append({"pid": child_pid, "wait_status": status})
        if (
            deadline_monotonic_ns is not None
            and time.monotonic_ns() >= deadline_monotonic_ns
        ):
            break
    return reaped


def terminate_surviving_process_group(
    process_group_id: int,
    deadline_monotonic_ns: int | None = None,
) -> dict[str, Any]:
    """TERM then KILL any descendants left after the timed leader exits."""

    started_ns = time.monotonic_ns()
    term_sent = False
    kill_sent = False
    reaped: list[dict[str, int]] = []
    try:
        os.killpg(process_group_id, signal.SIGTERM)
        term_sent = True
    except ProcessLookupError:
        reaped.extend(
            reap_process_group(process_group_id, deadline_monotonic_ns)
        )
        return {
            "term_sent": False,
            "kill_sent": False,
            "absence_verified": True,
            "polls": 0,
            "reaped": reaped,
            "elapsed_ns": time.monotonic_ns() - started_ns,
        }
    if deadline_monotonic_ns is None:
        time.sleep(0.02)
    else:
        remaining_seconds = (
            deadline_monotonic_ns - time.monotonic_ns()
        ) / 1_000_000_000
        if remaining_seconds > 0:
            time.sleep(min(0.02, remaining_seconds))
    if process_group_exists(process_group_id):
        try:
            os.killpg(process_group_id, signal.SIGKILL)
            kill_sent = True
        except ProcessLookupError:
            pass
    polls = 0
    while process_group_exists(process_group_id) and polls < 20:
        reaped.extend(
            reap_process_group(process_group_id, deadline_monotonic_ns)
        )
        if not process_group_exists(process_group_id):
            break
        if (
            deadline_monotonic_ns is not None
            and time.monotonic_ns() >= deadline_monotonic_ns
        ):
            break
        polls += 1
        if deadline_monotonic_ns is None:
            time.sleep(0.005)
        else:
            remaining_seconds = (
                deadline_monotonic_ns - time.monotonic_ns()
            ) / 1_000_000_000
            if remaining_seconds <= 0:
                break
            time.sleep(min(0.005, remaining_seconds))
    reaped.extend(reap_process_group(process_group_id, deadline_monotonic_ns))
    return {
        "term_sent": term_sent,
        "kill_sent": kill_sent,
        "absence_verified": not process_group_exists(process_group_id),
        "polls": polls,
        "reaped": reaped,
        "elapsed_ns": time.monotonic_ns() - started_ns,
    }


def reconcile_process_terminal_observation(
    process: subprocess.Popen[bytes],
    process_group_cleanup: dict[str, Any],
) -> int | None:
    """Return an observation time only when a nonblocking path saw terminality."""

    if process.returncode is None:
        for reaped_child in process_group_cleanup.get("reaped", []):
            if reaped_child.get("pid") == process.pid:
                process.returncode = os.waitstatus_to_exitcode(
                    reaped_child["wait_status"]
                )
                break
    return time.monotonic_ns() if process.returncode is not None else None


def kill_group_for_stream_cap(process_group_id: int) -> None:
    try:
        os.killpg(process_group_id, signal.SIGKILL)
    except ProcessLookupError:
        pass


def drain_bounded_stream(
    stream: Any,
    path: Path,
    cap_bytes: int,
    process_group_id: int,
    state: dict[str, Any],
    stop_event: threading.Event | None = None,
) -> None:
    """Drain a child pipe without ever writing more than the frozen cap."""

    observed_bytes = 0
    written_bytes = 0
    cap_exceeded = False
    retained_digest = hashlib.sha256()
    retained_device = None
    retained_inode = None
    reached_eof = False
    stopped = False
    stream_fd = None
    try:
        stream_fd = stream.fileno()
        os.set_blocking(stream_fd, False)
    except (AttributeError, OSError, ValueError):
        stream_fd = None
    try:
        with path.open("xb") as output_handle:
            while True:
                if stream_fd is None:
                    chunk = stream.read(64 * 1024)
                else:
                    try:
                        chunk = os.read(stream_fd, 64 * 1024)
                    except BlockingIOError:
                        if stop_event is not None and stop_event.is_set():
                            stopped = True
                            break
                        if stop_event is None:
                            time.sleep(0.01)
                        else:
                            stop_event.wait(0.01)
                        continue
                if not chunk:
                    reached_eof = True
                    break
                observed_bytes += len(chunk)
                remaining = cap_bytes - written_bytes
                if remaining > 0:
                    retained = chunk[:remaining]
                    output_handle.write(retained)
                    retained_digest.update(retained)
                    written_bytes += len(retained)
                if observed_bytes > cap_bytes and not cap_exceeded:
                    cap_exceeded = True
                    kill_group_for_stream_cap(process_group_id)
            output_handle.flush()
            retained_stat = os.fstat(output_handle.fileno())
            retained_device = retained_stat.st_dev
            retained_inode = retained_stat.st_ino
    except Exception as error:
        state["drain_error"] = {
            "type": type(error).__name__,
            "message": str(error),
        }
        kill_group_for_stream_cap(process_group_id)
    finally:
        try:
            stream.close()
        except Exception:
            pass
        observation_complete = (
            "drain_error" not in state and reached_eof and not stopped
        )
        if stopped:
            termination_reason = "stop_requested_pipe_still_open"
        elif not observation_complete:
            termination_reason = "drain_error"
        elif cap_exceeded:
            termination_reason = "write_cap_exceeded_process_group_killed"
        else:
            termination_reason = "eof"
        state.update(
            {
                "cap_bytes": cap_bytes,
                "observed_bytes": observed_bytes,
                "observed_bytes_kind": (
                    "exact" if observation_complete else "lower_bound"
                ),
                "written_bytes": written_bytes,
                "retained_prefix": cap_exceeded or not observation_complete,
                "retained_prefix_bytes": written_bytes,
                "retained_prefix_sha256": retained_digest.hexdigest(),
                "retained_device": retained_device,
                "retained_inode": retained_inode,
                "artifact_binding_field": path.name,
                "cap_exceeded": cap_exceeded,
                "observation_complete": observation_complete,
                "termination_reason": termination_reason,
            }
        )


def _reject_json_constant(value: str) -> None:
    raise ValueError(f"non-finite JSON constant: {value}")


def _unique_json_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def strict_json_loads(encoded: str) -> Any:
    def reject_float(raw: str) -> None:
        raise ValueError(f"JSON floating-point number is forbidden: {raw}")

    return json.loads(
        encoded,
        object_pairs_hook=_unique_json_object,
        parse_constant=_reject_json_constant,
        parse_float=reject_float,
    )


def observed_header_bytes(encoded: bytes) -> dict[str, Any] | None:
    try:
        if len(encoded) > RAW_PRODUCER_JSON_MAX_BYTES:
            return None
        if not encoded.endswith(b"\n"):
            return None
        value = strict_json_loads(encoded.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError, ValueError):
        return None
    if not isinstance(value, dict):
        return None
    input_system = value.get("input_system")
    if isinstance(input_system, dict):
        encoded_input = json.dumps(
            input_system,
            ensure_ascii=False,
            separators=(",", ":"),
            sort_keys=True,
        ).encode("utf-8")
        input_schema = input_system.get("schema")
        input_case = input_system.get("case")
        computed_input_sha256 = hashlib.sha256(encoded_input).hexdigest()
    else:
        input_schema = None
        input_case = None
        computed_input_sha256 = None
    return {
        "schema": value.get("schema"),
        "case": value.get("case"),
        "input_system_schema": input_schema,
        "input_system_case": input_case,
        "claimed_input_system_sha256": value.get("input_system_sha256"),
        "computed_input_system_sha256": computed_input_sha256,
    }


def observed_header(stdout_path: Path) -> dict[str, Any] | None:
    try:
        encoded = stdout_path.read_bytes()
    except OSError:
        return None
    return observed_header_bytes(encoded)


def producer_interface_errors(
    observed: dict[str, Any] | None, expected_case: int
) -> list[str]:
    if observed is None:
        return ["stdout_is_not_one_json_object"]
    errors = []
    if observed.get("schema") != EXPECTED_PRODUCER_SCHEMA:
        errors.append("producer_schema_mismatch")
    if type(observed.get("case")) is not int or observed.get("case") != expected_case:
        errors.append("producer_case_mismatch")
    if observed.get("input_system_schema") != EXPECTED_INPUT_SCHEMA:
        errors.append("input_system_schema_mismatch")
    if (
        type(observed.get("input_system_case")) is not int
        or observed.get("input_system_case") != expected_case
    ):
        errors.append("input_system_case_mismatch")
    if (
        observed.get("computed_input_system_sha256") is None
        or observed.get("claimed_input_system_sha256")
        != observed.get("computed_input_system_sha256")
    ):
        errors.append("input_system_sha256_mismatch")
    return errors


def _nonnegative_float(raw: str) -> float:
    value = float(raw)
    if not math.isfinite(value) or value < 0:
        raise ValueError("expected a finite nonnegative number")
    return value


def _nonnegative_int(raw: str) -> int:
    value = int(raw)
    if value < 0:
        raise ValueError("expected a nonnegative integer")
    return value


def _elapsed_wall_seconds(raw: str) -> float:
    parts = raw.split(":")
    if len(parts) not in (2, 3):
        raise ValueError("elapsed wall time must be m:ss or h:mm:ss")
    seconds = _nonnegative_float(parts[-1])
    minutes = _nonnegative_int(parts[-2])
    if seconds >= 60 or (len(parts) == 3 and minutes >= 60):
        raise ValueError("elapsed wall time component is out of range")
    hours = _nonnegative_int(parts[0]) if len(parts) == 3 else 0
    return hours * 3600 + minutes * 60 + seconds


def time_v_report_bytes(
    encoded: bytes | None,
) -> tuple[dict[str, Any] | None, list[str]]:
    if encoded is None:
        return None, ["time_v_missing"]
    try:
        lines = encoded.decode("utf-8").splitlines()
    except UnicodeDecodeError:
        return None, ["time_v_not_utf8"]
    labels = {
        "User time (seconds)": ("user_seconds", _nonnegative_float),
        "System time (seconds)": ("system_seconds", _nonnegative_float),
        "Maximum resident set size (kbytes)": (
            "maximum_resident_set_kib",
            _nonnegative_int,
        ),
        "Exit status": ("time_exit_status", _nonnegative_int),
    }
    result: dict[str, Any] = {}
    errors: list[str] = []
    for line in lines:
        stripped = line.strip()
        elapsed_label = "Elapsed (wall clock) time (h:mm:ss or m:ss)"
        elapsed_prefix = f"{elapsed_label}: "
        if stripped.startswith(elapsed_prefix):
            if "elapsed_wall" in result:
                errors.append("time_v_duplicate:elapsed_wall")
                continue
            raw_value = stripped[len(elapsed_prefix) :]
            try:
                result["elapsed_wall_seconds"] = _elapsed_wall_seconds(raw_value)
                result["elapsed_wall"] = raw_value
            except ValueError:
                errors.append("time_v_malformed:elapsed_wall")
            continue
        for label, (name, converter) in labels.items():
            prefix = f"{label}: "
            if not stripped.startswith(prefix):
                continue
            if name in result:
                errors.append(f"time_v_duplicate:{name}")
                break
            raw_value = stripped[len(prefix) :]
            try:
                result[name] = converter(raw_value)
            except ValueError:
                errors.append(f"time_v_malformed:{name}")
            break
    for name in REQUIRED_TIME_V_FIELDS:
        if name not in result:
            errors.append(f"time_v_missing:{name}")
    return result, errors


def time_v_report(path: Path) -> tuple[dict[str, Any] | None, list[str]]:
    try:
        encoded = path.read_bytes()
    except OSError as error:
        return None, [f"time_v_unreadable:{type(error).__name__}"]
    return time_v_report_bytes(encoded)


def time_v_observations(path: Path) -> dict[str, Any] | None:
    return time_v_report(path)[0]


def time_v_resource_errors(path: Path, expected_exit_code: int | None) -> list[str]:
    observations, errors = time_v_report(path)
    if (
        observations is not None
        and expected_exit_code is not None
        and observations.get("time_exit_status") != expected_exit_code
    ):
        errors.append("time_v_exit_status_mismatch")
    return errors


def case_argv(
    case: int,
    binary: Path | str,
    time_binary: Path | str,
    output_dir: Path,
    time_output: str | None = None,
) -> list[str]:
    binary_arg = str(binary.resolve()) if isinstance(binary, Path) else binary
    time_arg = (
        str(time_binary.resolve())
        if isinstance(time_binary, Path)
        else time_binary
    )
    return [
        time_arg,
        "-v",
        "-o",
        time_output or str(output_dir / f"case-{case}.time-v.txt"),
        binary_arg,
        str(case),
    ]


def fork_fsync_until_deadline(
    descriptor: int, outer_deadline_ns: int
) -> dict[str, Any]:
    """Fsync one inherited FD in a helper bounded by the suite deadline."""

    started_ns = time.monotonic_ns()
    try:
        helper_pid = os.fork()
    except OSError as error:
        return {
            "attempted": False,
            "helper_pid": None,
            "started_monotonic_ns": started_ns,
            "finished_monotonic_ns": time.monotonic_ns(),
            "deadline_monotonic_ns": outer_deadline_ns,
            "timed_out": False,
            "kill_sent": False,
            "reaped": False,
            "wait_status": None,
            "exit_code": None,
            "success": False,
            "termination_reason": "fork_failed",
            "error": {"type": type(error).__name__, "message": str(error)},
        }
    if helper_pid == 0:
        try:
            os.fsync(descriptor)
        except BaseException:
            os._exit(111)
        os._exit(0)

    wait_status: int | None = None
    timed_out = False
    kill_sent = False
    reaped = False
    wait_error: dict[str, str] | None = None
    while True:
        try:
            waited_pid, observed_status = os.waitpid(helper_pid, os.WNOHANG)
        except InterruptedError:
            waited_pid, observed_status = 0, 0
        except (ChildProcessError, OSError) as error:
            wait_error = {"type": type(error).__name__, "message": str(error)}
            try:
                os.kill(helper_pid, signal.SIGKILL)
                kill_sent = True
            except ProcessLookupError:
                pass
            try:
                waited_pid, observed_status = os.waitpid(
                    helper_pid, os.WNOHANG
                )
            except (ChildProcessError, OSError):
                pass
            else:
                if waited_pid == helper_pid:
                    wait_status = observed_status
                    reaped = True
            break
        if waited_pid == helper_pid:
            wait_status = observed_status
            reaped = True
            break
        now_ns = time.monotonic_ns()
        if now_ns >= outer_deadline_ns:
            timed_out = True
            try:
                os.kill(helper_pid, signal.SIGKILL)
                kill_sent = True
            except ProcessLookupError:
                pass
            try:
                waited_pid, observed_status = os.waitpid(
                    helper_pid, os.WNOHANG
                )
            except (ChildProcessError, OSError) as error:
                wait_error = {
                    "type": type(error).__name__,
                    "message": str(error),
                }
            else:
                if waited_pid == helper_pid:
                    wait_status = observed_status
                    reaped = True
            break
        time.sleep(min(0.005, (outer_deadline_ns - now_ns) / 1_000_000_000))
    finished_ns = time.monotonic_ns()
    exit_code = (
        os.waitstatus_to_exitcode(wait_status)
        if wait_status is not None
        else None
    )
    success = (
        not timed_out
        and wait_error is None
        and reaped
        and exit_code == 0
        and finished_ns <= outer_deadline_ns
    )
    if timed_out:
        termination_reason = "outer_deadline_timeout"
    elif wait_error is not None:
        termination_reason = "wait_failed"
    elif exit_code != 0:
        termination_reason = "helper_nonzero_exit"
    elif finished_ns > outer_deadline_ns:
        termination_reason = "helper_finished_after_deadline"
    else:
        termination_reason = "helper_exit_zero"
    result = {
        "attempted": True,
        "helper_pid": helper_pid,
        "started_monotonic_ns": started_ns,
        "finished_monotonic_ns": finished_ns,
        "deadline_monotonic_ns": outer_deadline_ns,
        "timed_out": timed_out,
        "kill_sent": kill_sent,
        "reaped": reaped,
        "wait_status": wait_status,
        "exit_code": exit_code,
        "success": success,
        "termination_reason": termination_reason,
    }
    if wait_error is not None:
        result["error"] = wait_error
    return result


def _incomplete_endpoint_artifact(
    path: Path, before: os.stat_result | None
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "path": path.name,
        "present": before is not None,
        "bytes": None if before is None else before.st_size,
        "hash_complete": False,
        "sha256": None,
        "observed_prefix_bytes": 0,
        "observed_prefix_sha256": hashlib.sha256(b"").hexdigest(),
        "observed_bytes_kind": "lower_bound",
        "device": None if before is None else before.st_dev,
        "inode": None if before is None else before.st_ino,
        "mode": None if before is None else stat.S_IMODE(before.st_mode),
    }
    return result


def normalize_endpoint_fsync_receipt(
    raw: dict[str, Any],
    *,
    cap_bytes: int,
    outer_deadline_ns: int,
    pre_fsync_binding: dict[str, Any] | None,
    post_fsync_snapshot: dict[str, Any] | None,
) -> dict[str, Any]:
    values = {
        "attempted": False,
        "helper_pid": None,
        "cap_bytes": cap_bytes,
        "started_monotonic_ns": None,
        "finished_monotonic_ns": time.monotonic_ns(),
        "deadline_monotonic_ns": outer_deadline_ns,
        "timed_out": False,
        "kill_sent": False,
        "reaped": False,
        "wait_status": None,
        "exit_code": None,
        "termination_reason": "not_attempted",
        "success": False,
        "error": None,
        "pre_fsync_binding": pre_fsync_binding,
        "post_fsync_snapshot": post_fsync_snapshot,
    }
    field_order = tuple(values)
    values.update(raw)
    return {key: values[key] for key in field_order}


def fsync_and_snapshot_artifact(
    path: Path,
    stream_state: dict[str, Any],
    cap_bytes: int,
    outer_deadline_ns: int,
    *,
    retain_bytes: bool,
) -> tuple[dict[str, Any], dict[str, Any], bytes | None, list[str]]:
    """Deadline-fsync and snapshot one drain artifact from the same FD."""

    flags = (
        os.O_RDONLY
        | getattr(os, "O_NOFOLLOW", 0)
        | getattr(os, "O_NONBLOCK", 0)
    )
    try:
        descriptor = os.open(path, flags)
    except OSError as error:
        observed_ns = time.monotonic_ns()
        incomplete = _incomplete_endpoint_artifact(path, None)
        helper = normalize_endpoint_fsync_receipt(
            {
                "finished_monotonic_ns": observed_ns,
                "termination_reason": f"open_failed:{type(error).__name__}",
                "error": {"type": type(error).__name__, "message": str(error)},
            },
            cap_bytes=cap_bytes,
            outer_deadline_ns=outer_deadline_ns,
            pre_fsync_binding=None,
            post_fsync_snapshot=None,
        )
        return (
            helper,
            incomplete,
            None,
            [f"endpoint_open_failed:{type(error).__name__}"],
        )
    before: os.stat_result | None = None
    errors: list[str] = []
    try:
        before = os.fstat(descriptor)
        expected_bytes = stream_state.get("written_bytes")
        if not stat.S_ISREG(before.st_mode):
            errors.append("endpoint_not_regular")
        if type(expected_bytes) is not int or not 0 <= expected_bytes <= cap_bytes:
            errors.append("endpoint_expected_size_invalid")
        elif before.st_size != expected_bytes:
            errors.append("endpoint_size_mismatch_before_fsync")
        if before.st_dev != stream_state.get("retained_device"):
            errors.append("endpoint_device_mismatch_before_fsync")
        if before.st_ino != stream_state.get("retained_inode"):
            errors.append("endpoint_inode_mismatch_before_fsync")
        pre_fsync_binding = {
            "device": before.st_dev,
            "inode": before.st_ino,
            "bytes": before.st_size,
            "mode": stat.S_IMODE(before.st_mode),
            "mtime_ns": before.st_mtime_ns,
            "ctime_ns": before.st_ctime_ns,
        }
        if errors:
            observed_ns = time.monotonic_ns()
            incomplete = _incomplete_endpoint_artifact(path, before)
            helper = normalize_endpoint_fsync_receipt(
                {
                    "finished_monotonic_ns": observed_ns,
                    "termination_reason": "precheck_failed",
                    "error": {
                        "type": "PrecheckFailure",
                        "message": ",".join(errors),
                    },
                },
                cap_bytes=cap_bytes,
                outer_deadline_ns=outer_deadline_ns,
                pre_fsync_binding=pre_fsync_binding,
                post_fsync_snapshot=None,
            )
            return (
                helper,
                incomplete,
                None,
                errors,
            )
        if time.monotonic_ns() >= outer_deadline_ns:
            incomplete = _incomplete_endpoint_artifact(path, before)
            helper = normalize_endpoint_fsync_receipt(
                {
                    "finished_monotonic_ns": time.monotonic_ns(),
                    "timed_out": True,
                    "termination_reason": "deadline_exhausted_before_fork",
                },
                cap_bytes=cap_bytes,
                outer_deadline_ns=outer_deadline_ns,
                pre_fsync_binding=pre_fsync_binding,
                post_fsync_snapshot=None,
            )
            return (
                helper,
                incomplete,
                None,
                ["endpoint_fsync_timeout"],
            )
        helper_result = fork_fsync_until_deadline(descriptor, outer_deadline_ns)
        if not helper_result.get("success", False):
            incomplete = _incomplete_endpoint_artifact(path, before)
            errors.append(
                "endpoint_fsync_timeout"
                if helper_result.get("timed_out", False)
                else "endpoint_fsync_failed"
            )
            helper = normalize_endpoint_fsync_receipt(
                helper_result,
                cap_bytes=cap_bytes,
                outer_deadline_ns=outer_deadline_ns,
                pre_fsync_binding=pre_fsync_binding,
                post_fsync_snapshot=None,
            )
            return (
                helper,
                incomplete,
                None,
                errors,
            )
        post_fsync = os.fstat(descriptor)
        if (
            post_fsync.st_dev != before.st_dev
            or post_fsync.st_ino != before.st_ino
            or post_fsync.st_size != before.st_size
            or post_fsync.st_mtime_ns != before.st_mtime_ns
            or post_fsync.st_ctime_ns != before.st_ctime_ns
        ):
            errors.append("endpoint_changed_during_fsync")
        os.lseek(descriptor, 0, os.SEEK_SET)
        digest = hashlib.sha256()
        chunks: list[bytes] = []
        total = 0
        reached_eof = False
        assert isinstance(expected_bytes, int)
        while total <= expected_bytes:
            chunk = os.read(
                descriptor,
                min(1024 * 1024, expected_bytes + 1 - total),
            )
            if not chunk:
                reached_eof = True
                break
            digest.update(chunk)
            total += len(chunk)
            if retain_bytes and total <= expected_bytes:
                chunks.append(chunk)
        after = os.fstat(descriptor)
        if (
            not reached_eof
            or total != expected_bytes
            or after.st_dev != before.st_dev
            or after.st_ino != before.st_ino
            or after.st_size != before.st_size
            or after.st_mtime_ns != before.st_mtime_ns
            or after.st_ctime_ns != before.st_ctime_ns
        ):
            errors.append("endpoint_changed_during_snapshot")
        if digest.hexdigest() != stream_state.get("retained_prefix_sha256"):
            errors.append("endpoint_sha256_mismatch")
        snapshot = {
            "path": path.name,
            "present": True,
            "bytes": after.st_size,
            "sha256": digest.hexdigest() if not errors else None,
            "hash_complete": not errors,
            "observed_prefix_bytes": total,
            "observed_prefix_sha256": digest.hexdigest(),
            "observed_bytes_kind": "exact" if not errors else "lower_bound",
            "device": after.st_dev,
            "inode": after.st_ino,
            "mode": stat.S_IMODE(after.st_mode),
        }
        helper_result["success"] = helper_result["success"] and not errors
        if errors:
            helper_result["termination_reason"] = "post_fsync_snapshot_mismatch"
            helper_result["error"] = {
                "type": "SnapshotMismatch",
                "message": ",".join(errors),
            }
        helper = normalize_endpoint_fsync_receipt(
            helper_result,
            cap_bytes=cap_bytes,
            outer_deadline_ns=outer_deadline_ns,
            pre_fsync_binding=pre_fsync_binding,
            post_fsync_snapshot=snapshot,
        )
        buffered = b"".join(chunks) if retain_bytes and not errors else None
        return helper, snapshot, buffered, errors
    finally:
        os.close(descriptor)


class CaseLaunchFailure(RuntimeError):
    """A producer launch was attempted, but Popen created no child process."""

    def __init__(
        self,
        *,
        ordinal: int,
        case: int,
        argv: list[str],
        attempted_utc: str,
        attempted_monotonic_ns: int,
        cause: Exception,
    ) -> None:
        super().__init__(str(cause))
        self.ordinal = ordinal
        self.case = case
        self.argv = argv
        self.attempted_utc = attempted_utc
        self.attempted_monotonic_ns = attempted_monotonic_ns
        self.cause_type = type(cause).__name__
        self.cause_message = str(cause)


class StartedCaseFailure(RuntimeError):
    """A child exists, but its protected evidence endpoint raised."""

    def __init__(self, progress: dict[str, Any], cause: Exception) -> None:
        super().__init__(str(cause))
        self.progress = progress
        self.cause_type = type(cause).__name__
        self.cause_message = str(cause)


def run_case(
    ordinal: int,
    case: int,
    binary: Path | str,
    time_binary: Path | str,
    output_dir: Path,
    outer_deadline_ns: int,
    on_process_started: Any | None = None,
    inherited_execution_fds: tuple[int, ...] = (),
) -> dict[str, Any]:
    stdout_path = output_dir / f"case-{case}.stdout"
    stderr_path = output_dir / f"case-{case}.stderr"
    time_path = output_dir / f"case-{case}.time-v.txt"
    time_read_fd, time_write_fd = os.pipe()
    argv = case_argv(
        case,
        binary,
        time_binary,
        output_dir,
        f"/proc/self/fd/{time_write_fd}",
    )
    started_utc = utc_now()
    started_ns = time.monotonic_ns()
    timed_out = False
    deadline_kill_observation: dict[str, Any] | None = None
    child_environment = dict(FROZEN_CHILD_ENVIRONMENT)
    child_pass_fds = tuple(
        dict.fromkeys((time_write_fd, *inherited_execution_fds))
    )
    time_write_fd_open = True
    time_read_fd_open = True
    stream_states: dict[str, dict[str, Any]] = {
        "stdout": {},
        "stderr": {},
        "time_v": {},
    }
    drains: tuple[tuple[str, Any, Path, int], ...] = ()
    drain_threads: list[
        tuple[threading.Thread, str, Any, threading.Event]
    ] = []
    all_drain_threads_joined = True
    progress = {
        "case": case,
        "ordinal": ordinal,
        "argv": argv,
        "process_pid": None,
        "started_utc": started_utc,
        "started_monotonic_ns": started_ns,
        "process": None,
        "endpoint_fsync": {},
        "endpoint_fsync_errors": [],
    }
    post_popen_error: Exception | None = None
    try:
        process = subprocess.Popen(
            argv,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=child_environment,
            start_new_session=True,
            pass_fds=child_pass_fds,
        )
    except Exception as error:
        os.close(time_read_fd)
        os.close(time_write_fd)
        raise CaseLaunchFailure(
            ordinal=ordinal,
            case=case,
            argv=argv,
            attempted_utc=started_utc,
            attempted_monotonic_ns=started_ns,
            cause=error,
        ) from error
    try:
        progress["process_pid"] = process.pid
        progress["process"] = process
        if on_process_started is not None:
            on_process_started(progress)
        os.close(time_write_fd)
        time_write_fd_open = False
        assert process.stdout is not None and process.stderr is not None
        time_stream = os.fdopen(time_read_fd, "rb", buffering=0)
        time_read_fd_open = False
        drains = (
            ("stdout", process.stdout, stdout_path, RAW_PRODUCER_JSON_MAX_BYTES),
            ("stderr", process.stderr, stderr_path, STDERR_MAX_BYTES),
            ("time_v", time_stream, time_path, TIME_V_MAX_BYTES),
        )
        for name, stream, path, cap_bytes in drains:
            stop_event = threading.Event()
            thread = threading.Thread(
                target=drain_bounded_stream,
                args=(
                    stream,
                    path,
                    cap_bytes,
                    process.pid,
                    stream_states[name],
                    stop_event,
                ),
                name=f"rr9-{case}-{name}-drain",
                daemon=True,
            )
            thread.start()
            drain_threads.append((thread, name, stream, stop_event))
        remaining_seconds = (
            outer_deadline_ns - time.monotonic_ns()
        ) / 1_000_000_000
        if remaining_seconds <= 0:
            timed_out = True
            deadline_kill_observation = kill_group_at_outer_deadline(process)
            exit_code = process.returncode
        else:
            try:
                exit_code = process.wait(timeout=remaining_seconds)
            except subprocess.TimeoutExpired:
                timed_out = True
                deadline_kill_observation = kill_group_at_outer_deadline(process)
                exit_code = process.returncode
    except Exception as error:
        post_popen_error = error
    finally:
        if time_write_fd_open:
            os.close(time_write_fd)
        if process.poll() is None:
            final_kill_observation = kill_group_at_outer_deadline(process)
            if timed_out and deadline_kill_observation is None:
                deadline_kill_observation = final_kill_observation
            elif (
                timed_out
                and deadline_kill_observation is not None
                and final_kill_observation["leader_terminal_observed"]
            ):
                deadline_kill_observation = final_kill_observation
        process_group_cleanup = terminate_surviving_process_group(
            process.pid, outer_deadline_ns
        )
        process_finished_ns = reconcile_process_terminal_observation(
            process, process_group_cleanup
        )
        progress.update(
            {
                "process_finished_monotonic_ns": process_finished_ns,
                "exit_code": process.returncode,
                "outer_timeout": timed_out,
                "deadline_kill_observation": deadline_kill_observation,
                "process_group_cleanup": process_group_cleanup,
            }
        )
        if time_read_fd_open:
            os.close(time_read_fd)
        started_stream_ids = {
            id(stream) for _, _, stream, _ in drain_threads
        }
        unstarted_streams = [process.stdout, process.stderr]
        unstarted_streams.extend(stream for _, stream, _, _ in drains)
        closed_stream_ids: set[int] = set()
        for stream in unstarted_streams:
            if stream is None:
                continue
            stream_id = id(stream)
            if (
                stream_id not in started_stream_ids
                and stream_id not in closed_stream_ids
            ):
                try:
                    stream.close()
                except Exception:
                    pass
                closed_stream_ids.add(stream_id)
        # Group cleanup has attempted to make every in-domain writer terminal.
        # A set stop event only ends a drain on EAGAIN (for example, an escaped
        # writer retaining a pipe); queued bytes still drain and flush to EOF.
        # Deadline-bounded helper fsyncs run only after every drain has joined.
        for _, _, _, stop_event in drain_threads:
            stop_event.set()
        for thread, stream_name, stream, stop_event in drain_threads:
            remaining_seconds = max(
                0.0,
                (outer_deadline_ns - time.monotonic_ns()) / 1_000_000_000,
            )
            thread.join(timeout=remaining_seconds)
            if not thread.is_alive():
                continue
            all_drain_threads_joined = False
            stream_states[stream_name]["observation_complete"] = False
            stream_states[stream_name]["observed_bytes_kind"] = "lower_bound"
            stream_states[stream_name]["termination_reason"] = (
                "drain_incomplete_after_group_cleanup"
            )
            stream_states[stream_name]["drain_error"] = {
                "type": "DrainDidNotTerminate",
                "message": "pipe remained open after process-group cleanup",
            }
        progress["stream_caps"] = stream_states
        progress["all_drain_threads_joined"] = all_drain_threads_joined
    if post_popen_error is not None:
        raise StartedCaseFailure(progress, post_popen_error) from post_popen_error
    exit_code = process.returncode
    if not all_drain_threads_joined:
        raise RuntimeError("drain thread remained alive at evidence endpoint")
    stdout_fsync, stdout_artifact, stdout_bytes, stdout_endpoint_errors = (
        fsync_and_snapshot_artifact(
            stdout_path,
            stream_states["stdout"],
            RAW_PRODUCER_JSON_MAX_BYTES,
            outer_deadline_ns,
            retain_bytes=True,
        )
    )
    progress["endpoint_fsync"]["stdout"] = stdout_fsync
    progress["endpoint_fsync_errors"].extend(
        f"stdout:{error}" for error in stdout_endpoint_errors
    )
    stderr_fsync, stderr_artifact, _, stderr_endpoint_errors = (
        fsync_and_snapshot_artifact(
            stderr_path,
            stream_states["stderr"],
            STDERR_MAX_BYTES,
            outer_deadline_ns,
            retain_bytes=False,
        )
    )
    progress["endpoint_fsync"]["stderr"] = stderr_fsync
    progress["endpoint_fsync_errors"].extend(
        f"stderr:{error}" for error in stderr_endpoint_errors
    )
    time_fsync, time_artifact, time_bytes, time_endpoint_errors = (
        fsync_and_snapshot_artifact(
            time_path,
            stream_states["time_v"],
            TIME_V_MAX_BYTES,
            outer_deadline_ns,
            retain_bytes=True,
        )
    )
    progress["endpoint_fsync"]["time_v"] = time_fsync
    progress["endpoint_fsync_errors"].extend(
        f"time_v:{error}" for error in time_endpoint_errors
    )
    endpoint_fsync = {
        "stdout": stdout_fsync,
        "stderr": stderr_fsync,
        "time_v": time_fsync,
    }
    endpoint_fsync_errors = (
        [f"stdout:{error}" for error in stdout_endpoint_errors]
        + [f"stderr:{error}" for error in stderr_endpoint_errors]
        + [f"time_v:{error}" for error in time_endpoint_errors]
    )
    progress["endpoint_fsync"] = endpoint_fsync
    progress["endpoint_fsync_errors"] = endpoint_fsync_errors
    stream_binding_errors = stream_snapshot_binding_errors(
        stream_states,
        {
            "stdout": stdout_artifact,
            "stderr": stderr_artifact,
            "time_v": time_artifact,
        },
    )
    stdout_binding_failed = any(
        error.startswith("stdout_") for error in stream_binding_errors
    ) or not stream_state_parseable(stream_states["stdout"])
    time_binding_failed = any(
        error.startswith("time_v_") for error in stream_binding_errors
    ) or not stream_state_parseable(stream_states["time_v"])
    observed = (
        None
        if stdout_bytes is None or stdout_binding_failed
        else observed_header_bytes(stdout_bytes)
    )
    resource_observations, resource_errors = time_v_report_bytes(
        None if time_binding_failed else time_bytes
    )
    if (
        resource_observations is not None
        and resource_observations.get("time_exit_status") != exit_code
    ):
        resource_errors.append("time_v_exit_status_mismatch")
    interface_errors = producer_interface_errors(observed, case)
    raw_capture_errors = list(endpoint_fsync_errors)
    stream_cap_errors: list[str] = []
    for name, state in stream_states.items():
        if state.get("cap_exceeded", False):
            stream_cap_errors.append(f"{name}_write_cap_exceeded")
        if not state.get("observation_complete", False):
            stream_cap_errors.append(f"{name}_drain_incomplete")
        if state.get("drain_error") is not None:
            stream_cap_errors.append(f"{name}_drain_error")
    process_group_cleanup_errors = (
        []
        if process_group_cleanup["absence_verified"]
        else ["process_group_absence_not_verified"]
    )
    evidence_finished_ns = time.monotonic_ns()
    evidence_endpoint_complete = (
        all_drain_threads_joined
        and process_group_cleanup["absence_verified"]
        and (
            deadline_kill_observation is None
            or deadline_kill_observation["leader_terminal_observed"] is True
        )
        and evidence_finished_ns <= outer_deadline_ns
        and not endpoint_fsync_errors
        and all(
            entry.get("success") is True for entry in endpoint_fsync.values()
        )
        and all(
            state.get("observation_complete") is True
            for state in stream_states.values()
        )
    )
    deadline_errors = (
        ["case_evidence_sealed_after_outer_deadline"]
        if evidence_finished_ns > outer_deadline_ns
        else []
    )
    return {
        "ordinal": ordinal,
        "case": case,
        "argv": argv,
        "started_utc": started_utc,
        "finished_utc": utc_now(),
        "started_monotonic_ns": started_ns,
        "process_finished_monotonic_ns": process_finished_ns,
        "evidence_finished_monotonic_ns": evidence_finished_ns,
        "elapsed_ns": evidence_finished_ns - started_ns,
        "exit_code": exit_code,
        "process_started": True,
        "process_pid": process.pid,
        "outer_timeout": timed_out,
        "deadline_kill_observation": deadline_kill_observation,
        "dedicated_process_session": True,
        "post_leader_group_cleanup_attempted": True,
        "surviving_group_signaled": process_group_cleanup["term_sent"],
        "process_group_cleanup": process_group_cleanup,
        "process_group_cleanup_errors": process_group_cleanup_errors,
        "evidence_endpoint_complete": evidence_endpoint_complete,
        "endpoint_fsync": endpoint_fsync,
        "endpoint_fsync_errors": endpoint_fsync_errors,
        "stream_caps": stream_states,
        "stream_cap_errors": stream_cap_errors,
        "stream_binding_errors": stream_binding_errors,
        "stdout": stdout_artifact,
        "stderr": stderr_artifact,
        "time_v": time_artifact,
        "time_v_observations": resource_observations,
        "time_v_resource_errors": resource_errors,
        "observed_output_header": observed,
        "producer_interface_errors": interface_errors,
        "raw_artifact_capture_errors": raw_capture_errors,
        "custody_errors": [],
        "deadline_errors": deadline_errors,
    }


def write_json_new(path: Path, value: Any) -> dict[str, Any]:
    encoded = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    with path.open("xb") as handle:
        handle.write(encoded)
        handle.flush()
        os.fsync(handle.fileno())
        observed = os.fstat(handle.fileno())
    return {
        "path": str(path.resolve(strict=True)),
        "present": True,
        "bytes": len(encoded),
        "sha256": hashlib.sha256(encoded).hexdigest(),
        "hash_complete": True,
        "device": observed.st_dev,
        "inode": observed.st_ino,
        "mode": stat.S_IMODE(observed.st_mode),
    }


def write_json_replace(path: Path, value: Any) -> dict[str, Any]:
    encoded = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    binding_path = str(path.parent.resolve(strict=True) / path.name)
    encoded_sha256 = hashlib.sha256(encoded).hexdigest()
    descriptor = -1
    temporary: Path | None = None
    flags = (
        os.O_WRONLY
        | os.O_CREAT
        | os.O_EXCL
        | getattr(os, "O_NOFOLLOW", 0)
    )
    for _ in range(32):
        candidate = path.with_name(
            f".{path.name}.replacement-{uuid.uuid4().hex}.tmp"
        )
        try:
            descriptor = os.open(candidate, flags, 0o600)
        except FileExistsError:
            continue
        temporary = candidate
        break
    if descriptor < 0 or temporary is None:
        raise FileExistsError("could not create private JSON replacement")
    try:
        _write_all(descriptor, encoded)
        os.fsync(descriptor)
        written = os.fstat(descriptor)
        named = os.stat(temporary, follow_symlinks=False)
        if (written.st_dev, written.st_ino) != (named.st_dev, named.st_ino):
            raise OSError("private JSON replacement inode changed")
        os.close(descriptor)
        descriptor = -1
        os.replace(temporary, path)
        temporary = None
        installed = os.stat(path, follow_symlinks=False)
        if (written.st_dev, written.st_ino) != (
            installed.st_dev,
            installed.st_ino,
        ):
            raise OSError("installed JSON replacement inode mismatch")
        binding = {
            "path": binding_path,
            "present": True,
            "bytes": len(encoded),
            "sha256": encoded_sha256,
            "hash_complete": True,
            "device": installed.st_dev,
            "inode": installed.st_ino,
            "mode": stat.S_IMODE(installed.st_mode),
        }
        fsync_directory(path.parent)
    finally:
        if descriptor >= 0:
            os.close(descriptor)
        if temporary is not None:
            try:
                temporary.unlink()
            except FileNotFoundError:
                pass
    return binding


def recheck_bound_artifact(
    path: Path, expected: dict[str, Any], label: str
) -> dict[str, Any]:
    expected_bytes = expected.get("bytes")
    if type(expected_bytes) is not int or expected_bytes < 0:
        raise ValueError(f"{label} expected size is invalid")
    observed, _, errors = capture_raw_artifact(
        path, buffer_limit=expected_bytes
    )
    observed["path"] = expected.get("path", str(path))
    if errors or not artifact_matches(expected, observed):
        details = errors or ["binding_mismatch"]
        raise ValueError(f"{label} changed: {','.join(details)}")
    return observed


def validate_completion_artifact(
    path: Path,
    expected: dict[str, Any],
    *,
    schema: str,
    launch_attempt_id: str,
    label: str,
) -> dict[str, Any]:
    """Require the exact bound completion bytes and their identifying schema."""

    expected_bytes = expected.get("bytes")
    if type(expected_bytes) is not int or expected_bytes < 0:
        raise ValueError(f"{label} expected size is invalid")
    observed, encoded, errors = capture_raw_artifact(
        path, buffer_limit=expected_bytes
    )
    observed["path"] = expected.get("path", str(path))
    if errors or not artifact_matches(expected, observed) or encoded is None:
        details = errors or ["binding_mismatch"]
        raise ValueError(f"{label} changed: {','.join(details)}")
    try:
        decoded = json.loads(encoded)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"{label} is not valid JSON") from error
    if not isinstance(decoded, dict):
        raise ValueError(f"{label} must be a JSON object")
    if decoded.get("schema") != schema:
        raise ValueError(f"{label} schema mismatch")
    if decoded.get("launch_attempt_id") != launch_attempt_id:
        raise ValueError(f"{label} launch-attempt mismatch")
    return observed


def fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def resource_snapshot() -> dict[str, Any]:
    self_usage = resource.getrusage(resource.RUSAGE_SELF)
    child_usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    return {
        "self": {
            "user_seconds": self_usage.ru_utime,
            "system_seconds": self_usage.ru_stime,
            "maximum_resident_set_kib": self_usage.ru_maxrss,
        },
        "children": {
            "user_seconds": child_usage.ru_utime,
            "system_seconds": child_usage.ru_stime,
            "maximum_resident_set_kib": child_usage.ru_maxrss,
        },
    }


def resource_delta(
    started: dict[str, Any], finished: dict[str, Any]
) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for scope in ("self", "children"):
        result[scope] = {
            "user_seconds": (
                finished[scope]["user_seconds"]
                - started[scope]["user_seconds"]
            ),
            "system_seconds": (
                finished[scope]["system_seconds"]
                - started[scope]["system_seconds"]
            ),
            "maximum_resident_set_kib": finished[scope][
                "maximum_resident_set_kib"
            ],
        }
    return result


def producer_cap_passed(
    all_cases_attempted: bool,
    suite_finished_ns: int,
    outer_deadline_ns: int,
) -> bool:
    return all_cases_attempted and suite_finished_ns <= outer_deadline_ns


def producer_cap_endpoint(outer_deadline_ns: int) -> tuple[int, bool]:
    finished_ns = time.monotonic_ns()
    return finished_ns, finished_ns <= outer_deadline_ns


def seal_suite_receipt(
    output_dir: Path,
    receipt: dict[str, Any],
    *,
    sealing_attempt_start: dict[str, Any],
) -> dict[str, Any]:
    """Durably seal raw files and the receipt outside the producer cap."""

    started_ns = sealing_attempt_start["monotonic_ns"]
    started_usage = sealing_attempt_start["resource_usage"]
    if not all(
        row.get("evidence_endpoint_complete") is True
        for row in receipt.get("cases", [])
    ):
        raise ValueError("evidence endpoint is incomplete")
    seal_start_custody, custody_drift = custody_recheck(
        receipt["execution_custody"]
    )
    if custody_drift:
        raise ValueError(
            "staged custody drift before seal: " + ",".join(custody_drift)
        )
    drift = raw_artifact_errors(receipt, output_dir)
    if drift:
        raise ValueError("raw artifact drift before seal: " + ",".join(drift))
    seal_start_bootstrap, bootstrap_drift = bootstrap_binding_recheck(
        receipt["launcher_bootstrap"]
    )
    if bootstrap_drift:
        raise ValueError(
            "launcher bootstrap drift before seal: "
            + ",".join(bootstrap_drift)
        )
    memfd_drift = sealed_memfd_errors(receipt["sealed_execution_memfds"])
    memfd_drift.extend(
        memfd_proc_path_errors(receipt["sealed_execution_memfds"])
    )
    if memfd_drift:
        raise ValueError("sealed memfd drift before seal: " + ",".join(memfd_drift))
    containment_at_seal, containment_drift = containment_recheck(
        receipt["process_containment"]
    )
    if containment_drift:
        raise ValueError(
            "containment drift before seal: " + ",".join(containment_drift)
        )
    manifest_drift = bounded_file_binding_errors(
        receipt["custody_manifest"],
        CUSTODY_MANIFEST_MAX_BYTES,
        "custody_manifest",
    )
    if manifest_drift:
        raise ValueError(
            "custody manifest drift before seal: " + ",".join(manifest_drift)
        )
    endpoint_fsync_receipts: list[dict[str, Any]] = []
    for row in receipt.get("cases", []):
        case = row["case"]
        endpoint_fsync = row.get("endpoint_fsync")
        if not isinstance(endpoint_fsync, dict):
            raise ValueError(f"case {case} endpoint fsync receipt is missing")
        for field, _, _ in (
            ("stdout", "stdout", RAW_PRODUCER_JSON_MAX_BYTES),
            ("stderr", "stderr", STDERR_MAX_BYTES),
            ("time_v", "time-v.txt", TIME_V_MAX_BYTES),
        ):
            entry = endpoint_fsync.get(field)
            if (
                not isinstance(entry, dict)
                or entry.get("success") is not True
                or not isinstance(entry.get("post_fsync_snapshot"), dict)
                or not artifact_matches(entry["post_fsync_snapshot"], row[field])
            ):
                raise ValueError(
                    f"case {case} {field} endpoint fsync receipt is invalid"
                )
            endpoint_fsync_receipts.append(
                {"case": case, "stream": field, **entry}
            )
    if len(endpoint_fsync_receipts) != len(CASE_ORDER) * 3:
        raise ValueError("final seal requires exactly nine endpoint fsync receipts")
    drift = raw_artifact_errors(receipt, output_dir)
    if drift:
        raise ValueError("raw artifact drift during seal: " + ",".join(drift))
    final_bootstrap, bootstrap_drift = bootstrap_binding_recheck(
        receipt["launcher_bootstrap"]
    )
    if bootstrap_drift:
        raise ValueError(
            "launcher bootstrap drift during seal: "
            + ",".join(bootstrap_drift)
        )
    memfd_drift = sealed_memfd_errors(receipt["sealed_execution_memfds"])
    memfd_drift.extend(
        memfd_proc_path_errors(receipt["sealed_execution_memfds"])
    )
    if memfd_drift:
        raise ValueError("sealed memfd drift during seal: " + ",".join(memfd_drift))
    final_containment, containment_drift = containment_recheck(
        receipt["process_containment"]
    )
    if containment_drift:
        raise ValueError(
            "containment drift during seal: " + ",".join(containment_drift)
        )
    manifest_drift = bounded_file_binding_errors(
        receipt["custody_manifest"],
        CUSTODY_MANIFEST_MAX_BYTES,
        "custody_manifest",
    )
    if manifest_drift:
        raise ValueError(
            "custody manifest drift during seal: " + ",".join(manifest_drift)
        )
    final_custody, custody_drift = custody_recheck(
        receipt["execution_custody"]
    )
    if custody_drift:
        raise ValueError(
            "staged custody drift during seal: " + ",".join(custody_drift)
        )
    receipt["final_seal_containment"] = final_containment
    receipt["seal_start_custody"] = seal_start_custody
    receipt["final_seal_custody"] = final_custody
    receipt["final_seal_custody_errors"] = []
    receipt["seal_start_bootstrap"] = seal_start_bootstrap
    receipt["final_seal_bootstrap"] = final_bootstrap
    receipt["final_seal_bootstrap_errors"] = []
    receipt["final_seal_memfd_errors"] = []
    receipt["final_seal_custody_manifest_errors"] = []
    receipt_path = output_dir / "suite-receipt.json"
    receipt_binding = write_json_new(receipt_path, receipt)
    fsync_directory(output_dir)
    sealed_receipt_binding = recheck_bound_artifact(
        receipt_path, receipt_binding, "suite receipt"
    )
    final_raw_rehash: list[dict[str, Any]] = []
    for row in receipt.get("cases", []):
        case = row["case"]
        for field, suffix, cap_bytes in (
            ("stdout", "stdout", RAW_PRODUCER_JSON_MAX_BYTES),
            ("stderr", "stderr", STDERR_MAX_BYTES),
            ("time_v", "time-v.txt", TIME_V_MAX_BYTES),
        ):
            observed, _, capture_errors = capture_raw_artifact(
                output_dir / f"case-{case}.{suffix}",
                buffer_limit=cap_bytes,
            )
            if capture_errors or not artifact_matches(row[field], observed):
                details = capture_errors or ["binding_mismatch"]
                raise ValueError(
                    f"case {case} {field} final raw rehash failed: "
                    + ",".join(details)
                )
            observed["case"] = case
            observed["stream"] = field
            final_raw_rehash.append(observed)
    if len(final_raw_rehash) != len(CASE_ORDER) * 3:
        raise ValueError("final seal requires exactly nine raw rehash bindings")
    final_manifest_rehash = recheck_bound_artifact(
        Path(receipt["custody_manifest"]["path"]),
        receipt["custody_manifest"],
        "custody manifest",
    )
    seal = {
        "schema": "prime31.rr9.full8-f4-suite-seal.v1",
        "launch_attempt_id": receipt["launch_attempt_id"],
        "producer_cap_excluded": True,
        "started_monotonic_ns": started_ns,
        "required_completion_artifact": "suite-seal-completion.json",
        "receipt": sealed_receipt_binding,
        "raw_artifact_endpoint_fsync_receipts": endpoint_fsync_receipts,
        "raw_artifacts_refsynced_postcap": False,
        "raw_artifacts_final_rehash": final_raw_rehash,
        "custody_manifest_fsynced_preflight": receipt[
            "custody_manifest_fsynced_preflight"
        ],
        "custody_manifest_refsynced_postcap": False,
        "custody_manifest_final_rehash": final_manifest_rehash,
    }
    seal_path = output_dir / "suite-seal.json"
    seal_binding = write_json_new(seal_path, seal)
    fsync_directory(output_dir)
    sealed_receipt_binding = recheck_bound_artifact(
        receipt_path, sealed_receipt_binding, "suite receipt before completion"
    )
    sealed_seal_binding = recheck_bound_artifact(
        seal_path, seal_binding, "suite seal before completion"
    )
    finished_usage = resource_snapshot()
    finished_ns = time.monotonic_ns()
    completion = {
        "schema": "prime31.rr9.full8-f4-suite-seal-completion.v1",
        "launch_attempt_id": receipt["launch_attempt_id"],
        "producer_cap_excluded": True,
        "seal": sealed_seal_binding,
        "receipt": sealed_receipt_binding,
        "started_monotonic_ns": started_ns,
        "finished_monotonic_ns": finished_ns,
        "elapsed_ns": finished_ns - started_ns,
        "resource_start": started_usage,
        "resource_finish": finished_usage,
        "resource_delta": resource_delta(started_usage, finished_usage),
        "seal_and_directory_fsync_completed": True,
        "completion_receipt_emission_accounted_locally": False,
        "external_whole_job_accounting_required": True,
    }
    completion_path = output_dir / "suite-seal-completion.json"
    write_json_new(completion_path, completion)
    fsync_directory(output_dir)
    return seal


def persist_full_failure_receipt(
    output_dir: Path,
    receipt: dict[str, Any],
    error: Exception,
    *,
    sealing_attempt_start: dict[str, Any],
) -> None:
    """Persist the complete in-memory evidence before the small failure marker."""

    started_ns = sealing_attempt_start["monotonic_ns"]
    started_usage = sealing_attempt_start["resource_usage"]
    producer_cap_excluded = (
        sealing_attempt_start.get("stage") != "producer_suite"
    )
    receipt["launcher_error"] = True
    receipt["launcher_integrity_passed"] = False
    receipt["producer_suite_admissible"] = False
    receipt["receipt_seal_failure"] = {
        "type": type(error).__name__,
        "message": str(error),
        "recorded_utc": utc_now(),
        "recorded_monotonic_ns": time.monotonic_ns(),
        "resource_snapshot": resource_snapshot(),
    }
    receipt_path = output_dir / "suite-receipt.json"
    if receipt_path.exists():
        receipt_binding = write_json_replace(receipt_path, receipt)
    else:
        receipt_binding = write_json_new(receipt_path, receipt)
        fsync_directory(output_dir)
    receipt_binding = recheck_bound_artifact(
        receipt_path, receipt_binding, "failure suite receipt"
    )
    failure_path = output_dir / "suite-seal-failure.json"
    failure = {
        "schema": "prime31.rr9.full8-f4-suite-seal-failure.v1",
        "launch_attempt_id": receipt["launch_attempt_id"],
        "producer_cap_excluded": producer_cap_excluded,
        "required_completion_artifact": (
            "suite-seal-failure-completion.json"
        ),
        "receipt": receipt_binding,
        "type": type(error).__name__,
        "message": str(error),
    }
    if failure_path.exists():
        failure_binding = write_json_replace(failure_path, failure)
    else:
        failure_binding = write_json_new(failure_path, failure)
        fsync_directory(output_dir)
    receipt_binding = recheck_bound_artifact(
        receipt_path, receipt_binding, "failure suite receipt before completion"
    )
    failure_binding = recheck_bound_artifact(
        failure_path, failure_binding, "failure marker before completion"
    )
    finished_usage = resource_snapshot()
    finished_ns = time.monotonic_ns()
    completion = {
        "schema": (
            "prime31.rr9.full8-f4-suite-seal-failure-completion.v1"
        ),
        "launch_attempt_id": receipt["launch_attempt_id"],
        "producer_cap_excluded": producer_cap_excluded,
        "receipt": receipt_binding,
        "failure_marker": failure_binding,
        "started_monotonic_ns": started_ns,
        "finished_monotonic_ns": finished_ns,
        "elapsed_ns": finished_ns - started_ns,
        "resource_start": started_usage,
        "resource_finish": finished_usage,
        "resource_delta": resource_delta(started_usage, finished_usage),
        "failure_marker_and_directory_fsync_completed": True,
        "completion_receipt_emission_accounted_locally": False,
        "external_whole_job_accounting_required": True,
    }
    completion_path = output_dir / "suite-seal-failure-completion.json"
    if completion_path.exists():
        write_json_replace(completion_path, completion)
    else:
        write_json_new(completion_path, completion)
        fsync_directory(output_dir)


def _run_suite_impl(
    binary: Path,
    protocol: Path,
    source: Path,
    verifier_source: Path,
    verifier_executable: Path,
    time_binary: Path,
    cargo_lock: Path,
    cargo_manifest: Path,
    output_dir: Path,
    repo: Path,
    image_digest: str | None = None,
    process_containment_guarantee: str | None = None,
    *,
    memfd_handle_sink: dict[str, int],
    state_sink: dict[str, Any],
) -> tuple[dict[str, Any], int]:
    preflight_started_utc = utc_now()
    preflight_started_ns = time.monotonic_ns()
    preflight_started_usage = resource_snapshot()
    output_dir = output_dir.resolve()
    output_dir.mkdir(mode=0o700, parents=False, exist_ok=False)
    paths = input_paths(
        binary,
        protocol,
        source,
        verifier_source,
        verifier_executable,
        time_binary,
        cargo_lock,
        cargo_manifest,
    )
    launch_attempt_id = str(uuid.uuid4())
    memfd_handles: dict[str, int] = {}
    memfd_receipts: dict[str, dict[str, Any]] = {}
    try:
        if not image_digest:
            raise ValueError("container image digest is required")
        if process_containment_guarantee is None:
            raise ValueError("process containment guarantee is required")
        containment = process_containment_receipt(process_containment_guarantee)
        launcher_bootstrap = {
            "trust_model": "bootstrap_trusted_not_staged",
            "external_image_and_eks_corroboration_required": True,
            "launcher_source": bootstrap_binding(Path(__file__)),
            "python_executable": bootstrap_binding(Path(sys.executable)),
        }
        bindings = validate_inputs(
            binary,
            protocol,
            source,
            verifier_source,
            verifier_executable,
            time_binary,
            cargo_lock,
            cargo_manifest,
        )
        repository = repository_receipt(repo, paths)
        environment = environment_receipt(image_digest)
        custody = stage_inputs(output_dir / "custody", paths, bindings)
        fsync_directory(output_dir / "custody")
        memfd_handles, memfd_receipts = create_execution_memfds(custody)
        memfd_handle_sink.update(memfd_handles)
        memfd_path_errors = memfd_proc_path_errors(memfd_receipts)
        if memfd_path_errors:
            raise RuntimeError(
                "sealed memfd proc path check failed: "
                + ",".join(memfd_path_errors)
            )
        memfd_child_access_probe = probe_memfd_child_access(memfd_receipts)
        custody_manifest = {
            "schema": "prime31.rr9.full8-f4-custody.v1",
            "launch_attempt_id": launch_attempt_id,
            "container_image_digest": image_digest,
            "process_containment": containment,
            "launcher_bootstrap": launcher_bootstrap,
            "bindings": bindings,
            "staged": custody,
            "sealed_execution_memfds": memfd_receipts,
            "sealed_memfd_proc_path_errors": [],
            "sealed_memfd_child_access_probe": memfd_child_access_probe,
        }
        custody_manifest_path = output_dir / "custody-manifest.json"
        custody_manifest_fsynced_preflight = write_json_new(
            custody_manifest_path, custody_manifest
        )
        fsync_directory(output_dir)
        custody_manifest_artifact = bounded_file_binding(
            custody_manifest_path,
            CUSTODY_MANIFEST_MAX_BYTES,
            "custody_manifest",
        )
        if not artifact_matches(
            custody_manifest_fsynced_preflight, custody_manifest_artifact
        ):
            raise ValueError("custody manifest changed after preflight fsync")
    except Exception as error:
        preflight_finished_usage = resource_snapshot()
        preflight_finished_ns = time.monotonic_ns()
        failure_receipt = {
            "schema": SCHEMA,
            "mode": "execution",
            "launch_attempt_id": launch_attempt_id,
            "producer_suite_started": False,
            "preflight": {
                "producer_cap_excluded": True,
                "started_utc": preflight_started_utc,
                "finished_utc": utc_now(),
                "started_monotonic_ns": preflight_started_ns,
                "finished_monotonic_ns": preflight_finished_ns,
                "elapsed_ns": preflight_finished_ns - preflight_started_ns,
                "resource_start": preflight_started_usage,
                "resource_finish": preflight_finished_usage,
                "resource_delta": resource_delta(
                    preflight_started_usage, preflight_finished_usage
                ),
            },
            "preflight_failure": {
                "type": type(error).__name__,
                "message": str(error),
            },
            "producer_invoked": False,
            "producer_invocation_count": 0,
            "scientific_evaluation_performed": False,
            "comparison_admissible": False,
        }
        write_json_new(output_dir / "suite-receipt.json", failure_receipt)
        fsync_directory(output_dir)
        return failure_receipt, 1

    # The frozen producer-suite clock begins only after the complete preflight,
    # fresh directory, staged custody, and durable custody manifest exist.
    preflight_finished_usage = resource_snapshot()
    preflight_finished_ns = time.monotonic_ns()
    preflight = {
        "producer_cap_excluded": True,
        "started_utc": preflight_started_utc,
        "finished_utc": utc_now(),
        "started_monotonic_ns": preflight_started_ns,
        "finished_monotonic_ns": preflight_finished_ns,
        "elapsed_ns": preflight_finished_ns - preflight_started_ns,
        "resource_start": preflight_started_usage,
        "resource_finish": preflight_finished_usage,
        "resource_delta": resource_delta(
            preflight_started_usage, preflight_finished_usage
        ),
    }
    suite_start_ns = preflight_finished_ns
    outer_deadline_ns = suite_start_ns + OUTER_CAP_SECONDS * 1_000_000_000
    receipt: dict[str, Any] = {
        "schema": SCHEMA,
        "mode": "execution",
        "launch_attempt_id": launch_attempt_id,
        "case_order": list(CASE_ORDER),
        "outer_cap_seconds": OUTER_CAP_SECONDS,
        "outer_cap_scope": OUTER_CAP_SCOPE,
        "receipt_sealing_excluded_from_producer_cap": True,
        "started_utc": utc_now(),
        "started_monotonic_ns": suite_start_ns,
        "outer_deadline_monotonic_ns": outer_deadline_ns,
        "bindings": bindings,
        "execution_custody": custody,
        "sealed_execution_memfds": memfd_receipts,
        "sealed_memfd_child_access_probe": memfd_child_access_probe,
        "custody_manifest": custody_manifest_artifact,
        "custody_manifest_fsynced_preflight": (
            custody_manifest_fsynced_preflight
        ),
        "preflight": preflight,
        "repository": repository,
        "environment": environment,
        "process_containment": containment,
        "launcher_bootstrap": launcher_bootstrap,
        "producer_invoked": False,
        "producer_invocation_count": 0,
        "scientific_evaluation_performed": False,
        "comparison_admissible": False,
        "required_followup_links": followup_links(),
        "receipt_seal_artifact": "suite-seal.json",
        "cases": [],
    }
    state_sink["receipt"] = receipt
    state_sink["launch_attempt_id"] = launch_attempt_id
    state_sink["open_accounting_boundary"] = {
        "monotonic_ns": suite_start_ns,
        "resource_usage": preflight_finished_usage,
        "stage": "producer_suite",
    }
    launcher_error = False
    sealed_binary = memfd_receipts["binary"]["wrapper_owned_path"]
    sealed_time_binary = memfd_receipts["time_binary"]["proc_self_path"]
    sealed_time_fd = memfd_handles["time_binary"]
    current_ordinal: int | None = None
    current_case: int | None = None
    launch_state: dict[str, Any] = {"count": 0, "current": None}

    def mark_process_started(progress: dict[str, Any]) -> None:
        launch_state["count"] += 1
        launch_state["current"] = progress

    try:
        for ordinal, case in enumerate(CASE_ORDER):
            current_ordinal = ordinal
            current_case = case
            if time.monotonic_ns() >= outer_deadline_ns:
                case_raw, case_raw_errors = capture_case_raw_artifacts(
                    output_dir, case
                )
                receipt["cases"].append(
                    {
                        "ordinal": ordinal,
                        "case": case,
                        "argv": case_argv(
                            case, sealed_binary, sealed_time_binary, output_dir
                        ),
                        "not_started_outer_cap": True,
                        "started_utc": None,
                        "finished_utc": None,
                        "started_monotonic_ns": None,
                        "process_finished_monotonic_ns": None,
                        "evidence_finished_monotonic_ns": None,
                        "elapsed_ns": None,
                        "exit_code": None,
                        "process_started": False,
                        "process_pid": None,
                        "outer_timeout": True,
                        "stdout": case_raw["stdout"],
                        "stderr": case_raw["stderr"],
                        "time_v": case_raw["time_v"],
                        "time_v_observations": None,
                        "time_v_resource_errors": ["not_started_outer_cap"],
                        "observed_output_header": None,
                        "producer_interface_errors": ["not_started_outer_cap"],
                        "stream_caps": None,
                        "stream_cap_errors": [],
                        "stream_binding_errors": [],
                        "process_group_cleanup": None,
                        "process_group_cleanup_errors": [],
                        "evidence_endpoint_complete": False,
                        "endpoint_fsync": None,
                        "endpoint_fsync_errors": ["not_started_outer_cap"],
                        "raw_artifact_capture_errors": case_raw_errors,
                        "custody_errors": [],
                        "deadline_errors": ["not_started_outer_cap"],
                    }
                )
                launcher_error = True
                continue
            row = run_case(
                ordinal,
                case,
                sealed_binary,
                sealed_time_binary,
                output_dir,
                outer_deadline_ns,
                mark_process_started,
                (sealed_time_fd,),
            )
            receipt["cases"].append(row)
            launch_state["current"] = None
            if (
                row["outer_timeout"]
                or row["exit_code"] != 0
                or row["producer_interface_errors"]
                or row["time_v_resource_errors"]
                or row["stream_cap_errors"]
                or row["stream_binding_errors"]
                or row["endpoint_fsync_errors"]
                or row["process_group_cleanup_errors"]
                or row["raw_artifact_capture_errors"]
                or row["custody_errors"]
                or row["deadline_errors"]
            ):
                launcher_error = True
        current_ordinal = None
        current_case = None
    except Exception as error:  # preserve a failure receipt for launch/I/O errors
        launcher_error = True
        typed_case_error = isinstance(
            error, (CaseLaunchFailure, StartedCaseFailure)
        )
        reported_type = error.cause_type if typed_case_error else type(error).__name__
        reported_message = error.cause_message if typed_case_error else str(error)
        receipt["launcher_exception"] = {
            "type": reported_type,
            "message": reported_message,
            "ordinal": current_ordinal,
            "case": current_case,
        }
        already_recorded = {row["case"] for row in receipt["cases"]}
        if (
            isinstance(error, CaseLaunchFailure)
            and error.case not in already_recorded
        ):
            case_raw, case_raw_errors = capture_case_raw_artifacts(
                output_dir, error.case
            )
            receipt["cases"].append(
                {
                    "ordinal": error.ordinal,
                    "case": error.case,
                    "argv": error.argv,
                    "launch_failed_before_process_start": True,
                    "launch_failure": {
                        "type": error.cause_type,
                        "message": error.cause_message,
                    },
                    "launch_attempted_utc": error.attempted_utc,
                    "launch_attempted_monotonic_ns": (
                        error.attempted_monotonic_ns
                    ),
                    "started_utc": None,
                    "finished_utc": None,
                    "started_monotonic_ns": None,
                    "process_finished_monotonic_ns": None,
                    "evidence_finished_monotonic_ns": None,
                    "elapsed_ns": None,
                    "exit_code": None,
                    "process_started": False,
                    "process_pid": None,
                    "outer_timeout": False,
                    "stdout": case_raw["stdout"],
                    "stderr": case_raw["stderr"],
                    "time_v": case_raw["time_v"],
                    "time_v_observations": None,
                    "time_v_resource_errors": [
                        "launch_failed_before_process_start"
                    ],
                    "observed_output_header": None,
                    "producer_interface_errors": [
                        "launch_failed_before_process_start"
                    ],
                    "stream_caps": None,
                    "stream_cap_errors": [],
                    "stream_binding_errors": [],
                    "process_group_cleanup": None,
                    "process_group_cleanup_errors": [],
                    "evidence_endpoint_complete": False,
                    "endpoint_fsync": None,
                    "endpoint_fsync_errors": [
                        "launch_failed_before_process_start"
                    ],
                    "raw_artifact_capture_errors": case_raw_errors,
                    "custody_errors": [],
                    "deadline_errors": [],
                }
            )
            already_recorded.add(error.case)
        started_progress = (
            error.progress
            if isinstance(error, StartedCaseFailure)
            else launch_state.get("current")
        )
        if (
            isinstance(error, StartedCaseFailure)
            and launch_state.get("current") is not started_progress
        ):
            launch_state["count"] += 1
            launch_state["current"] = started_progress
        if (
            isinstance(started_progress, dict)
            and started_progress.get("case") not in already_recorded
        ):
            failed_case = started_progress["case"]
            failed_ordinal = started_progress["ordinal"]
            case_raw, case_raw_errors = capture_case_raw_artifacts(
                output_dir, failed_case
            )
            cleanup = started_progress.get("process_group_cleanup")
            cleanup_errors = (
                []
                if isinstance(cleanup, dict)
                and cleanup.get("absence_verified") is True
                else ["process_group_absence_not_verified"]
            )
            stream_states = started_progress.get("stream_caps")
            stream_cap_errors: list[str] = []
            if isinstance(stream_states, dict):
                for stream_name, state in stream_states.items():
                    if not isinstance(state, dict):
                        continue
                    if state.get("cap_exceeded", False):
                        stream_cap_errors.append(
                            f"{stream_name}_write_cap_exceeded"
                        )
                    if not state.get("observation_complete", False):
                        stream_cap_errors.append(f"{stream_name}_drain_incomplete")
                    if state.get("drain_error") is not None:
                        stream_cap_errors.append(f"{stream_name}_drain_error")
            failed_finished_ns = time.monotonic_ns()
            partial_endpoint_fsync = started_progress.get("endpoint_fsync")
            if not partial_endpoint_fsync:
                partial_endpoint_fsync = None
            receipt["cases"].append(
                {
                    "ordinal": failed_ordinal,
                    "case": failed_case,
                    "argv": started_progress["argv"],
                    "started_but_endpoint_failed": True,
                    "endpoint_failure": {
                        "type": reported_type,
                        "message": reported_message,
                    },
                    "process_pid": started_progress["process_pid"],
                    "process_started": True,
                    "started_utc": started_progress["started_utc"],
                    "finished_utc": utc_now(),
                    "started_monotonic_ns": started_progress[
                        "started_monotonic_ns"
                    ],
                    "process_finished_monotonic_ns": started_progress.get(
                        "process_finished_monotonic_ns"
                    ),
                    "evidence_finished_monotonic_ns": failed_finished_ns,
                    "elapsed_ns": (
                        failed_finished_ns
                        - started_progress["started_monotonic_ns"]
                    ),
                    "exit_code": started_progress.get("exit_code"),
                    "outer_timeout": started_progress.get("outer_timeout", False),
                    "deadline_kill_observation": started_progress.get(
                        "deadline_kill_observation"
                    ),
                    "dedicated_process_session": True,
                    "post_leader_group_cleanup_attempted": cleanup is not None,
                    "surviving_group_signaled": (
                        cleanup.get("term_sent")
                        if isinstance(cleanup, dict)
                        else None
                    ),
                    "stdout": case_raw["stdout"],
                    "stderr": case_raw["stderr"],
                    "time_v": case_raw["time_v"],
                    "time_v_observations": None,
                    "time_v_resource_errors": ["started_but_endpoint_failed"],
                    "observed_output_header": None,
                    "producer_interface_errors": ["started_but_endpoint_failed"],
                    "stream_caps": stream_states,
                    "stream_cap_errors": stream_cap_errors,
                    "stream_binding_errors": [],
                    "process_group_cleanup": cleanup,
                    "process_group_cleanup_errors": cleanup_errors,
                    "evidence_endpoint_complete": False,
                    "endpoint_fsync": partial_endpoint_fsync,
                    "endpoint_fsync_errors": (
                        started_progress.get("endpoint_fsync_errors")
                        or ["started_but_endpoint_failed"]
                    ),
                    "raw_artifact_capture_errors": case_raw_errors,
                    "custody_errors": [],
                    "deadline_errors": (
                        ["case_evidence_sealed_after_outer_deadline"]
                        if failed_finished_ns > outer_deadline_ns
                        else []
                    ),
                }
            )
            already_recorded.add(failed_case)
        for ordinal, case in enumerate(CASE_ORDER):
            if case in already_recorded:
                continue
            case_raw, case_raw_errors = capture_case_raw_artifacts(
                output_dir, case
            )
            receipt["cases"].append(
                {
                    "ordinal": ordinal,
                    "case": case,
                    "argv": case_argv(
                        case, sealed_binary, sealed_time_binary, output_dir
                    ),
                    "not_started_launcher_exception": True,
                    "started_utc": None,
                    "finished_utc": None,
                    "started_monotonic_ns": None,
                    "process_finished_monotonic_ns": None,
                    "evidence_finished_monotonic_ns": None,
                    "elapsed_ns": None,
                    "exit_code": None,
                    "process_started": False,
                    "process_pid": None,
                    "outer_timeout": False,
                    "stdout": case_raw["stdout"],
                    "stderr": case_raw["stderr"],
                    "time_v": case_raw["time_v"],
                    "time_v_observations": None,
                    "time_v_resource_errors": ["launcher_exception"],
                    "observed_output_header": None,
                    "producer_interface_errors": ["launcher_exception"],
                    "stream_caps": None,
                    "stream_cap_errors": [],
                    "stream_binding_errors": [],
                    "process_group_cleanup": None,
                    "process_group_cleanup_errors": [],
                    "evidence_endpoint_complete": False,
                    "endpoint_fsync": None,
                    "endpoint_fsync_errors": ["launcher_exception"],
                    "raw_artifact_capture_errors": case_raw_errors,
                    "custody_errors": [],
                    "deadline_errors": [],
                }
            )
    # This is the exact frozen producer-suite endpoint.  Custody rechecking and
    # durable receipt/directory sealing follow and are measured separately.
    suite_finished_usage = resource_snapshot()
    suite_finished_ns, endpoint_within_deadline = producer_cap_endpoint(
        outer_deadline_ns
    )
    postcap_started_ns = suite_finished_ns
    postcap_started_usage = suite_finished_usage
    receipt["finished_utc"] = utc_now()
    receipt["suite_finished_monotonic_ns"] = suite_finished_ns
    receipt["producer_suite_elapsed_ns"] = suite_finished_ns - suite_start_ns
    receipt["producer_suite_resource_accounting"] = {
        "outer_cap_included": True,
        "not_additive_with_case_time_v": True,
        "started_monotonic_ns": suite_start_ns,
        "finished_monotonic_ns": suite_finished_ns,
        "start": preflight_finished_usage,
        "finish": suite_finished_usage,
        "delta": resource_delta(preflight_finished_usage, suite_finished_usage),
        "includes_rusage_self": True,
        "includes_rusage_children": True,
    }
    state_sink["open_accounting_boundary"] = {
        "monotonic_ns": postcap_started_ns,
        "resource_usage": postcap_started_usage,
        "stage": "postcap_custody_recheck",
    }
    receipt["producer_invocation_count"] = launch_state["count"]
    receipt["producer_invoked"] = launch_state["count"] > 0
    receipt["scientific_evaluation_performed"] = launch_state["count"] > 0
    receipt["all_cases_attempted"] = all(
        not row.get("not_started_outer_cap", False)
        and not row.get("not_started_launcher_exception", False)
        and not row.get("launch_failed_before_process_start", False)
        for row in receipt["cases"]
    ) and len(receipt["cases"]) == len(CASE_ORDER)
    receipt["all_case_processes_terminal_without_outer_timeout"] = (
        receipt["all_cases_attempted"]
        and all(
            row.get("exit_code") is not None
            and not row.get("outer_timeout", False)
            and row.get("evidence_endpoint_complete") is True
            for row in receipt["cases"]
        )
    )
    launcher_integrity_error = launcher_error
    if not endpoint_within_deadline:
        launcher_error = True
        receipt["deadline_failure_stage"] = "producer_suite_postprocessing"
    complete_by_deadline = producer_cap_passed(
        receipt["all_case_processes_terminal_without_outer_timeout"],
        suite_finished_ns,
        outer_deadline_ns,
    )
    receipt["outer_cap_attestation"] = {
        "enforced": True,
        "boundary": (
            "starts after validation/fresh-directory/custody preflight; ends at "
            "suite_finished_monotonic_ns after all ordered producers exit, process-"
            "group drain, deadline-bounded helper fsync and same-FD snapshots for "
            "stdout/stderr/time-v, strict interface and resource parsing, and the "
            "producer-suite resource finish snapshot"
        ),
        "receipt_and_directory_sealing_excluded_and_separately_accounted": True,
        "complete_suite_finished_by_deadline": complete_by_deadline,
    }
    postcap_custody, final_custody_errors = custody_recheck(custody)
    final_raw_artifact_errors = raw_artifact_errors(receipt, output_dir)
    postcap_bootstrap, final_bootstrap_errors = bootstrap_binding_recheck(
        launcher_bootstrap
    )
    final_memfd_errors = sealed_memfd_errors(memfd_receipts)
    final_memfd_errors.extend(memfd_proc_path_errors(memfd_receipts))
    postcap_containment, final_containment_errors = containment_recheck(
        containment
    )
    final_custody_manifest_errors = bounded_file_binding_errors(
        custody_manifest_artifact,
        CUSTODY_MANIFEST_MAX_BYTES,
        "custody_manifest",
    )
    postcap_finished_usage = resource_snapshot()
    postcap_finished_ns = time.monotonic_ns()
    receipt["final_custody_errors"] = final_custody_errors
    receipt["postcap_custody"] = postcap_custody
    receipt["final_raw_artifact_errors"] = final_raw_artifact_errors
    receipt["postcap_bootstrap"] = postcap_bootstrap
    receipt["final_bootstrap_errors"] = final_bootstrap_errors
    receipt["final_memfd_errors"] = final_memfd_errors
    receipt["postcap_containment"] = postcap_containment
    receipt["final_containment_errors"] = final_containment_errors
    receipt["final_custody_manifest_errors"] = final_custody_manifest_errors
    receipt["postcap_custody_recheck"] = {
        "producer_cap_excluded": True,
        "started_monotonic_ns": postcap_started_ns,
        "finished_monotonic_ns": postcap_finished_ns,
        "elapsed_ns": postcap_finished_ns - postcap_started_ns,
        "resource_start": postcap_started_usage,
        "resource_finish": postcap_finished_usage,
        "resource_delta": resource_delta(
            postcap_started_usage, postcap_finished_usage
        ),
    }
    sealing_attempt_start = {
        "monotonic_ns": postcap_finished_ns,
        "resource_usage": postcap_finished_usage,
        "stage": "receipt_sealing_attempt",
    }
    state_sink["open_accounting_boundary"] = sealing_attempt_start
    if (
        final_custody_errors
        or final_raw_artifact_errors
        or final_bootstrap_errors
        or final_memfd_errors
        or final_containment_errors
        or final_custody_manifest_errors
    ):
        launcher_integrity_error = True
        launcher_error = True
    receipt["launcher_integrity_passed"] = not launcher_integrity_error
    receipt["producer_suite_admissible"] = (
        complete_by_deadline and not launcher_integrity_error
    )
    if not receipt["producer_suite_admissible"]:
        launcher_error = True
    receipt["launcher_error"] = launcher_error
    try:
        seal_suite_receipt(
            output_dir,
            receipt,
            sealing_attempt_start=sealing_attempt_start,
        )
    except Exception as error:
        launcher_error = True
        persist_full_failure_receipt(
            output_dir,
            receipt,
            error,
            sealing_attempt_start=sealing_attempt_start,
        )
    return receipt, 1 if launcher_error else 0


def run_suite(
    binary: Path,
    protocol: Path,
    source: Path,
    verifier_source: Path,
    verifier_executable: Path,
    time_binary: Path,
    cargo_lock: Path,
    cargo_manifest: Path,
    output_dir: Path,
    repo: Path,
    image_digest: str | None = None,
    process_containment_guarantee: str | None = None,
) -> tuple[dict[str, Any], int]:
    """Run one attempt while guaranteeing sealed execution-FD closure."""

    memfd_handles: dict[str, int] = {}
    state: dict[str, Any] = {}
    try:
        return _run_suite_impl(
            binary,
            protocol,
            source,
            verifier_source,
            verifier_executable,
            time_binary,
            cargo_lock,
            cargo_manifest,
            output_dir,
            repo,
            image_digest,
            process_containment_guarantee,
            memfd_handle_sink=memfd_handles,
            state_sink=state,
        )
    except Exception as error:
        receipt = state.get("receipt")
        if not isinstance(receipt, dict):
            raise
        receipt["unexpected_postcap_exception"] = {
            "type": type(error).__name__,
            "message": str(error),
        }
        sealing_attempt_start = state.get("open_accounting_boundary")
        if not isinstance(sealing_attempt_start, dict):
            raise
        try:
            persist_full_failure_receipt(
                output_dir.resolve(),
                receipt,
                error,
                sealing_attempt_start=sealing_attempt_start,
            )
        except Exception as persistence_error:
            receipt["failure_receipt_persistence_error"] = {
                "type": type(persistence_error).__name__,
                "message": str(persistence_error),
            }
        return receipt, 1
    finally:
        for descriptor in memfd_handles.values():
            try:
                os.close(descriptor)
            except OSError:
                pass


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--binary", type=Path, required=True)
    result.add_argument("--protocol", type=Path, default=DEFAULT_PROTOCOL)
    result.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    result.add_argument("--verifier-source", type=Path, required=True)
    result.add_argument("--verifier-executable", type=Path, required=True)
    result.add_argument("--time-binary", type=Path, default=DEFAULT_TIME)
    result.add_argument("--cargo-lock", type=Path, default=DEFAULT_CARGO_LOCK)
    result.add_argument("--cargo-manifest", type=Path, default=DEFAULT_CARGO_MANIFEST)
    result.add_argument("--repo", type=Path, default=REPO_ROOT)
    result.add_argument(
        "--image-digest",
        default=os.environ.get("RR9_FULL8_IMAGE_DIGEST"),
        help="container image digest to bind (or RR9_FULL8_IMAGE_DIGEST)",
    )
    result.add_argument(
        "--process-containment-guarantee",
        choices=(PROCESS_CONTAINMENT_GUARANTEE,),
        default=os.environ.get("RR9_FULL8_PROCESS_CONTAINMENT_GUARANTEE"),
        help=(
            "required runner guarantee (or "
            "RR9_FULL8_PROCESS_CONTAINMENT_GUARANTEE)"
        ),
    )
    result.add_argument("--output-dir", type=Path)
    result.add_argument("--dry-run", action="store_true")
    return result


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    if args.dry_run:
        if args.output_dir is not None and args.output_dir.exists():
            raise SystemExit("dry run refuses an existing --output-dir")
        print(
            json.dumps(
                plan(
                    args.binary,
                    args.protocol,
                    args.source,
                    args.verifier_source,
                    args.verifier_executable,
                    args.time_binary,
                    args.cargo_lock,
                    args.cargo_manifest,
                    args.repo,
                    args.image_digest,
                    args.process_containment_guarantee,
                ),
                indent=2,
                sort_keys=True,
            )
        )
        return 0
    if args.output_dir is None:
        raise SystemExit("--output-dir is required unless --dry-run is set")
    _, status = run_suite(
        args.binary,
        args.protocol,
        args.source,
        args.verifier_source,
        args.verifier_executable,
        args.time_binary,
        args.cargo_lock,
        args.cargo_manifest,
        args.output_dir,
        args.repo,
        args.image_digest,
        args.process_containment_guarantee,
    )
    return status


if __name__ == "__main__":
    raise SystemExit(main())
