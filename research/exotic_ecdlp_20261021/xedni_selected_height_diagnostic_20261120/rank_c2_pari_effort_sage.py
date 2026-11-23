#!/usr/bin/env python3
"""Proof-enabled PARI rank with bounded extra effort on frozen curve C2."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ


ROOT = Path(__file__).resolve().parent


def main():
    fixtures = json.loads((ROOT / "fixtures.json").read_text())
    item, = (x for x in fixtures["curves"] if x["id"] == "C2")
    curve = EllipticCurve(QQ, [QQ(x) for x in item["weierstrass_coefficients"]])
    rank = curve.rank(use_database=False, proof=True, algorithm="pari",
                      only_use_mwrank=True, pari_effort=3)
    print(json.dumps({"id": "C2", "algorithm": "pari", "pari_effort": 3,
                      "rank": int(rank), "status": "proved_by_sage"}, sort_keys=True))


if __name__ == "__main__":
    main()
