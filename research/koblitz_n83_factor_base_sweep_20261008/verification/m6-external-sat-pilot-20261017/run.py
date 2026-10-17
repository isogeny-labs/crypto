#!/usr/bin/env python3
"""Export one frozen N83 chain instance and audit a bounded CryptoMiniSat call."""

from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

import blake3

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
ROOT = HERE.parents[3]
FIXTURE_DIR = STUDY / "verification/hosted-m6-witness-ladder-20261017"
PRIOR_DIR = STUDY / "verification/k600-gray-hash-paired-protocol-20261014"
sys.path[:0] = [str(FIXTURE_DIR), str(PRIOR_DIR), str(STUDY)]
import prepare  # noqa: E402
import receipt_audit as prior  # noqa: E402
from chain_search_supervisor import field_mul, point_add  # noqa: E402

CMS = Path("/opt/homebrew/bin/cryptominisat5")
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
FIXTURE_SHA256 = "76b78ec1a68807d130fc11e54e4220b5cde619d677316e313dc247869f727a9d"
CAPS = {"all_x": (600, 5000, 10), "summand_x": (900, 10000, 6),
        "open": (1800, 20000, 0)}


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def save(path: Path, row: dict) -> None:
    with path.open("x") as stream:
        json.dump(row, stream, indent=2, sort_keys=True)
        stream.write("\n")


def model_from_stdout(stdout: str, variables: int) -> tuple[str, list[bool] | None]:
    statuses = [line.removeprefix("s ").strip() for line in stdout.splitlines()
                if line.startswith("s ")]
    require(len(statuses) == 1 and statuses[0] in
            {"SATISFIABLE", "UNSATISFIABLE", "UNKNOWN", "INDETERMINATE"},
            "missing or repeated solver disposition")
    if statuses[0] != "SATISFIABLE":
        return statuses[0], None
    model: list[bool | None] = [None] * variables
    for line in stdout.splitlines():
        if not line.startswith("v "):
            continue
        for token in line[2:].split():
            literal = int(token)
            if literal == 0:
                continue
            index = abs(literal) - 1
            require(0 <= index < variables, "model variable outside header")
            previous = model[index]
            require(previous is None or previous is (literal > 0),
                    "contradictory model literal")
            model[index] = literal > 0
    require(all(value is not None for value in model),
            "SAT output omits a declared variable")
    return statuses[0], [bool(value) for value in model]


def check_dimacs(path: Path, model: list[bool]) -> tuple[int, int]:
    clauses = xors = 0
    declared = None
    with path.open() as stream:
        for line_number, line in enumerate(stream, 1):
            line = line.strip()
            if not line or line.startswith("c"):
                continue
            if line.startswith("p "):
                parts = line.split()
                require(declared is None and len(parts) == 4 and parts[1] == "cnf"
                        and int(parts[2]) == len(model), "DIMACS header differs")
                declared = int(parts[3])
                continue
            require(declared is not None, "constraint precedes DIMACS header")
            xor = line.startswith("x")
            literals = [int(token) for token in (line[1:] if xor else line).split()]
            require(literals and literals[-1] == 0, "unterminated constraint")
            value = False
            for literal in literals[:-1]:
                index = abs(literal) - 1
                require(0 <= index < len(model), "constraint variable outside header")
                atom = model[index] ^ (literal < 0)
                value = (value ^ atom) if xor else (value or atom)
            require(value, f"model violates {'XOR' if xor else 'CNF'} line {line_number}")
            xors += int(xor)
            clauses += int(not xor)
    require(declared == clauses + xors, "DIMACS constraint count differs")
    return clauses, xors


def decoded_blocks(model: list[bool]) -> list[int]:
    require(len(model) >= 11 * 83, "model lacks eleven x blocks")
    return [sum(int(model[block * 83 + bit]) << bit for bit in range(83))
            for block in range(11)]


def s3_residual(x: int, y: int, z: int) -> int:
    xy = field_mul(x, y)
    pair = xy ^ field_mul(x, z) ^ field_mul(y, z)
    return field_mul(xy, z) ^ field_mul(pair, pair) ^ 1


