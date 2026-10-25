#!/usr/bin/env python3
"""Replay raw primary-N83 wide-XOR solver and model-verification receipts."""

import argparse
import gzip
import hashlib
import json
from pathlib import Path
import re


HERE = Path(__file__).resolve().parent
FROZEN_PROTOCOL_SHA256 = "364f780a5fca7f3b4915f5ca83678e4a128fdd7e37a84d9553396ba2eb0de9b2"
FROZEN_SMOKE_RECEIPT_SHA256 = "6e7dbf42ca12a0c2c395f42da02826feabdd889aa0aba67be55e92d7e5faa635"


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def audit(run_dir, formulas_dir, archives_dir, worker, solver):
    require(sha256(HERE / "protocol.json") == FROZEN_PROTOCOL_SHA256,
            "frozen protocol changed")
    require(sha256(HERE / "smoke/receipt.json") == FROZEN_SMOKE_RECEIPT_SHA256,
            "smoke receipt changed")
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    smoke = json.loads((HERE / "smoke/receipt.json").read_bytes())
    smoke_stdout = gzip.decompress((HERE / "smoke/stdout.gz").read_bytes())
    smoke_stderr = gzip.decompress((HERE / "smoke/stderr.gz").read_bytes())
    require(hashlib.sha256(smoke_stdout).hexdigest() == smoke["stdout_sha256"]
            and hashlib.sha256(smoke_stderr).hexdigest() == smoke["stderr_sha256"]
            and smoke_stderr == b"" and smoke["exit_code"] == 15
            and b"332 x20916" in smoke_stdout
            and b"Using 10 matrices recovered from 3486 xors" in smoke_stdout,
            "exploratory matrix admission replay")
    require(sha256(worker) == protocol["worker_binary_sha256"]
            and sha256(solver) == protocol["solver_binary_sha256"],
            "binary digest")
    formula = formulas_dir / "hash_4_2/instance.xcnf"
    require(sha256(formula) == protocol["xcnf_sha256"], "XCNF digest")
    for role in ("base", "pool"):
        archive = archives_dir / f"s3-{protocol[role + '_object_blake3']}.jsonl.gz"
        require(sha256(archive) == protocol[role + "_object_sha256"],
                f"{role} archive digest")
    process = json.loads((run_dir / "process.json").read_bytes())
    summary = json.loads((run_dir / "summary.json").read_bytes())
    stdout = (run_dir / "solver.stdout").read_bytes()
    stderr = (run_dir / "solver.stderr").read_bytes()
    require(process["schema"] == "n83.local-xor-natural-access-process/v1"
            and process["command"] == [str(solver), *protocol["solver_flags"], str(formula)]
            and process["xcnf_sha256"] == protocol["xcnf_sha256"]
            and process["solver_binary_sha256"] == protocol["solver_binary_sha256"],
            "process identity")
    require(hashlib.sha256(stdout).hexdigest() == process["stdout_sha256"]
            and hashlib.sha256(stderr).hexdigest() == process["stderr_sha256"]
            and len(stdout) == process["stdout_bytes"]
            and len(stderr) == process["stderr_bytes"],
            "raw solver stream integrity")
    require(b"332 x20916" in stdout
            and b"Using 10 matrices recovered from 3486 xors" in stdout,
            "wide XOR matrices not admitted")
    statuses = re.findall(rb"^s ([A-Z]+)\s*$", stdout, re.MULTILINE)
    counters = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
    conflicts = int(counters[0]) if len(counters) == 1 else None
    status = process["status"]
    if status == "WATCHDOG":
        require(process["exit_code"] < 0 and not statuses,
                "watchdog classification")
    else:
        require(len(statuses) == 1 and statuses[0].decode() == status
                and conflicts == process["conflicts"]
                and (status, process["exit_code"]) in {
                    ("INDETERMINATE", 15), ("SATISFIABLE", 10),
                    ("UNSATISFIABLE", 20)}, "solver status/counter")
    require(summary["schema"] == "n83.local-xor-natural-access-summary/v1"
            and summary["status"] == status
            and summary["conflicts"] == process["conflicts"]
            and summary["solver_wall_seconds_L0"] == process["wall_seconds_L0"]
            and summary["selected_best_total_runtime"] is None
            and process["selected_best_total_runtime"] is None,
            "summary accounting")
    require(0 < process["wall_seconds_L0"] <= protocol["process_watchdog_seconds"] + 1,
            "process wall bound")
    verdict = None
    if status == "SATISFIABLE":
        verify = json.loads((run_dir / "verify-process.json").read_bytes())
        out = (run_dir / "verify.stdout").read_bytes()
        err = (run_dir / "verify.stderr").read_bytes()
        require(hashlib.sha256(out).hexdigest() == verify["stdout_sha256"]
                and hashlib.sha256(err).hexdigest() == verify["stderr_sha256"],
                "verifier stream integrity")
        if verify["exit_code"] == 0:
            verdict = json.loads((run_dir / "verify/result.json").read_bytes())
            require(verdict == verify["verdict"]
                    and verdict["status"] in ("PASS_ONE_EXTERNAL_PARTIAL",
                                              "GROUP_REJECTED_EXTERNAL_MODEL")
                    and verdict["selected_best_total_runtime"] is None,
                    "external model verdict")
        require(summary["verification_status"] ==
                (verdict["status"] if verdict else None), "summary verifier verdict")
    else:
        require(not (run_dir / "verify-process.json").exists()
                and summary["verification_status"] is None,
                "unexpected verifier on censored arm")
    return {
        "schema": "n83.local-xor-natural-access-replay/v1",
        "status": "PASS_BOUNDED_REPLAY",
        "protocol_sha256": FROZEN_PROTOCOL_SHA256,
        "solver_status": status,
        "conflicts": process["conflicts"],
        "verification_status": verdict["status"] if verdict else None,
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--formulas-dir", type=Path, required=True)
    parser.add_argument("--archives-dir", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.run_dir, args.formulas_dir, args.archives_dir,
                           args.worker, args.solver), indent=2, sort_keys=True))
