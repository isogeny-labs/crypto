"""Checked-Sage replay of every first CRT representative."""

import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent


def main():
    rows = [json.loads(line) for path in sorted((HERE / "outcomes").glob("[0-9][0-9][0-9].jsonl"))
            for line in path.read_text().splitlines()]
    checked = []
    for row in rows:
        if row["status"] != "local_bridge":
            continue
        E = EllipticCurve(QQ, [QQ(x) for x in row["crt_coefficients"]])
        P, Q = [E(QQ(x), QQ(y)) for x, y in row["points"]]
        rational = bool(P+6*Q == E(0))
        assert rational == row["crt_rational_relation"]
        local = {}
        for p, desired in ((7, 4), (11, 8)):
            Ep = E.change_ring(GF(p))
            Pp, Qp = [Ep(GF(p)(x), GF(p)(y)) for x, y in row["points"]]
            local[str(p)] = {"order": int(Ep.cardinality()),
                             "relation": bool(Pp+6*Qp == Ep(0))}
            assert local[str(p)] == {"order": desired, "relation": True}
        checked.append({"pair_index": row["pair_index"],
                        "rational_relation": rational,
                        "discriminant_nonzero": bool(E.discriminant() != 0),
                        "local": local})
    assert len(checked) == 22
    print(json.dumps({"schema": "xedni-bridge6-source-policy-sage-audit-v1",
                      "status": "PASS", "checked_crt_curves": len(checked),
                      "rational_relations": sum(x["rational_relation"] for x in checked),
                      "rows": checked}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
