"""Finite control for the square-generator and doubling-norm criterion.

Run: python3 research/p192_ray_character_20261123/doubling_norm_control.py
The order-97 curves are fixed; the P-192 checks here are scalar arithmetic only.
"""

from math import gcd


def add(left, right, p, a):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    u, v = right
    if x == u and (y + v) % p == 0:
        return None
    slope = (
        (3 * x * x + a) * pow(2 * y, -1, p)
        if left == right
        else (v - y) * pow(u - x, -1, p)
    ) % p
    x3 = (slope * slope - x - u) % p
    return x3, (slope * (x - x3) - y) % p


def multiply(scalar, point, p, a):
    result = None
    while scalar:
        if scalar & 1:
            result = add(result, point, p, a)
        point = add(point, point, p, a)
        scalar >>= 1
    return result


def character(value, prime):
    power = pow(value % prime, (prime - 1) // 2, prime)
    return 0 if power == 0 else 1 if power == 1 else -1


def sextic(x, a, b, p):
    return (
        x**6 + 5 * a * x**4 + 20 * b * x**3
        - 5 * a * a * x * x - 4 * a * b * x - a**3 - 8 * b * b
    ) % p


def y_product_sign(point, indices, p, a):
    product = 1
    for scalar in indices:
        product = product * multiply(scalar, point, p, a)[1] % p
    return character(product, p)


def check_small_curve(a, b, base, expected_failures, indices):
    p, r, half = 79, 97, 48
    assert multiply(r, base, p, a) is None
    assert base[1] * base[1] % p == (base[0] ** 3 + a * base[0] + b) % p
    assert 1 + sum(
        1 if (x**3 + a * x + b) % p == 0
        else 2 if character(x**3 + a * x + b, p) == 1 else 0
        for x in range(p)
    ) == r
    base_sign = y_product_sign(base, indices, p, a)
    failures = []
    exact_mismatches = 0
    for scalar in range(1, half + 1):
        point = multiply(scalar, base, p, a)
        doubled = multiply(2, point, p, a)
        ratio = y_product_sign(doubled, indices, p, a) * y_product_sign(
            point, indices, p, a
        )
        norm = 1
        for index in indices:
            orbit_point = multiply(index, point, p, a)
            x, y = orbit_point
            f = (x**3 + a * x + b) % p
            derivative = (3 * x * x + a) % p
            assert f != 0
            doubling_ratio = (
                (12 * x * f * derivative - derivative**3 - 8 * f * f)
                * pow(8 * f * f, -1, p)
            ) % p
            assert doubling_ratio == sextic(x, a, b, p) * pow(8 * f * f, -1, p) % p
            assert doubling_ratio == multiply(2, orbit_point, p, a)[1] * pow(y, -1, p) % p
            norm = norm * sextic(x, a, b, p) % p
        assert character(norm, p) == ratio
        if ratio != 1:
            failures.append(scalar)
        if y_product_sign(point, indices, p, a) * base_sign != character(scalar, r):
            exact_mismatches += 1
    assert len(failures) == exact_mismatches == expected_failures
    print(
        f"p={p} a={a} b={b} base={base} classes={half} "
        f"doubling_failures={len(failures)} exact_mismatches={exact_mismatches} "
        f"first_witnesses={failures[:5]}"
    )


small_r, small_m = 97, 16
small_half = (small_r - 1) // 2
primitive = 5
assert pow(primitive, small_half, small_r) == small_r - 1
assert all(pow(primitive, small_half // prime, small_r) not in (1, small_r - 1)
           for prime in (2, 3))
h = pow(primitive, small_half // small_m, small_r)
indices = sorted({min(pow(h, j, small_r), small_r - pow(h, j, small_r))
                  for j in range(small_m)})
assert len(indices) == small_m
assert pow(2, small_m, small_r) not in (1, small_r - 1)
assert pow(2, small_half, small_r) == 1
check_small_curve(0, 3, (1, 2), 0, indices)
check_small_curve(1, 6, (1, 18), 32, indices)

p192_r = 6277101735386680763835789423176059013767194773182842284081
p192_half = (p192_r - 1) // 2
p192_square_order = p192_half // 2
p192_q = 9564682313913860059195669
p192_m = 8 * p192_q
p192_index = p192_half // p192_m
factors = (2, 5, 2389, p192_q, 3433859179316188682119986911)
assert p192_square_order == 4 * factors[1] * factors[2] * factors[3] * factors[4]
assert pow(2, p192_square_order, p192_r) == p192_r - 1
assert all(pow(2, p192_square_order // prime, p192_r) not in (1, p192_r - 1)
           for prime in factors)
assert gcd(p192_square_order, p192_m) == p192_m // 2
assert p192_square_order // gcd(p192_square_order, p192_m) == p192_index
assert character(2, p192_r) == 1
print(f"P-192 scalar quotient: ord(2H)={p192_index}, chi_r(2)=+1")
