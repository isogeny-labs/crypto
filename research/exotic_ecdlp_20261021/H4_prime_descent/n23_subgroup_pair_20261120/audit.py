#!/usr/bin/env python3
"""Independent source-instance and direct scalar-triple replay for n23."""

from sage.all import *
import ast
from collections import Counter
import hashlib
import json
from pathlib import Path
import random
import time
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
SOURCE = ROOT / SPEC["source"]
assert RESULT["status"] == "complete"
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]

def point_record(P):
    return [to_int(P[0]), to_int(P[1])]

t = time.monotonic()
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
needed = {"gfmul", "gfmul_s", "to_int", "popcount", "build_instance"}
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
        returns[0].value.keywords.append(ast.keyword(
            arg="a2", value=ast.Call(func=ast.Name(id="to_int", ctx=ast.Load()),
                                       args=[ast.Name(id="a2", ctx=ast.Load())], keywords=[])))
ast.fix_missing_locations(tree)
M64 = np.uint64
SEED = 20261022
exec(compile(ast.Module(body=funcs, type_ignores=[]), str(SOURCE), "exec"), globals())
system = build_instance(23, 0, "planted")
assert system["basis"] == RESULT["subspace_basis"]
assert system["poly"] == RESULT["field_modulus"]
assert system["a6"] == RESULT["curve_a6"]
assert system["a2"] == RESULT["curve_a2"]
source_build_seconds = time.monotonic() - t

t = time.monotonic()
n, poly = 23, RESULT["field_modulus"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
E = EllipticCurve(K, [1, elt(RESULT["curve_a2"]), 0, 0, elt(RESULT["curve_a6"])])
assert int(E.cardinality()) == RESULT["group_order"]
raw = []
for bits in range(1 << len(RESULT["subspace_basis"])):
    x = 0
    for j, v in enumerate(RESULT["subspace_basis"]):
        if (bits >> j) & 1:
            x ^= v
    raw.extend(E.lift_x(elt(x), all=True))
assert len(raw) == RESULT["raw_lifted_points"]
assert hashlib.sha256(json.dumps([point_record(P) for P in raw],
       separators=(",", ":")).encode()).hexdigest() == RESULT["raw_points_sha256"]
h, r = RESULT["cofactor"], RESULT["prime_subgroup_order"]
zero = E(0)
cleared_all = [h * P for P in raw]
cleared = sorted(set(cleared_all)-{zero}, key=lambda P: point_record(P))
assert len(cleared) == RESULT["cleared_nonidentity_points"]
assert hashlib.sha256(json.dumps([point_record(P) for P in cleared],
       separators=(",", ":")).encode()).hexdigest() == RESULT["cleared_points_sha256"]
H = E(elt(RESULT["generator"][0]), elt(RESULT["generator"][1]))
assert H == next(P for P in cleared_all if P != zero)
scalar_of = {}
current = zero
for k in range(r):
    assert current not in scalar_of
    scalar_of[current] = k
    current += H
assert current == zero and len(scalar_of) == r
assert sum(P in scalar_of for P in raw) == RESULT["direct_subgroup_points"]
base = [scalar_of[P] for P in cleared]
reconstruction_seconds = time.monotonic() - t

t = time.monotonic()
pair_targets = set()
triple_counts = [0] * r
for a in base:
    for b in base:
        ab = (a + b) % r
        pair_targets.add(ab)
        for c in base:
            triple_counts[(ab + c) % r] += 1
assert len(pair_targets) == RESULT["distinct_pair_sums"]
assert sum(triple_counts) == RESULT["exact_ordered_triples"]
assert sum(v > 0 for v in triple_counts) == RESULT["exact_three_sum_targets"]
assert sum(v > 0 for v in triple_counts[1:]) == RESULT["exact_nonidentity_three_sum_targets"]
assert {str(k): v for k, v in sorted(Counter(v for v in triple_counts if v > 0).items())} == RESULT["multiplicity_histogram"]
assert triple_counts[RESULT["target_scalar_fixture"]] == RESULT["target_ordered_triples"]
direct_triple_seconds = time.monotonic() - t

target = E(elt(RESULT["target"][0]), elt(RESULT["target"][1]))
witness = [E(elt(p[0]), elt(p[1])) for p in RESULT["target_witness"]]
assert sum(witness, zero) == target
assert all(P in cleared for P in witness)
assert target == RESULT["target_scalar_fixture"] * H
audit = {"schema": "h4-n23-subgroup-pair-audit-v1", "status": "passed",
         "result_sha256": hashlib.sha256(RESULT_PATH.read_bytes()).hexdigest(),
         "source_build_seconds": source_build_seconds,
         "reconstruction_seconds": reconstruction_seconds,
         "direct_triple_seconds": direct_triple_seconds,
         "source_basis_match": True, "source_modulus_match": True,
         "source_curve_coefficients_match": True,
         "raw_points": len(raw), "cleared_points": len(cleared),
         "subgroup_points": len(scalar_of), "pair_sum_targets": len(pair_targets),
         "three_sum_targets": sum(v > 0 for v in triple_counts),
         "target_witness_verified": True}
(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, indent=2, sort_keys=True))
