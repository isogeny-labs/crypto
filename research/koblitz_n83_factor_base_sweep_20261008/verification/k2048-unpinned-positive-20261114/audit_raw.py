#!/usr/bin/env python3
"""Independently audit retained K2048 unpinned control input and terminal evidence."""

import gzip
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence"
LOCK = json.loads((HERE / "LOCK.json").read_text())


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path, compressed=False):
    digest = hashlib.sha256()
    opener = gzip.open if compressed else open
    with opener(path, "rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def main():
    export = EVIDENCE / "export"
    solver = EVIDENCE / "solver"
    manifest = json.loads((export / ".artifacts.json").read_text())
    entry = manifest["files"]["result.xcnf.gz"]
    require(manifest["schema"] == "artifacts.manifest/v1" and
            entry["sha256"] == sha(export / "result.xcnf.gz") and
            entry["bytes"] == (export / "result.xcnf.gz").stat().st_size,
            "artifact-store pointer or hydrated bytes")
    require(sha(export / "result.json") == LOCK["export_receipt_sha256"], "export copy")
    require(sha(export / "known-model-audit.json") == LOCK["known_model_audit_sha256"],
            "independent model audit copy")
    require(sha(export / "result.xcnf.gz", compressed=True) == LOCK["formula_sha256"],
            "formula archive")
    require(sha(solver / "stdout.txt.gz", compressed=True) ==
            json.loads((solver / "receipt.json").read_text())["stdout_sha256"],
            "solver stdout archive")
    require(sha(solver / "stderr.txt") ==
            json.loads((solver / "receipt.json").read_text())["stderr_sha256"],
            "solver stderr copy")
    export_receipt = json.loads((export / "result.json").read_text())
    known_model = json.loads((export / "known-model-audit.json").read_text())
    run = json.loads((solver / "receipt.json").read_text())
    samples = json.loads((solver / "host-preflight.json").read_text())
    command = json.loads((solver / "command.json").read_text())
    require(export_receipt["mode"] == "positive-unpinned" and
            export_receipt["target"] == LOCK["target"] and
            export_receipt["shape"]["positive_x_pins"] == 0, "export identity")
    require(known_model["status"] == "PASS_ORIGINAL_XCNF" and
            (known_model["checked_cnf"], known_model["checked_xor"]) ==
            (1_140_557, 4_980), "known-model replay receipt")
    require(command == [LOCK["solver"], *LOCK["solver_flags"], LOCK["formula"]],
            "solver command")
    require(len(samples) == 3 and samples == run["host_preflight"] and
            all(item["load_1m"] <= 24 for item in samples) and
            len({item["swapouts"] for item in samples}) == 1 and
            run["swapouts_after"] == samples[0]["swapouts"], "host gate")
    with gzip.open(solver / "stdout.txt.gz", "rt", errors="replace") as stream:
        transcript = stream.read()
    statuses = [line for line in transcript.splitlines() if line.startswith("s ")]
    conflicts = [line for line in transcript.splitlines()
                 if line.startswith("c conflicts                :")]
    require(statuses == ["s INDETERMINATE"] and len(conflicts) == 1 and
            int(conflicts[0].split(":", 1)[1]) == 1_000_001 and
            not any(line.startswith("v ") for line in transcript.splitlines()),
            "terminal status or model")
    require(run["status"] == "INDETERMINATE" and run["exit_code"] == 15 and
            run["terminal_conflicts"] == 1_000_001 and not run["timed_out"] and
            run["formula_sha256"] == LOCK["formula_sha256"] and
            run["solver_sha256"] == LOCK["solver_sha256"] and
            run["lock_sha256"] == sha(HERE / "LOCK.json") and
            run["verified_relation"] is None and run["novel_rank"] is None,
            "terminal receipt")
    require((export / "stdout.txt").stat().st_size == 0 and
            (export / "stderr.txt").read_text().strip() ==
            'Error: "compiled chained-S3 source differs from frozen source"' and
            (export / "stderr-2.txt").stat().st_size == 0,
            "retained export attempts")
    receipt = {
        "schema": "n83.k2048-unpinned-raw-audit/v1",
        "status": "PASS_CUSTODY_AND_TERMINAL_CLASSIFICATION",
        "formula_sha256": LOCK["formula_sha256"],
        "solver_stdout_sha256": run["stdout_sha256"],
        "terminal": run["status"],
        "terminal_conflicts": run["terminal_conflicts"],
        "known_model_status": known_model["status"],
    }
    output = EVIDENCE / "audit.json"
    if output.exists():
        require(json.loads(output.read_text()) == receipt, "audit idempotence")
    else:
        output.write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
