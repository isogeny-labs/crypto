#!/usr/bin/env python3
"""Paired CNF/native-XOR first-witness check on frozen H4 n=19 systems."""

from sage.all import *
import ast
import base64
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
OLD_ENCODER = ROOT / SPEC["cnf_encoder"]
NATIVE_ENCODER = ROOT / SPEC["native_xor_encoder"]
SOLVER = Path(SPEC["solver"])
for path, digest in ((SOURCE, SPEC["source_sha256"]),
                     (OLD_ENCODER, SPEC["cnf_encoder_sha256"]),
                     (NATIVE_ENCODER, SPEC["native_xor_encoder_sha256"]),
                     (SOLVER, SPEC["solver_sha256"])):
    assert hashlib.sha256(path.read_bytes()).hexdigest() == digest

needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
          "eval_eqs_at", "enumerate_solutions"}
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
functions = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name in needed]
assert {n.name for n in functions} == needed
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=functions, type_ignores=[]), str(SOURCE), "exec"), globals())

old_tree = ast.parse(OLD_ENCODER.read_text(), filename=str(OLD_ENCODER))
old_functions = [n for n in old_tree.body if isinstance(n, ast.FunctionDef)
                 and n.name in {"add_xor", "encode"}]
assert {n.name for n in old_functions} == {"add_xor", "encode"}
exec(compile(ast.Module(body=old_functions, type_ignores=[]),
             str(OLD_ENCODER), "exec"), globals())
old_encode = encode

native_tree = ast.parse(NATIVE_ENCODER.read_text(), filename=str(NATIVE_ENCODER))
native_functions = [n for n in native_tree.body if isinstance(n, ast.FunctionDef)
                    and n.name == "encode_native_xor"]
assert len(native_functions) == 1
exec(compile(ast.Module(body=native_functions, type_ignores=[]),
             str(NATIVE_ENCODER), "exec"), globals())

def write_input(name, nvars, clauses, xors):
    path = HERE / (name + ".cnf")
    h = hashlib.sha256()
    with path.open("wb") as f:
        def emit(line):
            data = line.encode()
            f.write(data)
            h.update(data)
        emit(f"p cnf {nvars} {len(clauses) + len(xors)}\n")
        for clause in clauses:
            emit(" ".join(map(str, clause)) + " 0\n")
        for xor in xors:
            emit("x" + " ".join(map(str, xor)) + " 0\n")
    return path, h.hexdigest()

def parse_model(output):
    model = {}
    for line in output.splitlines():
        if line.startswith("v "):
            for token in line.split()[1:]:
                lit = int(token)
                if lit:
                    model[abs(lit)] = lit > 0
    return model

def constraints_hold(model, clauses, xors):
    for clause in clauses:
        if not any(model[abs(lit)] == (lit > 0) for lit in clause):
            return False
    for xor in xors:
        if sum(model[abs(lit)] == (lit > 0) for lit in xor) % 2 != 1:
            return False
    return True

def checkpoint(result):
    path = HERE / "result.json"
    tmp = path.with_suffix(".json.tmp")
    tmp.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    os.replace(tmp, path)

result = {
    "schema": "h4-n19-native-xor-scale-result-v1",
    "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
    "source_sha256": SPEC["source_sha256"],
    "cnf_encoder_sha256": SPEC["cnf_encoder_sha256"],
    "native_xor_encoder_sha256": SPEC["native_xor_encoder_sha256"],
    "solver_sha256": SPEC["solver_sha256"],
    "status": "running", "cells": {},
}
checkpoint(result)
systems = {}
for arm in SPEC["instance"]["arms"]:
    system = build_instance(19, 0, arm)
    assert system is not None
    solutions = set(enumerate_solutions(system))
    assert arm != "planted" or (system["planted"] in solutions
                                and eval_eqs_at(system["eqs"], system["planted"]) == 0)
    systems[arm] = (system, solutions)

