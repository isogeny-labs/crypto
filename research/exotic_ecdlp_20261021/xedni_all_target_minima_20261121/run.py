"""Exact minimum xedni lift height for each reduced nonidentity point."""

import hashlib
import json
import math
from fractions import Fraction as F
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCES = {
    "source_coverage_sha256": ROOT / "xedni_height_coverage_profile_20261121/result.json",
    "source_generator_sha256": ROOT / "xedni_rank1_generator_certificate_20261121/result.json",
    "source_height_input_sha256": ROOT / "xedni_explicit_height_exclusion_20261121/result.json",
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


def red(value):
    assert value.denominator % 47 != 0
    return value.numerator % 47 * pow(value.denominator, -1, 47) % 47


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    amendment = json.loads((HERE / "protocol-amendment.json").read_text())
    assert amendment["field"] == "source_coverage_sha256"
    assert protocol[amendment["field"]] == amendment["frozen_value"]
    for key, path in SOURCES.items():
        expected = amendment["correct_value"] if key == amendment["field"] else protocol[key]
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
    q = F(17), F(10)
    point = None
    finite = []
    positive_reductions = set()
    for n in range(1, 32):
        point = add(point, q)
        assert point is not None
        x, y = point
        assert y*y == x*x*x - 47*x*x + 751*x - 3997
        b = math.isqrt(x.denominator)
        assert b > 0 and b*b == x.denominator
        assert math.gcd(abs(x.numerator), b) == 1
        height = max(abs(x.numerator), b)
        reduction = red(x), red(y)
        assert (reduction[1]**2 - reduction[0]**3 + reduction[0] + 2) % 47 == 0
        assert reduction not in positive_reductions
        positive_reductions.add(reduction)
        finite.append({
            "abs_n": n, "H": str(height), "H_bits": height.bit_length(),
            "H_sha256": hashlib.sha256(str(height).encode()).hexdigest(),
            "x_sha256": hashlib.sha256(str(x).encode()).hexdigest(),
            "reduction": list(reduction),
            "negative_reduction": [reduction[0], -reduction[1] % 47],
        })
    assert len(positive_reductions) == 31
    assert max(item["H_bits"] for item in finite[:21]) == 306
    assert all(item["H_bits"] > 306 for item in finite[21:])

    targets = []
    for residue in range(1, 43):
        candidates = [n for n in range(-31, 32) if n and n % 43 == residue]
        assert candidates
        minimizer = min(candidates, key=lambda n: int(finite[abs(n)-1]["H"]))
        tied = [n for n in candidates if finite[abs(n)-1]["H"] ==
                finite[abs(minimizer)-1]["H"]]
        centered = residue if residue <= 21 else residue - 43
        assert tied == [centered]
        source = finite[abs(centered)-1]
        coordinates = source["reduction"] if centered > 0 else source["negative_reduction"]
        targets.append({
            "residue": residue, "minimizing_n": centered,
            "reduction": coordinates, "H": source["H"],
            "H_bits": source["H_bits"], "H_sha256": source["H_sha256"],
            "finite_candidates": candidates,
        })
    assert len({tuple(item["reduction"]) for item in targets}) == 42
    assert [item["residue"] for item in targets if item["H_bits"] == 306] == [21, 22]

    hq_lower = (F(2893) * F(2, 3) - 14) / 4225
    tail_naive_lower = 1024 * hq_lower - 14
    rep_naive_upper = 612 * F(7, 10)
    assert tail_naive_lower == F(5704406, 12675)
    assert rep_naive_upper == F(2142, 5)
    assert tail_naive_lower > rep_naive_upper
    result = {
        "schema": "xedni-all-target-minima-result-v1", "status": "PASS",
        "finite_multiples": finite, "targets": targets,
        "target_count": 42, "max_minimum_H_bits": 306,
        "worst_residues": [21, 22],
        "tail_abs_n_at_least": 32,
        "tail_naive_height_lower_rational": str(tail_naive_lower),
        "all_centered_representatives_naive_height_upper_rational": str(rep_naive_upper),
        "conditional_global_inputs": ["E(Q)=ZQ", "good reduction is a homomorphism",
                                      "Silverman bound B<14", "canonical-height quadraticity"],
    }
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: value for key, value in result.items()
                      if key not in ("finite_multiples", "targets")}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
