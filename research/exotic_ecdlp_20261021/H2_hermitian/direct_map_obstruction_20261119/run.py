#!/usr/bin/env python3
"""Independent point-count check for the finished H2 Legendre cells."""

from sage.all import *
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
spec = json.loads((HERE / "protocol.json").read_text())
source = ROOT / spec["source_result"]
assert hashlib.sha256(source.read_bytes()).hexdigest() == spec["source_result_sha256"]
original = json.loads(source.read_text())
result = {
    "schema": "h2-direct-map-obstruction-result-v1",
    "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
    "source_result_sha256": spec["source_result_sha256"],
    "cells": [],
}

for input_cell in spec["cells"]:
    p = input_cell["p"]
    lam = input_cell["lambda"]
    assert original["cells"][str(p)]["lambda"] == lam
    F = GF(p * p, "a")
    L = F(lam)
    E = EllipticCurve(F, [0, -(F(1) + L), 0, L, 0])
    naive = 1
    for x in F:
        rhs = x * (x - 1) * (x - L)
        naive += 1 if rhs == 0 else (2 if rhs.is_square() else 0)
    sage_order = int(E.cardinality())
    assert naive == sage_order
    assert sage_order == original["cells"][str(p)]["E_order_over_Fq"]
    result["cells"].append({
        "p": p, "lambda": lam, "field_order": p * p,
        "naive_curve_order": int(naive), "sage_curve_order": sage_order,
        "trace": p * p + 1 - sage_order,
        "hermitian_maximal_quotient_count": (p + 1) ** 2,
        "matches_quotient_count": sage_order == (p + 1) ** 2,
    })

(HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
print(json.dumps(result, sort_keys=True))
