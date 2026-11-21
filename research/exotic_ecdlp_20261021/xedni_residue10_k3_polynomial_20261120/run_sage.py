"""Certify integer k3 shifts of reduced-compatible coefficient-10 classes."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, QQ, ZZ, lcm, prime_range

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CHECKS = ROOT / "xedni_residue10_rational_20261120/rational-checks.jsonl.gz"
POINTS = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def packed_rows(rows):
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n"
                  for row in rows).encode()
    return raw, gzip.compress(raw, mtime=0)


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
    p = json.loads((HERE / "protocol.json").read_text())
    checks_bytes, points_bytes = CHECKS.read_bytes(), POINTS.read_bytes()
    assert sha(checks_bytes) == p["source_checks_sha256"]
    assert sha(points_bytes) == p["point_source_sha256"]
    checks = [json.loads(x) for x in gzip.decompress(checks_bytes).splitlines()]
    points = [json.loads(x) for x in gzip.decompress(points_bytes).splitlines()]
    assert len(checks) == 1488 and len(points) == 1722
    e47 = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    selection = []
    compatible = []
    for position, row in enumerate(checks):
        src = points[row["pair_index"]]["lifted_points"]
        P, Q = [e47(GF(47)(x), GF(47)(y)) for x, y in src]
        selected = bool(P+10*Q == e47(0))
        selection.append({"source_position": position, "pair_index": row["pair_index"],
                          "local_indices": row["local_indices"],
                          "reduced_compatible": selected})
        if selected:
            compatible.append(row)
    assert len(compatible) == p["expected_selected"]
    R = PolynomialRing(QQ, "t")
    t = R.gen()
    K = R.fraction_field()
    primes = list(prime_range(13, 258))
    output = []
    counts = Counter()
    for row in compatible:
        src = points[row["pair_index"]]["lifted_points"]
        k1, k2, k3 = row["crt_shifts"]
        record = {"pair_index": row["pair_index"], "local_indices": row["local_indices"],
                  "centered_k": [k1, k2, k3], "points": src, "status": None}
        try:
            centered = coefficients(src, k1, k2, k3)
            assert list(map(str, centered)) == row["curve_coefficients"]
            a = coefficients(src, k1, k2, K(k3+77*t))
            E = EllipticCurve(K, [K(c) for c in a])
            assert E.discriminant() != 0
            (x1, y1), (x2, y2) = src
            P, Q = E(K(x1), K(y1)), E(K(x2), K(y2))
            X = (10*Q)[0]-K(x1)
            numerator = primitive(X.numerator())
            denominator = primitive(X.denominator())
            assert numerator != 0 and denominator != 0
            certificate = None
            for prime in primes:
                f = numerator.change_ring(GF(prime))
                if not any(f(z) == 0 for z in GF(prime)):
                    certificate = int(prime)
                    break
            record.update({"numerator_degree": int(numerator.degree()),
                           "denominator_degree": int(denominator.degree()),
                           "numerator_coefficients": [str(c) for c in numerator.list()],
                           "denominator_coefficients": [str(c) for c in denominator.list()],
                           "certificate_prime": certificate,
                           "status": "all_integer_shifts_excluded" if certificate else "unresolved"})
        except Exception as exc:
            record.update({"status": "producer_failure", "error": repr(exc)})
        output.append(record)
        counts[record["status"]] += 1
    select_raw, select_packed = packed_rows(selection)
    rows_raw, rows_packed = packed_rows(output)
    (HERE / "selection.jsonl.gz").write_bytes(select_packed)
    row_dir = HERE / "polynomial-rows"
    row_dir.mkdir(exist_ok=True)
    row_lines = rows_raw.splitlines(keepends=True)
    assert len(row_lines) == len(output)
    for i, line in enumerate(row_lines):
        (row_dir / f"{i:03d}.jsonl").write_bytes(line)
    print(json.dumps({"schema": "xedni-residue10-k3-polynomial-result-v1",
                      "status": "PASS" if not counts["producer_failure"] else "PARTIAL",
                      "source_checks_sha256": sha(checks_bytes),
                      "point_source_sha256": sha(points_bytes),
                      "screened": len(selection), "selected": len(output),
                      "counts": dict(counts), "selection_raw_sha256": sha(select_raw),
                      "selection_packed_sha256": sha(select_packed),
                      "polynomial_rows_raw_sha256": sha(rows_raw),
                      "polynomial_rows_packed_sha256": sha(rows_packed)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
