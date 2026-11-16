"""Bounded current-source K2304 control on an earlier public N83 point."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
SOURCE_SHA256 = "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
BINARY_SHA256 = "07f7b5b424d0f1ecea1cd7e6564522c4d9c64557a820441c381e7474b13a388a"
CANDIDATE = (ROOT / "research/n83_k2304_current_source_prefix_20261025"
             / "candidate_IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h06c953ec903b.json")
PUBLIC = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWERS = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
CAP_SECONDS = 1200
RSS_CAP_BYTES = 12 * 1024**3
MIN_FREE_PERCENT = 10
MAX_SWAP_INCREASE_MIB = 1024


def sha256(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def rss_bytes(pid: int) -> int | None:
    result = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True,
                            text=True, check=False, timeout=5)
    value = result.stdout.strip()
    return int(value) * 1024 if result.returncode == 0 and value.isdigit() else None


def free_percent() -> int | None:
    result = subprocess.run(["memory_pressure", "-Q"], capture_output=True,
                            text=True, check=False, timeout=5)
    match = re.search(r"System-wide memory free percentage: (\d+)%", result.stdout)
    return int(match.group(1)) if result.returncode == 0 and match else None


def swap_used_mib() -> float | None:
    result = subprocess.run(["sysctl", "vm.swapusage"], capture_output=True,
                            text=True, check=False, timeout=5)
    match = re.search(r"\bused = ([0-9.]+)M\b", result.stdout)
    return float(match.group(1)) if result.returncode == 0 and match else None


def run(binary: Path, output: Path, preflight: bool = False) -> dict:
    binary = binary.resolve(strict=True)
    if sha256(SOURCE) != SOURCE_SHA256 or sha256(binary) != BINARY_SHA256:
        raise ValueError("current source or binary hash differs from the frozen control")
    candidate = json.loads(CANDIDATE.read_text())
    if (candidate["candidate_id"] !=
            "IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h06c953ec903b" or
            candidate["factor_base"]["actual_usable_points"] != 382464 or
            candidate["factor_base"]["effective_columns"] != 2304):
        raise ValueError("K2304 current-source candidate manifest differs")
    point = json.loads(PUBLIC.read_text().splitlines()[0])
    answer = json.loads(ANSWERS.read_text().splitlines()[0])
    if point != answer["point"] or answer["fixture_index"] != 0:
        raise ValueError("earlier public point fixture differs")
    initial_free = free_percent()
    initial_swap = swap_used_mib()
    if initial_free is None or initial_free < 50 or initial_swap is None:
        raise RuntimeError("host admission needs 50% free and swap telemetry")
    from research.n83_crossfamily_current_20261025 import run as panel
    flags = panel.ic_environment(output, "root11_dynamic", "k2304")
    flags.pop("PATH")
    command = [str(binary), "construct:83:1:2304", str(output / "target.jsonl"),
               "29", str(output / "targets_result.jsonl")]
    config = {"schema": "n83.k2304-current-source-local-control/v1",
              "runner_sha256": sha256(Path(__file__).resolve()),
              "source_sha256": SOURCE_SHA256, "binary_sha256": BINARY_SHA256,
              "candidate_manifest_sha256": sha256(CANDIDATE),
              "public_targets_sha256": sha256(PUBLIC),
              "answer_sidecar_sha256": sha256(ANSWERS),
              "target": point, "command": command, "algorithm_flags": flags,
              "cap_seconds": CAP_SECONDS, "rss_cap_bytes": RSS_CAP_BYTES,
              "min_host_free_percent": MIN_FREE_PERCENT,
              "max_host_swap_increase_mib": MAX_SWAP_INCREASE_MIB,
              "initial_host_free_percent": initial_free,
              "initial_host_swap_used_mib": initial_swap}
    if preflight:
        return {"status": "PASS_PREFLIGHT", "config": config}
    output.mkdir(parents=True, exist_ok=False)
    (output / "target.jsonl").write_text(json.dumps(point, sort_keys=True) + "\n")
    (output / "config.json").write_text(json.dumps(config, indent=2, sort_keys=True) + "\n")
    environment = {key: os.environ[key] for key in ("PATH", "HOME", "TMPDIR", "LANG")
                   if key in os.environ}
    environment.update(flags)
    started = time.monotonic()
    samples = []
    stop_reason = None
    with (output / "stdout.jsonl").open("wb") as stdout, (output / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=ROOT, env=environment,
                                 stdout=stdout, stderr=stderr, start_new_session=True)
        while child.poll() is None:
            elapsed = time.monotonic() - started
            rss = rss_bytes(child.pid)
            host_free = free_percent() if len(samples) % 10 == 0 else None
            host_swap = swap_used_mib() if len(samples) % 10 == 0 else None
            samples.append({"elapsed_seconds": round(elapsed, 3), "rss_bytes": rss,
                            "host_free_percent": host_free,
                            "host_swap_used_mib": host_swap})
            if elapsed >= CAP_SECONDS:
                stop_reason = "TIME_CAP"
            elif rss is not None and rss > RSS_CAP_BYTES:
                stop_reason = "RSS_CAP"
            elif host_free is not None and host_free < MIN_FREE_PERCENT:
                stop_reason = "HOST_MEMORY_GATE"
            elif host_swap is None and host_free is not None:
                stop_reason = "HOST_SWAP_TELEMETRY_FAILURE"
            elif host_swap is not None and host_swap - initial_swap > MAX_SWAP_INCREASE_MIB:
                stop_reason = "HOST_SWAP_GATE"
            if stop_reason:
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                break
            time.sleep(0.5)
    (output / "samples.json").write_text(json.dumps(samples, indent=2) + "\n")
    status = stop_reason or (
        "PASS_PROCESS_EXIT" if child.returncode == 0 else
        "INTERRUPTED_EXTERNAL" if child.returncode in (-signal.SIGTERM, -signal.SIGINT) else
        "PRODUCER_FAILURE_EXIT")
    result = {"schema": "n83.k2304-current-source-local-receipt/v1",
              "status": status, "exit_code": child.returncode,
              "wall_seconds": round(time.monotonic() - started, 3),
              "stopped_utc": datetime.now(timezone.utc).isoformat(),
              "samples_sha256": sha256(output / "samples.json"),
              "stdout_sha256": sha256(output / "stdout.jsonl"),
              "stderr_sha256": sha256(output / "stderr.log"),
              "target_sha256": sha256(output / "target.jsonl"),
              "base_dump_sha256": sha256(output / "base_dump.jsonl")
              if (output / "base_dump.jsonl").is_file() else None,
              "targets_result_sha256": sha256(output / "targets_result.jsonl")
              if (output / "targets_result.jsonl").is_file() else None,
              "max_sampled_rss_bytes": max((item["rss_bytes"] or 0 for item in samples),
                                            default=0)}
    (output / "receipt.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--preflight", action="store_true")
    args = parser.parse_args()
    print(json.dumps(run(args.binary, args.output, args.preflight), sort_keys=True))
