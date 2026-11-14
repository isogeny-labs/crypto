#!/usr/bin/env python3
"""Execute one frozen K2048 constructed-target XCNF search."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def swapouts():
    output = subprocess.check_output(["/usr/bin/vm_stat"], text=True)
    return int(output.split("Swapouts:", 1)[1].split()[0].rstrip("."))


def sample():
    return {"unix_ms": time.time_ns() // 1_000_000,
            "load_1m": os.getloadavg()[0], "swapouts": swapouts()}


def terminal(exit_code, timed_out, stdout):
    if timed_out:
        return "WATCHDOG"
    statuses = [line for line in stdout.splitlines() if line.startswith("s ")]
    if exit_code == 10 and statuses == ["s SATISFIABLE"]:
        return "SAT_PENDING_REPLAY"
    if exit_code == 15 and statuses == ["s INDETERMINATE"]:
        return "INDETERMINATE"
    if exit_code == 20 and statuses == ["s UNSATISFIABLE"]:
        return "UNSAT_UNVERIFIED"
    return "PRODUCER_FAILURE"


def check_lock(lock, repo, output):
    require(lock["schema"] == "n83.k2048-unpinned-solver-lock/v1", "lock schema")
    require(lock["date_utc"] == "2026-11-14", "lock date")
    runner = Path(__file__).resolve()
    require(sha256(runner) == lock["runner_sha256"], "runner source hash")
    relative = runner.relative_to(repo)
    committed = subprocess.check_output(
        ["git", "-C", str(repo), "show", f'{lock["runner_source_commit"]}:{relative}'])
    require(hashlib.sha256(committed).hexdigest() == lock["runner_sha256"],
            "runner commit binding")
    require(not subprocess.check_output(
        ["git", "-C", str(repo), "status", "--porcelain", "--untracked-files=normal"]),
        "dirty source checkout")
    require(output == Path(lock["output"]) and not output.exists(), "fresh output path")
    require(shutil.disk_usage(output.parent).free >= 2 * 1024**3, "scratch gate")
    for key, hash_key in (("solver", "solver_sha256"),
                          ("formula", "formula_sha256"),
                          ("export_receipt", "export_receipt_sha256"),
                          ("known_model_audit", "known_model_audit_sha256")):
        require(sha256(Path(lock[key])) == lock[hash_key], key + " hash")
    formula = Path(lock["formula"])
    require(formula.stat().st_size == 20_511_067, "formula byte count")
    require(formula.open("rb").readline().strip() == b"p cnf 111478 1145537",
            "formula header")
    export = json.loads(Path(lock["export_receipt"]).read_text())
    audit = json.loads(Path(lock["known_model_audit"]).read_text())
    require(export["mode"] == "positive-unpinned" and
            export["status"] == "PASS_XCNF_EXPORT_ONLY" and
            export["source_commit"] == lock["export_source_commit"] and
            export["binary_sha256"] == lock["export_binary_sha256"] and
            export["xcnf_sha256"] == lock["formula_sha256"] and
            export["shape"]["positive_x_pins"] == 0 and
            export["target"] == lock["target"], "export identity")
    require(audit["status"] == "PASS_ORIGINAL_XCNF" and
            audit["xcnf_sha256"] == lock["formula_sha256"] and
            (audit["checked_cnf"], audit["checked_xor"]) == (1_140_557, 4_980),
            "known-model audit")
    require(lock["solver_sha256"] ==
            "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac",
            "solver differs from paired arm")
    require(lock["solver_flags"] == [
        "--threads", "1", "--random", "20261114", "--maxtime", "600",
        "--maxconfl", "1000000", "--verb", "1", "--verbstat", "1",
        "--printsol", "1", "--maxnummatrices", "32", "--autodisablegauss", "0",
        "--gaussusefulcutoff", "0", "--maxmatrixcols", "25000",
        "--branchstr", "vsids"], "solver flags")
    require(lock["outer_watchdog_seconds"] == 660, "watchdog cap")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lock", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[4]
    lock = json.loads(args.lock.read_text())
    check_lock(lock, repo, args.output)
    samples = []
    for index in range(3):
        samples.append(sample())
        if index < 2:
            time.sleep(15)
    args.output.mkdir(exist_ok=False)
    (args.output / "host-preflight.json").write_text(json.dumps(samples, indent=2) + "\n")
    fresh = sample()
    if any(item["load_1m"] > 24 for item in samples + [fresh]) or \
            len({item["swapouts"] for item in samples + [fresh]}) != 1:
        (args.output / "rejection.json").write_text(json.dumps({
            "status": "HOST_REJECTED", "samples": samples,
            "launch_sample": fresh}, indent=2) + "\n")
        raise ValueError("host load or swapout gate")
    command = [lock["solver"], *lock["solver_flags"], lock["formula"]]
    (args.output / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    start = time.monotonic()
    timed_out = False
    with (args.output / "stdout.txt").open("wb") as stdout, \
            (args.output / "stderr.txt").open("wb") as stderr:
        child = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        while True:
            pid, raw, usage = os.wait4(child.pid, os.WNOHANG)
            if pid:
                code = os.waitstatus_to_exitcode(raw)
                break
            if time.monotonic() - start >= lock["outer_watchdog_seconds"]:
                timed_out = True
                os.killpg(child.pid, signal.SIGTERM)
                time.sleep(5)
                pid, raw, usage = os.wait4(child.pid, os.WNOHANG)
                if not pid:
                    os.killpg(child.pid, signal.SIGKILL)
                    _, raw, usage = os.wait4(child.pid, 0)
                code = os.waitstatus_to_exitcode(raw)
                break
            time.sleep(0.5)
    stdout_text = (args.output / "stdout.txt").read_text(errors="replace")
    conflicts = [line for line in stdout_text.splitlines()
                 if line.startswith("c conflicts                :")]
    result = {
        "schema": "n83.k2048-unpinned-solver-run/v1",
        "status": terminal(code, timed_out, stdout_text),
        "exit_code": code, "timed_out": timed_out,
        "terminal_conflicts": int(conflicts[0].split(":", 1)[1]) if len(conflicts) == 1 else None,
        "wall_seconds": time.monotonic() - start,
        "cpu_user_seconds": usage.ru_utime, "cpu_system_seconds": usage.ru_stime,
        "wait4_peak_rss_bytes": usage.ru_maxrss,
        "host_preflight": samples, "swapouts_after": swapouts(),
        "lock_sha256": sha256(args.lock),
        "formula_sha256": lock["formula_sha256"],
        "solver_sha256": lock["solver_sha256"],
        "stdout_sha256": sha256(args.output / "stdout.txt"),
        "stderr_sha256": sha256(args.output / "stderr.txt"),
        "verified_relation": None, "novel_rank": None,
        "single_target_online_ms": None, "selected_best_total_runtime": None,
    }
    (args.output / "receipt.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: result[key] for key in
                      ("status", "exit_code", "terminal_conflicts", "wall_seconds")}))


if __name__ == "__main__":
    main()
