"""Independent exact-coordinate replay in the accepted Sage runtime."""

import hashlib
import json
import math
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent


def main():
    result = json.loads((HERE / "result.json").read_text())
    assert result["status"] == "PASS"
    E = EllipticCurve(QQ, [0, -47, 0, 751, -3997])
    Q = E([17, 10])
    E0 = E.change_ring(GF(47))
    T = E0([10, 1])
    rows = []
    for row in result["finite_representatives"]:
        n = row["n"]
        P = n*Q
        x, y = P.xy()
        assert E0([GF(47)(x), GF(47)(y)]) == T
        a = int(x.numerator())
        d = int(x.denominator())
        b = math.isqrt(d)
        assert b*b == d
        H = max(abs(a), b)
        assert H == int(row["H"])
        assert H.bit_length() == row["H_bits"]
        assert hashlib.sha256(str(H).encode()).hexdigest() == row["H_sha256"]
        rows.append({"n": n, "H_bits": H.bit_length(), "H_sha256": row["H_sha256"]})
    print(json.dumps({"schema": "xedni-minimum-lift-height-sage-replay-v1",
                      "status": "PASS", "rows": rows}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
