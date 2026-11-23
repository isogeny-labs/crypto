"""Independent Sage replay of every K=2 selected curve and point sum."""

import gzip
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    packed = (HERE / "pair-rows.jsonl.gz").read_bytes()
    assert sha(packed) == result["packed_rows_sha256"]
    rows = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    source = [json.loads(x) for x in gzip.decompress((PRIOR / "source-rows-v2.jsonl.gz").read_bytes()).splitlines()]
    arm_rows = 0
    for row, input_row in zip(rows, source):
        for arm, pick in row["selected"]["2"].items():
            if pick is None:
                continue
            arm_rows += 1
            a = [QQ(x) for x in pick["coefficients"]]
            E = EllipticCurve(QQ, a)
            P, Q = [E(tuple(QQ(v) for v in xy)) for xy in input_row["lifted_points"]]
            assert bool(P+2*Q == E(0)) == pick["rational_relation"]
            for prime, order in ((47, 43), (7, 4), (11, 6 if arm == "strict" else 7)):
                assert all(x.denominator() % prime for x in a)
                assert E.discriminant().valuation(prime) == 0
                Ep = E.change_ring(GF(prime))
                Pp, Qp = [Ep((GF(prime)(xy[0]), GF(prime)(xy[1]))) for xy in input_row["lifted_points"]]
                assert int(Ep.cardinality()) == order
                local = bool(Pp+2*Qp == Ep(0))
                if prime == 47:
                    assert local == input_row["reduced_relation"]
                else:
                    assert local == pick["p"+str(prime)+"_relation"]
    assert arm_rows == result["counts"]["2"]["strict_selected_pairs"] + result["counts"]["2"]["weak_selected_pairs"]
    print(json.dumps({"status": "PASS", "k2_selected_arm_rows": arm_rows,
                      "packed_rows_sha256": sha(packed)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
