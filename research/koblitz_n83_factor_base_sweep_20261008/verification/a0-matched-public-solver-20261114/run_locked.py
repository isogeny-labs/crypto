#!/usr/bin/env python3
"""Run one exact N83 a=0 public XCNF under a committed two-arm lock."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import signal
import subprocess
import time


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def swapouts():
    output = subprocess.check_output(["/usr/bin/vm_stat"], text=True)
    return int(output.split("Swapouts:", 1)[1].split()[0].rstrip("."))


def host_sample():
    return {"unix_ms": time.time_ns() // 1_000_000,
            "load_1m": os.getloadavg()[0], "swapouts": swapouts()}


def check_lock(lock, arm, repo):
    require(lock["schema"] == "n83.a0-matched-public-solver-lock/v1", "lock schema")
    require(lock["fixture"] == 1 and arm in ("d18", "k2048"), "fixture or arm")
    require(lock["run_order"] == ["d18", "k2048"], "run order")
    require(digest(Path(__file__)) == lock["runner_sha256"], "runner source hash")
    commit = lock["runner_source_commit"]
    shown = subprocess.check_output(
        ["git", "-C", str(repo), "show", f"{commit}:" +
         "research/koblitz_n83_factor_base_sweep_20261008/verification/"
         "a0-matched-public-solver-20261114/run_locked.py"])
    require(hashlib.sha256(shown).hexdigest() == lock["runner_sha256"], "runner commit hash")
    require(not subprocess.check_output(
        ["git", "-C", str(repo), "status", "--porcelain", "--untracked-files=normal"]),
        "source checkout is dirty")
    solver = Path(lock["solver"])
    require(digest(solver) == lock["solver_sha256"], "solver hash")
    require(lock["solver_flags_base"] == [
        "--threads", "1", "--random", "20261114", "--maxtime", "600",
        "--maxconfl", "1000000", "--verb", "1", "--verbstat", "1",
        "--printsol", "1", "--maxnummatrices", "32", "--autodisablegauss", "0",
        "--gaussusefulcutoff", "0"], "solver flags base")
    entry = lock["arms"][arm]
    require(entry["flags_tail"] in (["--branchstr", "vsids"],
            ["--maxmatrixcols", "25000", "--branchstr", "vsids"]),
            "solver flags tail")
    require((arm == "d18") == (entry["flags_tail"] == ["--branchstr", "vsids"]),
            "arm flag policy")
    formula = Path(entry["formula"])
    receipt_path = Path(entry["export_receipt"])
    require(digest(formula) == entry["formula_sha256"] and
            formula.stat().st_size == entry["formula_bytes"], "formula bytes")
    require(formula.open("rb").readline().decode().strip() == entry["formula_header"],
            "formula header")
    require(digest(receipt_path) == entry["export_receipt_sha256"], "export receipt hash")
    receipt = json.loads(receipt_path.read_text())
    require(receipt["status"] == "PASS_XCNF_EXPORT_ONLY" and
            receipt["solver_search_executed"] is False and
            receipt["mode"] == "natural" and receipt["public_fixture"] == 1 and
            receipt["target"] == lock["target"] and
            receipt["xcnf_sha256"] == entry["formula_sha256"] and
            receipt["source_commit"] == entry["export_source_commit"] and
            receipt["curve_a"] == 0, "export identity")
    positive_path = repo / entry["positive_replay"]
    require(digest(positive_path) == entry["positive_replay_sha256"], "positive replay hash")
    require(json.loads(positive_path.read_text())["status"] ==
            entry["positive_replay_status"], "positive replay status")
    require(lock["conflict_cap"] == 1_000_000 and
            lock["solver_time_cap_seconds"] == 600 and
            lock["outer_watchdog_seconds"] == 660, "caps")
    return solver, formula, entry


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


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("lock", type=Path)
    parser.add_argument("arm", choices=("d18", "k2048"))
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[4]
    lock = json.loads(args.lock.read_text())
    solver, formula, entry = check_lock(lock, args.arm, repo)
    run_root = Path(lock["run_root"])
    require(args.output == run_root / args.arm, "output differs from frozen run order")
    if args.arm == "k2048":
        prior = json.loads((run_root / "d18/receipt.json").read_text())
        require(prior["schema"] == "n83.a0-matched-public-solver-run/v1" and
                prior["arm"] == "d18" and prior["fixture"] == 1 and
                prior["lock_sha256"] == digest(args.lock), "D18 predecessor differs")
    require(not args.output.exists(), "output exists")
    require(shutil.disk_usage(args.output.parent).free >= 2 * 1024**3, "scratch gate")
    samples = []
    for index in range(3):
        samples.append(host_sample())
        if index < 2:
            time.sleep(15)
    args.output.mkdir(exist_ok=False)
    (args.output / "host-preflight.json").write_text(json.dumps(samples, indent=2) + "\n")
    fresh = host_sample()
    if (any(sample["load_1m"] > 24 for sample in samples + [fresh]) or
            len({sample["swapouts"] for sample in samples + [fresh]}) != 1):
        (args.output / "rejection.json").write_text(json.dumps({
            "status": "HOST_REJECTED", "samples": samples, "launch_sample": fresh}, indent=2) + "\n")
        raise ValueError("host load or swapout gate")
    command = [str(solver), *lock["solver_flags_base"],
               *entry["flags_tail"], str(formula)]
    (args.output / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    start = time.monotonic()
    timed_out = False
    with (args.output / "stdout.txt").open("wb") as stdout, \
            (args.output / "stderr.txt").open("wb") as stderr:
        child = subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=True)
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
    transcript = (args.output / "stdout.txt").read_text(errors="replace")
    conflicts = [line.strip() for line in transcript.splitlines()
                 if line.startswith("c conflicts                :")]
    result = {
        "schema": "n83.a0-matched-public-solver-run/v1", "arm": args.arm,
        "fixture": lock["fixture"], "target": lock["target"],
        "status": terminal(code, timed_out, transcript), "exit_code": code,
        "timed_out": timed_out, "wall_seconds": time.monotonic() - start,
        "cpu_user_seconds": usage.ru_utime, "cpu_system_seconds": usage.ru_stime,
        "wait4_peak_rss_bytes": usage.ru_maxrss,
        "terminal_conflicts": int(conflicts[0].split(":", 1)[1]) if len(conflicts) == 1 else None,
        "host_preflight": samples, "swapouts_after": swapouts(),
        "lock_sha256": digest(args.lock), "solver_sha256": lock["solver_sha256"],
        "formula_sha256": entry["formula_sha256"],
        "stdout_sha256": digest(args.output / "stdout.txt"),
        "stderr_sha256": digest(args.output / "stderr.txt"),
        "verified_relation": None, "novel_rank": None,
        "single_target_online_ms": None, "selected_best_total_runtime": None,
    }
    (args.output / "receipt.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: result[key] for key in
                      ("arm", "status", "exit_code", "terminal_conflicts", "wall_seconds")}))


if __name__ == "__main__":
    main()
