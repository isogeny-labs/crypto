"""Replay the sole cell without a prime certificate under the original cap."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, prime_range

HERE = Path(__file__).resolve().parent


def coefficients(k1, k2, k3):
    (x1, y1), (x2, y2) = (QQ(2), QQ(2)), (QQ(-16), QQ(17))
    a1, a2, a3 = 47*k1, 47*k2, 47*k3
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def evaluate(coeffs, t):
    y = 0
    for c in reversed(coeffs):
        y = y*t+c
    return y


def roots_mod(coeffs, p):
    residues = [c % p for c in coeffs]
    roots = []
    for t in range(p):
        y = 0
        for c in reversed(residues):
            y = (y*t+c) % p
        if y == 0:
            roots.append(t)
    return roots


def local(E, p):
    if any(c.denominator() % p == 0 for c in E.a_invariants()):
        return {"status": "nonintegral"}
    if E.discriminant().valuation(p) != 0:
        return {"status": "bad_reduction"}
    Ep = E.change_ring(GF(p))
    P, Q = Ep(GF(p)(2), GF(p)(2)), Ep(GF(p)(-16), GF(p)(17))
    return {"status": "good", "order": int(Ep.cardinality()),
            "relation": bool(P+6*Q == Ep(0))}


def main():
    protocol = json.loads((HERE / "survivor-protocol.json").read_text())
    raw = (HERE / protocol["source_row"]).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == protocol["source_row_sha256"]
    row = json.loads(raw)
    assert (row["u"], row["v"], row["status"]) == (-5, -8, "unresolved")
    coeffs = [int(x) for x in row["coefficients"]]
    roots = [t for t in range(-10000, 10001) if evaluate(coeffs, t) == 0]
    extended_certificate = None
    if not roots:
        extended_certificate = next((int(p) for p in prime_range(13, 2004)
                                     if not roots_mod(coeffs, int(p))), None)
    replays = []
    for t in roots:
        k1, k2, k3 = -3+77*row["u"], -11+77*row["v"], 1+77*t
        curve_coeffs = coefficients(k1, k2, k3)
        E = EllipticCurve(QQ, curve_coeffs)
        P, Q = E(QQ(2), QQ(2)), E(QQ(-16), QQ(17))
        R = P+6*Q
        replays.append({"t": t, "k": [k1, k2, k3],
                        "coefficients": [str(c) for c in curve_coeffs],
                        "discriminant": str(E.discriminant()),
                        "relation": bool(R == E(0)),
                        "remainder": "O" if R == E(0) else [str(x) for x in R.xy()],
                        "local_7": local(E, 7), "local_11": local(E, 11)})
    print(json.dumps({"schema": "xedni-bridge6-wide-sieve-survivor-result-v1",
                      "status": "PASS", "cell": [-5, -8], "scan_t": [-10000, 10000],
                      "integer_roots": roots, "extended_prime_cap": 2003,
                      "extended_certificate_prime": extended_certificate,
                      "replays": replays}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
