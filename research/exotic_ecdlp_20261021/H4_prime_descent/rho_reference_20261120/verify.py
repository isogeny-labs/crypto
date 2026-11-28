#!/usr/bin/env python3
"""Independent point and identity replay for the five Sage rho runs."""

from sage.all import *
import hashlib
import inspect
import json
from pathlib import Path
import statistics
from sage.groups.generic import discrete_log_rho

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
assert RESULT["status"] == "complete"
assert hashlib.sha256((HERE / "run.py").read_bytes()).hexdigest() == SPEC["runner_sha256"]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest() == SPEC["sage_function_source_sha256"]
for path_key, hash_key in (("candidate", "candidate_sha256"),
                           ("workload", "workload_sha256"),
                           ("original_result", "original_result_sha256")):
    assert hashlib.sha256((ROOT / SPEC[path_key]).read_bytes()).hexdigest() == SPEC[hash_key]
C = json.loads((ROOT / SPEC["candidate"]).read_text())
W = json.loads((ROOT / SPEC["workload"]).read_text())
O = json.loads((ROOT / SPEC["original_result"]).read_text())
assert RESULT["candidate_id"] == C["candidate_id"] == O["candidate_id"]
assert RESULT["workload_id"] == W["workload_id"] == O["workload_id"]
assert RESULT["target"] == W["record"]["target"] == O["target"]
assert [run["seed"] for run in RESULT["runs"]] == SPEC["walk_seeds"]

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
G = point(C["record"]["curve"]["G"])
Q = point(RESULT["target"])
r = C["record"]["curve"]["r"]
assert r * G == r * Q == E(0)
assert all(run["status"] == "verified" and run["scalar"] == O["verified_scalar"]
           and run["scalar"] * G == Q and run["wall_ns"] > 0 and run["cpu_ns"] > 0
           for run in RESULT["runs"])
times = [run["wall_ns"] for run in RESULT["runs"]]
audit = {"schema": "h4-n23-sage-rho-reference-audit-v1", "status": "passed",
         "result_sha256": hashlib.sha256(RESULT_PATH.read_bytes()).hexdigest(),
         "verified_runs": len(times), "target_scalar": O["verified_scalar"],
         "min_wall_ns": min(times), "median_wall_ns": int(statistics.median(times)),
         "max_wall_ns": max(times), "original_floyd_wall_ns": O["rho_online_wall_ns"],
         "ic_online_wall_ns": O["online_wall_ns"]}
(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, indent=2, sort_keys=True))
