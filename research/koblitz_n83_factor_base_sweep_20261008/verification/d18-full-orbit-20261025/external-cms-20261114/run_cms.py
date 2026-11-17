#!/usr/bin/env python3
"""Run one locked D18 XCNF through a bounded CryptoMiniSat process group."""

import argparse
import base64
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def utc_now():
    return datetime.now(timezone.utc).isoformat()


def swapouts():
    output = subprocess.check_output(["vm_stat"], text=True)
    return int(output.split("Swapouts:", 1)[1].split()[0].rstrip("."))


def preflight(args, lock):
    root = args.root
    entry = lock["exports"][args.mode]
    source_commit = subprocess.check_output(["git", "-C", str(args.source), "rev-parse", "HEAD"], text=True).strip()
    require(source_commit == lock["source_commit"], "source commit differs")
    require(not subprocess.check_output(["git", "-C", str(args.source), "status", "--porcelain"], text=True), "source checkout dirty")
    require(sha256(args.worker) == lock["worker_binary_sha256"], "worker binary differs")
    require(sha256(args.solver) == lock["solver_binary_sha256"], "solver binary differs")
    for name, field in (("manifest.json", "producer_manifest_sha256"), ("s3-manifest.json", "s3_manifest_sha256")):
        require(sha256(root / "input" / name) == lock[field], f"input differs: {name}")
    object_path = root / "input/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
    require(sha256(object_path) == lock["compressed_object_sha256"], "D18 object differs")
    for name, digest in (("manifest-get-metadata.json", lock["s3_manifest_sha256"]), ("object-get-metadata.json", lock["compressed_object_sha256"])):
        metadata = json.loads((root / "input" / name).read_text())
        require(metadata["ChecksumType"] == "FULL_OBJECT", "provider checksum scope differs")
        require(metadata["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(digest)).decode(), "provider checksum differs")
    export = root / f"export-{args.mode}"
    require((export / "result.json.exit.txt").read_text().strip() == "0", "export exit differs")
    require(sha256(export / "result.json") == entry["worker_receipt_sha256"], "export receipt differs")
    xcnf = export / "result.xcnf"
    require(sha256(xcnf) == entry["xcnf_sha256"], "XCNF differs")
    require(xcnf.stat().st_size == entry["xcnf_bytes"], "XCNF byte count differs")
    require(xcnf.open("rb").readline().decode().strip() == f'p cnf {entry["variables"]} {entry["dimacs_header_clauses"]}', "XCNF header differs")
    samples = (root / f"host-preflight-{args.mode}.txt").read_text().splitlines()
    require(len(samples) == 12, "three host samples required")
    loads = [float(samples[index].split()[1]) for index in (1, 5, 9)]
    swaps = [int(samples[index].split()[1].rstrip(".")) for index in (2, 6, 10)]
    require(all(value <= 24 for value in loads), "host load gate failed")
    require(len(set(swaps)) == 1, "host swapouts changed")
    require(os.getloadavg()[0] <= 24, "launch load gate failed")
    require(swapouts() == swaps[-1], "launch swapouts changed")
    require(shutil.disk_usage(root).free >= 2 * 1024**3, "scratch gate failed")
    return xcnf, loads, swaps[-1]


def solver_status(exit_code, timed_out, stdout):
    if timed_out:
        return "WATCHDOG"
    statuses = [line for line in stdout.splitlines() if line.startswith("s ")]
    if exit_code == 10 and statuses == ["s SATISFIABLE"]:
        return "SAT_PENDING_XCNF_REPLAY"
    if exit_code == 20 and statuses == ["s UNSATISFIABLE"]:
        return "UNSAT_FIRST_PATTERN"
    if exit_code == 15 and statuses == ["s INDETERMINATE"]:
        return "INDETERMINATE"
    return "PRODUCER_FAILURE"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("positive", "natural"))
    parser.add_argument("root", type=Path)
    parser.add_argument("source", type=Path)
    parser.add_argument("worker", type=Path)
    parser.add_argument("solver", type=Path)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    lock_path = Path(__file__).with_name("EXPORT_LOCK.json")
    lock = json.loads(lock_path.read_text())
    require(lock["schema"] == "n83.d18-external-cms-export-lock/v1", "lock schema differs")
    xcnf, loads, baseline_swaps = preflight(args, lock)
    if args.preflight_only:
        print(json.dumps({"status": "PREFLIGHT_PASS", "load_1m": loads, "swapouts": baseline_swaps}, sort_keys=True))
        return
    out = args.root / f"solver-{args.mode}"
    out.mkdir(exist_ok=False)
    command = [str(args.solver), *lock["solver_flags"], str(xcnf)]
    (out / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    started_utc = utc_now()
    start = time.monotonic()
    timed_out = False
    peak_rss_kb = 0
    with (out / "stdout.txt").open("wb") as stdout, (out / "stderr.txt").open("wb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=True)
        while True:
            pid, wait_status, usage = os.wait4(process.pid, os.WNOHANG)
            if pid:
                exit_code = os.waitstatus_to_exitcode(wait_status)
                process.returncode = exit_code
                break
            sample = subprocess.run(["ps", "-o", "rss=", "-p", str(process.pid)], capture_output=True, text=True)
            if sample.returncode == 0 and sample.stdout.strip().isdigit():
                peak_rss_kb = max(peak_rss_kb, int(sample.stdout.strip()))
            if time.monotonic() - start >= lock["outer_watchdog_seconds"]:
                timed_out = True
                os.killpg(process.pid, signal.SIGTERM)
                time.sleep(5)
                pid, wait_status, usage = os.wait4(process.pid, os.WNOHANG)
                if not pid:
                    os.killpg(process.pid, signal.SIGKILL)
                    pid, wait_status, usage = os.wait4(process.pid, 0)
                exit_code = os.waitstatus_to_exitcode(wait_status)
                process.returncode = exit_code
                break
            time.sleep(1)
    wall = time.monotonic() - start
    stdout_text = (out / "stdout.txt").read_text(errors="replace")
    record = {
        "schema": "n83.d18-external-cms-run/v1",
        "mode": args.mode,
        "status": solver_status(exit_code, timed_out, stdout_text),
        "start_utc": started_utc,
        "end_utc": utc_now(),
        "command": command,
        "source_commit": lock["source_commit"],
        "worker_binary_sha256": lock["worker_binary_sha256"],
        "solver_binary_sha256": lock["solver_binary_sha256"],
        "xcnf_sha256": lock["exports"][args.mode]["xcnf_sha256"],
        "exit_code": exit_code,
        "outer_timeout": timed_out,
        "wall_seconds": wall,
        "cpu_user_seconds": usage.ru_utime,
        "cpu_system_seconds": usage.ru_stime,
        "sampled_peak_rss_kb": peak_rss_kb,
        "host_load_1m_preflight": loads,
        "swapouts_preflight": baseline_swaps,
        "swapouts_after": swapouts(),
        "stdout_sha256": sha256(out / "stdout.txt"),
        "stderr_sha256": sha256(out / "stderr.txt"),
        "selected_best_total_runtime": None,
    }
    (out / "receipt.json").write_text(json.dumps(record, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: record[key] for key in ("mode", "status", "exit_code", "wall_seconds", "sampled_peak_rss_kb")}, sort_keys=True))


if __name__ == "__main__":
    main()
