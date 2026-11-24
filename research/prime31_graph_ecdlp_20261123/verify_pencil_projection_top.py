#!/usr/bin/env python3
"""Independent finite-field check of the Boolean secant top-ideal claim.

This checks a complete two-bit-per-slot presentation and its exact one-bit
projection over M31. The accompanying note gives the proof for arbitrary k.
Requires SymPy; no curve-point search or discrete-log computation occurs.
"""

from itertools import combinations_with_replacement

import sympy as sp


P = 2**31 - 1
m, x, z, ux0, ux1, uz0, uz1 = sp.symbols("m x z ux0 ux1 uz0 uz1")
a, b, c, t0, w0, w1 = 3, 5, 7, 11, 13, 17
curve_a, curve_b = 847514846, 2087444724
rx, ry = 625037715, 2100627251  # Frozen public B677 generator.
e, f = 1, -1
assert (ry**2 - rx**3 - curve_a * rx - curve_b) % P == 0


def top(poly, variables):
    p = sp.Poly(poly, *variables, modulus=P)
    degree = p.total_degree()
    return sp.expand(
        sum(
            int(coeff) * sp.prod((v**power for v, power in zip(variables, powers)), start=sp.Integer(1))
            for powers, coeff in p.terms()
            if sum(powers) == degree
        )
    )


def same_ideal(generators, expected, variables):
    actual_gb = sp.groebner(generators, *variables, order="grevlex", modulus=P)
    expected_gb = sp.groebner(expected, *variables, order="grevlex", modulus=P)
    assert all(actual_gb.reduce(g)[1] == 0 for g in expected)
    assert all(expected_gb.reduce(g)[1] == 0 for g in generators)


def core_hilbert(generators):
    variables = (m, x, z)
    gb = sp.groebner(generators, *variables, order="grevlex", modulus=P)
    leads = [tuple(int(power) for power in g.LM(order="grevlex").exponents) for g in gb.polys]
    counts = []
    for degree in range(6):
        standard = 0
        for indices in combinations_with_replacement(range(3), degree):
            powers = tuple(indices.count(i) for i in range(3))
            standard += not any(
                all(power >= lead for power, lead in zip(powers, leading))
                for leading in leads
            )
        counts.append(standard)
    return counts


g1 = m**2 - x - z - rx
g2 = x * z - rx**2 - rx * m**2 - curve_a - 2 * m * ry
dx = e * (a * x**2 + b * x + c + t0 + w1 * ux1) - m * (x - rx) + ry
dz = f * (a * z**2 + b * z + c + t0 + w1 * uz1) - m * (z - rx) + ry
qx = dx + e * w0 * ux0
qz = dz + f * w0 * uz0
all_variables = (m, x, z, ux0, ux1, uz0, uz1)
projected_variables = (m, x, z, ux1, uz1)

assert sp.expand((dx + e * w0 * ux0) * (dx + e * w0 - e * w0 * ux0)
                 + w0**2 * (ux0**2 - ux0) - dx * (dx + e * w0)) == 0
assert sp.expand((dz + f * w0 * uz0) * (dz + f * w0 - f * w0 * uz0)
                 + w0**2 * (uz0**2 - uz0) - dz * (dz + f * w0)) == 0

alpha = pow(e * a, -1, P)
beta = pow(f * a, -1, P)
core_all = [m**2, x * z, x**2 - alpha * m * x, z**2 - beta * m * z]
core_projected = [m**2, x * z, (x**2 - alpha * m * x)**2,
                  (z**2 - beta * m * z)**2]
bits_all = [v**2 for v in (ux0, ux1, uz0, uz1)]
bits_projected = [v**2 for v in (ux1, uz1)]

all_equations = [g1, g2, qx, qz] + [v**2 - v for v in (ux0, ux1, uz0, uz1)]
projected_equations = [g1, g2, dx * (dx + e * w0), dz * (dz + f * w0)] \
    + [v**2 - v for v in (ux1, uz1)]
same_ideal([top(g, all_variables) for g in all_equations],
           core_all + bits_all, all_variables)
same_ideal([top(g, projected_variables) for g in projected_equations],
           core_projected + bits_projected, projected_variables)

assert core_hilbert(core_all) == [1, 3, 2, 0, 0, 0]
assert core_hilbert(core_projected) == [1, 3, 4, 4, 2, 0]
assert sum(core_hilbert(core_all)) * 2**16 == 393216
assert sum(core_hilbert(core_projected)) * 2**14 == 229376
print("M31 projected-top verification: PASS")
print("all-bit core Hilbert: [1, 3, 2]; k=8 top quotient length: 393216")
print("projected core Hilbert: [1, 3, 4, 4, 2]; k=8 top quotient length: 229376")
