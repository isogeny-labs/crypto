#!/usr/bin/env python3
"""Check C2 Simon rank bounds against equivalent known-point lists."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ


ROOT = Path(__file__).resolve().parent


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    item, = (x for x in fixtures["curves"] if x["id"] == "C2")
    E = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
    P = E([QQ(-20), QQ(14)])
    Q = E([QQ(11), QQ(7)])
    four = [E([QQ(x), QQ(y)]) for x, y in item["points"]]
    assert set(four) == {P, -P, Q, -Q}
    arms = (("none", []), ("one", [P]), ("two", [P, Q]),
            ("four_with_inverses", four))
    rows = []
    for name, points in arms:
        try:
            lower, upper, witnesses = E.simon_two_descent(
                known_points=points, limbigprime=0)
            rows.append({"arm": name, "status": "complete", "known_count": len(points),
                         "rank_lower": int(lower), "rank_upper": int(upper),
                         "returned_points": [[str(R[0]), str(R[1])] for R in witnesses]})
        except Exception as exc:
            rows.append({"arm": name, "status": "error", "known_count": len(points),
                         "error": repr(exc)})
    print(json.dumps({"schema": "xedni-selected-c2-simon-perturb-v1",
                      "curve_id": "C2", "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
