"""Exact fixed-curve xedni height profile for all small generator multiples."""

import hashlib
import json
import math
from fractions import Fraction as F
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = {
    "source_generator_result_sha256": ROOT / "xedni_rank1_generator_certificate_20261121/result.json",
    "source_height_result_sha256": ROOT / "xedni_explicit_height_exclusion_20261121/result.json",
    "source_height_audit_sha256": ROOT / "xedni_explicit_height_exclusion_20261121/audit-result.json",
    "source_finite_order_lean_sha256": ROOT / "xedni_finite_order_lean_20261121/formal/FiniteOrder.lean",
}


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and y1 == -y2:
        return None
    slope = ((3*x1*x1 - 94*x1 + 751) / (2*y1)
             if left == right else (y2-y1) / (x2-x1))
    x3 = slope*slope + 47 - x1 - x2
    return x3, slope*(x1-x3)-y1


def reduce_fraction(z, p):
    assert z.denominator % p != 0
    return z.numerator % p * pow(z.denominator, -1, p) % p


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    for key, path in SOURCE.items():
        assert hashlib.sha256(path.read_bytes()).hexdigest() == protocol[key]
    q = (F(17), F(10))
    point = None
    rows = []
    targets = set()
    eligible = set()
    for n in range(1, 21):
        point = add(point, q)
        assert point is not None
        x, y = point
        assert y*y == x*x*x - 47*x*x + 751*x - 3997
        b = math.isqrt(x.denominator)
        assert b > 0 and b*b == x.denominator
        assert math.gcd(abs(x.numerator), b) == 1
        height = max(abs(x.numerator), b)
        reduced = (reduce_fraction(x, 47), reduce_fraction(y, 47))
        assert (reduced[1]**2 - reduced[0]**3 + reduced[0] + 2) % 47 == 0
        neg_reduced = (reduced[0], -reduced[1] % 47)
        assert reduced != neg_reduced
        targets.update((reduced, neg_reduced))
        if height <= 2**130:
            eligible.update((reduced, neg_reduced))
        rows.append({
            "abs_n": n,
            "H": str(height),
            "H_bits": height.bit_length(),
            "H_sha256": hashlib.sha256(str(height).encode()).hexdigest(),
            "x_sha256": hashlib.sha256(str(x).encode()).hexdigest(),
            "x_denominator_bits": x.denominator.bit_length(),
            "reduction": list(reduced),
            "negative_reduction": list(neg_reduced),
            "under_cap": height <= 2**130,
        })
    assert len(targets) == 40
    assert [row["abs_n"] for row in rows if row["under_cap"]] == list(range(1, 14))
    assert len(eligible) == 26
    hq_lower = (F(2893) * F(2, 3) - 14) / 4225
    tail_naive_lower = 441 * hq_lower - 14
    cap_naive_upper = 260 * F(7, 10)
    assert tail_naive_lower == F(785218, 4225)
    assert tail_naive_lower > cap_naive_upper == 182
    result = {
        "schema": "xedni-height-coverage-profile-result-v1",
        "status": "PASS",
        "rows": rows,
        "finite_under_cap_abs_n": list(range(1, 14)),
        "finite_over_cap_abs_n": list(range(14, 21)),
        "tail_abs_n_at_least": 21,
        "tail_naive_height_lower_rational": str(tail_naive_lower),
        "cap_naive_height_upper_rational": str(cap_naive_upper),
        "nonidentity_target_count": 42,
        "covered_target_count": 26,
        "uncovered_target_count": 16,
        "covered_fraction": "13/21",
        "conditional_global_inputs": ["E(Q)=ZQ", "good reduction is a homomorphism",
                                      "model-specific Silverman bound B<14",
                                      "canonical-height quadraticity"],
    }
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "rows"},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
