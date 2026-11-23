"""Independent symbolic group-law and modular-root replay."""

import hashlib
import json
import re
from math import gcd
from pathlib import Path

import sympy as s

HERE = Path(__file__).resolve().parent
BRIDGE = HERE.parent / "xedni_bridge6_aux8_20261120"
t = s.symbols("t")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def primitive(poly):
    out = s.Poly(poly, t).clear_denoms()[1].primitive()[1]
    return -out if out.LC() < 0 else out


def add(P, Q, a1, a2, a3, a4):
    if P is None:
        return Q
    if Q is None:
        return P
    x, y = P
    u, v = Q
    if P == Q:
        lam = s.cancel((3*x*x+2*a2*x+a4-a1*y)/(2*y+a1*x+a3))
    else:
        lam = s.cancel((v-y)/(u-x))
    nu = s.cancel(y-lam*x)
    xx = s.cancel(lam*lam+a1*lam-a2-x-u)
    yy = s.cancel(-(lam+a1)*xx-nu-a3)
    return xx, yy


def mod_value(coeff, z, p):
    v = 0
    for c in reversed(coeff):
        v = (v*z+c) % p
    return v


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    assert sha((BRIDGE / "result.json").read_bytes()) == protocol["source_bridge_result_sha256"]
    assert sha((BRIDGE / "protocol.json").read_bytes()) == protocol["source_bridge_protocol_sha256"]
    payload = (HERE / "polynomial.jsonl").read_bytes()
    assert sha(payload) == result["polynomial_sha256"]
    row = json.loads(payload)
    assert row["certificate_prime"] == result["certificate_prime"]
    (x1, y1), (x2, y2) = protocol["fixed_pair"]
    a1, a2, a3 = s.Integer(-141), s.Integer(-517), 47*(1+77*t)
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = s.cancel((z1-z2)/(x1-x2))
    a6 = s.cancel(z1-a4*x1)
    assert [str(c.subs(t, 0)) for c in (a1, a2, a3, a4, a6)] == json.loads((BRIDGE / "result.json").read_text())["crt_coefficients"]
    Q = (s.Integer(x2), s.Integer(y2))
    Q2 = add(Q, Q, a1, a2, a3, a4)
    Q4 = add(Q2, Q2, a1, a2, a3, a4)
    Q6 = add(Q4, Q2, a1, a2, a3, a4)
    fraction = s.fraction(s.cancel(Q6[0]-x1))
    num, den = [primitive(v) for v in fraction]
    num_coeff = tuple(int(c) for c in reversed(num.all_coeffs()))
    den_coeff = tuple(int(c) for c in reversed(den.all_coeffs()))
    assert num_coeff == tuple(map(int, row["numerator_coefficients"]))
    lean_text = (HERE / "formal/Polynomial.lean").read_text()
    lean_list = lean_text.split("def coeffs : List ℤ := [", 1)[1].split("]", 1)[0]
    assert tuple(map(int, re.findall(r"-?\d+", lean_list))) == num_coeff
    assert den_coeff == tuple(map(int, row["denominator_coefficients"]))
    assert num.degree() == result["numerator_degree"] == 24
    assert den.degree() == result["denominator_degree"] == 22
    assert gcd(*num_coeff) == 1 and num_coeff[-1] > 0
    prime = row["certificate_prime"]
    assert prime == 23
    earlier_primes = [p for p in (13, 17, 19) if p < prime]
    assert all(any(mod_value(num_coeff, z, p) == 0 for z in range(p)) for p in earlier_primes)
    assert all(mod_value(num_coeff, z, prime) != 0 for z in range(prime))
    print(json.dumps({"schema": "xedni-bridge6-k3-audit-v1", "status": "PASS",
                      "symbolic_numerator_degree": num.degree(),
                      "symbolic_denominator_degree": den.degree(),
                      "rootless_modulus": prime,
                      "polynomial_sha256": sha(payload)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
