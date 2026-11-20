#!/usr/bin/env python3
"""Exact group-pair index on the frozen H4 n=19 instances."""

from sage.all import *
import ast
import hashlib
import json
from pathlib import Path
import random
import time
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source"]
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
          "eval_eqs_at", "enumerate_solutions"}
funcs = [node for node in tree.body if isinstance(node, ast.FunctionDef)
         and node.name in needed]
assert {node.name for node in funcs} == needed
for node in funcs:
    if node.name == "build_instance":
        returns = [sub for sub in ast.walk(node) if isinstance(sub, ast.Return)
                   and isinstance(sub.value, ast.Call)
                   and isinstance(sub.value.func, ast.Name)
                   and sub.value.func.id == "dict"]
        assert len(returns) == 1
        returns[0].value.keywords += [ast.keyword(arg="E", value=ast.Name(id="E", ctx=ast.Load())),
                                      ast.keyword(arg="R", value=ast.Name(id="R", ctx=ast.Load()))]
ast.fix_missing_locations(tree)
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=funcs, type_ignores=[]), str(SOURCE), "exec"), globals())

def point_record(P):
    return [to_int(P[0]), to_int(P[1])]

def system_digest(system):
    payload = {key: system[key] for key in ("n", "l", "N", "poly", "basis", "a6", "xR", "planted")}
    payload["eqs"] = [list(map(int, eq)) for eq in system["eqs"]]
    return hashlib.sha256(json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()).hexdigest()

def run_arm(spec):
    system = build_instance(spec["n"], spec["index"], spec["arm"])
    assert system is not None
    E, R, l = system.pop("E"), system.pop("R"), system["l"]
    assert to_int(R[0]) == system["xR"]
    t = time.monotonic()
    s4 = set(enumerate_solutions(system))
    enumeration_seconds = time.monotonic() - t
    if spec["arm"] == "planted":
        assert system["planted"] in s4
        assert eval_eqs_at(system["eqs"], system["planted"]) == 0
    t = time.monotonic()
    point_rows = []
    for bits in range(1 << l):
        xi = 0
        for j, v in enumerate(system["basis"]):
            if (bits >> j) & 1:
                xi ^= v
        for P in E.lift_x(E.base_ring()([(xi >> i) & 1 for i in range(system["n"])]), all=True):
            point_rows.append((bits, P))
    sums = {}
    for bits_a, A in point_rows:
        for bits_b, B in point_rows:
            sums.setdefault(A + B, []).append((bits_a, bits_b))
    setup_seconds = time.monotonic() - t
    t = time.monotonic()
    group_x = set()
    triple_count = 0
    for bits_c, C in point_rows:
        for bits_a, bits_b in sums.get(R - C, ()):
            word = bits_a | (bits_b << l) | (bits_c << (2 * l))
            assert eval_eqs_at(system["eqs"], word) == 0
            group_x.add(word)
            triple_count += 1
    query_seconds = time.monotonic() - t
    missing = sorted(s4 - group_x)
    extra = sorted(group_x - s4)
    assert not extra
    return {"arm": spec["arm"], "n": spec["n"], "index": spec["index"],
            "system_sha256": system_digest(system), "curve_a2": to_int(E.a2()),
            "curve_a6": system["a6"], "field_modulus": system["poly"],
            "basis": system["basis"], "target": point_record(R),
            "factor_base_points": len(point_rows), "pair_entries": len(point_rows)**2,
            "distinct_pair_sums": len(sums), "setup_seconds": setup_seconds,
            "query_seconds": query_seconds, "s4_enumeration_seconds": enumeration_seconds,
            "s4_solution_count": len(s4), "group_x_solution_count": len(group_x),
            "group_triple_count": triple_count,
            "s4_minus_group_count": len(missing), "s4_minus_group_examples": missing[:12],
            "group_minus_s4_count": len(extra),
            "planted_residual": (None if system["planted"] is None
                                 else eval_eqs_at(system["eqs"], system["planted"]))}

result = {"schema": "h4-pair-index-result-v1", "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
          "source_sha256": SPEC["source_sha256"], "cells": []}
for instance in SPEC["instances"]:
    result["cells"].append(run_arm(instance))
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
print(json.dumps(result, indent=2, sort_keys=True))
