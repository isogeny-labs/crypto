#!/usr/bin/env python3
"""Exact GF(3) arithmetic diagnostic for a degree-two Mumford support.

This is a finite polynomial check, not a Lean theorem or a DLP experiment.
Coefficients are low-to-high and reduced modulo 3.
"""

P = 3


def trim(a):
    a = [x % P for x in a]
    while a and a[-1] == 0:
        a.pop()
    return a


def mul(a, b):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % P
    return trim(out)


def derivative(a):
    return trim([(i * a[i]) % P for i in range(1, len(a))])


def remainder(a, b):
    a, b = trim(a), trim(b)
    inv = pow(b[-1], -1, P)
    while len(a) >= len(b):
        shift = len(a) - len(b)
        lead = a[-1] * inv % P
        for i, x in enumerate(b):
            a[i + shift] = (a[i + shift] - lead * x) % P
        a = trim(a)
    return a


def monic_gcd(a, b):
    while b:
        a, b = b, remainder(a, b)
    return [(x * pow(a[-1], -1, P)) % P for x in a]


def eval_at(a, x):
    return sum(c * pow(x, i, P) for i, c in enumerate(a)) % P


def main():
    u = [1, 0, 1]             # first support X^2 + 1
    u2 = [2, 1, 1]            # second support X^2 + X + 2
    h = mul([2, 1], u2)       # (X + 2) * u2 = X^3 + X + 1
    f = mul(u, h)             # X^5 + 2X^3 + X^2 + X + 1
    values = [eval_at(u, x) for x in range(P)]
    values2 = [eval_at(u2, x) for x in range(P)]
    gcd = monic_gcd(f, derivative(f))
    assert values == [1, 2, 2]
    assert values2 == [2, 1, 2]
    assert h == [1, 1, 0, 1]
    assert f == [1, 1, 1, 2, 0, 1]
    assert remainder(f, u) == []
    assert remainder(f, u2) == []
    assert gcd == [1]
    q1, r1, q2, r2 = 0, 1, 1, 2
    b = ((q1 - q2) * (q2 * r1 - q1 * r2) - (r1 - r2) ** 2) % P
    assert b == 1
    support = [1, 2]         # -(X + 2) = 2X + 1
    assert mul(mul(u, u2), support) == [(2 * x) % P for x in f]
    print(f"u_values={values}")
    print(f"u2_values={values2}")
    print(f"f_coeff_low_to_high={f}")
    print(f"gcd_f_derivative={gcd}")
    print("both_supports_divide_f=True")
    print("both_Mumford_pairs_v0_valid=True")
    print(f"source_common22_determinant_b={b}")
    print("norm_identity_n0_support_2X_plus_1=True")
    print("rational_x_support_count_for_each=0")


if __name__ == "__main__":
    main()
