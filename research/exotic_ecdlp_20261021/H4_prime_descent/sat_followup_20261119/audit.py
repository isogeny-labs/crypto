#!/usr/bin/env python3
"""Independently replay CNF models and original H4 equations."""

from sage.all import *
import ast
import gzip
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
source = ROOT / spec["source"]
assert hashlib.sha256(source.read_bytes()).hexdigest() == spec["source_sha256"]
assert hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest() == result["protocol_sha256"]
assert hashlib.sha256(Path(spec["solver"]).read_bytes()).hexdigest() == spec["solver_sha256"]

needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
          "eval_eqs_at", "enumerate_solutions"}
tree = ast.parse(source.read_text(), filename=str(source))
functions = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name in needed]
assert {n.name for n in functions} == needed
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=functions, type_ignores=[]), str(source), "exec"), globals())
encoder_source = HERE / "run.py"
encoder_tree = ast.parse(encoder_source.read_text(), filename=str(encoder_source))
encoder_functions = [n for n in encoder_tree.body
                     if isinstance(n, ast.FunctionDef) and n.name in {"add_xor", "encode"}]
assert {n.name for n in encoder_functions} == {"add_xor", "encode"}
exec(compile(ast.Module(body=encoder_functions, type_ignores=[]),
             str(encoder_source), "exec"), globals())

audit = {"schema": "h4-n17-sat-followup-audit-v1", "arms": {}}
for arm in spec["stage_order"]:
    rec = result["arms"][arm]
    system = build_instance(17, 0, arm)
    solutions = set(enumerate_solutions(system))
    assert len(solutions) == rec["exact_solution_count"]
    assert rec["variables"] == system["N"] and rec["equations"] == len(system["eqs"])
    cnf_vars, clauses, monomials = encode(system["eqs"], system["N"])
    assert cnf_vars == rec["cnf_vars"] and len(clauses) == rec["cnf_clauses"]
    assert monomials == rec["shared_monomial_gates"]
    digest = hashlib.sha256()
    digest.update(f"p cnf {cnf_vars} {len(clauses)}\n".encode())
    for clause in clauses:
        digest.update((" ".join(map(str, clause)) + " 0\n").encode())
    assert digest.hexdigest() == rec["cnf_sha256"]
    entry = {"exact_solution_count_replayed": len(solutions), "status": rec["status"]}
    if arm == "planted":
        entry["planted_residual_replayed"] = eval_eqs_at(system["eqs"], system["planted"])
        entry["planted_in_exact_set_replayed"] = system["planted"] in solutions
        assert entry["planted_residual_replayed"] == 0 and entry["planted_in_exact_set_replayed"]
    if rec["status"] == "sat_verified":
        model = {}
        lines = (HERE / (arm + "-solver-output.txt")).read_text().splitlines()
        for line in lines:
            if line.startswith("v "):
                for token in line.split()[1:]:
                    literal = int(token)
                    if literal:
                        model[abs(literal)] = literal > 0
        assert len(model) == rec["cnf_vars"]
        witness = sum(1 << (i - 1) for i in range(1, system["N"] + 1) if model[i])
        assert witness == rec["witness"]
        entry["witness_residual_replayed"] = eval_eqs_at(system["eqs"], witness)
        entry["witness_in_exact_set_replayed"] = witness in solutions
        assert entry["witness_residual_replayed"] == 0 and entry["witness_in_exact_set_replayed"]
        assert all(any(model[abs(lit)] == (lit > 0) for lit in clause)
                   for clause in clauses)
        entry["cnf_clauses_checked"] = len(clauses)
    entry["cnf_sha256_replayed"] = digest.hexdigest()
    audit["arms"][arm] = entry

(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, sort_keys=True))
