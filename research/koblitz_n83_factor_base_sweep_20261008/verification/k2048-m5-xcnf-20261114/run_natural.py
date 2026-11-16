#!/usr/bin/env python3
"""Run one locked public K2048 m5 natural query with raw host and solver custody."""

from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time

from run_positive import require, sha256, swapouts


HERE = Path(__file__).resolve().parent
LOCK = json.loads((HERE / "NATURAL_SOLVER_LOCK.json").read_text())


def utc():
    return datetime.now(timezone.utc).isoformat()


def status(code, timed_out, stdout):
    if timed_out:
        return "WATCHDOG"
    lines = [line for line in stdout.splitlines() if line.startswith("s ")]
    if code == 10 and lines == ["s SATISFIABLE"]:
        return "SAT_PENDING_REPLAY"
    if code == 20 and lines == ["s UNSATISFIABLE"]:
        return "UNSAT_UNVERIFIED"
    if code == 15 and lines == ["s INDETERMINATE"]:
        return "INDETERMINATE"
    return "PRODUCER_FAILURE"


def main():
    require(LOCK["schema"] == "n83.k2048-m5-natural-solver-lock/v1", "lock schema")
    source = HERE.parents[3]
    commit = subprocess.check_output(["git", "-C", str(source), "cat-file", "-t", LOCK["source_commit"]], text=True).strip()
    require(commit == "commit", "frozen source commit unavailable")
    root = Path(LOCK["raw_root"])
    worker, solver = Path(LOCK["worker"]), Path(LOCK["solver"])
    export = root / "export-natural"
    receipt_path, xcnf = export / "result.json", export / "result.xcnf"
    positive_replay_path = root / "solver-positive/replay.json"
    require(sha256(worker) == LOCK["worker_sha256"], "worker binary differs")
    require(sha256(solver) == LOCK["solver_sha256"], "solver binary differs")
    require(sha256(positive_replay_path) == LOCK["positive_replay_sha256"], "positive replay differs")
    positive = json.loads(positive_replay_path.read_text())
    require(positive["status"] == "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY", "positive replay gate")
    require(sha256(receipt_path) == LOCK["export_receipt_sha256"], "export receipt differs")
    export_receipt = json.loads(receipt_path.read_text())
    require(export_receipt["status"] == "PASS_XCNF_EXPORT_ONLY" and export_receipt["source_commit"] == LOCK["source_commit"], "export source differs")
    require(export_receipt["mode"] == "natural" and export_receipt["shape"]["positive_x_pins"] == 0, "natural formula policy differs")
    require(sha256(xcnf) == LOCK["xcnf_sha256"] and xcnf.stat().st_size == LOCK["xcnf_bytes"], "XCNF differs")
    require(xcnf.open("rb").readline().decode().strip() == f'p cnf {LOCK["variables"]} {LOCK["cnf_clauses"] + LOCK["xor_rows"]}', "XCNF shape differs")
    require(shutil.disk_usage(root).free >= LOCK["scratch_min_bytes"], "scratch gate")
    preflight = []
    for index in range(LOCK["sample_count"]):
        preflight.append({"utc": utc(), "load_1m": os.getloadavg()[0], "swapouts": swapouts()})
        if index + 1 < LOCK["sample_count"]:
            time.sleep(LOCK["sample_spacing_seconds"])
    preflight_path = root / ("host-preflight-natural-" + utc().replace(":", "-") + ".json")
    with preflight_path.open("x") as stream:
        json.dump(preflight, stream, sort_keys=True, indent=2)
        stream.write("\n")
    require(all(item["load_1m"] <= LOCK["load_gate_1m"] for item in preflight), "host load gate")
    require(len({item["swapouts"] for item in preflight}) == 1, "host swapout gate")
    require(os.getloadavg()[0] <= LOCK["load_gate_1m"] and swapouts() == preflight[-1]["swapouts"], "launch host gate")
    out = root / "solver-natural"
    out.mkdir(exist_ok=False)
    command = [str(solver), *LOCK["solver_flags"], str(xcnf)]
    (out / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    started_utc = utc()
    began = time.monotonic()
    timed_out = False
    peak_rss_kb = 0
    with (out / "stdout.txt").open("wb") as stdout, (out / "stderr.txt").open("wb") as stderr:
        child = subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=True)
        while True:
            pid, wait_status, usage = os.wait4(child.pid, os.WNOHANG)
            if pid:
                code = os.waitstatus_to_exitcode(wait_status)
                break
            sample = subprocess.run(["ps", "-o", "rss=", "-p", str(child.pid)], capture_output=True, text=True)
            if sample.returncode == 0 and sample.stdout.strip().isdigit():
                peak_rss_kb = max(peak_rss_kb, int(sample.stdout.strip()))
            if time.monotonic() - began >= LOCK["outer_watchdog_seconds"]:
                timed_out = True
                os.killpg(child.pid, signal.SIGTERM)
                time.sleep(5)
                pid, wait_status, usage = os.wait4(child.pid, os.WNOHANG)
                if not pid:
                    os.killpg(child.pid, signal.SIGKILL)
                    pid, wait_status, usage = os.wait4(child.pid, 0)
                code = os.waitstatus_to_exitcode(wait_status)
                break
            time.sleep(1)
    report = {
        "schema": "n83.k2048-m5-natural-solver/v1",
        "status": status(code, timed_out, (out / "stdout.txt").read_text(errors="replace")),
        "source_commit": LOCK["source_commit"], "worker_sha256": LOCK["worker_sha256"],
        "solver_sha256": LOCK["solver_sha256"], "xcnf_sha256": LOCK["xcnf_sha256"],
        "command": command, "start_utc": started_utc, "end_utc": utc(),
        "exit_code": code, "outer_timeout": timed_out,
        "wall_seconds": time.monotonic() - began,
        "cpu_user_seconds": usage.ru_utime, "cpu_system_seconds": usage.ru_stime,
        "wait4_peak_rss_bytes": usage.ru_maxrss, "sampled_peak_rss_kb": peak_rss_kb,
        "host_preflight": preflight, "host_preflight_path": str(preflight_path),
        "swapouts_after": swapouts(),
        "stdout_sha256": sha256(out / "stdout.txt"), "stderr_sha256": sha256(out / "stderr.txt"),
        "selected_best_total_runtime": None,
    }
    (out / "receipt.json").write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ("status", "exit_code", "wall_seconds", "wait4_peak_rss_bytes")}, sort_keys=True))


if __name__ == "__main__":
    main()
