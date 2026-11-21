#!/usr/bin/env python3
"""Independent regeneration and witness replay for the H4 XOR comparison."""

from sage.all import *
import ast
import hashlib
import itertools
import json
import math
from pathlib import Path
import random
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
spec = json.loads((HERE / "protocol.json").read_text())
result = json.loads((HERE / "result.json").read_text())
assert hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest() == result["protocol_sha256"]
for field, key in (("source", "source_sha256"),
                   ("cnf_encoder", "cnf_encoder_sha256"),
                   ("solver", "solver_sha256")):
    assert hashlib.sha256((ROOT / spec[field] if field != "solver"
                           else Path(spec[field])).read_bytes()).hexdigest() == spec[key]

def load_functions(path, names):
    tree = ast.parse(path.read_text(), filename=str(path))
    nodes = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name in names]
    assert {n.name for n in nodes} == names
    exec(compile(ast.Module(body=nodes, type_ignores=[]), str(path), "exec"), globals())

M64 = np.uint64
SEED = 20261022
load_functions(ROOT / spec["source"],
               {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
                "eval_eqs_at", "enumerate_solutions"})
load_functions(ROOT / spec["cnf_encoder"], {"add_xor", "encode"})
old_encode = encode
load_functions(HERE / "run.py", {"encode_native_xor", "constraints_hold"})

audit = {"schema": "h4-n17-native-xor-pair-audit-v1", "cells": {}}
for item in spec["stage_order"]:
    arm, encoding = item["arm"], item["encoding"]
    key = arm + "_" + encoding
    rec = result["cells"][key]
    system = build_instance(17, 0, arm)
    solutions = set(enumerate_solutions(system))
    assert len(solutions) == rec["exact_solution_count"]
    if encoding == "cnf":
        nvars, clauses, monomials = old_encode(system["eqs"], system["N"])
        xors = []
    else:
        nvars, clauses, xors, monomials = encode_native_xor(system["eqs"], system["N"])
    assert (nvars, len(clauses), len(xors), monomials) == (
        rec["input_variables"], rec["cnf_clauses"], rec["xor_constraints"],
        rec["shared_monomial_gates"])
    h = hashlib.sha256()
    h.update(f"p cnf {nvars} {len(clauses) + len(xors)}\n".encode())
    for clause in clauses:
        h.update((" ".join(map(str, clause)) + " 0\n").encode())
    for xor in xors:
        h.update(("x" + " ".join(map(str, xor)) + " 0\n").encode())
    assert h.hexdigest() == rec["input_sha256"]
    entry = {"status": rec["status"], "input_sha256_replayed": h.hexdigest(),
             "exact_solution_count_replayed": len(solutions)}
    if arm == "planted":
        assert system["planted"] in solutions
        assert eval_eqs_at(system["eqs"], system["planted"]) == 0
    if rec["status"] == "sat_verified":
        model = {}
        for line in (HERE / (key + "-solver-output.txt")).read_text().splitlines():
            if line.startswith("v "):
                for token in line.split()[1:]:
                    lit = int(token)
                    if lit:
                        model[abs(lit)] = lit > 0
        assert len(model) == nvars
        assert constraints_hold(model, clauses, xors)
        witness = sum(1 << (i - 1) for i in range(1, system["N"] + 1) if model[i])
        assert witness == rec["witness"] and witness in solutions
        assert eval_eqs_at(system["eqs"], witness) == 0
        entry["constraints_checked"] = len(clauses) + len(xors)
        entry["witness_replayed"] = witness
    audit["cells"][key] = entry

(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, sort_keys=True))
