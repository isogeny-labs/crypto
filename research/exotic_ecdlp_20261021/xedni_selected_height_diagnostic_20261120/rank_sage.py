#!/usr/bin/env python3
"""One capped rank-bound computation on a frozen selected curve."""

import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, QQ


ROOT = Path(__file__).resolve().parent


def main():
    curve_id = sys.argv[1]
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    item, = (x for x in fixtures["curves"] if x["id"] == curve_id)
    curve = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
    points = [curve([QQ(x), QQ(y)]) for x, y in item["points"]]
    lower, upper = curve.rank_bounds(known_points=points, limbigprime=0)
    print(json.dumps({"id": curve_id, "status": "complete",
                      "rank_lower": int(lower), "rank_upper": int(upper)},
                     sort_keys=True))


if __name__ == "__main__":
    main()
