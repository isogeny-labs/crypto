#!/usr/bin/env python3
"""Capped K5500 baseline/byte-filter index comparison on one admitted Pod."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal
import json
import os
from pathlib import Path
import platform
import tarfile

from research.n83_k5500_split4_20261025 import pod_run as custody
from research.n83_two_tier_root_20261021 import probe_pod, runpod_guard


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
TARGETS = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
SOURCE_SHA256 = "05b64717b10e17c8cd09e8a6823e6efec600ac4b53c8a0e8e0476771c702cb7b"
TARGETS_SHA256 = "2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d"
BASE_HASH = "0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f"
STATE_COUNT = 1_255_377_750
ROOT_ENTRIES = 2_510_698_617
STATE_DIGEST = "9da41c8ab29021df544383ba46a7a380df24598d18257f9ece15db38f36e599f"
TABLE_DIGEST = "706486ebe92dd362d51e61b411438dac97aaf877694a4768267374c7e11564b3"
OVERFLOW_DIGEST = "32b38fbb451e10ac2b86949107bb93af67950ab6c6182b1ad0e39fc85b81fb33"
SIDECAR_BYTES = 3_221_225_472
ORDER = ("baseline", "fp8", "fp8", "baseline")
STAGE_CAP_SECONDS = 2400
BUILD_CAP_SECONDS = 1200
MARGIN = timedelta(minutes=20)
FLAGS = {
    "KIC_PACKED_ROOT_TABLE": "1", "KIC_SYMMETRIC_S3": "1",
    "KIC_ROOT_ONLY11": "1", "KIC_INDEXED_ROOT4": "1",
    "KIC_ROOT_BLOOM": "1", "KIC_ROOT_BLOOM_PROFILE": "compact4",
    "KIC_SPARSE_CANONICAL": "1", "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "4", "KIC_INDEX_BUILD_ONLY": "1",
    "KIC_INDEXED4_SPLIT30": "1", "KIC_INDEXED4_SPLIT_AT": "4549",
}
SHARED_FIELDS = (
    "base_hash", "factor_base_points", "orbit_columns", "regular_states",
    "root_table_entries", "primary_table_entries", "overflow_table_entries",
    "state_digest_blake3", "table_digest_blake3", "overflow_digest_blake3",
    "root_table_allocated_bytes", "root_bloom_bytes", "root_table_layout",
)


def stage_summary(path: Path) -> dict | None:
    matches = []
    for line in path.read_text(errors="replace").splitlines():
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict) and value.get("kind") == "compact_orbit_index_stage":
            matches.append(value)
    return matches[0] if len(matches) == 1 else None


def parity_status(stages: list[dict]) -> str:
    if len(stages) < 2 or any(
        row["status"] != "PROCESS_EXITED" or row.get("summary") is None
        for row in stages[:2]
    ):
        return "INCOMPLETE"
    completed = [row for row in stages if row["status"] == "PROCESS_EXITED"
                 and row.get("summary") is not None]
    if [row["arm"] for row in stages[:2]] != ["baseline", "fp8"]:
        return "INCOMPLETE"
    reference = completed[0]["summary"]
    if (reference.get("root_fingerprint_sidecar_bytes") != 0
            or any(reference.get(field) != row["summary"].get(field)
                   for row in completed[1:] for field in SHARED_FIELDS)
            or any(row["summary"].get("base_hash") != BASE_HASH
                   or row["summary"].get("factor_base_points") != 913_000
                   or row["summary"].get("orbit_columns") != 5500
                   or row["summary"].get("regular_states") != STATE_COUNT
                   or row["summary"].get("root_table_entries") != ROOT_ENTRIES
                   or row["summary"].get("state_digest_blake3") != STATE_DIGEST
                   or row["summary"].get("table_digest_blake3") != TABLE_DIGEST
                   or row["summary"].get("overflow_digest_blake3") != OVERFLOW_DIGEST
                   or row["summary"].get("indexed4_split_at") != 4549
                   or row["summary"].get("root_fingerprint_sidecar_bytes") !=
                   (SIDECAR_BYTES if row["arm"] == "fp8" else 0)
                   for row in completed)):
        return "EXACT_PARITY_FAILED"
    if len(stages) == len(ORDER) and len(completed) == len(ORDER):
        return "PASS_ABBA_EXACT_PARITY"
    return "PASS_PAIR_EXACT_PARITY_WITH_INCOMPLETE_REPEATS"


def run(allocation_path: Path, bundle: Path, bundle_sha256: str,
        output: Path, archive: Path) -> int:
    if archive == output or output in archive.parents:
        raise ValueError("archive must be outside the result directory")
    allocation = json.loads(allocation_path.read_text())
    runpod_guard.MAX_RATE = Decimal("0.70")
    runpod_guard.gate_allocation(allocation)
    created = datetime.fromisoformat(allocation["created_utc"].replace("Z", "+00:00"))
    deadline = created + timedelta(seconds=allocation["max_lifetime_seconds"]) - MARGIN
    output.mkdir(parents=True, exist_ok=False)
    final = {
        "schema": "n83.k5500-fp8-pod-run/v1", "status": "INCOMPLETE",
        "allocation_sha256": custody.digest(allocation_path),
        "bundle_sha256": custody.digest(bundle),
        "source_sha256": custody.digest(SOURCE),
        "public_targets_sha256": custody.digest(TARGETS),
        "producer_deadline_utc": deadline.isoformat(),
        "arm_order": ORDER, "stage_cap_seconds": STAGE_CAP_SECONDS,
        "host": None, "build": None, "stages": [],
    }
    try:
        if (final["bundle_sha256"] != bundle_sha256
                or final["source_sha256"] != SOURCE_SHA256
                or final["public_targets_sha256"] != TARGETS_SHA256):
            final["status"] = "SOURCE_MISMATCH"
            return 2
        host, affinity = custody.host_receipt(allocation)
        host["schema"] = "n83.k5500-fp8-host/v1"
        host["allocation_sha256"] = final["allocation_sha256"]
        host["host_kernel"] = platform.release()
        custody.save(output / "host.json", host)
        final["host"] = "host.json"
        if host["status"] != "PASS_CAPACITY_PREFLIGHT":
            final["status"] = host["status"]
            return 2
        environment = {key: value for key, value in os.environ.items()
                       if not key.startswith("KIC_")}
        environment["CARGO_TARGET_DIR"] = "/workspace/n83_k5500_fp8_target"
        environment["CARGO_BUILD_JOBS"] = "4"
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        environment["PYTHONPATH"] = str(ROOT)
        build_cap = min(BUILD_CAP_SECONDS, int((deadline - custody.now()).total_seconds()))
        build = custody.run_capped(
            ["cargo", "build", "--release", "--locked", "--example",
             "koblitz_orbit_dlp_fast"], ROOT, environment, build_cap,
            output / "build.stdout", output / "build.stderr",
        )
        final["build"] = build
        if build["status"] != "PROCESS_EXITED":
            final["status"] = "BUILD_INCOMPLETE"
            return 2
        binary = Path(environment["CARGO_TARGET_DIR"]) / "release/examples/koblitz_orbit_dlp_fast"
        final["binary_sha256"] = custody.digest(binary)
        for index, arm in enumerate(ORDER):
            remaining = int((deadline - custody.now()).total_seconds())
            cap = min(STAGE_CAP_SECONDS, remaining - 90)
            if cap < 300:
                final["stages"].append({"arm": arm, "status": "SKIPPED_DEADLINE",
                                        "remaining_seconds": remaining})
                continue
            flags = dict(FLAGS)
            if arm == "fp8":
                flags["KIC_INDEXED4_FINGERPRINT8"] = "1"
            stage_env = {key: value for key, value in environment.items()
                         if not key.startswith("KIC_")}
            stage_env.update(flags)
            command = [str(binary), "construct:83:1:5500", str(TARGETS), "29",
                       str(output / f"{index}-unused-target-result.jsonl")]
            before = {"process_cpu": probe_pod.proc_cpu_counters(),
                      "cgroup": probe_pod.cgroup_snapshot()}
            stdout = output / f"{index}-{arm}.stdout.jsonl"
            stderr = output / f"{index}-{arm}.stderr.log"
            stage = custody.run_capped(command, ROOT, stage_env, cap, stdout, stderr, affinity)
            stage.update({"arm": arm, "flags": flags, "counters_before": before,
                          "counters_after": {
                              "process_cpu": probe_pod.proc_cpu_counters(),
                              "cgroup": probe_pod.cgroup_snapshot(),
                          }, "summary": stage_summary(stdout)})
            final["stages"].append(stage)
            custody.save(output / "pod_run.json", final)
        host["cpu_counters_after"] = probe_pod.proc_cpu_counters()
        host["cgroup_after"] = probe_pod.cgroup_snapshot()
        custody.save(output / "host.json", host)
        final["status"] = parity_status(final["stages"])
        return 0 if final["status"] == "PASS_ABBA_EXACT_PARITY" else 2
    except Exception as error:
        final["status"] = "PRODUCER_ERROR"
        final["error_type"] = type(error).__name__
        return 2
    finally:
        final["stopped_utc"] = custody.now().isoformat()
        custody.save(output / "pod_run.json", final)
        with tarfile.open(archive, "w:gz") as stream:
            stream.add(output, arcname="results")
        (archive.with_suffix(archive.suffix + ".sha256")).write_text(
            custody.digest(archive) + "  " + archive.name + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--bundle-sha256", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--archive", type=Path, required=True)
    options = parser.parse_args()
    raise SystemExit(run(options.allocation, options.bundle, options.bundle_sha256,
                         options.output, options.archive))
