#!/usr/bin/env python3
"""Independent bit-field replay of the finite census and named subgroup points."""
import json
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
MOD = 0x0EB  # w^12 + w^7 + w^6 + w^5 + w^3 + w + 1


def mul(a, b):
    out = 0
    for _ in range(12):
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & 0x1000:
            a ^= 0x1000 | MOD
    return out


def powf(a, n):
    out = 1
    while n:
        if n & 1:
            out = mul(out, a)
        a = mul(a, a)
        n >>= 1
    return out


def inv(a):
    assert a
    return powf(a, 4094)


def trace(a):
    out = 0
    for _ in range(12):
        out ^= a
        a = mul(a, a)
    assert out in (0, 1)
    return out


def on_curve(P, a, b):
    if P is None:
        return True
    x, y = P
    return mul(y, y) ^ mul(x, y) == mul(mul(x, x), x) ^ mul(a, mul(x, x)) ^ b


def add(P, Q, a):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if y1 ^ y2 == x1:
            return None
        assert x1 and y1 == y2
        slope = x1 ^ mul(y1, inv(x1))
        x3 = mul(slope, slope) ^ slope ^ a
        y3 = mul(x1, x1) ^ mul(slope ^ 1, x3)
    else:
        slope = mul(y1 ^ y2, inv(x1 ^ x2))
        x3 = mul(slope, slope) ^ slope ^ x1 ^ x2 ^ a
        y3 = mul(slope, x1 ^ x3) ^ x3 ^ y1
    return x3, y3


def scale(P, n, a):
    out = None
    while n:
        if n & 1:
            out = add(out, P, a)
        P = add(P, P, a)
        n >>= 1
    return out


def check_point_pair(a, b, r, g, q):
    G, Q = tuple(g), tuple(q)
    assert on_curve(G, a, b) and on_curve(Q, a, b)
    assert G != Q and scale(G, r, a) is None
    assert scale(G, 7, a) == Q


class Quadratic:
    """K[t]/(t^2+t+s), used only for an actual nonsplit kernel point."""

    def __init__(self, s):
        self.s = s

    def plus(self, x, y):
        return x[0] ^ y[0], x[1] ^ y[1]

    def times(self, x, y):
        a, b = x
        c, d = y
        bd = mul(b, d)
        return mul(a, c) ^ mul(bd, self.s), mul(a, d) ^ mul(b, c) ^ bd

    def inverse(self, x):
        a, b = x
        norm = mul(a, a) ^ mul(a, b) ^ mul(mul(b, b), self.s)
        assert norm
        k = inv(norm)
        return mul(a ^ b, k), mul(b, k)

    def add_points(self, P, Q, a):
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if self.plus(y1, y2) == x1:
                return None
            assert x1 != (0, 0) and y1 == y2
            slope = self.plus(x1, self.times(y1, self.inverse(x1)))
            x3 = self.plus(self.plus(self.times(slope, slope), slope), (a, 0))
            y3 = self.plus(self.times(x1, x1), self.times(self.plus(slope, (1, 0)), x3))
        else:
            slope = self.times(self.plus(y1, y2), self.inverse(self.plus(x1, x2)))
            x3 = self.plus(self.plus(self.plus(self.times(slope, slope), slope), x1),
                           self.plus(x2, (a, 0)))
            y3 = self.plus(self.plus(self.times(slope, self.plus(x1, x3)), x3), y1)
        return x3, y3


def verify_velu(w):
    """Independently evaluate the 3-isogeny at named source points."""
    a, b, alpha = 32, w["source_b"], w["kernel_x"]
    rhs = mul(mul(alpha, alpha), alpha) ^ mul(a, mul(alpha, alpha)) ^ b
    rational_y = next((y for y in range(4096) if mul(y, y) ^ mul(alpha, y) == rhs), None)
    if rational_y is None:
        s = mul(rhs, inv(mul(alpha, alpha)))
        assert trace(s) == 1
        field = Quadratic(s)
        R = ((alpha, 0), (0, alpha))
    else:
        field = Quadratic(0)
        R = ((alpha, 0), (rational_y, 0))
    minus_R = (R[0], field.plus(R[1], R[0]))
    for key in ("G", "Q"):
        src = w["source_" + key]
        target = w["target_" + key]
        P = ((src[0], 0), (src[1], 0))
        plus = field.add_points(P, R, a)
        minus = field.add_points(P, minus_R, a)
        assert plus is not None and minus is not None
        # Vélu's formula in characteristic two, then y -> y + alpha.
        image_x = field.plus(field.plus(P[0], plus[0]), minus[0])
        image_y = field.plus(field.plus(P[1], plus[1]), minus[1])
        assert image_x == (target[0], 0), (w["order"], key, image_x, target)
        assert image_y == (target[1], 0), (w["order"], key, image_y, target)


def main():
    cert = json.loads((HERE / "certificate.json").read_text())
    assert trace(32) == 1
    assert trace(1) == 0
    fibers = defaultdict(int)
    for x in range(4096):
        x2 = mul(x, x)
        fibers[mul(x2, x2) ^ mul(x2, x)] += 1
    assert fibers[0] == 2
    patterns = Counter(fibers[b] for b in range(1, 4096))
    assert dict(patterns) == {0: 1024, 1: 2730, 4: 341}
    assert sorted(b for b in range(1, 4096) if fibers[b] == 4) == cert["scalar_b"]
    assert cert["summary"] == {
        "all_source_models": 8190, "scalar_three_source_models": 682,
        "all_one_step_classes": 28, "all_one_step_models": 2530,
        "residual_catalog_classes": 44, "residual_scalar_classes": 8,
        "residual_scalar_models": 177,
        "residual_one_step_classes": 8, "residual_one_step_models": 651,
    }
    for row in cert["exact_controls"]:
        check_point_pair(32, row["source_b"], row["subgroup_prime"],
                         row["source_G"], row["source_Q"])
        iso = row.get("isogeny_to_scalar_model")
        if iso:
            check_point_pair(32, iso["target_b"], row["subgroup_prime"],
                             iso["target_G"], iso["target_Q"])
    f = cert["new_focus_4198"]
    check_point_pair(f["a"], f["b"], f["subgroup_prime"], f["G"], f["Q"])
    assert len(cert["one_step_witnesses"]) == 8
    for w in cert["one_step_witnesses"]:
        assert w["order"] % 9 in (0, 4)
        assert w["source_b"] not in cert["scalar_b"]
        assert w["target_b"] in cert["scalar_b"]
        check_point_pair(32, w["source_b"], w["subgroup_prime"],
                         w["source_G"], w["source_Q"])
        check_point_pair(32, w["target_b"], w["subgroup_prime"],
                         w["target_G"], w["target_Q"])
        verify_velu(w)
    print("independent audit PASS", dict(patterns), "8 classes/651 one-step models")


if __name__ == "__main__":
    main()
