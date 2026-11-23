#!/usr/bin/env python3
"""Freeze five distinct public points after candidate selection."""

from sage.all import *
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = ROOT / "research/exotic_ecdlp_20261021/H4_prime_descent/n29_full_pipeline_20261120"
C = json.loads((SOURCE / "candidate.json").read_text())
O = json.loads((SOURCE / "result.json").read_text())

def digest(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()

field = C["record"]["field"]
n, poly = field["n"], field["defining_polynomial_int"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
coeff = C["record"]["curve"]["coefficients"]
E = EllipticCurve(K, [1, elt(coeff["a2"]), 0, 0, elt(coeff["a6"])])
def point(pair):
    return E(elt(pair[0]), elt(pair[1]))
def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))
def record(P):
    return [to_int(P[0]), to_int(P[1])]
G = point(C["record"]["curve"]["G"])
r = C["record"]["curve"]["r"]
assert r * G == E(0)
used = {entry["known_scalar"] for entry in O["relation_trace"]}
used.add(O["target_fixture_scalar"])
targets = []
for i in range(5):
    counter = 0
    while True:
        label = f"h4-n29-panel-target-{i}-{counter}"
        scalar = int.from_bytes(hashlib.sha256(label.encode()).digest(), "big") % (r - 1) + 1
        if scalar not in used:
            break
        counter += 1
    used.add(scalar)
    Q = scalar * G
    assert Q != E(0)
    workload = {"curve_id": C["record"]["curve"]["curve_id"],
                "G": record(G), "target": record(Q), "target_count": 1,
                "target_law": "sha256_nonzero_scalar_times_G_excluding_precompute_and_prior_target",
                "target_label": label, "precomputation_state": "ready"}
    targets.append({"index": i, "fixture_scalar": scalar, "target": record(Q),
                    "target_point_sha256": digest(record(Q)),
                    "target_label": label, "workload_id": "W" + digest(workload)[:12],
                    "workload_record": workload})
fixture = {"schema": "h4-n29-five-target-fixture-v1",
           "candidate_id": C["candidate_id"], "curve_id": C["record"]["curve"]["curve_id"],
           "generator": record(G), "subgroup_order": r,
           "selection_law": "first SHA256-derived nonzero scalar for each indexed label outside prior relation and target scalars and earlier selected targets",
           "excluded_prior_relation_scalars": len({entry["known_scalar"] for entry in O["relation_trace"]}),
           "targets": targets}
(HERE / "fixtures.json").write_text(json.dumps(fixture, indent=2, sort_keys=True) + "\n")
print(json.dumps({"candidate_id": fixture["candidate_id"], "targets": targets}, indent=2))
