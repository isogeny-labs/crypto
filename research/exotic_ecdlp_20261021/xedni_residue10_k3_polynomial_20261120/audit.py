"""Independent SymPy group law and modular-root audit of the k3 certificates."""

import gzip
import hashlib
import importlib.util
import json
from math import gcd
from pathlib import Path

import sympy as s

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_residue10_rational_20261120/rational-checks.jsonl.gz"
POINTS = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
ARITHMETIC = ROOT / "xedni_relation_rank_20261120/run.py"
spec = importlib.util.spec_from_file_location("independent_f47_group", ARITHMETIC)
arith = importlib.util.module_from_spec(spec)
spec.loader.exec_module(arith)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def mod_value(coefficients, x, prime):
    value = 0
    for coefficient in reversed(coefficients):
        value = (value*x+coefficient) % prime
    return value


def independent_numerator(row):
    (x1, y1), (x2, y2) = row["points"]
    k1, k2, k3 = row["centered_k"]
    t = s.symbols("t")
    a1, a2 = s.Integer(47*k1), s.Integer(47*k2)
    a3 = s.Integer(47*k3)+s.Integer(47*77)*t
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = s.cancel((z1-z2)/s.Integer(x1-x2))

    def add(P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        x, y = P
        u, v = Q
        if P == Q:
            slope = s.cancel((3*x*x+2*a2*x+a4-a1*y)/(2*y+a1*x+a3))
        else:
            slope = s.cancel((v-y)/(u-x))
        nu = s.cancel(y-slope*x)
        xx = s.cancel(slope*slope+a1*slope-a2-x-u)
        yy = s.cancel(-(slope+a1)*xx-nu-a3)
        return xx, yy

    Q = (s.Integer(x2), s.Integer(y2))
    Q2 = add(Q, Q)
    Q4 = add(Q2, Q2)
    Q8 = add(Q4, Q4)
    Q10 = add(Q8, Q2)
    numerator = s.Poly(s.fraction(s.cancel(Q10[0]-x1))[0], t)
    numerator = numerator.clear_denoms()[1].primitive()[1]
    if numerator.LC() < 0:
        numerator = -numerator
    return tuple(int(c) for c in reversed(numerator.all_coeffs()))


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes, point_bytes = SOURCE.read_bytes(), POINTS.read_bytes()
    assert sha(source_bytes) == protocol["source_checks_sha256"]
    assert sha(point_bytes) == protocol["point_source_sha256"]
    checks = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    points = [json.loads(x) for x in gzip.decompress(point_bytes).splitlines()]
    selected_bytes = (HERE / "selection.jsonl.gz").read_bytes()
    row_files = sorted((HERE / "polynomial-rows").glob("[0-9][0-9][0-9].jsonl"))
    assert len(row_files) == 32
    selected_raw = gzip.decompress(selected_bytes)
    polynomial_raw = b"".join(path.read_bytes() for path in row_files)
    polynomial_bytes = gzip.compress(polynomial_raw, mtime=0)
    assert sha(selected_bytes) == result["selection_packed_sha256"]
    assert sha(selected_raw) == result["selection_raw_sha256"]
    assert sha(polynomial_bytes) == result["polynomial_rows_packed_sha256"]
    assert sha(polynomial_raw) == result["polynomial_rows_raw_sha256"]
    selection = [json.loads(x) for x in selected_raw.splitlines()]
    rows = [json.loads(x) for x in polynomial_raw.splitlines()]
    assert len(checks) == len(selection) == 1488 and len(rows) == 32
    matched = []
    for i, (old, entry) in enumerate(zip(checks, selection)):
        assert entry["source_position"] == i and entry["pair_index"] == old["pair_index"]
        assert entry["local_indices"] == old["local_indices"]
        P, Q = [tuple(v % 47 for v in xy) for xy in points[old["pair_index"]]["lifted_points"]]
        ten = None
        for _ in range(10):
            ten = arith.add(ten, Q, 47)
        compatible = arith.add(P, ten, 47) is None
        assert entry["reduced_compatible"] == compatible
        if compatible:
            matched.append(old)
    assert len(matched) == 32
    primes = [int(p) for p in s.primerange(13, 258)]
    partner = {}
    coefficients = []
    for row, old in zip(rows, matched):
        assert row["pair_index"] == old["pair_index"]
        assert row["local_indices"] == old["local_indices"]
        assert row["centered_k"] == old["crt_shifts"]
        assert row["points"] == points[old["pair_index"]]["lifted_points"]
        assert row["status"] == "all_integer_shifts_excluded"
        coeff = tuple(int(x) for x in row["numerator_coefficients"])
        assert coeff and coeff[-1] > 0
        assert gcd(*coeff) == 1
        assert len(coeff)-1 == row["numerator_degree"]
        prime = row["certificate_prime"]
        assert prime in primes
        for p in primes[:primes.index(prime)]:
            assert any(mod_value(coeff, x, p) == 0 for x in range(p))
        assert all(mod_value(coeff, x, prime) != 0 for x in range(prime))
        coefficients.append(coeff)
    index = {coeff: i for i, coeff in enumerate(coefficients)}
    for i, coeff in enumerate(coefficients):
        reflected = tuple((-1)**j*v for j, v in enumerate(coeff))
        j = index[reflected]
        assert j != i and index[tuple((-1)**n*v for n, v in enumerate(coefficients[j]))] == i
        a, b = rows[i], rows[j]
        assert [[x, -y] for x, y in a["points"]] == b["points"]
        ka, kb = a["centered_k"], b["centered_k"]
        assert kb == [-ka[0], ka[1], -ka[2]]
        partner[i] = j
    representatives = [i for i in range(32) if i < partner[i]]
    assert len(representatives) == 16
    for i in representatives:
        assert independent_numerator(rows[i]) == coefficients[i]
    print(json.dumps({"schema": "xedni-residue10-k3-polynomial-audit-v1",
                      "status": "PASS", "screened": 1488, "selected": 32,
                      "modular_rootless_certificates": 32,
                      "independent_symbolic_replays": len(representatives),
                      "verified_sign_reflections": 16,
                      "polynomial_rows_raw_sha256": sha(polynomial_raw)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
