#!/usr/bin/env python3
"""Independently audit raw custody and censored N83 external-SAT receipts."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess

import blake3

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
ROOT = HERE.parents[3]
DISPATCH = STUDY / "verification/hosted-m6-highcap-20261018/dispatch.json"
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"
POOLS = {
    "gray_pool": "966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47",
    "hash_pool": "b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e",
}
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def digest(path: Path, algorithm: str) -> str:
    state = hashlib.sha256() if algorithm == "sha256" else blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            state.update(block)
    return state.hexdigest()


def formula_counts(path: Path, variables: int) -> tuple[int, int]:
    cnf = xor = 0
    with path.open() as stream:
        header = stream.readline().split()
        require(header[:3] == ["p", "cnf", str(variables)] and len(header) == 4,
                "formula header differs")
        declared = int(header[3])
        for row in stream:
            is_xor = row.startswith("x")
            words = row[1:].split() if is_xor else row.split()
            require(words and words[-1] == "0", "formula row lacks terminator")
            for word in words[:-1]:
                require(0 < abs(int(word)) <= variables, "formula literal out of range")
            cnf += not is_xor
            xor += is_xor
    require(cnf + xor == declared, "formula row count differs")
    return cnf, xor


def audit(raw: Path, inputs: Path, worker: Path) -> dict:
    result = json.loads((raw / "result.json").read_text())
    export = json.loads((raw / "export.stdout").read_text())
    dispatch = json.loads(DISPATCH.read_text())
    arm = result.get("arm")
    require(arm in POOLS and result.get("schema") == "n83.m6-external-natural-local/v1",
            "result arm or schema differs")
    require(result.get("export") == export and result.get("export_exit_code") == 0,
            "export receipt or exit differs")
    require(export.get("schema") == "n83.mixed-orbit-natural-xor-dimacs-export/v1"
            and export.get("status") == "EXPORTED_ONE_IDENTITY_PATTERN"
            and export.get("base_object_blake3") == BASE
            and export.get("pool_object_blake3") == POOLS[arm]
            and export.get("base_summands") == 4
            and export.get("residual_summands") == 2
            and export.get("identity_mask") == 0
            and export.get("ordering") == "equal_domain_runs"
            and export.get("a") == "1833078656608906180176350"
            and export.get("b") == "163089659398503559150141"
            and export.get("target") == ["3547082699798970859000627",
                                         "6877207017147849837699235"]
            and export.get("public_corpus_canonical_json_blake3")
            == "831eaa7be2387634fd610f6706103b2af9b4e274f0db59923c0280af3cbed74b",
            "frozen natural-query export identity differs")
    hosted = json.loads((STUDY / "verification/hosted-m6-highcap-20261018/results/39100306846"
                         / f"{arm}-result.json").read_text())
    require((export.get("variables"), export.get("clauses"), export.get("xors"))
            == (hosted["search"]["peak_variables"],
                hosted["search"]["peak_clauses"], hosted["search"]["peak_xors"]),
            "hosted native pre-solve circuit shape differs")
    require(result.get("source_dirty") is False
            and result.get("solver_conflict_cap") == 5_000_000
            and result.get("solver_wall_cap_seconds") == 600
            and result.get("conservative_arm_envelope_seconds") == 960
            and result.get("selected_best_total_runtime") is None,
            "source cleanliness, cap, charge or selection differs")
    require(result.get("solver_binary_sha256") == CMS_SHA256
            and result.get("worker_binary_sha256") == digest(worker, "sha256")
            and export.get("source_blake3") == digest(
                ROOT / "examples/koblitz_n83_mixed_orbit_worker.rs", "blake3"),
            "source or solver bytes differ")
    require(isinstance(result.get("source_commit"), str)
            and re.fullmatch(r"[0-9a-f]{40}", result["source_commit"]) is not None,
            "source commit identifier differs")
    require(subprocess.check_output(["git", "cat-file", "-t", result["source_commit"]],
                                    cwd=ROOT, text=True).strip() == "commit",
            "source commit unavailable")
    committed_source = subprocess.check_output(
        ["git", "show", f"{result['source_commit']}:examples/koblitz_n83_mixed_orbit_worker.rs"],
        cwd=ROOT)
    require(blake3.blake3(committed_source).hexdigest() == export["source_blake3"],
            "source commit does not bind exporter bytes")
    for name in (BASE, POOLS[arm]):
        path = inputs / f"{name}.jsonl.gz"
        require(digest(path, "sha256") == dispatch["source_archive_sha256"][name]
                and digest(path, "blake3") == name,
                "archived S3 object differs")
    require(digest(STUDY / "pilot-01/manifest.json", "sha256")
            == dispatch["manifest_sha256"]
            and digest(STUDY / "pilot-01/probe-corpus.json", "sha256")
            == dispatch["corpus_sha256"], "manifest or corpus differs")
    raw_hashes = result.get("raw_sha256")
    require(isinstance(raw_hashes, dict)
            and set(raw_hashes) == {"natural-mask0.xor.cnf", "export.stdout",
                                   "export.stderr", "solver.stdout", "solver.stderr"},
            "raw inventory differs")
    for name, value in raw_hashes.items():
        require(digest(raw / name, "sha256") == value, f"raw {name} digest differs")
    formula = raw / "natural-mask0.xor.cnf"
    require(formula.stat().st_size == export.get("dimacs_bytes")
            and digest(formula, "blake3") == export.get("dimacs_blake3"),
            "formula content differs")
    cnf, xor = formula_counts(formula, export["variables"])
    require((cnf, xor) == (export["clauses"], export["xors"]),
            "formula counts differ")
    statuses = re.findall(r"(?m)^s (\S+)\s*$", (raw / "solver.stdout").read_text())
    disposition = result.get("solver_disposition")
    exit_codes = {"SATISFIABLE": 10, "UNSATISFIABLE": 20,
                  "UNKNOWN": 0, "INDETERMINATE": 15}
    if disposition == "UNKNOWN_outer_wall_cap":
        require(result.get("solver_exit_code") == -9
                and result.get("status") == disposition and not result.get("checks"),
                "outer wall cap receipt differs")
    else:
        require(len(statuses) == 1 and statuses[0] == disposition
                and result.get("solver_exit_code") == exit_codes.get(disposition),
                "raw solver disposition or exit differs")
    rss = re.search(r"(?m)^\s*(\d+)\s+maximum resident set size\s*$",
                    (raw / "solver.stderr").read_text())
    require((rss is not None and result.get("peak_rss_bytes_L0") == int(rss.group(1)))
            or (disposition == "UNKNOWN_outer_wall_cap"
                and result.get("peak_rss_bytes_L0") is None),
            "raw memory receipt differs")
    require(all(type(result.get(key)) in (int, float)
                and math.isfinite(result[key]) and 0 <= result[key] <= 960
                for key in ("solver_wall_seconds_L0", "outer_wall_seconds_L0")),
            "wall receipt differs")
    if disposition in {"UNKNOWN", "INDETERMINATE", "UNKNOWN_outer_wall_cap"}:
        require(result.get("status") == ("UNKNOWN_outer_wall_cap" if disposition
                 == "UNKNOWN_outer_wall_cap" else "UNKNOWN_solver_limit")
                and result.get("checks") == {}, "censored status differs")
        status = "PASS_CUSTODY_CENSORED_ONE_MASK"
    elif disposition == "UNSATISFIABLE":
        require(result.get("status") == "UNSAT_one_identity_pattern_only"
                and result.get("checks") == {}, "one-mask UNSAT scope differs")
        status = "PASS_CUSTODY_UNSAT_REPORTED_ONE_MASK"
    else:
        status = "MODEL_REPLAY_REQUIRED"
    return {"schema": "n83.m6-external-natural-audit/v1", "status": status,
            "arm": arm, "source_commit": result["source_commit"],
            "formula_sha256": raw_hashes["natural-mask0.xor.cnf"],
            "raw_files_checked": len(raw_hashes), "cnf_clauses_checked": cnf,
            "xor_rows_checked": xor, "conservative_arm_envelope_seconds": 960,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw", type=Path, required=True)
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    row = audit(args.raw, args.inputs, args.worker)
    with args.output.open("x") as stream:
        json.dump(row, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(row, sort_keys=True))


if __name__ == "__main__":
    main()
