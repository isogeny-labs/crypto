#!/usr/bin/env python3
"""Read the frozen VSIDS raw files independently of the process supervisor."""

import argparse
import hashlib
import json
from pathlib import Path
import re


def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_json(path):
    return json.loads(path.read_text())


def check_host(path, receipt):
    lines = path.read_text().splitlines()
    require(len(lines) == 12, f"{path.name}: sample count")
    loads = [float(lines[i].split()[1]) for i in (1, 5, 9)]
    swaps = [int(lines[i].split()[1].rstrip(".")) for i in (2, 6, 10)]
    require(loads == receipt["host_load_1m_preflight"], f"{path.name}: loads")
    require(all(load <= 24 for load in loads), f"{path.name}: load gate")
    require(len(set(swaps)) == 1, f"{path.name}: swap gate")
    require(swaps[0] == receipt["swapouts_preflight"] == receipt["swapouts_after"],
            f"{path.name}: swap receipt")
    return {"loads": loads, "swapouts": swaps[0], "sha256": digest(path)}


def check_run(root, mode, lock, expected_status):
    folder = root / f"solver-{mode}"
    receipt = read_json(folder / "receipt.json")
    stdout_path = folder / "stdout.txt"
    stderr_path = folder / "stderr.txt"
    stdout = stdout_path.read_text()
    require(receipt["mode"] == mode and receipt["status"] == expected_status, "receipt status")
    require(receipt["solver_binary_sha256"] == lock["solver_binary_sha256"], "solver binary")
    require(receipt["worker_binary_sha256"] == lock["worker_binary_sha256"], "worker binary")
    require(receipt["source_commit"] == lock["source_commit"], "source commit")
    require(receipt["xcnf_sha256"] == lock["exports"][mode]["xcnf_sha256"], "formula binding")
    require(digest(root / f"export-{mode}/result.xcnf") == receipt["xcnf_sha256"], "formula bytes")
    require(digest(stdout_path) == receipt["stdout_sha256"], "stdout hash")
    require(digest(stderr_path) == receipt["stderr_sha256"], "stderr hash")
    expected_command = ["/opt/homebrew/bin/cryptominisat5", *lock["solver_flags"],
                        str(root / f"export-{mode}/result.xcnf")]
    require(receipt["command"] == expected_command == read_json(folder / "command.json"), "command")
    require(not receipt["outer_timeout"] and receipt["wall_seconds"] <= 660, "watchdog")
    statuses = re.findall(r"^s (.+)$", stdout, re.MULTILINE)
    if mode == "positive":
        require(receipt["exit_code"] == 10 and statuses == ["SATISFIABLE"], "positive terminal")
        verify = read_json(folder / "verification.json")
        group = read_json(folder / "group-replay.json")
        require(verify["status"] == "PASS_ORIGINAL_XCNF_REPLAY", "positive XCNF replay")
        require((verify["checked_cnf_clauses"], verify["checked_xor_rows"]) ==
                (2807999, 3901), "positive formula counts")
        require(group["status"] == "PASS_ORIGINAL_GROUP_AND_ROW_REPLAY", "positive group replay")
        require(group["solver_stdout_sha256"] == receipt["stdout_sha256"], "group source stdout")
        conflicts = int(re.search(r"^c conflicts\s*:\s*(\d+)", stdout, re.MULTILINE).group(1))
    else:
        require(receipt["exit_code"] == 15 and statuses == ["INDETERMINATE"], "natural terminal")
        require(not re.search(r"^v ", stdout, re.MULTILINE), "natural model present")
        conflicts = int(re.search(r"^c conflicts\s*:\s*(\d+)", stdout, re.MULTILINE).group(1))
        require(conflicts in (1000000, 1000001, 1000002), "natural conflict cap")
    host_file = root / receipt.get("host_preflight_file", f"host-preflight-{mode}.txt")
    host = check_host(host_file, receipt)
    if "host_preflight_sha256" in receipt:
        require(host["sha256"] == receipt["host_preflight_sha256"], "host file hash")
    return {"status": receipt["status"], "exit_code": receipt["exit_code"],
            "conflicts": conflicts, "wall_seconds": receipt["wall_seconds"],
            "cpu_user_seconds": receipt["cpu_user_seconds"],
            "sampled_peak_rss_kb": receipt["sampled_peak_rss_kb"],
            "stdout_sha256": receipt["stdout_sha256"], "receipt_sha256": digest(folder / "receipt.json"),
            "host": host}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    base = Path(__file__).parent
    positive_lock = read_json(base / "EXPORT_LOCK.json")
    natural_lock = read_json(base / "NATURAL_SOLVER_LOCK.json")
    positive = check_run(args.root, "positive", positive_lock, "SAT_PENDING_XCNF_REPLAY")
    natural = check_run(args.root, "natural", natural_lock, "INDETERMINATE")
    require(digest(args.root / "solver-positive/group-replay.json") ==
            natural_lock["positive_group_replay_sha256"], "natural positive gate")
    report = {"schema": "n83.d18-vsids-raw-audit/v1", "status": "PASS_CENSORED_NATURAL",
              "positive": positive, "natural": natural,
              "natural_verified_relations": 0, "selected_best_total_runtime": None}
    with (args.root / "audit.json").open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "positive_conflicts": positive["conflicts"],
                      "natural_conflicts": natural["conflicts"]}, sort_keys=True))


if __name__ == "__main__":
    main()
