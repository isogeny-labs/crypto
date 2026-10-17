#!/usr/bin/env python3
"""Independently audit the bounded local N83 CryptoMiniSat ladder and controls."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import re

import blake3
import audit_native_export

SOURCE = "73ee5686a62c9e75cd8d29acd5e73fd3410bb494"
FIXTURE_SHA256 = "76b78ec1a68807d130fc11e54e4220b5cde619d677316e313dc247869f727a9d"
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
WORKER_SHA256 = "d397a0e13cf80fd6e39e9f35fd0b2d8368f80f6d839e3a0bbf27b1aa03a1bfa1"
DIMACS_SHA256 = "c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def tree_hash(root: Path) -> tuple[int, str]:
    files = sorted(path for path in root.rglob("*") if path.is_file())
    body = "".join(f"{path.relative_to(root)}  {sha256(path)}\n" for path in files)
    return len(files), hashlib.sha256(body.encode()).hexdigest()


def audit_call(root: Path, multiplier: int) -> dict:
    receipt = json.loads((root / "result.json").read_text())
    require(receipt["schema"] == "n83.m6-external-sat-local-pilot/v1"
            and receipt["role"] == "gray_pool" and receipt["stage"] == "all_x"
            and receipt["status"] == "UNKNOWN_solver_limit"
            and receipt["solver_disposition"] == "INDETERMINATE"
            and receipt["source_commit"] == SOURCE
            and receipt["source_dirty"] is False
            and receipt["fixture_sha256"] == FIXTURE_SHA256
            and receipt["cms_sha256"] == CMS_SHA256
            and receipt["worker_sha256"] == WORKER_SHA256
            and receipt["selected_best_total_runtime"] is None,
            "local run identity or disposition differs")
    require(receipt["solver_conflict_multiplier"] == multiplier
            and receipt["native_control_conflict_cap"] == 5000
            and receipt["solver_conflict_cap"] == 5000 * multiplier
            and receipt["wall_cap_seconds"] == 600
            and receipt["variables"] == 118684
            and receipt["clauses"] == 548445
            and receipt["xors"] == 6059,
            "solver limits or formula shape differ")
    dimacs = root / "instance.xor.cnf"
    require(sha256(dimacs) == receipt["dimacs_sha256"] == DIMACS_SHA256
            and blake3.blake3(dimacs.read_bytes()).hexdigest()
            == receipt["dimacs_blake3"]
            == audit_native_export.EXPECTED_DIMACS_BLAKE3,
            "frozen XOR-DIMACS bytes differ")
    exported = json.loads((root / "export.stdout").read_text())
    require(exported["schema"] == "n83.m6-xor-dimacs-export/v1"
            and exported["dimacs_blake3"] == receipt["dimacs_blake3"]
            and exported["dimacs_bytes"] == dimacs.stat().st_size
            and exported["variables"] == receipt["variables"]
            and exported["clauses"] == receipt["clauses"]
            and exported["xors"] == receipt["xors"],
            "worker export receipt differs")
    stdout = root / "solver.stdout"
    stderr = root / "solver.stderr"
    require(stdout.read_text() == "s INDETERMINATE\n"
            and sha256(stdout) == receipt["solver_stdout_sha256"]
            and sha256(stderr) == receipt["solver_stderr_sha256"],
            "raw solver outputs differ")
    rss = re.search(r"(?m)^\s*(\d+)\s+maximum resident set size\s*$",
                    stderr.read_text())
    require(rss is not None and int(rss.group(1)) == receipt["peak_rss_bytes_L0"]
            and math.isfinite(receipt["solver_wall_seconds_L0"])
            and 0 < receipt["solver_wall_seconds_L0"] < 600
            and math.isfinite(receipt["full_wall_seconds_L0"])
            and receipt["solver_wall_seconds_L0"] <= receipt["full_wall_seconds_L0"] < 630,
            "solver resource receipt differs")
    command = receipt["solver_command"]
    require(command[0:3] == ["/usr/bin/time", "-l", "/opt/homebrew/bin/cryptominisat5"]
            and command[-1] == str(dimacs)
            and command[command.index("--maxconfl") + 1] == str(5000 * multiplier)
            and command[command.index("--maxtime") + 1] == "600"
            and command[command.index("--threads") + 1] == "1",
            "solver invocation differs")
    count, digest = tree_hash(root)
    return {"conflict_multiplier": multiplier,
            "solver_conflict_cap": 5000 * multiplier,
            "solver_disposition": "INDETERMINATE",
            "solver_wall_seconds_L0": receipt["solver_wall_seconds_L0"],
            "full_wall_seconds_L0": receipt["full_wall_seconds_L0"],
            "peak_rss_bytes_L0": receipt["peak_rss_bytes_L0"],
            "raw_file_count": count, "raw_tree_sha256": digest}


def audit_failure(root: Path, reason: str) -> dict:
    failure = json.loads((root / "failure.json").read_text())
    require(failure["schema"] == "n83.m6-external-sat-local-pilot/v1"
            and failure["status"] == "PRODUCER_FAILURE"
            and failure["role"] == "gray_pool"
            and failure["stage"] == "all_x"
            and failure["reason"] == reason
            and (root / "solver.stdout").read_text() == "s INDETERMINATE\n"
            and sha256(root / "instance.xor.cnf") == DIMACS_SHA256,
            "retained producer failure differs")
    count, digest = tree_hash(root)
    return {"reason": reason, "raw_file_count": count, "raw_tree_sha256": digest}


def audit(runs: list[Path], failures: list[Path], model_bits: Path,
          bases: Path) -> dict:
    require(len(runs) == 3 and len(failures) == 2, "expected frozen call inventory")
    calls = [audit_call(path, multiplier) for path, multiplier in
             zip(runs, (1, 20, 200))]
    failed = [audit_failure(failures[0], "missing or repeated solver disposition"),
              audit_failure(failures[1], "solver exit and disposition differ")]
    native = audit_native_export.audit(runs[0] / "instance.xor.cnf", model_bits, bases)
    require(native["status"] == "PASS_NATIVE_MODEL_SATISFIES_EXPORTED_XOR_DIMACS",
            "independent satisfiability control differs")
    return {"schema": "n83.m6-external-sat-local-audit/v1",
            "status": "PASS_CENSORED_EXTERNAL_CALLS_AND_NATIVE_SAT_EXPORT",
            "source_commit": SOURCE, "role": "gray_pool", "stage": "all_x",
            "dimacs_sha256": DIMACS_SHA256,
            "dimacs_blake3": audit_native_export.EXPECTED_DIMACS_BLAKE3,
            "calls": calls, "retained_producer_failures": failed,
            "native_assignment_audit": native,
            "conservative_six_call_envelope_seconds": 3600,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run", type=Path, action="append", required=True)
    parser.add_argument("--failure", type=Path, action="append", required=True)
    parser.add_argument("--model-bits", type=Path, required=True)
    parser.add_argument("--bases", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.run, args.failure, args.model_bits, args.bases),
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
