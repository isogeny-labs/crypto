"""Independent 4-bit field replay of four exact quotient identities and orders."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
CERT = HERE / "certificate.json"
MODELS = HERE.parent / "ghs_q16_4126_hess_g4_20261027/EXACT_MODELS.md"
EXPECTED = {
    4006: ([3, 14, 7, 0, 5], [3, 9, 4, 14, 1, 10, 0, 0, 3, 6], 88132, 22),
    4054: ([10, 13, 0, 0, 5], [7, 14, 11, 6, 7, 0, 0, 0, 15, 7], 56756, 14),
    4078: ([10, 1, 9, 0, 5], [6, 9, 5, 13, 14, 14, 0, 0, 1, 9], 57092, 14),
    4126: ([3, 1, 8, 0, 5], [15, 15, 2, 7, 0, 2, 0, 0, 13, 14], 57764, 14),
}


def mul(a, b):
    result = 0
    while b:
        if b & 1:
            result ^= a
        b >>= 1
        a <<= 1
        if a & 16:
            a ^= 0x13  # a^4+a+1
    return result


def add_poly(p, q):
    out = [0] * max(len(p), len(q))
    for i, c in enumerate(p):
        out[i] ^= c
    for i, c in enumerate(q):
        out[i] ^= c
    return out


def mul_poly(p, q):
    out = [0] * (len(p) + len(q) - 1)
    for i, c in enumerate(p):
        for j, d in enumerate(q):
            out[i+j] ^= mul(c, d)
    return out


def compose_poly(p, q):
    out = [0]
    for c in reversed(p):
        out = add_poly(mul_poly(out, q), [c])
    return out


def trim(p):
    p = list(p)
    while len(p) > 1 and p[-1] == 0:
        p.pop()
    return p


def elliptic_count(a2, a4, a6):
    count = 1  # unique point at infinity
    for x in range(16):
        rhs = mul(mul(x, x), x) ^ mul(a2, mul(x, x)) ^ mul(a4, x) ^ a6
        for y in range(16):
            if mul(y, y) ^ mul(x, y) == rhs:
                count += 1
    return count


def main():
    cert = json.loads(CERT.read_text())
    assert cert["model_ledger_sha256"] == hashlib.sha256(MODELS.read_bytes()).hexdigest()
    assert cert["constructor_sha256"] == hashlib.sha256((HERE / "construct.py").read_bytes()).hexdigest()
    assert {r["source_order"] for r in cert["records"]} == set(EXPECTED)
    for row in cert["records"]:
        order = row["source_order"]
        h, f, jac_order, expected_quotient = EXPECTED[order]
        assert row["h"] == h and row["f"] == f
        assert row["hess_jacobian_order"] == jac_order
        assert row["L"] == [0] + h[1:]
        assert row["L"][1] != 0
        B, F, L = row["B"], row["F"], row["L"]
        lhs = add_poly(add_poly(f, mul_poly(B, B)), mul_poly(h, B))
        rhs = compose_poly(F, L)
        assert trim(lhs) == trim(rhs)
        assert F[3] != 0
        shifted = compose_poly(F, [h[0], 1])
        c3 = shifted[3]
        a2, a4, a6 = shifted[2], mul(c3, shifted[1]), mul(mul(c3, c3), shifted[0])
        assert row["elliptic_weierstrass_a"] == [1, a2, 0, a4, a6]
        assert a6 != 0
        count = elliptic_count(a2, a4, a6)
        assert count == row["elliptic_order"] == expected_quotient
        assert jac_order == order * count
        assert row["elliptic_trace"] == 17 - count
        assert row["translation_kernel_size_over_GF4096"] == 4
    print("PASS: four independent GF(16) map identities, elliptic equations, and point counts")


if __name__ == "__main__":
    main()
