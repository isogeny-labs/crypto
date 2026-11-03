#!/usr/bin/env python3
"""Independently replay the stopped N83 heuristic and matrix smoke receipts."""

import argparse
import gzip
import hashlib
import json
import pathlib
import re


HERE = pathlib.Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence-01"
FROZEN_PROTOCOL_SHA256 = "4e55e687bab219877c6a284332b35478ee41329f93b6f7b5ab99013729719646"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def raw_output(directory: pathlib.Path, receipt: dict) -> tuple[bytes, bytes]:
    stdout = gzip.decompress((directory / "solver.stdout.gz").read_bytes())
    stderr = gzip.decompress((directory / "solver.stderr.gz").read_bytes())
    require(sha256_bytes(stdout) == receipt["stdout_sha256"], f"{directory}: stdout hash")
    require(sha256_bytes(stderr) == receipt["stderr_sha256"], f"{directory}: stderr hash")
    if "stdout_bytes" in receipt:
        require(len(stdout) == receipt["stdout_bytes"]
                and len(stderr) == receipt["stderr_bytes"], f"{directory}: stream lengths")
    return stdout, stderr


def audit(formulas_dir: pathlib.Path, archives_dir: pathlib.Path,
          worker: pathlib.Path, solver: pathlib.Path) -> dict:
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    require(sha256_bytes(protocol_bytes) == FROZEN_PROTOCOL_SHA256, "frozen protocol changed")
    protocol = json.loads(protocol_bytes)
    require(sha256_file(worker) == protocol["worker_binary_sha256"], "worker binary hash")
    require(sha256_file(solver) == protocol["solver_binary_sha256"], "solver binary hash")
    base_hash = protocol["base_object_blake3"]
    require(sha256_file(archives_dir / f"s3-{base_hash}.jsonl.gz")
            == protocol["base_object_sha256"], "base archive hash")
    for name, spec in protocol["inputs"].items():
        require(sha256_file(formulas_dir / name / "instance.xcnf") == spec["xcnf_sha256"],
                f"{name}: formula hash")
        require(sha256_file(archives_dir / f"s3-{spec['pool_object_blake3']}.jsonl.gz")
                == spec["pool_object_sha256"], f"{name}: pool archive hash")

    summary = json.loads((EVIDENCE / "summary.json").read_bytes())
    require(summary["schema"] == "n83.external-xor-heuristic-panel/v1"
            and summary["status"] == "STOPPED" and summary["stop_reason"] == "WATCHDOG"
            and summary["completed_arms"] == ["hash_A1", "gray_A1"], "stopped panel summary")
    expected = [("hash_A1", "INDETERMINATE", 15, 1000001),
                ("gray_A1", "WATCHDOG", -9, None)]
    panel_rows = []
    for arm_id, status, exit_code, conflicts in expected:
        directory = EVIDENCE / arm_id
        receipt = json.loads((directory / "process.json").read_bytes())
        arm = next(row for row in protocol["arms"] if row["id"] == arm_id)
        require(receipt["schema"] == "n83.external-xor-heuristic-arm/v1"
                and receipt["arm"] == arm and receipt["status"] == status
                and receipt["exit_code"] == exit_code and receipt["conflicts"] == conflicts,
                f"{arm_id}: process classification")
        require(receipt["xcnf_sha256"] == protocol["inputs"][arm["input"]]["xcnf_sha256"]
                and receipt["solver_binary_sha256"] == protocol["solver_binary_sha256"],
                f"{arm_id}: input identity")
        stdout, stderr = raw_output(directory, receipt)
        require(not stderr, f"{arm_id}: stderr present")
        statuses = re.findall(rb"^s ([A-Z]+)\s*$", stdout, re.MULTILINE)
        counters = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
        if status == "INDETERMINATE":
            require(statuses == [b"INDETERMINATE"] and counters == [b"1000001"],
                    f"{arm_id}: final solver status/counter")
        else:
            require(not statuses and not counters and b"950K" in stdout,
                    f"{arm_id}: watchdog output shape")
        require(0 < receipt["wall_seconds_L0"] <= protocol["process_watchdog_seconds"] + 1,
                f"{arm_id}: process wall")
        panel_rows.append({"arm": arm_id, "status": status,
                           "wall_seconds_L0": receipt["wall_seconds_L0"],
                           "conflicts": conflicts})
    require(abs(sum(row["wall_seconds_L0"] for row in panel_rows)
                - summary["summed_solver_wall_seconds_L0"]) < 1e-8,
            "panel wall sum")

    smoke = json.loads((EVIDENCE / "matrix-smoke/summary.json").read_bytes())
    require(smoke["schema"] == "n83.external-xor-matrix-admission-exploratory/v1"
            and len(smoke["arms"]) == 6, "matrix smoke summary")
    smoke_rows = []
    for row in smoke["arms"]:
        arm_id = row["evidence_id"]
        directory = EVIDENCE / "matrix-smoke"
        receipt = json.loads((directory / f"{arm_id}.json").read_bytes())
        stdout = gzip.decompress((directory / f"{arm_id}.stdout.gz").read_bytes())
        stderr = gzip.decompress((directory / f"{arm_id}.stderr.gz").read_bytes())
        require(sha256_bytes(stdout) == receipt["stdout_sha256"] == row["stdout_sha256"]
                and sha256_bytes(stderr) == receipt["stderr_sha256"] and not stderr,
                f"{arm_id}: raw output hashes")
        require(receipt["exit_code"] == row["exit_code"] == 15
                and re.findall(rb"^s ([A-Z]+)\s*$", stdout, re.MULTILINE) == [b"INDETERMINATE"],
                f"{arm_id}: capped solver status")
        counters = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
        require(len(counters) == 1 and 20000 <= int(counters[0]) <= 20100,
                f"{arm_id}: conflict cap")
        spec = protocol["inputs"][receipt["name"]]
        require(receipt["xcnf_sha256"] == row["formula_sha256"] == spec["xcnf_sha256"],
                f"{arm_id}: formula hash")
        if row["matrix_admission"] == "wide":
            require(b"332 x20916" in stdout and b"Using 10 matrices" in stdout,
                    f"{arm_id}: large XOR matrices not admitted")
        else:
            require(b"Too many columns in matrix: 20916" in stdout
                    and b"Using 5 matrices" in stdout,
                    f"{arm_id}: default matrix gate changed")
        smoke_rows.append({"arm": arm_id, "status": "INDETERMINATE",
                           "wall_seconds_L0": receipt["wall_seconds_L0"],
                           "conflicts": int(counters[0])})
    return {"schema": "n83.external-xor-stopped-independent-replay/v1", "status": "PASS",
            "protocol_sha256": FROZEN_PROTOCOL_SHA256, "panel": panel_rows,
            "matrix_smoke": smoke_rows, "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--formulas-dir", type=pathlib.Path, required=True)
    parser.add_argument("--archives-dir", type=pathlib.Path, required=True)
    parser.add_argument("--worker", type=pathlib.Path, required=True)
    parser.add_argument("--solver", type=pathlib.Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.formulas_dir, args.archives_dir, args.worker, args.solver),
                     indent=2, sort_keys=True))
