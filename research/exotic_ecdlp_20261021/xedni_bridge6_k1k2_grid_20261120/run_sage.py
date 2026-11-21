"""Twenty-five generic coefficient-six shift-polynomial screens."""

import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, QQ, ZZ, lcm, prime_range

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BRIDGE = ROOT / "xedni_bridge6_aux8_20261120/result.json"
SINGLE = ROOT / "xedni_bridge6_k3_polynomial_20261120/result.json"
POINTS = [[2, 2], [-16, 17]]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def coefficients(k1, k2, k3):
    (x1, y1), (x2, y2) = [(QQ(x), QQ(y)) for x, y in POINTS]
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


def local(E, coeffs, p):
    if any(QQ(c).denominator() % p == 0 for c in coeffs):
        return {"status": "nonintegral"}
    if E.discriminant().valuation(p) != 0:
        return {"status": "bad_reduction"}
    Ep = E.change_ring(GF(p))
    P, Q = [Ep(GF(p)(x), GF(p)(y)) for x, y in POINTS]
    order = int(Ep.cardinality())
    relation = bool(P+6*Q == Ep(0))
    return {"status": "good", "order": order, "relation": relation}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(BRIDGE.read_bytes()) == protocol["source_bridge_result_sha256"]
    assert sha(SINGLE.read_bytes()) == protocol["source_single_shift_result_sha256"]
    R = PolynomialRing(QQ, "t")
    t = R.gen()
    K = R.fraction_field()
    primes = list(prime_range(13, 258))
    outdir = HERE / "rows"
    outdir.mkdir(exist_ok=True)
    rows = []
    counts = Counter()
    for u in range(-2, 3):
        for v in range(-2, 3):
            k1, k2 = -3+77*u, -11+77*v
            row = {"u": u, "v": v, "k1": k1, "k2": k2, "status": None}
            try:
                centered = coefficients(k1, k2, QQ(1))
                E0 = EllipticCurve(QQ, centered)
                assert not E0.is_singular()
                P0, Q0 = [E0(QQ(x), QQ(y)) for x, y in POINTS]
                row["center_rational_relation"] = bool(P0+6*Q0 == E0(0))
                row["center_local_7"] = local(E0, centered, 7)
                row["center_local_11"] = local(E0, centered, 11)
                assert row["center_local_7"] == {"status": "good", "order": 4, "relation": True}
                assert row["center_local_11"] == {"status": "good", "order": 8, "relation": True}
                coeffs = coefficients(k1, k2, K(1+77*t))
                E = EllipticCurve(K, [K(c) for c in coeffs])
                assert E.discriminant() != 0
                P, Q = [E(K(x), K(y)) for x, y in POINTS]
                X = (6*Q)[0]-K(POINTS[0][0])
                num, den = primitive(X.numerator()), primitive(X.denominator())
                assert num != 0 and den != 0
                certificate = None
                for prime in primes:
                    f = num.change_ring(GF(prime))
                    if not any(f(z) == 0 for z in GF(prime)):
                        certificate = int(prime)
                        break
                row.update({"numerator_degree": int(num.degree()),
                            "denominator_degree": int(den.degree()),
                            "numerator_coefficients": [str(c) for c in num.list()],
                            "denominator_coefficients": [str(c) for c in den.list()],
                            "certificate_prime": certificate,
                            "status": "all_integer_k3_shifts_excluded" if certificate else "unresolved"})
            except Exception as exc:
                row.update({"status": "producer_failure", "error": repr(exc)})
            counts[row["status"]] += 1
            rows.append(row)
    raw = b""
    for i, row in enumerate(rows):
        line = (json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n").encode()
        (outdir / f"{i:03d}.jsonl").write_bytes(line)
        raw += line
    print(json.dumps({"schema": "xedni-bridge6-k1k2-grid-result-v1",
                      "status": "PASS" if not counts["producer_failure"] else "PARTIAL",
                      "source_bridge_result_sha256": sha(BRIDGE.read_bytes()),
                      "source_single_shift_result_sha256": sha(SINGLE.read_bytes()),
                      "cells": len(rows), "counts": dict(counts),
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
