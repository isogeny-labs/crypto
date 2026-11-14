#!/usr/bin/env python3
"""Validator's own j-invariant / automorphism check for y^2 = x^3 + a2 x^2 + a4 x + a6.
j = c4^3 / Delta with b2 = 4a2, b4 = 2a4, b6 = 4a6, b8 = 4 a2 a6 - a4^2,
c4 = b2^2 - 24 b4, c6 = -b2^3 + 36 b2 b4 - 216 b6, Delta = -b2^2 b8 - 8 b4^3 - 27 b6^2 + 9 b2 b4 b6.
Over F_p, p >= 5: Aut(E) is larger than {+-1} iff j = 0 (c4 = 0) or j = 1728 (c6 = 0)."""

def jinfo(p, a2, a4, a6):
    b2, b4, b6 = 4 * a2 % p, 2 * a4 % p, 4 * a6 % p
    b8 = (4 * a2 * a6 - a4 * a4) % p
    c4 = (b2 * b2 - 24 * b4) % p
    c6 = (-b2 ** 3 + 36 * b2 * b4 - 216 * b6) % p
    D = (-b2 * b2 * b8 - 8 * b4 ** 3 - 27 * b6 * b6 + 9 * b2 * b4 * b6) % p
    assert D != 0, 'singular'
    j = c4 ** 3 * pow(D, -1, p) % p
    # consistency: 1728 Delta = c4^3 - c6^2
    assert (1728 * D - (c4 ** 3 - c6 * c6)) % p == 0
    return {'j': j, 'c4_zero': c4 == 0, 'c6_zero': c6 == 0,
            'extra_automorphism': c4 == 0 or c6 == 0}
