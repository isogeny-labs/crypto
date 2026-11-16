#!/usr/bin/env python3
"""Independently audit the retained bounded N83 public solver pair."""

import gzip
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent


def sha(path):
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def check(ok, reason):
    if not ok:
        raise ValueError(reason)


def main():
    lock_path = ROOT / "LOCK.json"
    lock = json.loads(lock_path.read_text())
    corpus_path = Path(lock["corpus"])
    check(sha(corpus_path) == lock["corpus_sha256"], "public corpus hash")
    corpus = json.loads(corpus_path.read_text())
    selected = [row for row in corpus["targets"]
                if row["a"] == 0 and row["fixture"] == 1]
    check(len(selected) == 1 and selected[0]["point"] == lock["target"],
          "public corpus point")
    repo = ROOT.parents[3]
    rows = []
    for arm in lock["run_order"]:
        retained = ROOT / "evidence" / arm
        record = json.loads((retained / "receipt.json").read_text())
        command = json.loads((retained / "command.json").read_text())
        host = json.loads((retained / "host-preflight.json").read_text())
        stdout = gzip.decompress((retained / "stdout.txt.gz").read_bytes())
        text = stdout.decode(errors="replace")
        statuses = [line for line in text.splitlines() if line.startswith("s ")]
        conflicts = [line for line in text.splitlines()
                     if line.startswith("c conflicts                :")]
        entry = lock["arms"][arm]
        export_path = Path(entry["export_receipt"])
        check(sha(export_path) == entry["export_receipt_sha256"],
              f"{arm}: export receipt hash")
        export = json.loads(export_path.read_text())
        check(export["public_fixture"] == 1 and
              export["target"] == lock["target"] and
              export["source_commit"] == entry["export_source_commit"] and
              export["xcnf_sha256"] == entry["formula_sha256"],
              f"{arm}: export identity")
        replay_path = repo / entry["positive_replay"]
        check(sha(replay_path) == entry["positive_replay_sha256"] and
              json.loads(replay_path.read_text())["status"] ==
              entry["positive_replay_status"], f"{arm}: positive replay")
        check(record["arm"] == arm and record["fixture"] == 1 and
              record["target"] == lock["target"], f"{arm}: target")
        check(record["lock_sha256"] == sha(lock_path) and
              record["formula_sha256"] == entry["formula_sha256"], f"{arm}: lock")
        check(sha(Path(entry["formula"])) == entry["formula_sha256"],
              f"{arm}: raw formula")
        check(sha(Path(lock["solver"])) == lock["solver_sha256"],
              f"{arm}: solver")
        check(command == [lock["solver"], *lock["solver_flags_base"],
                          *entry["flags_tail"], entry["formula"]], f"{arm}: command")
        check(record["stdout_sha256"] == hashlib.sha256(stdout).hexdigest() and
              record["stderr_sha256"] == sha(retained / "stderr.txt"),
              f"{arm}: transcripts")
        check(statuses == ["s INDETERMINATE"] and record["exit_code"] == 15 and
              record["status"] == "INDETERMINATE" and
              len(conflicts) == 1 and
              int(conflicts[0].split(":", 1)[1]) == record["terminal_conflicts"] and
              record["terminal_conflicts"] >= lock["conflict_cap"],
              f"{arm}: terminal status")
        check(len(host) == 3 and all(row["load_1m"] <= 24 for row in host)
              and len({row["swapouts"] for row in host}) == 1
              and record["host_preflight"] == host and
              record["swapouts_after"] == host[0]["swapouts"], f"{arm}: host")
        check(record["verified_relation"] is None and
              record["novel_rank"] is None and
              record["single_target_online_ms"] is None and
              record["selected_best_total_runtime"] is None,
              f"{arm}: scientific boundary")
        rows.append({"arm": arm, "fixture": 1,
                     "target": lock["target"], "status": record["status"],
                     "conflicts": record["terminal_conflicts"],
                     "wall_seconds": record["wall_seconds"],
                     "cpu_seconds": record["cpu_user_seconds"] +
                                    record["cpu_system_seconds"],
                     "peak_rss_bytes": record["wait4_peak_rss_bytes"],
                     "host_load_1m": [sample["load_1m"] for sample in host],
                     "stdout_sha256": record["stdout_sha256"]})
    check(rows[0]["target"] == rows[1]["target"], "same point")
    result = {"schema": "n83.a0-matched-public-raw-audit/v1",
              "status": "PASS", "lock_sha256": sha(lock_path), "rows": rows}
    print(json.dumps(result, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
