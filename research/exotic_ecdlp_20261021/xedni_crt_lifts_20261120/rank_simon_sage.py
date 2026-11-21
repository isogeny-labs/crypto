"""Simon two-descent with the selected known rational point."""

import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE = Path(__file__).resolve().parent


def main():
    protocol = json.loads((HERE / "rank-simon-protocol.json").read_text())
    E = EllipticCurve(QQ, [QQ(v) for v in protocol["curve_coefficients"]])
    Q = E([QQ(v) for v in protocol["known_point"]])
    lower, upper, witnesses = E.simon_two_descent(known_points=[Q], limbigprime=0)
    print(json.dumps({"status": "complete", "rank_lower": int(lower),
                      "rank_upper": int(upper),
                      "witnesses": [[str(P[0]), str(P[1])] for P in witnesses]}, sort_keys=True))


if __name__ == "__main__":
    main()
