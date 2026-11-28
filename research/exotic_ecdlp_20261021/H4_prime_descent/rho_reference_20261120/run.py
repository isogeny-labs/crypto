#!/usr/bin/env python3
"""Five frozen same-point runs of installed Sage's multi-partition rho."""

from sage.all import *
from sage.groups.generic import discrete_log_rho
import hashlib
import inspect
import json
from pathlib import Path
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
for path_key, hash_key in (("candidate", "candidate_sha256"),
                           ("workload", "workload_sha256"),
                           ("original_result", "original_result_sha256")):
    assert hashlib.sha256((ROOT / SPEC[path_key]).read_bytes()).hexdigest() == SPEC[hash_key]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest() == SPEC["sage_function_source_sha256"]
CANDIDATE = json.loads((ROOT / SPEC["candidate"]).read_text())
WORKLOAD = json.loads((ROOT / SPEC["workload"]).read_text())
ORIGINAL = json.loads((ROOT / SPEC["original_result"]).read_text())

def elt_factory():
    field = CANDIDATE["record"]["field"]
    n, poly = field["n"], field["defining_polynomial_int"]
    F2 = GF(2)
    PR = PolynomialRing(F2, "z")
    z = PR.gen()
    mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
    K = GF(1 << n, "z", modulus=mod)
    return K, lambda v: K([(v >> i) & 1 for i in range(n)])

K, elt = elt_factory()
coeff = CANDIDATE["record"]["curve"]["coefficients"]
E = EllipticCurve(K, [1, elt(coeff["a2"]), 0, 0, elt(coeff["a6"])])
def point(pair):
    return E(elt(pair[0]), elt(pair[1]))
G = point(CANDIDATE["record"]["curve"]["G"])
Q = point(WORKLOAD["record"]["target"])
r = CANDIDATE["record"]["curve"]["r"]
assert CANDIDATE["candidate_id"] == ORIGINAL["candidate_id"]
assert WORKLOAD["workload_id"] == ORIGINAL["workload_id"]
assert Q == point(ORIGINAL["target"])
assert r * G == E(0) and r * Q == E(0)

def coord_hash(P):
    if P == E(0):
        return 0
    x = sum(int(c) << i for i, c in enumerate(P[0].polynomial().list()))
    y = sum(int(c) << i for i, c in enumerate(P[1].polynomial().list()))
    return ((x * 0x9e3779b97f4a7c15) ^ (y * 0xbf58476d1ce4e5b9)) & ((1 << 64) - 1)

def timeout(*_):
    raise TimeoutError("frozen 30-second rho cap")

result = {"schema": "h4-n23-sage-rho-reference-result-v1",
          "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
          "candidate_id": CANDIDATE["candidate_id"], "workload_id": WORKLOAD["workload_id"],
          "target": ORIGINAL["target"], "target_point_sha256": ORIGINAL["target_point_sha256"],
          "sage_function_source_sha256": SPEC["sage_function_source_sha256"],
          "status": "running", "runs": []}

def checkpoint():
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

checkpoint()
signal.signal(signal.SIGALRM, timeout)
for seed in SPEC["walk_seeds"]:
    set_random_seed(seed)
    start = time.perf_counter_ns()
    cpu_start = time.process_time_ns()
    signal.alarm(SPEC["per_run_cap_seconds"])
    try:
        scalar = int(discrete_log_rho(Q, G, ord=r, operation="+", hash_function=coord_hash))
        assert scalar * G == Q
        assert scalar == ORIGINAL["verified_scalar"]
        status = "verified"
        error = None
    except TimeoutError as exc:
        scalar = None
        status = "censored_timeout"
        error = str(exc)
    except Exception as exc:
        scalar = None
        status = "error"
        error = repr(exc)
    finally:
        signal.alarm(0)
    result["runs"].append({"seed": seed, "status": status, "scalar": scalar,
                           "wall_ns": time.perf_counter_ns() - start,
                           "cpu_ns": time.process_time_ns() - cpu_start,
                           "peak_rss_so_far_bytes": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                           "error": error})
    checkpoint()
result["status"] = "complete" if all(run["status"] == "verified" for run in result["runs"]) else "incomplete"
checkpoint()
print(json.dumps(result, indent=2, sort_keys=True))
