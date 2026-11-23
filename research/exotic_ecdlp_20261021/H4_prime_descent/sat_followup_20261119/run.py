#!/usr/bin/env python3
"""Frozen H4 n=17 builder with a full Tseitin-CNF first-witness solver."""

from sage.all import *
import ast
import gzip
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import random
import resource
import subprocess
import time
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source"]
SOLVER = Path(SPEC["solver"])
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]
assert hashlib.sha256(SOLVER.read_bytes()).hexdigest() == SPEC["solver_sha256"]

needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
          "eval_eqs_at", "enumerate_solutions"}
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
functions = [node for node in tree.body
             if isinstance(node, ast.FunctionDef) and node.name in needed]
assert {node.name for node in functions} == needed
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=functions, type_ignores=[]), str(SOURCE), "exec"), globals())

result = {
    "schema": "h4-n17-sat-followup-result-v1",
    "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
    "source_sha256": SPEC["source_sha256"],
    "solver_sha256": SPEC["solver_sha256"],
    "status": "running", "arms": {},
}

def checkpoint():
    path = HERE / "result.json"
    tmp = path.with_suffix(".json.tmp")
    tmp.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    os.replace(tmp, path)

def add_xor(clauses, left, right, output):
    # output iff left XOR right, checked by its four truth-table rows.
    clauses.extend([
        (-left, -right, -output), (left, right, -output),
        (left, -right, output), (-left, right, output),
    ])

def encode(eqs, nbase):
    clauses = []
    monomial_vars = {}
    nextvar = nbase

    def literal(mask):
        nonlocal nextvar
        if mask & (mask - 1) == 0:
            assert mask != 0
            return mask.bit_length()
        if mask in monomial_vars:
            return monomial_vars[mask]
        nextvar += 1
        gate = nextvar
        monomial_vars[mask] = gate
        bits = [i + 1 for i in range(nbase) if mask & (1 << i)]
        assert len(bits) >= 2
        clauses.extend([(-gate, bit) for bit in bits])
        clauses.append((gate, *(-bit for bit in bits)))
        return gate

    for poly in eqs:
        masks = [int(m) for m in poly]
        parity = masks.count(0) & 1
        lits = [literal(mask) for mask in masks if mask != 0]
        if not lits:
            if parity:
                clauses.append(())
            continue
        acc = lits[0]
        for lit in lits[1:]:
            nextvar += 1
            out = nextvar
            add_xor(clauses, acc, lit, out)
            acc = out
        clauses.append((acc if parity else -acc,))
    return nextvar, clauses, len(monomial_vars)

def write_cnf(arm, nvars, clauses):
    path = HERE / (arm + ".cnf")
    with path.open("w") as f:
        f.write(f"p cnf {nvars} {len(clauses)}\n")
        for clause in clauses:
            f.write(" ".join(map(str, clause)) + " 0\n")
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    with path.open("rb") as src, gzip.open(str(path) + ".gz", "wb", compresslevel=6) as dst:
        for chunk in iter(lambda: src.read(1024 * 1024), b""):
            dst.write(chunk)
    path.unlink()
    return path, digest

def read_witness(output, nbase):
    assignments = {}
    for line in output.splitlines():
        if line.startswith("v "):
            for token in line.split()[1:]:
                lit = int(token)
                if lit:
                    assignments[abs(lit)] = lit > 0
    if not all(i in assignments for i in range(1, nbase + 1)):
        return None
    return sum(1 << (i - 1) for i in range(1, nbase + 1) if assignments[i])

checkpoint()
for arm in SPEC["stage_order"]:
    started = time.monotonic()
    result["arms"][arm] = {"status": "running"}
    checkpoint()
    sd = build_instance(17, 0, arm)
    assert sd is not None
    solutions = enumerate_solutions(sd)
    planted_residual = eval_eqs_at(sd["eqs"], sd["planted"]) if arm == "planted" else None
    assert arm != "planted" or (planted_residual == 0 and sd["planted"] in set(solutions))
    t_build = time.monotonic()
    nvars, clauses, monomials = encode(sd["eqs"], sd["N"])
    cnf_path, cnf_sha = write_cnf(arm, nvars, clauses)
    build_seconds = time.monotonic() - t_build
    # Decompress the retained exact CNF only for the solver invocation.
    with gzip.open(str(cnf_path) + ".gz", "rb") as src, cnf_path.open("wb") as dst:
        for chunk in iter(lambda: src.read(1024 * 1024), b""):
            dst.write(chunk)
    command = [str(SOLVER), "--verb", "0", "--printsol", "1", str(cnf_path)]
    t_solve = time.monotonic()
    status = "solver_error"
    stdout = ""
    stderr = ""
    returncode = None
    try:
        proc = subprocess.run(command, capture_output=True, text=True,
                              timeout=SPEC["per_arm_solver_deadline_seconds"])
        stdout, stderr, returncode = proc.stdout, proc.stderr, proc.returncode
        if "s SATISFIABLE" in stdout:
            status = "sat_verified"  # subject to the independent check below
        elif "s UNSATISFIABLE" in stdout:
            status = "unsat_solver_reported"
        else:
            status = "solver_unknown"
    except subprocess.TimeoutExpired as exc:
        status = "censored_solver_timeout"
        stdout = (exc.stdout or b"").decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr = (exc.stderr or b"").decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
    solver_seconds = time.monotonic() - t_solve
    cnf_path.unlink()
    witness = read_witness(stdout, sd["N"]) if status == "sat_verified" else None
    witness_residual = eval_eqs_at(sd["eqs"], witness) if witness is not None else None
    witness_in_exact_set = witness in set(solutions) if witness is not None else None
    if status == "sat_verified" and (witness_residual != 0 or not witness_in_exact_set):
        status = "invalid_witness"
    if status == "unsat_solver_reported" and solutions:
        status = "invalid_unsat_report"
    emitted = stdout + "\nSTDERR\n" + stderr
    (HERE / (arm + "-solver-output.txt")).write_text(
        "\n".join(line.rstrip() for line in emitted.splitlines()) + "\n")
    result["arms"][arm] = {
        "status": status, "field_degree": 17, "instance_index": 0,
        "variables": sd["N"], "equations": len(sd["eqs"]),
        "exact_solution_count": len(solutions),
        "planted_residual": planted_residual,
        "planted_in_exact_set": sd["planted"] in set(solutions) if arm == "planted" else None,
        "cnf_vars": nvars, "cnf_clauses": len(clauses),
        "shared_monomial_gates": monomials, "cnf_sha256": cnf_sha,
        "build_seconds": build_seconds, "solver_seconds": solver_seconds,
        "wall_seconds": time.monotonic() - started,
        "solver_returncode": returncode,
        "solver_peak_rss_bytes": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "witness": witness, "witness_residual": witness_residual,
        "witness_in_exact_set": witness_in_exact_set,
    }
    checkpoint()
result["status"] = "complete"
checkpoint()
print(json.dumps({"status": result["status"], "arms": result["arms"]}, sort_keys=True), flush=True)
