#!/usr/bin/env python3
"""Replay two frozen a=0 K600 natural-query XOR-SAT instances locally."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import itertools
import json
import os
from pathlib import Path
import platform
import re
import signal
import subprocess
import sys
import time

import blake3

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
ROOT = HERE.parents[3]
PRIOR = STUDY / "verification/m6-external-sat-pilot-20261017/run.py"
spec = importlib.util.spec_from_file_location("n83_prior_external_sat", PRIOR)
assert spec is not None and spec.loader is not None
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)
from chain_search_supervisor import PRIMARY_GENERATOR, point_mul  # noqa: E402

CMS = Path("/opt/homebrew/bin/cryptominisat5")
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
DISPATCH = STUDY / "verification/hosted-m6-highcap-20261018/dispatch.json"
MANIFEST = STUDY / "pilot-01/manifest.json"
CORPUS = STUDY / "pilot-01/probe-corpus.json"
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"
POOLS = {
    "gray_pool": "966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47",
    "hash_pool": "b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e",
}
MAX_CONFLICTS = 5_000_000
MAX_WALL_SECONDS = 600


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def blake3_file(path: Path) -> str:
    digest = blake3.blake3()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def save(path: Path, row: dict) -> None:
    with path.open("x") as stream:
        json.dump(row, stream, indent=2, sort_keys=True)
        stream.write("\n")


def exact_input(path: Path, digest: str) -> None:
    require(path.is_file() and sha256(path) == digest, f"source bytes differ: {path.name}")


def check_formula_shape(path: Path, variables: int, clauses: int, xors: int) -> None:
    cnf_seen = xor_seen = 0
    with path.open() as stream:
        header = stream.readline().split()
        require(header == ["p", "cnf", str(variables), str(clauses + xors)],
                "XOR-DIMACS header differs")
        for line in stream:
            if line.startswith("c "):
                continue
            xor = line.startswith("x")
            literals = line[1:].split() if xor else line.split()
            require(literals and literals[-1] == "0", "unterminated formula row")
            require(all(0 < abs(int(word)) <= variables for word in literals[:-1]),
                    "formula literal outside variable range")
            xor_seen += xor
            cnf_seen += not xor
    require((cnf_seen, xor_seen) == (clauses, xors),
            "XOR-DIMACS row counts differ")


def group_lifts(blocks: list[int], target: tuple[int, int],
                base: Path, pool: Path, shared: int) -> int:
    base_points, residual = prior.prior.domains(base, pool, shared)
    choices = []
    for position, x in enumerate(blocks[:6]):
        allowed = base_points if position < 4 else residual
        matches = [point for point in allowed if point[0] == x]
        require(0 < len(matches) <= 2, "SAT x has no exact full-point lift")
        choices.append(matches)
    return sum(
        sum_points(points) == target for points in itertools.product(*choices)
    )


def sum_points(points: tuple[tuple[int, int], ...]) -> tuple[int, int] | None:
    total = None
    for point in points:
        total = prior.point_add(total, point)
    return total


def run_arm(worker: Path, input_dir: Path, arm: str, output: Path,
            dispatch: dict) -> dict:
    require(arm in POOLS, "unknown arm")
    output.mkdir(parents=True, exist_ok=False)
    pool_hash = POOLS[arm]
    base_path = input_dir / f"{BASE}.jsonl.gz"
    pool_path = input_dir / f"{pool_hash}.jsonl.gz"
    for digest, path in ((BASE, base_path), (pool_hash, pool_path)):
        exact_input(path, dispatch["source_archive_sha256"][digest])
        require(blake3_file(path) == digest, "compressed object BLAKE3 differs")
    prior_result = json.loads((STUDY / "verification/hosted-m6-highcap-20261018/results/39100306846"
                               / f"{arm}-result.json").read_text())
    require(prior_result["base_object_blake3"] == BASE
            and prior_result["pool_object_blake3"] == pool_hash
            and prior_result["ordering"] == "equal_domain_runs", "prior arm differs")
    formula = output / "natural-mask0.xor.cnf"
    export_command = [str(worker), "export-cross-pilot-canonical", BASE,
                      str(MANIFEST), str(base_path), pool_hash, str(MANIFEST),
                      str(pool_path), str(CORPUS), "4", "2", "0", str(formula)]
    started = time.monotonic()
    export = subprocess.run(export_command, capture_output=True, text=True,
                            timeout=300, check=False)
    (output / "export.stdout").write_text(export.stdout)
    (output / "export.stderr").write_text(export.stderr)
    require(export.returncode == 0, "natural-query export failed")
    receipt = json.loads(export.stdout)
    expected = {
        "schema": "n83.mixed-orbit-natural-xor-dimacs-export/v1",
        "status": "EXPORTED_ONE_IDENTITY_PATTERN",
        "base_object_blake3": BASE,
        "pool_object_blake3": pool_hash,
        "public_corpus_canonical_json_blake3":
            "831eaa7be2387634fd610f6706103b2af9b4e274f0db59923c0280af3cbed74b",
        "source_blake3": blake3_file(ROOT / "examples/koblitz_n83_mixed_orbit_worker.rs"),
        "a": prior_result["a"], "b": prior_result["b"],
        "base_summands": 4, "residual_summands": 2,
        "identity_mask": 0, "ordering": "equal_domain_runs",
        "small_signed_orbits": 600,
        "residual_signed_orbits": prior_result["residual_signed_orbits"],
        "variables": prior_result["search"]["peak_variables"],
        "clauses": prior_result["search"]["peak_clauses"],
        "xors": prior_result["search"]["peak_xors"],
        "selected_best_total_runtime": None,
    }
    for key, value in expected.items():
        require(receipt.get(key) == value, f"export receipt {key} differs")
    require(formula.stat().st_size == receipt["dimacs_bytes"]
            and blake3_file(formula) == receipt["dimacs_blake3"],
            "exported XOR-DIMACS bytes differ")
    check_formula_shape(formula, receipt["variables"],
                        receipt["clauses"], receipt["xors"])
    target = prior.point_add(
        point_mul(PRIMARY_GENERATOR, int(receipt["a"])),
        point_mul(prior.prior.PUBLIC_TARGET, int(receipt["b"])),
    )
    require(target == tuple(map(int, receipt["target"])),
            "independent natural target reconstruction differs")
    command = ["/usr/bin/time", "-l", str(CMS), "--threads", "1",
               "--verb", "0", "--printsol", "1", "--maxtime",
               str(MAX_WALL_SECONDS), "--maxconfl", str(MAX_CONFLICTS), str(formula)]
    host_load = os.getloadavg()
    solver_started = time.monotonic()
    solver = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                              text=True, start_new_session=True)
    try:
        stdout, stderr = solver.communicate(timeout=MAX_WALL_SECONDS + 30)
        outer_timeout = False
    except subprocess.TimeoutExpired:
        os.killpg(solver.pid, signal.SIGKILL)
        stdout, stderr = solver.communicate()
        outer_timeout = True
    solver_wall = time.monotonic() - solver_started
    (output / "solver.stdout").write_text(stdout)
    (output / "solver.stderr").write_text(stderr)
    if outer_timeout:
        disposition, model = "UNKNOWN_outer_wall_cap", None
    else:
        disposition, model = prior.model_from_stdout(stdout, receipt["variables"])
        require(solver.returncode == {"SATISFIABLE": 10, "UNSATISFIABLE": 20,
                                      "UNKNOWN": 0, "INDETERMINATE": 15}[disposition],
                "CryptoMiniSat exit status differs from disposition")
    rss = re.search(r"(?m)^\s*(\d+)\s+maximum resident set size\s*$", stderr)
    require(outer_timeout or rss is not None, "peak RSS receipt missing")
    checks: dict = {}
    status = ("UNKNOWN_solver_limit" if disposition in {"UNKNOWN", "INDETERMINATE"}
              else disposition)
    if model is not None:
        clauses, xors = prior.check_dimacs(formula, model)
        require(clauses == receipt["clauses"] and xors == receipt["xors"],
                "DIMACS original constraint counts differ")
        blocks = prior.decoded_blocks(model)
        require(blocks[10] == target[0], "SAT target x differs")
        nodes = ((blocks[0], blocks[1], blocks[6]),
                 (blocks[6], blocks[2], blocks[7]),
                 (blocks[7], blocks[3], blocks[8]),
                 (blocks[8], blocks[4], blocks[9]),
                 (blocks[9], blocks[5], blocks[10]))
        require(all(prior.s3_residual(*node) == 0 for node in nodes),
                "SAT model fails original-field S3")
        lifts = group_lifts(blocks, target, base_path, pool_path,
                            prior_result["shared_signed_orbits"])
        checks = {"clauses_checked": clauses, "xor_rows_checked": xors,
                  "original_field_s3_nodes_checked": 5,
                  "decoded_x_blocks": list(map(str, blocks)),
                  "group_lifts_to_exact_target": lifts}
        status = "SAT_group_lifted" if lifts else "SAT_field_model_only"
    elif disposition == "UNSATISFIABLE":
        status = "UNSAT_one_identity_pattern_only"
    return {
        "schema": "n83.m6-external-natural-local/v1", "arm": arm,
        "status": status, "solver_disposition": disposition,
        "export_exit_code": export.returncode,
        "solver_exit_code": solver.returncode,
        "export": receipt, "checks": checks,
        "solver_conflict_cap": MAX_CONFLICTS,
        "solver_wall_cap_seconds": MAX_WALL_SECONDS,
        "conservative_arm_envelope_seconds": 960,
        "solver_wall_seconds_L0": solver_wall,
        "outer_wall_seconds_L0": time.monotonic() - started,
        "peak_rss_bytes_L0": int(rss.group(1)) if rss is not None else None,
        "solver_binary_sha256": sha256(CMS),
        "host": {"platform": platform.platform(), "cpu_count": os.cpu_count(),
                 "loadavg_before_solver": host_load},
        "worker_binary_sha256": sha256(worker),
        "source_commit": subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "source_dirty": bool(subprocess.check_output(
            ["git", "status", "--porcelain"], cwd=ROOT, text=True)),
        "raw_sha256": {path.name: sha256(path) for path in output.iterdir() if path.is_file()},
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--input-dir", type=Path, required=True)
    parser.add_argument("--arm", choices=POOLS, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    dispatch = json.loads(DISPATCH.read_text())
    exact_input(MANIFEST, dispatch["manifest_sha256"])
    exact_input(CORPUS, dispatch["corpus_sha256"])
    exact_input(CMS, CMS_SHA256)
    require(args.worker.is_file(), "worker missing")
    row = run_arm(args.worker, args.input_dir, args.arm, args.output, dispatch)
    save(args.output / "result.json", row)
    print(json.dumps(row, sort_keys=True))


if __name__ == "__main__":
    main()
