"""Exact rational-function obstruction for the local bridge's k3 shifts."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, QQ, ZZ, lcm, prime_range

HERE = Path(__file__).resolve().parent
BRIDGE = HERE.parent / "xedni_bridge6_aux8_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(points, k1, k2, k3):
    (x1, y1), (x2, y2) = [(QQ(x), QQ(y)) for x, y in points]
    a1, a2, a3 = 47*k1, 47*k2, 47*k3
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def primitive(poly):
    scale = lcm([c.denominator() for c in poly.list()])
    out = PolynomialRing(ZZ, "t")(scale*poly)
    out = out // out.content()
    return -out if out.leading_coefficient() < 0 else out


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha((BRIDGE / "result.json").read_bytes()) == protocol["source_bridge_result_sha256"]
    assert sha((BRIDGE / "protocol.json").read_bytes()) == protocol["source_bridge_protocol_sha256"]
    bridge = json.loads((BRIDGE / "result.json").read_text())
    points = protocol["fixed_pair"]
    assert bridge["points"] == points and bridge["crt_k"] == [-3, -11, 1]
    R = PolynomialRing(QQ, "t")
    t = R.gen()
    K = R.fraction_field()
    coeffs = coefficients(points, -3, -11, K(1+77*t))
    E = EllipticCurve(K, [K(c) for c in coeffs])
    assert E.discriminant() != 0
    P, Q = [E(K(x), K(y)) for x, y in points]
    X = (6*Q)[0]-K(points[0][0])
    numerator = primitive(X.numerator())
    denominator = primitive(X.denominator())
    assert numerator != 0 and denominator != 0
    first = None
    for prime in prime_range(13, 258):
        f = numerator.change_ring(GF(prime))
        if not any(f(z) == 0 for z in GF(prime)):
            first = int(prime)
            break
    polynomial = {"schema": "xedni-bridge6-k3-polynomial-v1",
                  "numerator_coefficients": [str(c) for c in numerator.list()],
                  "denominator_coefficients": [str(c) for c in denominator.list()],
                  "numerator_degree": int(numerator.degree()),
                  "denominator_degree": int(denominator.degree()),
                  "certificate_prime": first}
    payload = (json.dumps(polynomial, sort_keys=True, separators=(",", ":"))+"\n").encode()
    (HERE / "polynomial.jsonl").write_bytes(payload)
    print(json.dumps({"schema": "xedni-bridge6-k3-result-v1",
                      "status": "all_integer_shifts_excluded" if first else "unresolved",
                      "source_bridge_result_sha256": protocol["source_bridge_result_sha256"],
                      "polynomial_sha256": sha(payload),
                      "numerator_degree": int(numerator.degree()),
                      "denominator_degree": int(denominator.degree()),
                      "certificate_prime": first}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