for cell in SPEC["stage_order"]:
    arm, encoding = cell["arm"], cell["encoding"]
    key = arm + "_" + encoding
    system, solutions = systems[arm]
    result["cells"][key] = {"status": "running"}
    checkpoint(result)
    t_build = time.monotonic()
    if encoding == "cnf":
        nvars, clauses, monomials = old_encode(system["eqs"], system["N"])
        xors = []
    else:
        nvars, clauses, xors, monomials = encode_native_xor(system["eqs"], system["N"])
    path, input_hash = write_input(key, nvars, clauses, xors)
    build_seconds = time.monotonic() - t_build
    t_solve = time.monotonic()
    stdout, stderr, code = "", "", None
    status = "solver_unknown"
    try:
        proc = subprocess.run([str(SOLVER), "--verb", "0", "--printsol", "1", str(path)],
                              capture_output=True, text=True,
                              timeout=SPEC["per_cell_solver_deadline_seconds"])
        stdout, stderr, code = proc.stdout, proc.stderr, proc.returncode
        if "s SATISFIABLE" in stdout:
            status = "sat_pending_replay"
        elif "s UNSATISFIABLE" in stdout:
            status = "unsat_solver_reported"
    except subprocess.TimeoutExpired as exc:
        status = "censored_solver_timeout"
        stdout = (exc.stdout or b"").decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr = (exc.stderr or b"").decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
    solver_seconds = time.monotonic() - t_solve
    path.unlink()
    emitted = stdout + "\nSTDERR\n" + stderr
    (HERE / (key + "-solver-output.raw")).write_text(emitted)
    (HERE / (key + "-solver-output.txt")).write_text(
        "\n".join(line.rstrip() for line in emitted.splitlines()
                  if not line.startswith("v ")) + "\n")
    model = parse_model(stdout) if status == "sat_pending_replay" else {}
    model_bits_b64 = None
    model_bits_sha256 = None
    if status == "sat_pending_replay" and len(model) == nvars:
        bits = bytearray((nvars + 7) // 8)
        for i in range(1, nvars + 1):
            if model[i]:
                bits[(i - 1) // 8] |= 1 << ((i - 1) % 8)
        model_bits_b64 = base64.b64encode(bytes(bits)).decode("ascii")
        model_bits_sha256 = hashlib.sha256(bits).hexdigest()
    witness = None
    witness_residual = None
    witness_in_exact_set = None
    all_constraints_hold = None
    if status == "sat_pending_replay":
        all_constraints_hold = len(model) == nvars and constraints_hold(model, clauses, xors)
        if all(i in model for i in range(1, system["N"] + 1)):
            witness = sum(1 << (i - 1) for i in range(1, system["N"] + 1) if model[i])
            witness_residual = eval_eqs_at(system["eqs"], witness)
            witness_in_exact_set = witness in solutions
        status = ("sat_verified" if all_constraints_hold and witness_residual == 0
                  and witness_in_exact_set else "invalid_witness")
    if status == "unsat_solver_reported" and solutions:
        status = "invalid_unsat_report"
    result["cells"][key] = {
        "arm": arm, "encoding": encoding, "status": status,
        "exact_solution_count": len(solutions),
        "planted_residual": eval_eqs_at(system["eqs"], system["planted"]) if arm == "planted" else None,
        "input_variables": nvars, "cnf_clauses": len(clauses),
        "xor_constraints": len(xors), "shared_monomial_gates": monomials,
        "input_sha256": input_hash, "build_seconds": build_seconds,
        "solver_seconds": solver_seconds, "solver_returncode": code,
        "child_peak_rss_so_far_bytes": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "witness": witness, "witness_residual": witness_residual,
        "witness_in_exact_set": witness_in_exact_set,
        "all_constraints_hold": all_constraints_hold,
        "model_bits_b64": model_bits_b64,
        "model_bits_sha256": model_bits_sha256,
        "raw_solver_output_sha256": hashlib.sha256(emitted.encode()).hexdigest(),
    }
    checkpoint(result)
    print(json.dumps({"cell": key, "status": status, "solver_seconds": solver_seconds}), flush=True)
result["status"] = "complete_with_censoring" if any(
    c["status"].startswith("censored") for c in result["cells"].values()
) else "complete"
checkpoint(result)
