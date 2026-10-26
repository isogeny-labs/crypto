#!/usr/bin/env python3
"""Independently replay raw Gray 5+1 access evidence and any complete model."""

import argparse
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import tempfile


HERE = Path(__file__).resolve().parent
STUDY = HERE.parent.parent
BUNDLE = HERE.parent / "linux-xor-complete-inputs-20261025"
CORE = HERE.parent / "local-yield-frontier-20261025/run_access.py"
SOURCE = HERE.parent / "external-xor-20261025/receipt.json"
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"
POOL = "966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47"


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def file_sha(path: Path) -> str:
    return sha(path.read_bytes())


def module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


bundle = module("n83_gray51_independent_bundle", BUNDLE / "audit.py")


def raw(run: Path, name: str) -> bytes:
    path = run / name
    compressed = run / f"{name}.gz"
    require(path.exists() != compressed.exists(), f"{name}: one raw stream")
    return path.read_bytes() if path.exists() else gzip.decompress(compressed.read_bytes())


def replay_model(inputs: Path, worker: Path, solver_stdout: bytes) -> dict:
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    with tempfile.TemporaryDirectory(prefix="n83-gray51-model-replay-") as directory:
        root = Path(directory)
        export = root / "export"
        export.mkdir()
        (export / "instance.xcnf").symlink_to(inputs / "gray_5_1.xcnf")
        (export / "export.json").symlink_to(
            inputs / "exports/gray_5_1/export.json")
        stdout = root / "solver.stdout"
        stdout.write_bytes(solver_stdout)
        command = [
            str(worker), "verify-cross-xcnf", BASE, str(manifest),
            str(inputs / "archives" / f"s3-{BASE}.jsonl.gz"),
            POOL, str(manifest),
            str(inputs / "archives" / f"s3-{POOL}.jsonl.gz"),
            str(corpus), "5", "1", str(export), str(stdout),
            str(root / "verdict"),
        ]
        result = subprocess.run(command, capture_output=True, timeout=300)
        require(result.returncode == 0 and (root / "verdict/result.json").exists(),
                "independent model verifier must return")
        return json.loads((root / "verdict/result.json").read_bytes())


def audit(run: Path, inputs: Path, worker: Path, solver: Path) -> dict:
    protocol_path = HERE / "protocol.json"
    protocol = json.loads(protocol_path.read_bytes())
    require(protocol["schema"] == "n83.local-gray51-wide-natural-access/v1"
            and protocol["selected_best_total_runtime"] is None
            and protocol["auditor_sha256"] == file_sha(Path(__file__))
            and protocol["runner_sha256"] == file_sha(HERE / "run.py")
            and protocol["core_runner_sha256"] == file_sha(CORE)
            and protocol["source_receipt_sha256"] == file_sha(SOURCE)
            and protocol["input_manifest_sha256"] == file_sha(BUNDLE / "manifest.json")
            and protocol["input_receipt_sha256"] == file_sha(BUNDLE / "receipt.json")
            and protocol["worker_binary_sha256"] == file_sha(worker)
            and protocol["solver_binary_sha256"] == file_sha(solver),
            "frozen source, binary and S3-custody identity")
    require(bundle.audit(inputs)["status"] == "PASS_COMPLETE_INPUT_REPLAY"
            and file_sha(inputs / "gray_5_1.xcnf") == protocol["xcnf_sha256"],
            "original S3 inputs")
    process = json.loads((run / "process.json").read_bytes())
    summary = json.loads((run / "summary.json").read_bytes())
    output = raw(run, "solver.stdout")
    errors = raw(run, "solver.stderr")
    require(process["schema"] == "n83.local-xor-natural-access-process/v1"
            and process["stdout_sha256"] == sha(output)
            and process["stderr_sha256"] == sha(errors)
            and process["stdout_bytes"] == len(output)
            and process["stderr_bytes"] == len(errors)
            and process["xcnf_sha256"] == protocol["xcnf_sha256"]
            and process["solver_binary_sha256"] == protocol["solver_binary_sha256"]
            and process["selected_best_total_runtime"] is None
            and process["wall_seconds_L0"] >= 0,
            "process receipt and raw streams")
    command = process["command"]
    require(command[0] == str(solver)
            and command[1:-1] == protocol["solver_flags"]
            and Path(command[-1]).parts[-2:] == ("formula", "instance.xcnf"),
            "exact solver command")
    statuses = re.findall(rb"^s ([A-Z]+)\s*$", output, re.MULTILINE)
    counters = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", output,
                          re.MULTILINE)
    conflicts = int(counters[0]) if len(counters) == 1 else None
    status = process["status"]
    if status == "WATCHDOG":
        require(process["exit_code"] == -9
                and process["wall_seconds_L0"] >= protocol["process_watchdog_seconds"]
                and b"SATISFIABLE" not in statuses,
                "watchdog process and no accepted SAT terminal")
    elif status == "MALFORMED":
        require(errors or len(statuses) != 1 or conflicts is None
                or (statuses[0].decode(), process["exit_code"]) not in
                (("INDETERMINATE", 15), ("SATISFIABLE", 10),
                 ("UNSATISFIABLE", 20)),
                "malformed classification")
    else:
        require(len(statuses) == 1 and not errors and conflicts is not None
                and (status, process["exit_code"]) in
                (("INDETERMINATE", 15), ("SATISFIABLE", 10),
                 ("UNSATISFIABLE", 20))
                and statuses[0].decode() == status,
                "terminal solver classification")
    require(process["conflicts"] == conflicts, "exact final conflict counter")
    require(summary["schema"] == "n83.local-gray51-wide-summary/v1"
            and summary["protocol_sha256"] == file_sha(protocol_path)
            and summary["status"] == status
            and summary["conflicts"] == conflicts
            and summary["solver_wall_seconds_L0"] == process["wall_seconds_L0"]
            and summary["selected_best_total_runtime"] is None,
            "summary classification and null selection")
    require((run / "host-before.txt").stat().st_size > 0
            and (run / "host-after.txt").stat().st_size > 0,
            "host load snapshots")
    if status == "SATISFIABLE":
        verify = json.loads((run / "verify-process.json").read_bytes())
        verdict = replay_model(inputs, worker, output)
        require(verify["schema"] == "n83.local-gray51-wide-verify/v1"
                and verify["status"] == "RETURNED"
                and verify["exit_code"] == 0
                and verify["verdict"] == verdict
                and summary["verification_status"] == verdict["status"]
                and verify["selected_best_total_runtime"] is None,
                "independent exact external-model replay")
    else:
        require(not (run / "verify-process.json").exists()
                and summary["verification_status"] is None,
                "no model was promoted")
    return {"schema": "n83.local-gray51-wide-replay/v1",
            "status": "PASS_BOUNDED_REPLAY", "solver_status": status,
            "conflicts": conflicts, "stdout_sha256": sha(output),
            "curve": protocol["curve_slug"],
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--inputs-dir", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.run_dir, args.inputs_dir,
                           args.worker, args.solver), indent=2, sort_keys=True))
