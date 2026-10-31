#!/usr/bin/env python3
"""Check the base-field-model gate for the published Tian cubic source.

The trace-zero variety over F_523 requires an elliptic curve defined over
F_523.  A non-base j-invariant rules out such a model for this fixture.
"""

import json

P = 523


def add(a, b):
    return tuple((x+y) % P for x, y in zip(a, b))


def mul(a, b):
    coefficients = [0]*5
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            coefficients[i+j] += ai*bj
    # w^3 = -2 in F_523[w]/(w^3+2).
    for i in [4, 3]:
        coefficients[i-3] -= 2*coefficients[i]
    return tuple(x % P for x in coefficients[:3])


def power(a, exponent):
    out = (1, 0, 0)
    while exponent:
        if exponent & 1:
            out = mul(out, a)
        a = mul(a, a)
        exponent >>= 1
    return out


def scale(a, n):
    return tuple(n*x % P for x in a)


def run():
    a = (140, 92, 82)
    b = (243, 339, 359)
    four_a_cubed = scale(power(a, 3), 4)
    denominator = add(four_a_cubed, scale(power(b, 2), 27))
    assert denominator != (0, 0, 0)
    j = scale(mul(four_a_cubed, power(denominator, P**3-2)), 1728)
    assert mul(j, denominator) == scale(four_a_cubed, 1728)
    j_frob = power(j, P)
    return {"schema": "cubic-trace-zero-gate/v1", "field": "F_523[w]/(w^3+2)",
            "a": a, "b": b, "j": j, "j_frobenius": j_frob,
            "j_in_base_field": j == j_frob,
            "base_field_model_possible": j == j_frob,
            "gate": "a non-base j rules out an F_523 elliptic model"}


if __name__ == "__main__":
    print(json.dumps(run(), sort_keys=True))
