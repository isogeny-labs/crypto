"""Proof-enabled Sage rank call on the frozen compact CRT witness."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent


def main():
    protocol = json.loads((HERE / "rank-protocol.json").read_text())
    E = EllipticCurve(QQ, [QQ(v) for v in protocol["curve_coefficients"]])
    rank = E.rank(use_database=False, proof=True, algorithm="pari", only_use_mwrank=True)
    print(json.dumps({"status": "complete", "rank": int(rank),
                      "algorithm": "pari", "proof": True}, sort_keys=True))


if __name__ == "__main__":
    main()
