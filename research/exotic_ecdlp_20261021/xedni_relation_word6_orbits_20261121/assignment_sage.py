"""Map the Lean scalar ambiguity onto the saved coefficient-six curve rows."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def xy(P):
    return int(P[0]), int(P[1])


def main():
    protocol = json.loads((HERE / "assignment-protocol.json").read_text())
    result_bytes = (HERE / "result.json").read_bytes()
    assert sha(result_bytes) == protocol["source_result_sha256"]
    result = json.loads(result_bytes)
    raw = gzip.decompress((HERE / "rows.jsonl.gz").read_bytes())
    assert sha(raw) == result["rows_raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    E = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    assert E.cardinality() == 43
    points = sorted((P for P in E.points() if P != E(0)), key=xy)
    G = points[0]
    scalar = {xy(i*G): i for i in range(1, 43)}
    assert len(scalar) == 42
    orbit2 = {2*pow(-6, i, 43) % 43 for i in range(6)}
    assert len(orbit2) == 6 and 1 not in orbit2 and 2 in orbit2

    def actual(P):
        return scalar[xy(P)]

    def alternative(P):
        k = actual(P)
        return (k + (k if k in orbit2 else 0)) % 43

    for P in points:
        assert actual(-P) == (-actual(P)) % 43
        assert alternative(-P) == (-alternative(P)) % 43
    assert actual(G) == alternative(G) == 1
    assert actual(2*G) == 2 and alternative(2*G) == 4
    col = {tuple(P): j for j, P in enumerate(result["folded_column_points"])}
    checked = 0
    for row in rows:
        P = E(*[GF(47)(x) for x in row["P"]])
        Q = E(*[GF(47)(x) for x in row["Q"]])
        assert P+6*Q == E(0)
        assert (actual(P)+6*actual(Q)) % 43 == 0
        assert (alternative(P)+6*alternative(Q)) % 43 == 0
        for assignment in (actual, alternative):
            values = []
            for coords in result["folded_column_points"]:
                point = E(*[GF(47)(x) for x in coords])
                values.append(assignment(point))
            assert sum(c*v for c, v in zip(row["folded_row"], values)) % 43 == 0
        checked += 1
    assert checked == 42
    print(json.dumps({"schema": "xedni-word6-anchored-assignment-result-v1",
                      "status": "PASS", "generator": list(xy(G)),
                      "double_generator": list(xy(2*G)),
                      "orbit2_scalars": sorted(orbit2),
                      "known_generator_values": [actual(G), alternative(G)],
                      "double_generator_values": [actual(2*G), alternative(2*G)],
                      "rows_checked": checked, "rows_raw_sha256": sha(raw)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
