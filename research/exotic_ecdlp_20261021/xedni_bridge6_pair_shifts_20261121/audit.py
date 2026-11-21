"""Independent symbolic and modular replay of the 22 shift polynomials."""

import hashlib
import json
import re
from collections import Counter
from math import gcd
from pathlib import Path

import sympy as s

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_bridge6_source_policy_20261121"
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


def independent_polynomials(points, k):
    (x1, y1), (x2, y2) = points
    k1, k2, k3 = k
    a1, a2, a3 = s.Integer(47*k1), s.Integer(47*k2), 47*(k3+77*t)
    b = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    c = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = s.cancel((b-c)/s.Integer(x1-x2))
    Q = (s.Integer(x2), s.Integer(y2))
    Q2 = add(Q, Q, a1, a2, a3, a4)
    Q4 = add(Q2, Q2, a1, a2, a3, a4)
    Q6 = add(Q4, Q2, a1, a2, a3, a4)
    num, den = s.fraction(s.cancel(Q6[0]-x1))
    return primitive(num), primitive(den)


def mod_value(coeffs, z, p):
    return sum((c % p)*pow(z, i, p) for i, c in enumerate(coeffs)) % p


def main():
    protocol = json.loads((HERE / "audit-protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    files = sorted((HERE / "rows").glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(file.read_bytes() for file in files)
    assert sha(raw) == result["rows_raw_sha256"] == protocol["source_rows_sha256"]
    assert len(files) == result["cells"] == 22
    rows = [json.loads(file.read_text()) for file in files]
    source_rows = [json.loads(line) for line in (SOURCE / "outcomes" / "000.jsonl").read_text().splitlines()]
    source_rows = [x for x in source_rows if x["status"] == "local_bridge"]
    assert [x["pair_index"] for x in rows] == [x["pair_index"] for x in source_rows]
    primes = list(s.primerange(13, 2004))
    prime_counts = Counter()
    for i, (row, source) in enumerate(zip(rows, source_rows)):
        assert row["status"] == "excluded_all_integer_t"
        assert row["points"] == source["points"] and row["crt_k"] == source["crt_k"]
        assert row["center_relation"] is False
        assert row["center_local_7"] == {"order": 4, "relation": True}
        assert row["center_local_11"] == {"order": 8, "relation": True}
        num, den = independent_polynomials(row["points"], row["crt_k"])
        coeff = [int(c) for c in reversed(num.all_coeffs())]
        dcoeff = [int(c) for c in reversed(den.all_coeffs())]
        assert coeff == list(map(int, row["numerator_coefficients"]))
        assert dcoeff == list(map(int, row["denominator_coefficients"]))
        assert num.degree() == row["numerator_degree"] == 24
        assert den.degree() == row["denominator_degree"] == 22
        assert gcd(*coeff) == 1 and coeff[-1] > 0
        p = row["certificate_prime"]
        assert p in primes
        assert all(any(mod_value(coeff, z, q) == 0 for z in range(q))
                   for q in primes[:primes.index(p)])
        assert all(mod_value(coeff, z, p) != 0 for z in range(p))
        prime_counts[p] += 1
    lean_text = (HERE / "formal" / "PairShifts.lean").read_text()
    lean_lists = re.findall(r"def coeffs(\d{3}) : List ℤ := \[(.*?)\]", lean_text, re.S)
    assert len(lean_lists) == 22
    for i, (label, body) in enumerate(lean_lists):
        assert int(label) == i
        assert tuple(map(int, re.findall(r"-?\d+", body))) == tuple(
            map(int, rows[i]["numerator_coefficients"]))
    build_output = (HERE / "formal" / "build-output.txt").read_text()
    assert build_output.count("depends on axioms:") == 44
    assert "sorryAx" not in build_output and "error:" not in build_output
    assert not (HERE / "formal" / "build-stderr.txt").read_text()
    print(json.dumps({"schema": "xedni-bridge6-pair-shifts-audit-v1",
                      "status": "PASS", "independent_symbolic_replays": 22,
                      "modular_certificates": 22, "lean_polynomial_bindings": 22,
                      "certificate_prime_counts": dict(sorted(prime_counts.items())),
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
