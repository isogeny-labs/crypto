#!/usr/bin/env python3
"""Independent replay of the n29 instance-zero subgroup eligibility gate."""
from sage.all import *
import hashlib, json, random
from pathlib import Path
HERE = Path(__file__).resolve().parent
spec = json.loads((HERE / "protocol.json").read_text())
result = json.loads((HERE / "result.json").read_text())
assert result["protocol_sha256"] == hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest()
assert result["runner_sha256"] == hashlib.sha256((HERE / "run.py").read_bytes()).hexdigest()
assert result["status"] == "error" and result["error"].startswith("AssertionError")
n = 29
rng = random.Random(20261022 * 1000 + n * 10)
set_random_seed(20261022 + n * 10)
mod = PolynomialRing(GF(2), "x").irreducible_element(n)
K = GF(2**n, "z", modulus=mod)
def elt(v): return K([(v >> i) & 1 for i in range(n)])
a2, a6 = elt(rng.getrandbits(n)), elt(rng.getrandbits(n))
while a6 == 0: a6 = elt(rng.getrandbits(n))
E = EllipticCurve(K, [1, a2, 0, 0, a6])
order = int(E.cardinality())
fac = [(int(p), int(e)) for p, e in factor(order)]
r = max(p for p, _ in fac)
assert order == 536895358 and fac == [(2, 1), (1889, 1), (142111, 1)]
assert r < 2**20
audit = {"schema": "h4-n29-instance-zero-eligibility-audit-v1", "status": "passed",
         "curve_order": order, "factorization": fac, "largest_prime_factor": r,
         "threshold": 2**20, "selection": "rejected_before_base_or_index"}
(HERE / "audit.json").write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")
print(json.dumps(audit, sort_keys=True))
