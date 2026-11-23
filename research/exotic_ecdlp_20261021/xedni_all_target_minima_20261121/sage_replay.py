"""Independent checked-Sage replay of all 42 target minimum certificates."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ


HERE = Path(__file__).resolve().parent


def main():
    report = json.loads((HERE / "result.json").read_text())
    curve = EllipticCurve(QQ, [0, -47, 0, 751, -3997])
    q = curve(17, 10)
    reduced_curve = curve.change_ring(GF(47))
    qbar = reduced_curve(17, 10)
    assert reduced_curve.order() == qbar.order() == 43
    heights = {}
    for row in report["finite_multiples"]:
        n = row["abs_n"]
        point = n * q
        x, y = point.xy()
        numerator = ZZ(x.numerator())
        denominator = ZZ(x.denominator())
        b = denominator.isqrt()
        assert b*b == denominator and abs(numerator.gcd(b)) == 1
        H = max(abs(numerator), b)
        assert str(H) == row["H"]
        assert H.nbits() == row["H_bits"]
        assert hashlib.sha256(str(H).encode()).hexdigest() == row["H_sha256"]
        assert hashlib.sha256(str(x).encode()).hexdigest() == row["x_sha256"]
        assert denominator % 47 and ZZ(y.denominator()) % 47
        reduced = reduced_curve(GF(47)(x), GF(47)(y))
        assert reduced == n * qbar
        assert [int(reduced[0]), int(reduced[1])] == row["reduction"]
        neg = -reduced
        assert [int(neg[0]), int(neg[1])] == row["negative_reduction"]
        heights[n] = H
    assert sorted(heights) == list(range(1, 32))
    for item in report["targets"]:
        residue = item["residue"]
        candidates = [n for n in range(-31, 32) if n and n % 43 == residue]
        best = min(candidates, key=lambda n: heights[abs(n)])
        assert best == item["minimizing_n"]
        assert [n for n in candidates if heights[abs(n)] == heights[abs(best)]] == [best]
        assert item["H"] == str(heights[abs(best)])
        reduced = best * qbar
        assert [int(reduced[0]), int(reduced[1])] == item["reduction"]
    assert len(report["targets"]) == 42
    print(json.dumps({"schema": "xedni-all-target-minima-sage-replay-v1",
                      "status": "PASS", "finite_multiples_checked": 31,
                      "public_targets_checked": 42,
                      "full_group_order": int(reduced_curve.order()),
                      "worst_minimum_H_bits": max(heights[n].nbits() for n in range(1, 22))},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
