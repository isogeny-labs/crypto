#!/usr/bin/env python3
"""Resume the source-pinned PKM witness tests with append-only receipts."""

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import re
import signal
import subprocess
import sys
import time
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SOURCE = "src/cryptanalysis/ic_framework/pkm_semaev.rs"
TESTS = (
    "kummer_m3_basis_descent_complete_small_curve_census",
    "zero_grid_crossover_uses_public_basis_descent_path",
    "descent_roots_match_ground_truth_at_m3",
)
WALL_CAP_S = 240
RSS_CAP_KIB = 2 * 1024 * 1024


def sha(data):
    return hashlib.sha256(data).hexdigest()


def append(path, row):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as stream:
        stream.write(json.dumps(row, sort_keys=True) + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def records(path):
    found = {}
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            row = json.loads(line)
            if row["id"] in found:
                raise ValueError(f"duplicate record {row['id']}")
            found[row["id"]] = row
    return found


def quiet_receipt():
    output = subprocess.run(["ps", "-axo", "pid,pcpu,comm"], check=True,
                            capture_output=True, text=True).stdout.splitlines()[1:]
    busy = []
    for line in output:
        parts = line.strip().split(None, 2)
        if len(parts) != 3:
            continue
        pid, cpu, command = int(parts[0]), float(parts[1]), parts[2]
        if pid == os.getpid():
            continue
        name = Path(command).name
        system = command.startswith(("/System/", "/usr/libexec/", "/Applications/"))
        if name in {"cargo", "rustc", "ic", "msolve", "pkm_mixed_ecdlp"} or (
            cpu >= 75 and not system and name != "ps"
        ):
            busy.append({"pid": pid, "cpu_percent": cpu, "comm": name[:48]})
    return {"at_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
            "loadavg": list(os.getloadavg()), "logical_cpus": os.cpu_count(),
            "competing_processes": busy}


def rss_kib(pid):
    result = subprocess.run(["ps", "-p", str(pid), "-o", "rss="],
                            capture_output=True, text=True, check=False)
    value = result.stdout.strip()
    return int(value) if value.isdigit() else 0


def run_one(binary, test, log_dir):
    stamp = str(time.time_ns())
    stdout_path = log_dir / f"{test}.{stamp}.stdout"
    stderr_path = log_dir / f"{test}.{stamp}.stderr"
    log_dir.mkdir(parents=True, exist_ok=True)
    started = time.perf_counter()
    cap_hit = None
    sampled_peak = 0
    with stdout_path.open("xb") as out, stderr_path.open("xb") as err:
        proc = subprocess.Popen([str(binary), test, "--nocapture"],
                                stdout=out, stderr=err, start_new_session=True)
        while True:
            child, status, usage = os.wait4(proc.pid, os.WNOHANG)
            if child:
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            sampled_peak = max(sampled_peak, rss_kib(proc.pid))
            if sampled_peak > RSS_CAP_KIB:
                cap_hit = "memory_cap"
            elif time.perf_counter() - started > WALL_CAP_S:
                cap_hit = "wall_cap"
            if cap_hit:
                os.killpg(proc.pid, signal.SIGKILL)
                _, status, usage = os.wait4(proc.pid, 0)
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            time.sleep(0.25)
    wall_s = time.perf_counter() - started
    stdout, stderr = stdout_path.read_bytes(), stderr_path.read_bytes()
    os_peak_kib = usage.ru_maxrss // 1024 if sys.platform == "darwin" else usage.ru_maxrss
    peak_kib = max(sampled_peak, os_peak_kib)
    if peak_kib > RSS_CAP_KIB:
        cap_hit = "memory_cap"
    output = stdout.decode(errors="replace") + stderr.decode(errors="replace")
    exact_one = re.search(r"running 1 test\b", output) is not None
    passed_one = re.search(r"test result: ok\. 1 passed;", output) is not None
    outcome = cap_hit or ("pass" if proc.returncode == 0 and exact_one and passed_one else "fail")
    return {"outcome": outcome, "exit_code": proc.returncode,
            "wall_s": wall_s, "process_cpu_s": usage.ru_utime + usage.ru_stime,
            "peak_rss_kib": peak_kib, "sampled_peak_rss_kib": sampled_peak,
            "os_peak_rss_kib": os_peak_kib, "exactly_one_test": exact_one,
            "one_test_passed": passed_one,
            "stdout": {"path": str(stdout_path), "sha256": sha(stdout), "bytes": len(stdout)},
            "stderr": {"path": str(stderr_path), "sha256": sha(stderr), "bytes": len(stderr)}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--binary-sha256", required=True)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--store", type=Path, required=True)
    parser.add_argument("--through-test", type=int, choices=(1, 2, 3), default=3)
    args = parser.parse_args()

    commit = subprocess.check_output(["git", "-C", str(REPO), "rev-parse",
                                      "--verify", f"{args.source_commit}^{{commit}}"], text=True).strip()
    pinned_source = subprocess.check_output(["git", "-C", str(REPO), "show", f"{commit}:{SOURCE}"])
    current_source = (REPO / SOURCE).read_bytes()
    if sha(current_source) != sha(pinned_source):
        parser.error("working source differs from the source commit")
    binary = args.bin.resolve(strict=True)
    binary_sha = sha(binary.read_bytes())
    if binary_sha != args.binary_sha256.lower():
        parser.error("test binary differs from the supplied pre-call SHA-256")
    header = {"id": "header", "schema": "pkm-native-witness-gate/v1",
              "protocol": "AMENDMENT_FORCE_DESCENT_20261125.md",
              "source_commit": commit, "source_sha256": sha(current_source),
              "binary_sha256": binary_sha, "binary_path": str(binary),
              "runner_sha256": sha(Path(__file__).read_bytes()),
              "toolchain": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "platform": platform.platform(), "tests": list(TESTS),
              "wall_cap_s": WALL_CAP_S, "rss_cap_kib": RSS_CAP_KIB}
    rec = records(args.store)
    if not rec:
        append(args.store, header)
        rec["header"] = header
    elif rec.get("header") != header:
        raise ValueError("existing store header differs from pinned inputs")

    for test in TESTS[:args.through_test]:
        key = f"test:{test}"
        if key in rec:
            if rec[key]["outcome"] != "pass":
                raise RuntimeError(f"{key} is failed or censored; preserve this store")
            continue
        prefix = f"load-check:{test}:"
        check = {"id": prefix + str(1 + sum(x.startswith(prefix) for x in rec)),
                 **quiet_receipt()}
        append(args.store, check)
        rec[check["id"]] = check
        if check["competing_processes"]:
            abort = {"id": f"aborted-prelaunch:{test}:{check['id'].split(':')[-1]}",
                     "load_check": check["id"], "test_child_started": False}
            append(args.store, abort)
            raise RuntimeError("competing build or benchmark process is active")
        result = {"id": key, "test": test, "load_check": check["id"],
                  **run_one(binary, test, args.store.parent / (args.store.stem + "-logs"))}
        append(args.store, result)
        rec[key] = result
        print(json.dumps({"test": test, "outcome": result["outcome"],
                          "wall_s": result["wall_s"], "peak_rss_kib": result["peak_rss_kib"]}),
              flush=True)
        if result["outcome"] != "pass":
            raise RuntimeError(f"{test} did not pass; preserve this store")


if __name__ == "__main__":
    main()
