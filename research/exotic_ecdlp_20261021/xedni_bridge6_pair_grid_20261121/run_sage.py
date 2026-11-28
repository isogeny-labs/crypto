"""Exact 22-by-25 nearby CRT coefficient screen with arbitrary third shift."""

import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, QQ, ZZ, lcm, prime_range

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_bridge6_source_policy_20261121"
PAIR_SHIFTS = ROOT / "xedni_bridge6_pair_shifts_20261121"


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
    return int(Ep.cardinality()), bool(P+6*Q == Ep(0))


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_raw = b"".join(p.read_bytes() for p in sorted((SOURCE / "outcomes").glob("[0-9][0-9][0-9].jsonl")))
    pair_raw = b"".join(p.read_bytes() for p in sorted((PAIR_SHIFTS / "rows").glob("[0-9][0-9][0-9].jsonl")))
    assert sha(source_raw) == protocol["source_outcomes_sha256"]
    assert sha(pair_raw) == protocol["source_pair_shifts_rows_sha256"]
    selected = [row for row in map(json.loads, source_raw.splitlines()) if row["status"] == "local_bridge"]
    assert len(selected) == 22
    (HERE / "events.jsonl").write_text("")
    event("started", pairs=len(selected), cells=550)
    R = PolynomialRing(QQ, "t")
    t = R.gen()
    K = R.fraction_field()
    primes = [int(p) for p in prime_range(13, 998)]
    outdir = HERE / "rows"
    outdir.mkdir(exist_ok=True)
    counts = Counter()
    prime_counts = Counter()
    rownum = 0
    raw_hash = hashlib.sha256()
    for source in selected:
        pair_index, points, (base1, base2, base3) = source["pair_index"], source["points"], source["crt_k"]
        for u in range(-2, 3):
            for v in range(-2, 3):
                k1, k2 = base1+77*u, base2+77*v
                row = {"pair_index": pair_index, "points": points,
                       "base_k": [base1, base2, base3], "u": u, "v": v,
                       "k1": k1, "k2": k2, "status": "producer_failure"}
                try:
                    a = coefficients(points, K(k1), K(k2), K(base3+77*t))
                    E = EllipticCurve(K, [K(x) for x in a])
                    assert E.discriminant() != 0
                    P, Q = [E(K(x), K(y)) for x, y in points]
                    X = (6*Q)[0]-K(points[0][0])
                    N, D = primitive(X.numerator()), primitive(X.denominator())
                    assert D != 0
                    certificate = None
                    if N != 0:
                        for p in primes:
                            f = N.change_ring(GF(p))
                            if all(f(z) != 0 for z in GF(p)):
                                certificate = p
                                break
                    row.update({"status": "generic_identity" if N == 0 else
                                ("excluded_all_integer_t" if certificate else "unresolved"),
                                "numerator_degree": int(N.degree()),
                                "denominator_degree": int(D.degree()),
                                "numerator_coefficients": [str(c) for c in N.list()],
                                "denominator_coefficients": [str(c) for c in D.list()],
                                "certificate_prime": certificate})
                    E0 = EllipticCurve(QQ, coefficients(points, QQ(k1), QQ(k2), QQ(base3)))
                    P0, Q0 = [E0(QQ(x), QQ(y)) for x, y in points]
                    row["center_relation"] = bool(P0+6*Q0 == E0(0))
                    row["center_local_7"] = local(E0, points, 7)
                    row["center_local_11"] = local(E0, points, 11)
                    assert row["center_local_7"] == (4, True)
                    assert row["center_local_11"] == (8, True)
                except Exception as exc:
                    row["status"] = "producer_failure"
                    row["error"] = repr(exc)
                counts[row["status"]] += 1
                if row.get("certificate_prime"):
                    prime_counts[row["certificate_prime"]] += 1
                line = (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode()
                assert len(line) < 48000
                (outdir / f"{rownum:05d}.jsonl").write_bytes(line)
                raw_hash.update(line)
                event("cell", pair_index=pair_index, u=u, v=v, status=row["status"],
                      certificate_prime=row.get("certificate_prime"))
                rownum += 1
    event("finished", counts=dict(counts))
    print(json.dumps({"schema": "xedni-bridge6-pair-grid-result-v1",
                      "status": "PASS" if not counts["producer_failure"] else "PARTIAL",
                      "cells": rownum, "counts": dict(counts),
                      "certificate_prime_counts": dict(sorted(prime_counts.items())),
                      "rows_raw_sha256": raw_hash.hexdigest(),
                      "source_outcomes_sha256": sha(source_raw),
                      "source_pair_shifts_rows_sha256": sha(pair_raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
