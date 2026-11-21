"""Independent symbolic and modular replay of the 25 shift polynomials."""

import hashlib
import json
import re
from collections import Counter
from math import gcd
from pathlib import Path

import sympy as s

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BRIDGE = ROOT / "xedni_bridge6_aux8_20261120/result.json"
SINGLE = ROOT / "xedni_bridge6_k3_polynomial_20261120/result.json"
SINGLE_POLY = ROOT / "xedni_bridge6_k3_polynomial_20261120/polynomial.jsonl"
POINTS = ((2, 2), (-16, 17))
t = s.symbols("t")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def primitive(expr):
    poly = s.Poly(expr, t).clear_denoms()[1].primitive()[1]
    return -poly if poly.LC() < 0 else poly


def add(P, Q, a1, a2, a3, a4):
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
    intercept = s.cancel(y-slope*x)
    xx = s.cancel(slope*slope+a1*slope-a2-x-u)
    yy = s.cancel(-(slope+a1)*xx-intercept-a3)
    return xx, yy


def independent_polynomials(k1, k2):
    (x1, y1), (x2, y2) = POINTS
    a1, a2, a3 = s.Integer(47*k1), s.Integer(47*k2), 47*(1+77*t)
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = s.cancel((z1-z2)/s.Integer(x1-x2))
    Q = (s.Integer(x2), s.Integer(y2))
    Q2 = add(Q, Q, a1, a2, a3, a4)
    Q4 = add(Q2, Q2, a1, a2, a3, a4)
    Q6 = add(Q4, Q2, a1, a2, a3, a4)
    num, den = s.fraction(s.cancel(Q6[0]-x1))
    return primitive(num), primitive(den)


def mod_value(coeffs, z, p):
    out = 0
    for c in reversed(coeffs):
        out = (out*z+c) % p
    return out


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    assert sha(BRIDGE.read_bytes()) == protocol["source_bridge_result_sha256"]
    assert sha(SINGLE.read_bytes()) == protocol["source_single_shift_result_sha256"]
    files = sorted((HERE / "rows").glob("[0-9][0-9][0-9].jsonl"))
    assert len(files) == result["cells"] == 25
    raw = b"".join(file.read_bytes() for file in files)
    assert sha(raw) == result["rows_raw_sha256"]
    rows = [json.loads(file.read_text()) for file in files]
    lean_text = (HERE / "formal/Grid.lean").read_text()
    lean_lists = re.findall(r"def coeffs(\d{3}) : List ℤ := \[(.*?)\]", lean_text, re.S)
    assert len(lean_lists) == 25
    for i, (label, body) in enumerate(lean_lists):
        assert int(label) == i
        assert tuple(map(int, re.findall(r"-?\d+", body))) == tuple(
            map(int, rows[i]["numerator_coefficients"]))
    old = json.loads(SINGLE_POLY.read_text())
    primes = [int(p) for p in s.primerange(13, 258)]
    prime_counts = Counter()
    for i, row in enumerate(rows):
        u, v = divmod(i, 5)
        u, v = u-2, v-2
        k1, k2 = -3+77*u, -11+77*v
        assert (row["u"], row["v"], row["k1"], row["k2"]) == (u, v, k1, k2)
        assert row["status"] == "all_integer_k3_shifts_excluded"
        assert row["center_local_7"] == {"status": "good", "order": 4, "relation": True}
        assert row["center_local_11"] == {"status": "good", "order": 8, "relation": True}
        assert row["center_rational_relation"] is False
        num, den = independent_polynomials(k1, k2)
        coeff = tuple(int(c) for c in reversed(num.all_coeffs()))
        dcoeff = tuple(int(c) for c in reversed(den.all_coeffs()))
        assert coeff == tuple(map(int, row["numerator_coefficients"]))
        assert dcoeff == tuple(map(int, row["denominator_coefficients"]))
        assert num.degree() == row["numerator_degree"] == 24
        assert den.degree() == row["denominator_degree"] == 22
        assert gcd(*coeff) == 1 and coeff[-1] > 0
        p = row["certificate_prime"]
        assert p in primes
        assert all(any(mod_value(coeff, z, q) == 0 for z in range(q))
                   for q in primes[:primes.index(p)])
        assert all(mod_value(coeff, z, p) != 0 for z in range(p))
        prime_counts[p] += 1
        if (u, v) == (0, 0):
            assert row["numerator_coefficients"] == old["numerator_coefficients"]
            assert row["denominator_coefficients"] == old["denominator_coefficients"]
            assert p == old["certificate_prime"]
    assert sum(prime_counts.values()) == 25
    build_output = (HERE / "formal/build-output.txt").read_text()
    assert build_output.count("depends on axioms:") == 50 and "sorryAx" not in build_output
    print(json.dumps({"schema": "xedni-bridge6-k1k2-grid-audit-v1", "status": "PASS",
                      "independent_symbolic_replays": 25,
                      "modular_rootless_certificates": 25,
                      "lean_polynomial_bindings": 25,
                      "certificate_prime_counts": dict(sorted(prime_counts.items())),
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