def lift(blocks: list[int], arm: dict, base: Path, pool: Path) -> int:
    base_points, residual_points = prior.domains(
        base, pool, arm["shared_signed_orbits"])
    choices = []
    for index, x in enumerate(blocks[:6]):
        domain = base_points if index < 4 else residual_points
        matches = [point for point in domain if point[0] == x]
        require(0 < len(matches) <= 2, "SAT x lacks exact full-point domain lift")
        choices.append(matches)
    target = tuple(map(int, arm["target"]))
    lifts = 0
    for points in itertools.product(*choices):
        total = None
        for point in points:
            total = point_add(total, point)
        lifts += int(total == target)
    return lifts


def run(worker: Path, bases: Path, role: str, stage: str,
        conflict_multiplier: int, output: Path) -> dict:
    started = time.monotonic()
    output.mkdir(parents=True, exist_ok=False)
    fixture_path = FIXTURE_DIR / "fixture.json"
    require(sha256(fixture_path) == FIXTURE_SHA256
            and sha256(CMS) == CMS_SHA256 and worker.is_file(),
            "frozen fixture, solver, or worker differs")
    fixture = json.loads(fixture_path.read_text())
    require(prepare.build(bases) == fixture, "exact base or group witness differs")
    arm = next((row for row in fixture["arms"] if row["role"] == role), None)
    require(arm is not None and stage in CAPS and conflict_multiplier in (1, 20, 200),
            "role, stage or conflict multiplier differs")
    wall_cap, native_conflicts, fixed = CAPS[stage]
    solver_conflicts = native_conflicts * conflict_multiplier
    source_commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                            text=True).strip()
    source_dirty = bool(subprocess.check_output(["git", "status", "--porcelain"],
                                                cwd=ROOT, text=True))
    dimacs = output / "instance.xor.cnf"
    export = subprocess.run([str(worker), role, stage, str(native_conflicts),
                             "--export-xor-dimacs", str(dimacs)],
                            capture_output=True, text=True, timeout=300, check=False)
    (output / "export.stdout").write_text(export.stdout)
    (output / "export.stderr").write_text(export.stderr)
    require(export.returncode == 0, "export worker failed")
    receipt = json.loads(export.stdout)
    require(receipt["schema"] == "n83.m6-xor-dimacs-export/v1"
            and receipt["status"] == "EXPORTED_XOR_DIMACS"
            and receipt["role"] == role and receipt["stage"] == stage
            and receipt["fixed_x_blocks"] == fixed
            and receipt["fixture_blake3"] == blake3.blake3(fixture_path.read_bytes()).hexdigest()
            and receipt["dimacs_bytes"] == dimacs.stat().st_size
            and receipt["dimacs_blake3"] == blake3.blake3(dimacs.read_bytes()).hexdigest(),
            "export receipt or DIMACS bytes differ")
    command = ["/usr/bin/time", "-l", str(CMS), "--threads", "1", "--verb", "0",
               "--printsol", "1", "--maxtime", str(wall_cap),
               "--maxconfl", str(solver_conflicts), str(dimacs)]
    solver_started = time.monotonic()
    solver = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                              text=True, start_new_session=True)
    try:
        stdout, stderr = solver.communicate(timeout=wall_cap + 30)
        elapsed = time.monotonic() - solver_started
        (output / "solver.stdout").write_text(stdout)
        (output / "solver.stderr").write_text(stderr)
        disposition, model = model_from_stdout(stdout, receipt["variables"])
        require(solver.returncode == {"SATISFIABLE": 10, "UNSATISFIABLE": 20,
                                      "UNKNOWN": 0, "INDETERMINATE": 0}[disposition],
                "solver exit and disposition differ")
    except subprocess.TimeoutExpired:
        try:
            os.killpg(solver.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        stdout, stderr = solver.communicate()
        elapsed = time.monotonic() - solver_started
        (output / "solver.stdout").write_text(stdout)
        (output / "solver.stderr").write_text(stderr)
        disposition, model = "UNKNOWN_outer_wall_cap", None
    checks = {}
    status = ("UNKNOWN_solver_limit" if disposition in ("UNKNOWN", "INDETERMINATE")
              else disposition)
    time_output = (output / "solver.stderr").read_text(errors="replace")
    rss_match = re.search(r"(?m)^\s*(\d+)\s+maximum resident set size\s*$", time_output)
    peak_rss = int(rss_match.group(1)) if rss_match else None
    require(disposition == "UNKNOWN_outer_wall_cap" or peak_rss is not None,
            "solver process memory receipt missing")
    if model is not None:
        clauses, xors = check_dimacs(dimacs, model)
        require(clauses == receipt["clauses"] and xors == receipt["xors"],
                "exported constraint counts differ")
        blocks = decoded_blocks(model)
        require(blocks[10] == int(arm["target_x"])
                and (stage == "open" or blocks[:6] == list(map(int, arm["ordered_xs"])))
                and (stage != "all_x" or blocks[6:10] == list(map(int, arm["partial_xs"]))),
                "SAT model violates target or fixed x blocks")
        nodes = ((blocks[0], blocks[1], blocks[6]),
                 (blocks[6], blocks[2], blocks[7]),
                 (blocks[7], blocks[3], blocks[8]),
                 (blocks[8], blocks[4], blocks[9]),
                 (blocks[9], blocks[5], blocks[10]))
        require(all(s3_residual(*node) == 0 for node in nodes),
                "SAT model violates original-field S3")
        lifts = lift(blocks, arm,
                     bases / f"{arm['base_object_blake3']}.jsonl.gz",
                     bases / f"{arm['pool_object_blake3']}.jsonl.gz")
        checks = {"cnf_clauses_checked": clauses, "xor_rows_checked": xors,
                  "original_s3_nodes_checked": 5, "decoded_x_blocks": list(map(str, blocks)),
                  "group_lifts_to_exact_target": lifts}
        status = "SAT_group_lifted" if lifts else "SAT_field_model_only"
    elif disposition == "UNSATISFIABLE":
        status = "PRODUCER_FAILURE_unsat_planted_encoding"
    result = {
        "schema": "n83.m6-external-sat-local-pilot/v1", "role": role, "stage": stage,
        "status": status, "source_commit": source_commit, "source_dirty": source_dirty,
        "fixture_sha256": FIXTURE_SHA256, "worker_sha256": sha256(worker),
        "cms_sha256": CMS_SHA256, "dimacs_sha256": sha256(dimacs),
        "dimacs_blake3": receipt["dimacs_blake3"],
        "variables": receipt["variables"], "clauses": receipt["clauses"],
        "xors": receipt["xors"], "solver_command": command,
        "solver_stdout_sha256": sha256(output / "solver.stdout"),
        "solver_stderr_sha256": sha256(output / "solver.stderr"),
        "solver_disposition": disposition, "solver_wall_seconds_L0": elapsed,
        "peak_rss_bytes_L0": peak_rss,
        "full_wall_seconds_L0": time.monotonic() - started,
        "wall_cap_seconds": wall_cap, "native_control_conflict_cap": native_conflicts,
        "solver_conflict_multiplier": conflict_multiplier,
        "solver_conflict_cap": solver_conflicts,
        "checks": checks, "natural_relations": None,
        "total_index_calculus_runtime_ms": None,
        "selected_best_total_runtime": None,
    }
    require(math.isfinite(result["full_wall_seconds_L0"]), "nonfinite wall")
    save(output / "result.json", result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--bases", type=Path, required=True)
    parser.add_argument("--role", choices=("gray_pool", "hash_pool"), required=True)
    parser.add_argument("--stage", choices=tuple(CAPS), required=True)
    parser.add_argument("--solver-conflict-multiplier", type=int, choices=(1, 20, 200),
                        default=1)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    preexisting_output = args.out.exists()
    try:
        result = run(args.worker, args.bases, args.role, args.stage,
                     args.solver_conflict_multiplier, args.out)
    except Exception as error:
        if not preexisting_output and args.out.is_dir():
            save(args.out / "failure.json", {
                "schema": "n83.m6-external-sat-local-pilot/v1",
                "status": "PRODUCER_FAILURE",
                "role": args.role, "stage": args.stage,
                "solver_conflict_multiplier": args.solver_conflict_multiplier,
                "error_type": type(error).__name__, "reason": str(error),
                "selected_best_total_runtime": None,
            })
        raise
    print(json.dumps(result, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
