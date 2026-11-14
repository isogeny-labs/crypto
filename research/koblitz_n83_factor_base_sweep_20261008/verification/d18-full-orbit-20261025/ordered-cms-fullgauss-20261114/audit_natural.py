#!/usr/bin/env python3
"""Audit the first natural D18 CryptoMiniSat cap from retained raw bytes."""

import argparse
import hashlib
import json
from pathlib import Path
import re


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
    root = args.root
    lock = json.loads(Path(__file__).with_name("NATURAL_SOLVER_LOCK.json").read_text())
    out = root / "solver-natural"
    receipt = json.loads((out / "receipt.json").read_text())
    command = json.loads((out / "command.json").read_text())
    export = json.loads((root / "export-natural/result.json").read_text())
    stdout = (out / "stdout.txt").read_text()
    stderr = out / "stderr.txt"
    sample_path = root / receipt["host_preflight_file"]
    samples = sample_path.read_text().splitlines()
    statuses = re.findall(r"^s (.+)$", stdout, re.MULTILINE)
    conflicts = re.findall(r"^c conflicts\s+: (\d+)\s*$", stdout, re.MULTILINE)
    matrices = re.findall(r"^c \[matrix\] Using (\d+) matrices", stdout, re.MULTILINE)
    require(receipt["schema"] == "n83.d18-ordered-cms-fullgauss-natural-run/v1", "run schema differs")
    require(receipt["status"] == "INDETERMINATE" and receipt["exit_code"] == 15, "terminal status differs")
    require(receipt["outer_timeout"] is False, "outer watchdog fired")
    require(statuses == ["INDETERMINATE"] and not re.search(r"^v ", stdout, re.MULTILINE), "solver status or model differs")
    require(conflicts == ["1000001"], "reported conflict cap differs")
    require(matrices and max(map(int, matrices)) == 28, "Gaussian matrix count differs")
    require(command == receipt["command"] and command[1:-1] == lock["solver_flags"], "solver command differs")
    require(receipt["solver_binary_sha256"] == lock["solver_binary_sha256"], "solver binary binding differs")
    require(receipt["source_commit"] == lock["source_commit"], "source binding differs")
    require(receipt["xcnf_sha256"] == lock["exports"]["natural"]["xcnf_sha256"] == sha256(root / "export-natural/result.xcnf"), "XCNF binding differs")
    require(sha256(root / "export-natural/result.json") == lock["exports"]["natural"]["worker_receipt_sha256"], "export receipt differs")
    require(export["target"] == lock["expected_target"] and export["status"] == "PASS_XCNF_EXPORT_ONLY", "natural export differs")
    require(sha256(out / "stdout.txt") == receipt["stdout_sha256"] and sha256(stderr) == receipt["stderr_sha256"], "solver transcript differs")
    require(sha256(sample_path) == receipt["host_preflight_sha256"] and len(samples) == 12, "host sample differs")
    loads = [float(samples[index].split()[1]) for index in (1, 5, 9)]
    swaps = [int(samples[index].split()[1].rstrip(".")) for index in (2, 6, 10)]
    require(all(load <= 24 for load in loads), "host load gate differs")
    require(len(set(swaps)) == 1 and swaps[0] == receipt["swapouts_preflight"] == receipt["swapouts_after"], "swapout gate differs")
    report = {
        "schema": "n83.d18-ordered-fullgauss-natural-audit/v1",
        "status": "PASS_RETAINED_INDETERMINATE",
        "solver_status": receipt["status"],
        "reported_conflicts": int(conflicts[0]),
        "maximum_gaussian_matrices_used": max(map(int, matrices)),
        "solver_wall_seconds": receipt["wall_seconds"],
        "stdout_sha256": receipt["stdout_sha256"],
        "receipt_sha256": sha256(out / "receipt.json"),
        "xcnf_sha256": receipt["xcnf_sha256"],
        "host_load_1m_preflight": loads,
        "swapouts": swaps[0],
        "relation_counted": 0,
        "relation_count_status": "CENSORED_AT_CONFLICT_CAP",
        "selected_best_total_runtime": None,
    }
    with (out / "audit.json").open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "conflicts": report["reported_conflicts"]}, sort_keys=True))


if __name__ == "__main__":
    main()
