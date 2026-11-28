#!/usr/bin/env python3
"""Replay subgroup coverage through scalar coordinates modulo 607."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
CONTROL = json.loads((ROOT / SPEC["control_result"]).read_text())["cells"]
assert RESULT["status"] == "complete"
row = CONTROL[0]
n, poly = row["n"], row["field_modulus"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
E = EllipticCurve(K, [1, elt(row["curve_a2"]), 0, 0, elt(row["curve_a6"])])
zero = E(0)
raw = []
for bits in range(1 << len(row["basis"])):
    x = 0
    for j, v in enumerate(row["basis"]):
        if (bits >> j) & 1:
            x ^= v
    raw.extend(E.lift_x(elt(x), all=True))
assert len(raw) == 119
h, q = SPEC["cofactor"], SPEC["prime_subgroup_order"]
projected_all = [h * P for P in raw]
H = next(P for P in projected_all if P != zero)
scalar_of = {k * H: k for k in range(q)}
assert len(scalar_of) == q
assert all(P in scalar_of for P in projected_all)
direct = sum(P in scalar_of for P in raw)
assert direct == RESULT["direct_prime_subgroup_points"] == 0
scalar_all = [scalar_of[P] for P in projected_all]
assert scalar_all.count(0) == RESULT["cleared_identity_count"]
base = set(scalar_all) - {0}
assert len(base) == RESULT["cleared_nonidentity_points"]
assert len(scalar_all) - len(set(scalar_all)) == RESULT["cleared_collision_count"]
pair_counts = Counter((a + b) % q for a in base for b in base)
triple_counts = Counter()
for ab, multiplicity in pair_counts.items():
    for c in base:
        triple_counts[(ab + c) % q] += multiplicity
assert len(pair_counts) == RESULT["projected_distinct_pair_sums"]
assert len(triple_counts) == RESULT["projected_three_sum_coverage"]
assert sum(triple_counts.values()) == RESULT["projected_ordered_triples"]
for prior, recorded in zip(CONTROL, RESULT["targets"]):
    R = E(elt(prior["target"][0]), elt(prior["target"][1]))
    assert triple_counts[scalar_of[h * R]] == recorded["projected_target_ordered_triples"]
audit = {"schema": "h4-subgroup-projection-audit-v1", "status": "passed",
         "result_sha256": hashlib.sha256(RESULT_PATH.read_bytes()).hexdigest(),
         "subgroup_scalars": len(scalar_of), "raw_points_in_subgroup": direct,
         "projected_nonidentity_scalars": len(base),
         "pair_sum_scalars": len(pair_counts), "triple_sum_scalars": len(triple_counts),
         "ordered_triples": sum(triple_counts.values())}
(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, indent=2, sort_keys=True))
