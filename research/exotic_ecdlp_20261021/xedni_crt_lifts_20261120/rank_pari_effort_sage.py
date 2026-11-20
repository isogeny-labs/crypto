"""Proof-enabled PARI rank at the frozen increased effort."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent


def main():
    protocol = json.loads((HERE / "rank-pari-effort-protocol.json").read_text())
    E = EllipticCurve(QQ, [QQ(v) for v in protocol["curve_coefficients"]])
    rank = E.rank(use_database=False, proof=True, algorithm="pari",
                  only_use_mwrank=True, pari_effort=3)
    print(json.dumps({"status": "complete", "rank": int(rank),
                      "algorithm": "pari", "pari_effort": 3, "proof": True}, sort_keys=True))


if __name__ == "__main__":
    main()
