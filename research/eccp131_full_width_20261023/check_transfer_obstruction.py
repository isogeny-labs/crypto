#!/usr/bin/env python3
"""Derive the finite-group transfer obstruction from retained exact orders."""
import hashlib
import json
import math
from pathlib import Path

here = Path(__file__).resolve().parent
source = here / "runs/replay.json"
record = json.loads(source.read_text())
n = int(record["parameters"]["n"], 16)
m = int(record["auxiliary"]["order"], 16)
assert record["parameters"]["p"] == "0x48e1d43f293469e33194c43186b3abc0b"
assert record["auxiliary"]["a"] == "0x1" and record["auxiliary"]["b"] == "0x12d"
assert n % 2 == 1 and m % 256 == 0
g = math.gcd(n, m)
assert g == 1
print(json.dumps({"schema": "eccp131-transfer-order-screen/v1",
                  "input_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
                  "published_order": n, "auxiliary_order": m, "gcd": g,
                  "published_order_odd": True, "auxiliary_order_divisible_by_256": True,
                  "consequence": "Every group homomorphism between the two rational-point groups has trivial image; no Fp-isogeny joins these curves."}, indent=2))
