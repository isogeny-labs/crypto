"""Independent Sage replay of all selected full-law candidates."""

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
    summary = json.loads((HERE / "result.json").read_text())
    pair_bytes = (HERE / "pair-rows.jsonl").read_bytes()
    assert summary["pair_rows_sha256"] == sha(pair_bytes)
    pair_rows = [json.loads(line) for line in pair_bytes.splitlines()]
    source = [json.loads(line) for line in gzip.decompress((PRIOR / "source-rows-v2.jsonl.gz").read_bytes()).splitlines()]
    checked = 0
    unique = set()
    for pair, prior in zip(pair_rows, source):
        for arm, choice in pair["selected"].items():
            if choice is None:
                continue
            checked += 1
            a = [QQ(x) for x in choice["coefficients"]]
            E = EllipticCurve(QQ, a)
            P, Q = [E(tuple(QQ(v) for v in xy)) for xy in prior["lifted_points"]]
            assert bool(P+2*Q == E(0)) == choice["rational_relation"]
            assert E.discriminant() != 0
            for p, target in ((47, 43), (7, 4), (11, 6 if arm == "strict" else 7)):
                assert all(x.denominator() % p for x in a)
                assert E.discriminant().valuation(p) == 0
                Ep = E.change_ring(GF(p))
                Pp, Qp = [Ep((GF(p)(xy[0]), GF(p)(xy[1]))) for xy in prior["lifted_points"]]
                assert int(Ep.cardinality()) == target
                rel = bool(Pp+2*Qp == Ep(0))
                if p == 47:
                    assert rel == prior["reduced_relation"]
                else:
                    assert rel == choice["p"+str(p)+"_relation"]
            unique.add((pair["pair_index"], tuple(choice["k"])))
    assert checked == summary["counts"]["strict_selected_pairs"] + summary["counts"]["weak_selected_pairs"]
    print(json.dumps({"status": "PASS", "selected_arm_rows": checked,
                      "unique_pair_choice_rows": len(unique),
                      "pair_rows_sha256": sha(pair_bytes)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
