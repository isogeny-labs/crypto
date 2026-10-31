#!/usr/bin/env python3
"""Independent exhaustive monic-cubic divisibility audit; no Sage/Sympy."""
import itertools
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def mul_x(rem, a, b, c, p):
    r0, r1, r2 = rem
    return ((-r2*c) % p, (r0-r2*b) % p, (r1-r2*a) % p)


def cubic_divides(order, p, a, b, c):
    rem = (1, 0, 0)
    powers = []
    for i in range(7):
        powers.append(rem)
        rem = mul_x(rem, a, b, c, p)
    trace = 4097-order
    return all((powers[6][i]-trace*powers[3][i]+(4096 if i == 0 else 0)) % p == 0
               for i in range(3))


def run():
    data = json.loads((HERE / "certificate.json").read_text())
    source = HERE.parent / "ghs_q16_simple_restriction_bound_20261027" / "certificate.json"
    closure_path = HERE.parent / "ghs_cubic_hess_closure_q16_20261027" / "certificate.json"
    assert hashlib.sha256(source.read_bytes()).hexdigest() == data["source_certificate_sha256"]
    assert hashlib.sha256(closure_path.read_bytes()).hexdigest() == data["closure_certificate_sha256"]
    closure = {r["order"]: r for r in json.loads(closure_path.read_text())["orders"]}
    assert len(data["rows"]) == 44
    for row in data["rows"]:
        assert (row["trace"] % 3 == 0) == (not row["primes"]["3"]["monic_cubic_divisor"])
        for p in data["primes"]:
            found = any(cubic_divides(row["order"], p, a, b, c)
                        for a, b, c in itertools.product(range(p), repeat=3))
            assert found == row["primes"][str(p)]["monic_cubic_divisor"], (row["order"], p)
    assert data["all_screened_primes_excluded"] == {"orders": [4076, 4118], "models": 72}
    assert 106 in closure[4076]["by_a"]["0"]["outside"]
    assert 106 in closure[4118]["by_a"]["32"]["outside"]
    print("independent cubic-divisor audit passed: 44 classes x 5 primes")


if __name__ == "__main__":
    run()
