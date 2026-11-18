#!/usr/bin/env python3
"""Independent exact Frobenius Galois-equivariance gate for CT80 / CT94."""
import json


def check(ok, why):
    if not ok:
        raise ValueError(why)


def mul(a, b, ell):
    return tuple(sum(a[2 * i + k] * b[2 * k + j] for k in range(2)) % ell
                 for i in range(2) for j in range(2))


def apply(a, vector, ell):
    return tuple((a[2 * i] * vector[0] + a[2 * i + 1] * vector[1]) % ell
                 for i in range(2))


def frobenius_data(ell):
    # The companion of X² + 2X + 2: column-vector convention.
    a = (0, (-2) % ell, 1, (-2) % ell)
    power = (1, 0, 0, 1)
    minimum = None
    for d in range(1, ell * ell + 1):
        power = mul(power, a, ell)
        if ((power[0] - 1) * (power[3] - 1) - power[1] * power[2]) % ell == 0:
            minimum = d
            break
    check(minimum is not None, "missing fixed vector")
    return a, minimum


def calculate():
    f2_points = 1 + sum(
        ((y * y + y - (x**3 - x*x - 10*x - 20)) % 2 == 0)
        for x in range(2) for y in range(2)
    )
    check(f2_points == 5, "curve count over F2")
    trace = 3 - f2_points
    check(trace == -2, "trace")
    table = [[ell, frobenius_data(ell)[1]] for ell in (3, 5, 7, 11, 13, 17, 19, 23)]
    check(table == [[3, 8], [5, 1], [7, 24], [11, 40], [13, 6], [17, 16], [19, 72], [23, 88]], "table")

    ell = 7
    a, min_d = frobenius_data(ell)
    power = (1, 0, 0, 1)
    powers = {}
    nonzero_fixed = []
    for d in range(1, 25):
        power = mul(power, a, ell)
        if d in (4, 12, 24):
            powers[str(d)] = [[power[0], power[1]], [power[2], power[3]]]
        count = sum(apply(power, (x, y), ell) == (x, y)
                    for x in range(ell) for y in range(ell) if x or y)
        nonzero_fixed.append(count)
    check(min_d == 24 and nonzero_fixed == [0] * 23 + [48], "fixed-vector gate")
    check(powers == {
        "4": [[3, 0], [0, 3]], "12": [[6, 0], [0, 6]],
        "24": [[1, 0], [0, 1]]}, "Frobenius powers")
    # Exhaust the actual F7^2 module, proving 2 cycles of length 24.
    unseen = {(x, y) for x in range(ell) for y in range(ell) if x or y}
    orbit_lengths = []
    while unseen:
        v0 = min(unseen)
        v = v0
        orb = set()
        while v not in orb:
            orb.add(v)
            v = apply(a, v, ell)
        check(v == v0, "noncycle")
        unseen.difference_update(orb)
        orbit_lengths.append(len(orb))
    orbit_lengths.sort()
    check(orbit_lengths == [24, 24], "orbit lengths")
    return {
        "curve_F2_points": f2_points,
        "frobenius_trace": trace,
        "ell7": {
            "extension_degree": min_d,
            "fixed_nonzero_d1_to_d24": nonzero_fixed,
            "frobenius_powers": powers,
            "nonzero_orbit_lengths": orbit_lengths,
            "equivariant_nonzero_maps_F2": 0,
            "equivariant_nonzero_maps_F2_24": 48,
        },
        "minimum_extension_by_ell": table,
    }


if __name__ == "__main__":
    print(json.dumps(calculate(), sort_keys=True, separators=(",", ":")))
