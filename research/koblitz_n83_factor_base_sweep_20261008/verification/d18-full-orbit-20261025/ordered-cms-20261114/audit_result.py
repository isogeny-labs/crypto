#!/usr/bin/env python3
"""Read raw ordered-CMS output separately from the process runner."""

import argparse
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    root = args.root.resolve()
    lock = json.loads((HERE / "EXPORT_LOCK.json").read_text())
    receipt_path = root / "solver-positive/receipt.json"
    receipt = json.loads(receipt_path.read_text())
    stdout = root / "solver-positive/stdout.txt"
    stderr = root / "solver-positive/stderr.txt"
    command = json.loads((root / "solver-positive/command.json").read_text())
    replay = root / "replay.json"
    xcnf = root / "export-positive/result.xcnf"
    require(sha256(replay) == lock["positive_replay_sha256"], "positive replay differs")
    require(sha256(xcnf) == lock["exports"]["positive"]["xcnf_sha256"], "formula differs")
    require(receipt["xcnf_sha256"] == lock["exports"]["positive"]["xcnf_sha256"], "receipt formula differs")
    require(receipt["solver_binary_sha256"] == lock["solver_binary_sha256"], "solver differs")
    require(receipt["source_commit"] == lock["source_commit"], "source differs")
    require(command == receipt["command"], "command differs")
    require(command[1:-1] == lock["solver_flags"], "solver flags differ")
    require(command[-1] == str(xcnf), "solver input path differs")
    require(sha256(stdout) == receipt["stdout_sha256"], "stdout digest differs")
    require(sha256(stderr) == receipt["stderr_sha256"], "stderr digest differs")
    require(receipt["exit_code"] == 15 and receipt["outer_timeout"] is False,
            "exit or watchdog differs")
    require(0 < receipt["wall_seconds"] < lock["outer_watchdog_seconds"], "wall outside cap")
    require(receipt["swapouts_after"] == receipt["swapouts_preflight"], "swapouts changed")
    lines = stdout.read_text(errors="replace").splitlines()
    statuses = [line for line in lines if line.startswith("s ")]
    require(statuses == ["s INDETERMINATE"], "solver status differs")
    conflict_lines = [line for line in lines if re.match(r"^c conflicts\s*:", line)]
    require(len(conflict_lines) == 1, "conflict count missing or duplicated")
    conflicts = int(conflict_lines[0].split(":", 1)[1].strip().split()[0])
    require(conflicts >= 1_000_000, "solver stopped below the frozen conflict cap")
    require(receipt["status"] == "INDETERMINATE", "runner status differs")
    report = {
        "schema": "n83.d18-ordered-cms-local-audit/v1",
        "status": "LOCAL_AUDIT_PASS_INDETERMINATE",
        "xcnf_sha256": receipt["xcnf_sha256"],
        "stdout_sha256": receipt["stdout_sha256"],
        "receipt_sha256": sha256(receipt_path),
        "exit_code": receipt["exit_code"],
        "solver_status": statuses[0],
        "reported_conflicts": conflicts,
        "wall_seconds": receipt["wall_seconds"],
        "swapouts_unchanged": True,
        "natural_arm_released": False,
        "selected_best_total_runtime": None,
    }
    with (root / "audit.json").open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "conflicts": conflicts}, sort_keys=True))


if __name__ == "__main__":
    main()
