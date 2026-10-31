#!/usr/bin/env python3
"""Bounded, binary-pinned Linux N83 first-pattern XOR search; no IC winner claim."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import resource
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
INPUT_MANIFEST = HERE.parent / "linux-xor-complete-inputs-20261025/manifest.json"
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def file_sha(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            hasher.update(block)
    return hasher.hexdigest()


def write_new(path: Path, value: dict) -> None:
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def optional_text(path: str) -> str | None:
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def cgroup_events() -> dict[str, int]:
    raw = optional_text("/sys/fs/cgroup/memory.events") or ""
    return {parts[0]: int(parts[1]) for line in raw.splitlines()
            if len(parts := line.split()) == 2 and parts[1].isdigit()}


def host() -> dict:
    return {
        "schema": "n83.linux-xor-natural-host/v1",
        "uname": platform.uname()._asdict(),
        "github_run_id": os.environ.get("GITHUB_RUN_ID"),
        "github_run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "github_sha": os.environ.get("GITHUB_SHA"),
        "cpu_count": os.cpu_count(),
        "cpu_max": optional_text("/sys/fs/cgroup/cpu.max"),
        "cpuset_cpus_effective": optional_text("/sys/fs/cgroup/cpuset.cpus.effective"),
        "memory_max": optional_text("/sys/fs/cgroup/memory.max"),
        "memory_swap_max": optional_text("/sys/fs/cgroup/memory.swap.max"),
        "memory_events_before": cgroup_events(),
        "selected_best_total_runtime": None,
    }


def preflight(protocol: dict, inputs: Path, solver: Path, worker: Path,
              source: Path) -> dict:
    require(protocol["schema"] == "n83.linux-xor-natural-panel-protocol/v1"
            and protocol["selected_best_total_runtime"] is None,
            "frozen protocol")
    require(platform.system() == "Linux"
            and platform.machine() in ("aarch64", "arm64"), "Linux ARM host")
    require(file_sha(INPUT_MANIFEST) == protocol["input_manifest_sha256"],
            "input manifest SHA-256")
    manifest = json.loads(INPUT_MANIFEST.read_bytes())
    require(manifest["curve"] == protocol["curve"]
            and len(manifest["entries"]) == 11, "exact primary-N83 input set")
    xcnfs = {row["name"]: row["sha256"] for row in manifest["entries"]
             if row["kind"] == "xcnf"}
    require(xcnfs == protocol["inputs"], "four frozen XCNF inputs")
    for row in manifest["entries"]:
        relative = row["key"].split("/inputs/", 1)[1]
        path = inputs / relative
        require(path.is_file() and path.stat().st_size == row["bytes"]
                and file_sha(path) == row["sha256"],
                f"{relative}: fresh S3 input identity")
    require(file_sha(solver) == protocol["solver_sha256"]
            and file_sha(worker) == protocol["worker_sha256"],
            "preflight binary bytes")
    version = subprocess.run([str(solver), "--version"], capture_output=True,
                             timeout=30, check=True)
    require(b"CryptoMiniSat version 5.11.15" in version.stdout,
            "frozen solver version")
    package = subprocess.run(
        ["dpkg-query", "-W", "-f=${Package}=${Version}\n", "cryptominisat"],
        capture_output=True, text=True, timeout=30, check=True)
    require(package.stdout.strip() == protocol["solver_package"],
            "frozen package version")
    source_commit = subprocess.run(["git", "-C", str(source), "rev-parse", "HEAD"],
                                   capture_output=True, text=True, check=True).stdout.strip()
    require(source_commit == protocol["source_commit"], "pinned worker source")
    ids = [arm["id"] for arm in protocol["arms"]]
    require(ids == ["hash51_A1", "hash51_A2", "gray51", "hash42", "gray42",
                    "hash51_A3"]
            and [arm["input"] for arm in protocol["arms"]]
            == ["hash_5_1", "hash_5_1", "gray_5_1", "hash_4_2",
                "gray_4_2", "hash_5_1"], "frozen A/A and candidate order")
    require(protocol["solver_flags"] == [
        "--threads", "1", "--random", "0", "--branchstr", "vmtf+vsids",
        "--autodisablegauss", "0", "--maxmatrixcols", "25000",
        "--maxnummatrices", "10", "--verb", "1", "--verbstat", "1",
        "--printsol", "1", "--maxconfl", "1000000", "--maxtime", "300",
    ] and protocol["conflict_cap"] == 1_000_000
            and protocol["solver_time_cap_seconds"] == 300
            and protocol["process_watchdog_seconds"] == 330
            and protocol["memory_address_space_limit_bytes"] == 8 * 1024**3,
            "frozen solver bounds")
    return {"schema": "n83.linux-xor-natural-preflight/v1",
            "status": "PASS_EXACT_INPUTS_BINARIES_AND_ORDER",
            "input_manifest_sha256": protocol["input_manifest_sha256"],
            "solver_sha256": protocol["solver_sha256"],
            "worker_sha256": protocol["worker_sha256"],
            "source_commit": source_commit,
            "selected_best_total_runtime": None}


def classify(stdout: bytes, stderr: bytes, exit_code: int,
             watchdog: bool, oom: bool) -> tuple[str, int | None]:
    statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE)\s*$", stdout)
    counts = re.findall(rb"(?m)^c conflicts\s+:\s+(\d+)\s*$", stdout)
    conflicts = int(counts[0]) if len(counts) == 1 else None
    if watchdog:
        return "WATCHDOG", conflicts
    if oom:
        return "OOM", conflicts
    if stderr or len(statuses) != 1 or conflicts is None:
        return "MALFORMED", conflicts
    verdict = statuses[0].decode()
    if (verdict, exit_code) in (("SATISFIABLE", 10),
                               ("UNSATISFIABLE", 20),
                               ("INDETERMINATE", 15)):
        return verdict, conflicts
    return "MALFORMED", conflicts


def run_arm(protocol: dict, arm: dict, inputs: Path, solver: Path, out: Path) -> dict:
    directory = out / arm["id"]
    directory.mkdir()
    command = [str(solver), *protocol["solver_flags"],
               str(inputs / f"{arm['input']}.xcnf")]
    before_events = cgroup_events()
    begin = time.monotonic_ns()
    with (directory / "solver.stdout").open("xb") as stdout, \
            (directory / "solver.stderr").open("xb") as stderr:
        def limits() -> None:
            limit = protocol["memory_address_space_limit_bytes"]
            resource.setrlimit(resource.RLIMIT_AS, (limit, limit))
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                   start_new_session=True, preexec_fn=limits)
        deadline = time.monotonic() + protocol["process_watchdog_seconds"]
        watchdog = False
        while True:
            pid, wait_status, usage = os.wait4(process.pid, os.WNOHANG)
            if pid == process.pid:
                break
            if time.monotonic() >= deadline:
                watchdog = True
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                _, wait_status, usage = os.wait4(process.pid, 0)
                break
            time.sleep(0.1)
        stdout.flush(); os.fsync(stdout.fileno())
        stderr.flush(); os.fsync(stderr.fileno())
    elapsed = (time.monotonic_ns() - begin) / 1e9
    exit_code = os.waitstatus_to_exitcode(wait_status)
    process.returncode = exit_code
    after_events = cgroup_events()
    oom = after_events.get("oom_kill", 0) > before_events.get("oom_kill", 0)
    stdout_raw = (directory / "solver.stdout").read_bytes()
    stderr_raw = (directory / "solver.stderr").read_bytes()
    verdict, conflicts = classify(stdout_raw, stderr_raw, exit_code, watchdog, oom)
    receipt = {
        "schema": "n83.linux-xor-natural-arm/v1",
        "arm": arm, "status": verdict, "command": command,
        "exit_code": exit_code, "watchdog": watchdog, "oom_kill_delta":
        after_events.get("oom_kill", 0) - before_events.get("oom_kill", 0),
        "wall_seconds_L0": elapsed,
        "user_cpu_seconds": usage.ru_utime,
        "system_cpu_seconds": usage.ru_stime,
        "peak_rss_bytes_linux": usage.ru_maxrss * 1024,
        "conflicts": conflicts,
        "stdout_sha256": sha(stdout_raw), "stdout_bytes": len(stdout_raw),
        "stderr_sha256": sha(stderr_raw), "stderr_bytes": len(stderr_raw),
        "xcnf_sha256": protocol["inputs"][arm["input"]],
        "solver_sha256": protocol["solver_sha256"],
        "selected_best_total_runtime": None,
    }
    write_new(directory / "process.json", receipt)
    return receipt


def verify_sat(arm: dict, inputs: Path, worker: Path,
               source: Path, directory: Path) -> dict:
    name = arm["input"]
    metadata = json.loads((inputs / "exports" / name / "export.json").read_bytes())
    require(metadata["base_object_blake3"] == BASE
            and metadata["base_summands"] + metadata["residual_summands"] == 6,
            "exported exact summand shape")
    pool = metadata["pool_object_blake3"]
    export = directory / "export"
    export.mkdir()
    (export / "instance.xcnf").symlink_to(inputs / f"{name}.xcnf")
    (export / "export.json").symlink_to(inputs / "exports" / name / "export.json")
    study = source / "research/koblitz_n83_factor_base_sweep_20261008"
    command = [str(worker), "verify-cross-xcnf", BASE,
               str(study / "pilot-01/manifest.json"),
               str(inputs / "archives" / f"s3-{BASE}.jsonl.gz"),
               pool, str(study / "pilot-01/manifest.json"),
               str(inputs / "archives" / f"s3-{pool}.jsonl.gz"),
               str(study / "pilot-01/probe-corpus.json"),
               str(metadata["base_summands"]), str(metadata["residual_summands"]),
               str(export), str(directory / "solver.stdout"),
               str(directory / "verdict")]
    begin = time.monotonic()
    result = subprocess.run(command, capture_output=True, timeout=180)
    (directory / "verify.stdout").write_bytes(result.stdout)
    (directory / "verify.stderr").write_bytes(result.stderr)
    verdict = directory / "verdict/result.json"
    data = json.loads(verdict.read_bytes()) if result.returncode == 0 and verdict.exists() else None
    status = data["status"] if data else "REJECTED_OR_INCOMPLETE_MODEL"
    require(status in ("PASS_ONE_EXTERNAL_PARTIAL", "GROUP_REJECTED_EXTERNAL_MODEL",
                       "REJECTED_OR_INCOMPLETE_MODEL"), "verifier status")
    receipt = {"schema": "n83.linux-xor-natural-model-verification/v1",
               "status": status, "exit_code": result.returncode,
               "wall_seconds_L0": time.monotonic() - begin,
               "stdout_sha256": sha(result.stdout), "stderr_sha256": sha(result.stderr),
               "result": data, "selected_best_total_runtime": None}
    write_new(directory / "verify-process.json", receipt)
    return receipt


def run(protocol: dict, inputs: Path, solver: Path, worker: Path,
        source: Path, out: Path) -> dict:
    admission = preflight(protocol, inputs, solver, worker, source)
    out.mkdir()
    write_new(out / "preflight.json", admission)
    write_new(out / "host.json", host())
    completed = []
    stop_reason = None
    for arm in protocol["arms"]:
        receipt = run_arm(protocol, arm, inputs, solver, out)
        completed.append(receipt)
        print(json.dumps({"arm": arm["id"], "status": receipt["status"],
                          "conflicts": receipt["conflicts"],
                          "wall_seconds_L0": receipt["wall_seconds_L0"]}), flush=True)
        if receipt["status"] == "SATISFIABLE":
            verified = verify_sat(arm, inputs, worker, source, out / arm["id"])
            stop_reason = verified["status"]
            break
        if receipt["status"] not in ("INDETERMINATE", "UNSATISFIABLE"):
            stop_reason = receipt["status"]
            break
    summary = {"schema": "n83.linux-xor-natural-panel/v1",
               "status": "COMPLETE_BOUNDED" if stop_reason is None else "STOPPED",
               "stop_reason": stop_reason,
               "completed_arms": [row["arm"]["id"] for row in completed],
               "summed_solver_wall_seconds_L0":
               sum(row["wall_seconds_L0"] for row in completed),
               "selected_best_total_runtime": None}
    write_new(out / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True), flush=True)
    return summary


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--protocol", type=Path, default=HERE / "protocol.json")
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_bytes())
    if args.preflight_only:
        print(json.dumps(preflight(protocol, args.inputs, args.solver,
                                   args.worker, args.source), sort_keys=True))
    else:
        run(protocol, args.inputs, args.solver, args.worker, args.source, args.out)
