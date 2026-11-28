"""Independent Sage replay of the frozen rational and reduced point census."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ


HERE = Path(__file__).resolve().parent


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    assert protocol["curve_a_invariants"] == ["0", "-47", "0", "751", "-3997"]
    curve = EllipticCurve(QQ, [0, -47, 0, 751, -3997])
    q = curve(17, 10)
    finite = curve.change_ring(GF(47))
    qbar = finite(17, 10)
    assert qbar.order() == 43
    assert finite.order() == 43
    checked = []
    for row in result["rows"]:
        n = row["abs_n"]
        point = n * q
        x, y = point.xy()
        xnum, xden = ZZ(x.numerator()), ZZ(x.denominator())
        b = xden.isqrt()
        assert b*b == xden and xnum.gcd(b).abs() == 1
        height = max(abs(xnum), b)
        assert str(height) == row["H"]
        assert height.nbits() == row["H_bits"]
        assert hashlib.sha256(str(height).encode()).hexdigest() == row["H_sha256"]
        assert hashlib.sha256(str(x).encode()).hexdigest() == row["x_sha256"]
        assert xden.nbits() == row["x_denominator_bits"]
        assert xden % 47 != 0 and ZZ(y.denominator()) % 47 != 0
        reduced = finite(GF(47)(x), GF(47)(y))
        assert reduced == n * qbar
        assert [int(reduced[0]), int(reduced[1])] == row["reduction"]
        negative = -reduced
        assert [int(negative[0]), int(negative[1])] == row["negative_reduction"]
        assert bool(height <= 2**130) == row["under_cap"]
        checked.append(n)
    assert checked == list(range(1, 21))
    assert sum(row["under_cap"] for row in result["rows"]) == 13
    print(json.dumps({"schema": "xedni-height-coverage-sage-replay-v1",
                      "status": "PASS", "checked_abs_multipliers": checked,
                      "finite_group_order": int(finite.order()),
                      "reduced_generator_order": int(qbar.order())},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
