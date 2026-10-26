"""Bounded Linux control for the frozen N83 a1 K4405 root index.

The provider deletion watchdog is a separate prerequisite. This wrapper caps
the producer and records its cgroup envelope; it does not stop Pod billing.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
SOURCE_FLAGS = ROOT / "research/n83_root4_frontier_current_20261018/highk_20261019/runs/k4405_wide3/source.json"
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
CAP_SECONDS = 1200
MEMORY_STOP_BYTES = 48 * 1024**3
MIN_MEMORY_MAX_BYTES = 56 * 1024**3
CGROUP = Path("/sys/fs/cgroup")


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def number(path: Path) -> int | None:
    value = path.read_text().strip()
    return None if value == "max" else int(value)


def snapshot() -> dict:
    return {
        "memory_current": number(CGROUP / "memory.current"),
        "memory_peak": number(CGROUP / "memory.peak"),
        "memory_max": number(CGROUP / "memory.max"),
        "memory_swap_current": number(CGROUP / "memory.swap.current"),
        "memory_events": (CGROUP / "memory.events").read_text(),
        "cpu_stat": (CGROUP / "cpu.stat").read_text(),
        "cpu_max": (CGROUP / "cpu.max").read_text().strip(),
        "affinity": sorted(os.sched_getaffinity(0)),
    }


def parse_cpu_max(value: str) -> float | None:
    quota, period = value.split()
    return None if quota == "max" else int(quota) / int(period)


def run(binary: Path, output: Path, *, preflight: bool = False) -> dict:
    binary = binary.resolve(strict=True)
    if digest(SOURCE) != SOURCE_SHA256:
        raise ValueError("producer source differs from the frozen K4405 source")
    source = json.loads(SOURCE_FLAGS.read_text())
    point = json.loads(PUBLIC.read_text().splitlines()[0])
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    if point != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("earlier public target fixture differs")
    initial = snapshot()
    if (initial["memory_max"] is None or
            initial["memory_max"] < MIN_MEMORY_MAX_BYTES or
            initial["memory_current"] is None or
            initial["memory_current"] > 8 * 1024**3 or
            initial["memory_swap_current"] is None or
            initial["memory_swap_current"] > 2 * 1024**3 or
            len(initial["affinity"]) < 4 or
            (parse_cpu_max(initial["cpu_max"]) is not None and
             parse_cpu_max(initial["cpu_max"]) < 4)):
        raise RuntimeError("Linux Pod requires a clean >=56 GiB cgroup and >=4 CPUs")
    flags = dict(source["algorithm_flags"])
    flags.update({"KIC_DUMP_BASE": str(output / "base_dump.jsonl"),
                  "KIC_INDEX_ROOT_THREADS": "4"})
    command = [str(binary), "construct:83:1:4405", str(output / "target.jsonl"),
               "29", str(output / "targets_result.jsonl")]
    config = {"schema": "n83.k4405-current-source-linux-control/v1",
              "runner_sha256": digest(Path(__file__)),
              "source_sha256": SOURCE_SHA256,
              "binary_sha256": digest(binary),
              "source_flags_sha256": digest(SOURCE_FLAGS),
              "public_targets_sha256": digest(PUBLIC),
              "answer_sidecar_sha256": digest(ANSWERS),
              "git_head": subprocess.check_output(
                  ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "target": point, "command": command, "algorithm_flags": flags,
              "cap_seconds": CAP_SECONDS,
              "memory_stop_bytes": MEMORY_STOP_BYTES,
              "initial_cgroup": initial,
              "pod_id": os.environ.get("RUNPOD_POD_ID")}
    if preflight:
        return {"status": "PASS_PREFLIGHT", "config": config}
    output.mkdir(parents=True, exist_ok=False)
    (output / "target.jsonl").write_text(json.dumps(point, sort_keys=True) + "\n")
    (output / "config.json").write_text(json.dumps(config, indent=2, sort_keys=True) + "\n")
    environment = {key: os.environ[key] for key in ("PATH", "HOME", "TMPDIR", "LANG")
                   if key in os.environ}
    environment.update(flags)
    samples: list[dict] = []
    stop_reason = None
    started = time.monotonic()
    interrupted = False

    def request_stop(_signum: int, _frame: object) -> None:
        nonlocal interrupted
        interrupted = True

    previous_term = signal.signal(signal.SIGTERM, request_stop)
    previous_int = signal.signal(signal.SIGINT, request_stop)
    with (output / "stdout.jsonl").open("wb") as stdout, (output / "stderr.log").open("wb") as stderr:
        try:
            child = subprocess.Popen(command, cwd=ROOT, env=environment,
                                     stdout=stdout, stderr=stderr, start_new_session=True)
            while child.poll() is None:
                elapsed = time.monotonic() - started
                try:
                    state = snapshot()
                except (OSError, ValueError):
                    stop_reason = "CGROUP_TELEMETRY_FAILURE"
                    state = None
                if state is not None:
                    samples.append({"elapsed_seconds": round(elapsed, 3), **state})
                if interrupted:
                    stop_reason = "INTERRUPTED_EXTERNAL"
                elif elapsed >= CAP_SECONDS:
                    stop_reason = "TIME_CAP"
                elif state is not None and (state["memory_current"] is None or
                                             state["memory_current"] >= MEMORY_STOP_BYTES):
                    stop_reason = "CGROUP_MEMORY_GATE"
                elif state is not None and (state["memory_swap_current"] is None or
                                             state["memory_swap_current"] > 2 * 1024**3):
                    stop_reason = "CGROUP_SWAP_GATE"
                if stop_reason:
                    os.killpg(child.pid, signal.SIGTERM)
                    try:
                        child.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        os.killpg(child.pid, signal.SIGKILL)
                        child.wait()
                    break
                time.sleep(0.5)
        finally:
            signal.signal(signal.SIGTERM, previous_term)
            signal.signal(signal.SIGINT, previous_int)
    (output / "samples.json").write_text(json.dumps(samples, indent=2) + "\n")
    status = stop_reason or ("PASS_PROCESS_EXIT" if child.returncode == 0 else
                             "PRODUCER_FAILURE_EXIT")
    receipt = {"schema": "n83.k4405-current-source-linux-receipt/v1",
               "status": status, "exit_code": child.returncode,
               "wall_seconds": round(time.monotonic() - started, 3),
               "stopped_utc": datetime.now(timezone.utc).isoformat(),
               "final_cgroup": snapshot(),
               "samples_sha256": digest(output / "samples.json"),
               "stdout_sha256": digest(output / "stdout.jsonl"),
               "stderr_sha256": digest(output / "stderr.log"),
               "target_sha256": digest(output / "target.jsonl"),
               "base_dump_sha256": digest(output / "base_dump.jsonl")
               if (output / "base_dump.jsonl").is_file() else None,
               "targets_result_sha256": digest(output / "targets_result.jsonl")
               if (output / "targets_result.jsonl").is_file() else None}
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    return receipt


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--preflight", action="store_true")
    args = parser.parse_args()
    print(json.dumps(run(args.binary, args.output, preflight=args.preflight), sort_keys=True))
