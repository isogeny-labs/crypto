"""Exact finite coefficient-choice screen under the frozen protocol."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from fractions import Fraction
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PROTOCOL = json.loads((HERE / "protocol.json").read_text())
SOURCE = HERE / "source-rows-v2.jsonl.gz"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def qstr(value):
    return str(value)


def incidence_rank(points, prime):
    rows = [tuple(v % prime for v in (y*y, x*y, y, x**3, x*x, x, 1))
            for x, y in points]
    return 1 if rows[0] == rows[1] else 2


def coefficients(points, ks):
    (x1, y1), (x2, y2) = [tuple(QQ(v) for v in p) for p in points]
    a1, a2, a3 = [47 * k for k in ks]
    d1 = y1*y1 + a1*x1*y1 + a3*y1 - x1**3 - a2*x1*x1
    d2 = y2*y2 + a1*x2*y2 + a3*y2 - x2**3 - a2*x2*x2
    if x1 == x2:
        return None
    a4 = (d1 - d2) / (x1 - x2)
    a6 = d1 - a4*x1
    return [QQ(a1), QQ(a2), QQ(a3), a4, a6]


def check_prime(E, coeffs, points, prime):
    if any(QQ(a).denominator() % prime == 0 for a in coeffs):
        return {"status": "nonintegral"}
    if E.discriminant().valuation(prime) != 0:
        return {"status": "bad_reduction"}
    Ep = E.change_ring(GF(prime))
    P, Q = [Ep((GF(prime)(x), GF(prime)(y))) for x, y in points]
    return {"status": "good", "order": int(Ep.cardinality()),
            "incidence_rank": incidence_rank(points, prime),
            "relation": bool(P + 2*Q == Ep(0))}


def main():
    source_bytes = SOURCE.read_bytes()
    assert digest(source_bytes) == PROTOCOL["source_sha256"]
    selected = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()
                if json.loads(line)["accepted"]]
    assert len(selected) == 20
    counts = Counter()
    first = {"strict": {}, "weak": {}}
    row_path = HERE / "rows.jsonl"
    with row_path.open("w") as out:
        for pair_index, prior in enumerate(selected):
            points = prior["lifted_points"]
            assert incidence_rank(points, 7) == 2
            for ks in itertools.product((-1, 0, 1), repeat=3):
                row = {"pair_index": pair_index, "k": ks,
                       "prior_reduced_relation": prior["reduced_relation"],
                       "prior_rational_relation": prior["rational_relation"]}
                coeffs = coefficients(points, ks)
                if coeffs is None:
                    row["status"] = "same_x"
                else:
                    row["coefficients"] = list(map(qstr, coeffs))
                    E = EllipticCurve(QQ, coeffs)
                    if not E.is_singular():
                        P, Q = [E(tuple(QQ(v) for v in xy)) for xy in points]
                        row["rational_relation"] = bool(P + 2*Q == E(0))
                        row["p47"] = check_prime(E, coeffs, points, 47)
                        row["p7"] = check_prime(E, coeffs, points, 7)
                        row["p11"] = check_prime(E, coeffs, points, 11)
                        assert row["p47"]["status"] == "good"
                        assert row["p47"]["order"] == 43
                        assert row["p47"]["relation"] == prior["reduced_relation"]
                        gate7 = row["p7"]["status"] == "good" and row["p7"]["incidence_rank"] == 2 and row["p7"]["order"] == 4
                        for arm, order in (("strict", 6), ("weak", 7)):
                            ok = gate7 and row["p11"]["status"] == "good" and row["p11"]["incidence_rank"] == 2 and row["p11"]["order"] == order
                            row[arm + "_accepted"] = ok
                            if ok and pair_index not in first[arm]:
                                first[arm][pair_index] = {"k": ks, "reduced_relation": prior["reduced_relation"], "rational_relation": row["rational_relation"]}
                            counts[arm + "_candidate_accept"] += int(ok)
                        row["status"] = "good_curve"
                    else:
                        row["status"] = "singular"
                counts[row["status"]] += 1
                if "p7" in row:
                    counts["p7_" + row["p7"]["status"]] += 1
                    counts["p11_" + row["p11"]["status"]] += 1
                    counts["rational_relation"] += int(row["rational_relation"])
                out.write(json.dumps(row, sort_keys=True) + "\n")
    summary = {"schema": "xedni-coefficient-choice-result-v1", "status": "PASS",
               "source_sha256": digest(source_bytes),
               "protocol_sha256": digest((HERE / "protocol.json").read_bytes()),
               "script_sha256": digest((HERE / "run_sage.py").read_bytes()),
               "rows_sha256": digest(row_path.read_bytes()),
               "pair_count": len(selected), "candidate_count": sum(counts[x] for x in ("same_x", "singular", "good_curve")),
               "counts": dict(counts),
               "selected": {arm: {"pairs": len(rows),
                                  "reduced_events": sum(v["reduced_relation"] for v in rows.values()),
                                  "rational_events": sum(v["rational_relation"] for v in rows.values()),
                                  "first_by_pair": rows} for arm, rows in first.items()}}
    print(json.dumps(summary, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
