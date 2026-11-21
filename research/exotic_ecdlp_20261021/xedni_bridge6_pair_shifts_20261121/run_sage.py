"""One-variable rational relation polynomials for all 22 local bridges."""

import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, QQ, ZZ, lcm, prime_range

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_bridge6_source_policy_20261121"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def event(stage, **data):
    with (HERE / "events.jsonl").open("a") as f:
        f.write(json.dumps({"stage": stage, **data}, sort_keys=True) + "\n")


def coefficients(points, k1, k2, k3):
    (x, y), (u, v) = [(QQ(a), QQ(b)) for a, b in points]
    a1, a2, a3 = 47*k1, 47*k2, 47*k3
    b = y*y+a1*x*y+a3*y-x**3-a2*x*x
    c = v*v+a1*u*v+a3*v-u**3-a2*u*u
    a4 = (b-c)/(x-u)
    return [a1, a2, a3, a4, b-a4*x]


def primitive(poly):
    R = PolynomialRing(ZZ, "t")
    scale = lcm([c.denominator() for c in poly.list()])
    out = R(scale*poly)
    out = out // out.content()
    return -out if out.leading_coefficient() < 0 else out


def local(E, points, p):
    Ep = E.change_ring(GF(p))
    P, Q = [Ep(GF(p)(x), GF(p)(y)) for x, y in points]
    return {"order": int(Ep.cardinality()), "relation": bool(P+6*Q == Ep(0))}


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result_bytes = (SOURCE / "result.json").read_bytes()
    outcome_bytes = b"".join(p.read_bytes() for p in sorted((SOURCE / "outcomes").glob("[0-9][0-9][0-9].jsonl")))
    assert sha(result_bytes) == protocol["source_result_sha256"]
    assert sha(outcome_bytes) == protocol["source_outcomes_sha256"]
    selected = [json.loads(line) for line in outcome_bytes.splitlines()
                if json.loads(line)["status"] == "local_bridge"]
    assert len(selected) == 22
    (HERE / "events.jsonl").write_text("")
    event("started", cells=len(selected))
    R = PolynomialRing(QQ, "t")
    t = R.gen()
    K = R.fraction_field()
    outdir = HERE / "rows"
    outdir.mkdir(exist_ok=True)
    rows = []
    counts = Counter()
    primes = [int(p) for p in prime_range(13, 2004)]
    for i, source in enumerate(selected):
        index, points, (k1, k2, k3) = source["pair_index"], source["points"], source["crt_k"]
        row = {"pair_index": index, "points": points, "crt_k": [k1, k2, k3],
               "status": "producer_failure"}
        event("cell_started", pair_index=index)
        try:
            a = coefficients(points, K(k1), K(k2), K(k3+77*t))
            E = EllipticCurve(K, [K(x) for x in a])
            assert E.discriminant() != 0
            P, Q = [E(K(x), K(y)) for x, y in points]
            X = (6*Q)[0]-K(points[0][0])
            N, D = primitive(X.numerator()), primitive(X.denominator())
            assert D != 0
            if N == 0:
                row.update({"status": "generic_identity", "numerator_coefficients": ["0"],
                            "denominator_coefficients": [str(c) for c in D.list()],
                            "certificate_prime": None})
            else:
                certificate = None
                for p in primes:
                    f = N.change_ring(GF(p))
                    if all(f(z) != 0 for z in GF(p)):
                        certificate = p
                        break
                row.update({"status": "excluded_all_integer_t" if certificate else "unresolved",
                            "numerator_degree": int(N.degree()),
                            "denominator_degree": int(D.degree()),
                            "numerator_coefficients": [str(c) for c in N.list()],
                            "denominator_coefficients": [str(c) for c in D.list()],
                            "certificate_prime": certificate})
            E0 = EllipticCurve(QQ, coefficients(points, QQ(k1), QQ(k2), QQ(k3)))
            P0, Q0 = [E0(QQ(x), QQ(y)) for x, y in points]
            row["center_relation"] = bool(P0+6*Q0 == E0(0))
            row["center_local_7"] = local(E0, points, 7)
            row["center_local_11"] = local(E0, points, 11)
            assert row["center_relation"] == source["crt_rational_relation"]
            assert row["center_local_7"] == {"order": 4, "relation": True}
            assert row["center_local_11"] == {"order": 8, "relation": True}
            event("cell_finished", pair_index=index, status=row["status"],
                  certificate_prime=row["certificate_prime"])
        except Exception as exc:
            row["status"] = "producer_failure"
            row["error"] = repr(exc)
            event("cell_failed", pair_index=index, error=repr(exc))
        rows.append(row)
        counts[row["status"]] += 1
        line = (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode()
        assert len(line) < 48000
        (outdir / f"{i:03d}.jsonl").write_bytes(line)
    raw = b"".join((outdir / f"{i:03d}.jsonl").read_bytes() for i in range(len(rows)))
    event("finished", counts=dict(counts))
    print(json.dumps({"schema": "xedni-bridge6-pair-shifts-result-v1",
                      "status": "PASS" if not counts["producer_failure"] else "PARTIAL",
                      "cells": len(rows), "counts": dict(counts),
                      "source_outcomes_sha256": sha(outcome_bytes),
                      "source_result_sha256": sha(result_bytes),
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
