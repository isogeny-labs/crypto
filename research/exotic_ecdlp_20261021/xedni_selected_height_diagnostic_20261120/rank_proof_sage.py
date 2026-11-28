#!/usr/bin/env python3
"""Proof-enabled rational Mordell-Weil rank on one frozen curve."""

import json
import sys
from pathlib import Path

from sage.all import EllipticCurve, QQ


ROOT = Path(__file__).resolve().parent


def main():
    curve_id, algorithm = sys.argv[1:]
    assert algorithm in ("pari", "mwrank_lib")
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    item, = (x for x in fixtures["curves"] if x["id"] == curve_id)
    curve = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
    rank = curve.rank(use_database=False, proof=True, algorithm=algorithm,
                      only_use_mwrank=True)
    print(json.dumps({"id": curve_id, "algorithm": algorithm,
                      "rank": int(rank), "status": "proved_by_sage"}, sort_keys=True))


if __name__ == "__main__":
    main()
