#!/usr/bin/env python3
"""Build and record both K5500 index layouts before a frozen Pod deadline."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import signal
import subprocess
import sys
import tarfile
import time

from research.n83_two_tier_root_20261021 import probe_pod, runpod_guard


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
SOURCE_SHA256 = "1c741f92a932e452b8210a8c080200ffa6d2511a605721ab156564e0663874ab"
GIB = 1024**3
MIN_MEMORY = 40 * GIB
MIN_CPU = 4
PRODUCER_MARGIN = timedelta(minutes=20)
BUILD_CAP_SECONDS = 1200
STAGE_CAP_SECONDS = 2400


def now() -> datetime:
    return datetime.now(timezone.utc)


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def save(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def run_capped(command: list[str], cwd: Path, env: dict, seconds: int,
               stdout_path: Path, stderr_path: Path,
               affinity: list[int] | None = None) -> dict:
    if seconds <= 0:
        return {"status": "SKIPPED_DEADLINE", "cap_seconds": seconds}
    started = now()
    began = time.monotonic()
    set_affinity = (
        (lambda: os.sched_setaffinity(0, affinity))
        if affinity is not None else None
    )
    with stdout_path.open("wb") as stdout, stderr_path.open("wb") as stderr:
        process = subprocess.Popen(
            command, cwd=cwd, env=env, stdout=stdout, stderr=stderr,
            start_new_session=True, preexec_fn=set_affinity,
        )
        try:
            code = process.wait(timeout=seconds)
            status = "PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"
        except subprocess.TimeoutExpired:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                process.wait(timeout=30)
            except subprocess.TimeoutExpired:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait()
            code, status = None, "TIMEOUT"
    return {
        "status": status, "exit_code": code,
        "started_utc": started.isoformat(),
        "stopped_utc": now().isoformat(),
        "wall_seconds": time.monotonic() - began,
        "cap_seconds": seconds,
        "command": command,
        "stdout_sha256": digest(stdout_path),
        "stderr_sha256": digest(stderr_path),
        "peak_cumulative_child_rss_bytes": (
            resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss * 1024
            if sys.platform != "darwin" else
            resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
        ),
    }


def host_receipt(allocation: dict) -> tuple[dict, list[int]]:
    affinity = sorted(os.sched_getaffinity(0))
    cgroup = probe_pod.cgroup_snapshot()
    physical_memory = probe_pod.mem_total_bytes()
    quota, period = cgroup["cpu_max"].split()
    cpus = (
        len(affinity) if quota in ("max", "-1")
        else int(quota) / int(period)
    )
    memory_max = cgroup["memory_max"]
    memory = (
        physical_memory if memory_max in ("max", "-1")
        else min(physical_memory, int(memory_max))
    )
    admitted = (
        allocation.get("schema") == "n83.maxk-runpod-allocation/v1"
        and allocation.get("gpu_type") == "NVIDIA A40"
        and Decimal(str(allocation.get("cost_per_hr_usd"))) <= Decimal("0.70")
        and Decimal(str(allocation.get("approved_cap_usd"))) == Decimal("2.25")
        and allocation.get("max_lifetime_seconds") == 9000
        and os.environ.get("RUNPOD_POD_ID") in (None, allocation.get("pod_id"))
        and len(affinity) >= MIN_CPU
        and cpus >= MIN_CPU
        and memory >= MIN_MEMORY
    )
    return {
        "schema": "n83.k5500-split4-host/v1",
        "status": "PASS_CAPACITY_PREFLIGHT" if admitted else "REJECTED_HOST_CAPACITY",
        "recorded_utc": now().isoformat(),
        "allocation_sha256": None,
        "machine_id": allocation.get("machine_id"),
        "architecture": platform.machine(),
        "os": platform.platform(),
        "cpu_model": probe_pod.cpu_model(),
        "affinity_ids": affinity,
        "selected_cpu_ids": affinity[:MIN_CPU],
        "effective_cpu_count": cpus,
        "effective_memory_bytes": memory,
        "physical_memory_bytes": physical_memory,
        "cgroup": cgroup,
        "cpu_counters_before": probe_pod.proc_cpu_counters(),
    }, affinity[:MIN_CPU]


def run(allocation_path: Path, bundle: Path, expected_bundle_sha: str,
        output: Path, archive: Path) -> int:
    if archive == output or output in archive.parents:
        raise ValueError("archive must be outside the result directory")
    allocation = json.loads(allocation_path.read_text())
    runpod_guard.MAX_RATE = Decimal("0.70")
    runpod_guard.gate_allocation(allocation)
    created = datetime.fromisoformat(
        allocation["created_utc"].replace("Z", "+00:00")
    )
    deadline = created + timedelta(
        seconds=allocation["max_lifetime_seconds"]
    ) - PRODUCER_MARGIN
    output.mkdir(parents=True, exist_ok=False)
    (output / "run.status").write_text("STARTED " + now().isoformat() + "\n")
    final = {"schema": "n83.k5500-split4-pod-run/v1",
             "status": "INCOMPLETE", "allocation_sha256": digest(allocation_path),
             "bundle_sha256": digest(bundle), "source_sha256": digest(SOURCE),
             "producer_deadline_utc": deadline.isoformat(), "stages": {}}
    try:
        if (final["bundle_sha256"] != expected_bundle_sha or
                final["source_sha256"] != SOURCE_SHA256):
            final["status"] = "SOURCE_MISMATCH"
            return 2
        host, affinity = host_receipt(allocation)
        host["allocation_sha256"] = final["allocation_sha256"]
        save(output / "host.json", host)
        if host["status"] != "PASS_CAPACITY_PREFLIGHT":
            final["status"] = host["status"]
            return 2
        environment = {key: value for key, value in os.environ.items()
                       if not key.startswith("KIC_")}
        environment["CARGO_TARGET_DIR"] = "/workspace/n83_k5500_target"
        environment["CARGO_BUILD_JOBS"] = "4"
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        environment["PYTHONPATH"] = str(ROOT)
        build_seconds = min(
            BUILD_CAP_SECONDS, int((deadline - now()).total_seconds())
        )
        build = run_capped(
            ["cargo", "build", "--release", "--locked", "--example",
             "koblitz_orbit_dlp_fast"],
            ROOT, environment, build_seconds,
            output / "build.stdout", output / "build.stderr",
        )
        final["stages"]["build"] = build
        (output / "run.status").write_text(
            "BUILD_" + build["status"] + " " + now().isoformat() + "\n"
        )
        if build["status"] != "PROCESS_EXITED":
            final["status"] = "BUILD_INCOMPLETE"
            return 2
        binary = Path(environment["CARGO_TARGET_DIR"]) / (
            "release/examples/koblitz_orbit_dlp_fast"
        )
        final["binary_sha256"] = digest(binary)
        for layout in ("split", "monolithic"):
            remaining = int((deadline - now()).total_seconds())
            cap = min(STAGE_CAP_SECONDS, remaining - 90)
            if cap < 300:
                final["stages"][layout] = {
                    "status": "SKIPPED_DEADLINE", "remaining_seconds": remaining,
                }
                continue
            command = [
                sys.executable, "-m",
                "research.n83_k5500_fourbyte_20261025.run_index_stage",
                "--binary", str(binary), "--output", str(output / layout),
                "--cap-seconds", str(cap), "--layout", layout,
            ]
            counters_before = probe_pod.proc_cpu_counters()
            cgroup_before = probe_pod.cgroup_snapshot()
            stage = run_capped(
                command, ROOT, environment, cap + 60,
                output / f"{layout}.stdout", output / f"{layout}.stderr",
                affinity,
            )
            stage["cpu_counters_before"] = counters_before
            stage["cpu_counters_after"] = probe_pod.proc_cpu_counters()
            stage["cgroup_before"] = cgroup_before
            stage["cgroup_after"] = probe_pod.cgroup_snapshot()
            receipt = output / layout / "receipt.json"
            if receipt.exists():
                stage["index_receipt_sha256"] = digest(receipt)
                stage["index_status"] = json.loads(receipt.read_text())["status"]
            final["stages"][layout] = stage
            (output / "run.status").write_text(
                layout.upper() + "_" + stage.get("index_status", stage["status"])
                + " " + now().isoformat() + "\n"
            )
        host["cpu_counters_after"] = probe_pod.proc_cpu_counters()
        host["cgroup_after"] = probe_pod.cgroup_snapshot()
        save(output / "host.json", host)
        final["status"] = (
            "BOTH_INDEX_STAGES_COMPLETE"
            if all(final["stages"].get(layout, {}).get("index_status") ==
                   "PROCESS_EXITED" for layout in ("split", "monolithic"))
            else "INCOMPLETE"
        )
        return 0 if final["status"] == "BOTH_INDEX_STAGES_COMPLETE" else 2
    except Exception as error:
        final["status"] = "PRODUCER_ERROR"
        final["error_type"] = type(error).__name__
        return 2
    finally:
        final["stopped_utc"] = now().isoformat()
        save(output / "pod_run.json", final)
        (output / "run.status").write_text(
            "DONE_" + final["status"] + " " + now().isoformat() + "\n"
        )
        with tarfile.open(archive, "w:gz") as stream:
            stream.add(output, arcname="results")
        (archive.with_suffix(archive.suffix + ".sha256")).write_text(
            digest(archive) + "  " + archive.name + "\n"
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--bundle-sha256", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    args = parser.parse_args()
    raise SystemExit(run(
        args.allocation, args.bundle, args.bundle_sha256,
        args.output, args.archive,
    ))
