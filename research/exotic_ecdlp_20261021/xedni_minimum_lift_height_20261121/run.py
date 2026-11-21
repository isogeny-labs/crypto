"""Exact finite representatives and a rational certificate for the infinite tail."""

import hashlib
import json
import math
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent


def add(p, q):
    if p is None:
        return q
    if q is None:
        return p
    x1, y1 = p
    x2, y2 = q
    if x1 == x2 and y1 == -y2:
        return None
    slope = ((3*x1*x1 - 94*x1 + 751) / (2*y1)
             if p == q else (y2-y1) / (x2-x1))
    x3 = slope*slope + 47 - x1 - x2
    return x3, slope*(x1-x3)-y1


def mul(n, p):
    if n < 0:
        x, y = mul(-n, p)
        return x, -y
    q = None
    while n:
        if n & 1:
            q = add(q, p)
        p = add(p, p)
        n //= 2
    return q


def red(z, prime=47):
    assert z.denominator % prime
    return (z.numerator % prime) * pow(z.denominator, -1, prime) % prime


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    for source, key in [
        ("xedni_rank1_generator_certificate_20261121/result.json", "source_generator_sha256"),
        ("xedni_rank1_generator_certificate_20261121/lift-class-result.json", "source_lift_class_sha256"),
        ("xedni_explicit_height_exclusion_20261121/result.json", "source_height_input_sha256"),
        ("xedni_explicit_height_exclusion_20261121/audit-result.json", "source_height_audit_sha256"),
    ]:
        assert sha(ROOT / source) == protocol[key]
    q = F(17), F(10)
    rows = []
    for n in protocol["finite_representatives"]:
        x, y = mul(n, q)
        assert y*y == x**3 - 47*x*x + 751*x - 3997
        assert (red(x), red(y)) == (10, 1)
        b = math.isqrt(x.denominator)
        assert b*b == x.denominator and math.gcd(abs(x.numerator), b) == 1
        height = max(abs(x.numerator), b)
        rows.append({
            "n": n,
            "x_numerator_bits": abs(x.numerator).bit_length(),
            "x_denominator_bits": x.denominator.bit_length(),
            "denominator_root_bits": b.bit_length(),
            "H": str(height),
            "H_bits": height.bit_length(),
            "H_sha256": hashlib.sha256(str(height).encode()).hexdigest(),
            "x_sha256": hashlib.sha256(str(x).encode()).hexdigest(),
        })
    assert rows[0]["H_bits"] == 306 and rows[1]["H_bits"] == 333
    assert int(rows[0]["H"]) < int(rows[1]["H"])
    assert int(rows[0]["H"]) < 2**306
    assert int(rows[1]["H"]) >= 2**332
    # All other n in the residue class satisfy |n| >= 64, hence n² >= 4096.
    hq_lower = (F(2893)*F(2, 3) - 14) / 4225
    tail_naive_lower = 4096*hq_lower - 14
    cap_naive_upper = 2600*F(7, 10)
    assert tail_naive_lower == F(23349974, 12675)
    assert cap_naive_upper == 1820 and tail_naive_lower > cap_naive_upper
    result = {
        "schema": "xedni-minimum-lift-height-result-v1",
        "status": "PASS",
        "finite_representatives": rows,
        "tail_minimum_abs_multiplier": 64,
        "tail_naive_height_lower_rational": str(tail_naive_lower),
        "cap_H": "2^1300",
        "cap_naive_height_upper_rational": str(cap_naive_upper),
        "unique_minimizer": -21,
        "formalization_scope": "integer and real inequalities checked in Lean; curve-specific MW and height theorem inputs external",
    }
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
