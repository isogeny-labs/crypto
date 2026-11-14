#!/usr/bin/env python3
"""Audit retained K2048 m5 bytes, host gates and capped solver statuses."""

from datetime import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return (EVIDENCE / path).read_bytes()


def decompress(path):
    return subprocess.check_output(["/opt/homebrew/bin/zstd", "-dc", str(EVIDENCE / path)])


def host_gate(mode, lock, receipt, manifest):
    names = [name for name in manifest["entries"] if name.startswith(f"host-preflight-{mode}-")]
    require(len(names) == 1, f"{mode} host sample count")
    samples = json.loads(read(names[0]))
    require(samples == receipt["host_preflight"] and len(samples) == lock["sample_count"], f"{mode} host receipt differs")
    require(all(item["load_1m"] <= lock["load_gate_1m"] for item in samples), f"{mode} load gate")
    require(len({item["swapouts"] for item in samples}) == 1, f"{mode} swapout gate")
    require(receipt["swapouts_after"] == samples[-1]["swapouts"], f"{mode} post-run swapouts")
    times = [datetime.fromisoformat(item["utc"]) for item in samples]
    require(all((later - earlier).total_seconds() >= lock["sample_spacing_seconds"] for earlier, later in zip(times, times[1:])), f"{mode} sample spacing")
    return [item["load_1m"] for item in samples]


def main():
    manifest = json.loads(read("RAW_MANIFEST.json"))
    require(manifest["schema"] == "n83.k2048-m5-raw-manifest/v1", "manifest schema")
    expected = set(manifest["entries"])
    actual = {str(p.relative_to(EVIDENCE)) for p in EVIDENCE.rglob("*") if p.is_file()}
    require(actual - {"RAW_MANIFEST.json", "audit.json"} == expected, "raw file set differs")
    for name, item in manifest["entries"].items():
        data = read(name)
        require(len(data) == item["bytes"] and sha256(data) == item["sha256"], f"raw hash differs: {name}")
    results = {}
    for mode in ("positive", "natural"):
        lock = json.loads((HERE / f"{mode.upper()}_SOLVER_LOCK.json").read_text())
        export = json.loads(read(f"export-{mode}/result.json"))
        solver = json.loads(read(f"solver-{mode}/receipt.json"))
        require(sha256(read(f"export-{mode}/result.json")) == lock["export_receipt_sha256"], f"{mode} export receipt")
        require(export["status"] == "PASS_XCNF_EXPORT_ONLY" and export["source_commit"] == lock["source_commit"], f"{mode} source binding")
        require(export["binary_sha256"] == lock["worker_sha256"] and solver["worker_sha256"] == lock["worker_sha256"], f"{mode} worker binding")
        require(solver["solver_sha256"] == lock["solver_sha256"] and solver["source_commit"] == lock["source_commit"], f"{mode} solver binding")
        require(solver["command"] == [lock["solver"], *lock["solver_flags"], str(Path(lock["raw_root"]) / f"export-{mode}/result.xcnf")], f"{mode} solver command")
        require(json.loads(read(f"solver-{mode}/command.json")) == solver["command"], f"{mode} command record")
        formula = decompress(f"export-{mode}/result.xcnf.zst")
        require(len(formula) == lock["xcnf_bytes"] and sha256(formula) == lock["xcnf_sha256"], f"{mode} formula round trip")
        require(formula.splitlines()[0].decode() == f'p cnf {lock["variables"]} {lock["cnf_clauses"] + lock["xor_rows"]}', f"{mode} XCNF header")
        stdout = decompress(f"solver-{mode}/stdout.txt.zst")
        require(sha256(stdout) == solver["stdout_sha256"] and sha256(read(f"solver-{mode}/stderr.txt")) == solver["stderr_sha256"], f"{mode} solver transcript")
        statuses = re.findall(rb"^s ([A-Z]+)$", stdout, re.MULTILINE)
        conflicts = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
        require(len(conflicts) == 1, f"{mode} terminal conflict counter")
        loads = host_gate(mode, lock, solver, manifest)
        require(solver["outer_timeout"] is False and solver["wall_seconds"] <= lock["outer_watchdog_seconds"], f"{mode} watchdog")
        if mode == "positive":
            replay = json.loads(read("solver-positive/replay.json"))
            require(sha256(read("solver-positive/replay.json")) == json.loads((HERE / "NATURAL_SOLVER_LOCK.json").read_text())["positive_replay_sha256"], "positive replay hash")
            require(replay["status"] == "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY", "positive replay status")
            require((replay["checked_cnf_clauses"], replay["checked_xor_rows"]) == (lock["cnf_clauses"], lock["xor_rows"]), "positive replay shape")
            require(solver["status"] == "SAT_PENDING_REPLAY" and solver["exit_code"] == 10 and statuses == [b"SATISFIABLE"], "positive solver status")
            require(re.findall(rb"^v ", stdout, re.MULTILINE), "positive model absent")
        else:
            require(solver["status"] == "INDETERMINATE" and solver["exit_code"] == 15 and statuses == [b"INDETERMINATE"], "natural solver status")
            require(conflicts == [b"1000001"], "natural conflict cap differs")
            require(not re.findall(rb"^v ", stdout, re.MULTILINE), "natural transcript unexpectedly contains a model")
            require(export["shape"]["positive_x_pins"] == 0 and export["target"] == ["1834913121178197242856238", "8797964190732846213981377"], "natural target differs")
        results[mode] = {
            "solver_status": solver["status"], "exit_code": solver["exit_code"],
            "conflicts": int(conflicts[0]), "wall_seconds": solver["wall_seconds"],
            "cpu_seconds": solver["cpu_user_seconds"] + solver["cpu_system_seconds"],
            "load_1m": loads, "formula_sha256": lock["xcnf_sha256"],
            "stdout_sha256": solver["stdout_sha256"],
            "wait4_peak_rss_bytes": solver.get("wait4_peak_rss_bytes"),
        }
    report = {
        "schema": "n83.k2048-m5-raw-audit/v1",
        "status": "PASS_RAW_CUSTODY_AND_TERMINAL_CLASSIFICATION",
        "raw_manifest_sha256": sha256(read("RAW_MANIFEST.json")),
        "results": results, "selected_best_total_runtime": None,
    }
    path = EVIDENCE / "audit.json"
    encoded = json.dumps(report, sort_keys=True, indent=2) + "\n"
    if path.exists():
        require(path.read_text() == encoded, "retained audit differs")
    else:
        with path.open("x") as stream:
            stream.write(encoded)
    print(json.dumps({"status": report["status"], "results": results}, sort_keys=True))


if __name__ == "__main__":
    main()
