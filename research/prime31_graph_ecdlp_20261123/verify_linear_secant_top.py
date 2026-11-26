#!/usr/bin/env python3
"""Check a redundant-generator linear secant top ideal over M31.

This verifies polynomial identities and leading ideals for four sign
branches. It performs no relation search or discrete-log computation.
"""

import json
from itertools import combinations_with_replacement, product

import sympy as sp


P = 2**31 - 1
CURVE_A, CURVE_B = 847514846, 2087444724
RX, RY = 625037715, 2100627251
assert (RY * RY - RX**3 - CURVE_A * RX - CURVE_B) % P == 0

m, x, z, ux0, ux1, uz0, uz1 = sp.symbols("m x z ux0 ux1 uz0 uz1")
variables = (m, x, z, ux0, ux1, uz0, uz1)
g1 = m * m - x - z - RX
g2 = x * z - RX**2 - RX * m * m - CURVE_A - 2 * m * RY
bits = [v * v - v for v in (ux0, ux1, uz0, uz1)]
source_top = [m * m, x * z, m * x, m * z]
source_top += [v * v for v in (ux0, ux1, uz0, uz1)]
source_expected_gb = sp.groebner(source_top, *variables, modulus=P, order="grevlex")
expected = [m * m, x * z, m * x, m * z, x**3, z**3]
expected += [v * v for v in (ux0, ux1, uz0, uz1)]
expected_gb = sp.groebner(expected, *variables, modulus=P, order="grevlex")


def line(value):
    return m * (value - RX) - RY


def curve(value):
    return value**3 + CURVE_A * value + CURVE_B


def top(expression):
    polynomial = sp.Poly(expression, *variables, modulus=P)
    degree = polynomial.total_degree()
    return sp.Add(*(
        int(coefficient) * sp.prod(variable**power for variable, power in zip(variables, powers))
        for powers, coefficient in polynomial.terms()
        if sum(powers) == degree
    ))


for sign_x, sign_z in product((1, -1), repeat=2):
    hx = 3 * x + 16 + 13 * ux0 + 17 * ux1
    hz = 3 * z + 16 + 13 * uz0 + 17 * uz1
    qx, qz = sign_x * hx - line(x), sign_z * hz - line(z)
    cx, cz = hx * hx - curve(x), hz * hz - curve(z)
    for value, h, sign, q, c in (
        (x, hx, sign_x, qx, cx), (z, hz, sign_z, qz, cz)
    ):
        certificate = q * (sign * h + line(value))
        certificate += (value - RX) * (value * g1 + g2)
        assert sp.Poly(c - certificate, *variables, modulus=P).is_zero
        assert sp.Poly(certificate, *variables, modulus=P).total_degree() <= 4
    original = [top(equation) for equation in (g1, g2, qx, qz, *bits)]
    original_gb = sp.groebner(original, *variables, modulus=P, order="grevlex")
    assert all(source_expected_gb.reduce(equation)[1] == 0 for equation in original)
    assert all(original_gb.reduce(equation)[1] == 0 for equation in source_top)
    assert original_gb.reduce(x**3)[1] == x**3
    assert original_gb.reduce(z**3)[1] == z**3
    actual = [top(equation) for equation in (g1, g2, qx, qz, *bits, cx, cz)]
    actual_gb = sp.groebner(actual, *variables, modulus=P, order="grevlex")
    assert all(expected_gb.reduce(equation)[1] == 0 for equation in actual)
    assert all(actual_gb.reduce(equation)[1] == 0 for equation in expected)
    assert actual_gb.reduce(x * x * ux0 * ux1 * uz0 * uz1)[1] != 0

core_gb = sp.groebner(expected[:6], m, x, z, modulus=P, order="grevlex")
core_hilbert = []
for degree in range(4):
    count = 0
    for indices in combinations_with_replacement((m, x, z), degree):
        monomial = sp.prod(indices, start=sp.Integer(1))
        count += core_gb.reduce(monomial)[1] != 0
    core_hilbert.append(count)
assert core_hilbert == [1, 3, 2, 0]

print(json.dumps({
    "field": P,
    "sign_branches": 4,
    "degree_three_cubics_independent": True,
    "core_hilbert": core_hilbert[:3],
    "k8_top_length": 6 * 2**16,
    "k8_first_vanishing": 19,
    "k10_top_length": 6 * 2**20,
    "k10_first_vanishing": 23,
}, sort_keys=True))
