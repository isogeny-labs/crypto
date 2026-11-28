#!/usr/bin/env python3
"""CT220: independent exact F101 rational-point/prime-order incompatibility audit.

Not an IC benchmark. The abelian-variety cardinality conclusion uses the
Weil bounds and a stated split-Jacobian hypothesis; the finite-point checks
below do not themselves prove an isogeny decomposition.
"""
import argparse
import json
from math import gcd, isqrt

P, R, GENERATOR = 101, 109, (2, 37)


def points(field, equation):
    return {(x, y) for x in range(field) for y in range(field)
            if (y * y - equation(x)) % field == 0}


def ec_add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x, y = a
    u, v = b
    if x == u and (y + v) % P == 0:
        return None
    if x == u:
        if y == 0:
            return None
        m = (3*x*x + 1) * pow(2*y, -1, P) % P
    else:
        m = (v-y) * pow(u-x, -1, P) % P
    z = (m*m-x-u) % P
    return (z, (m*(x-z)-y) % P)


def audit():
    assert gcd(3, P-1) == 1
    e = points(P, lambda z: z*z*z+z+46)
    c = points(P, lambda x: x**9+x**3+46)
    assert len(e) == len(c) == R-1
    image = {(pow(x, 3, P), y) for x, y in c}
    assert image == e and len(image) == len(c)
    assert len({pow(x, 3, P) for x in range(P)}) == P
    assert sum(y == 0 for _, y in e) == 0
    assert len({x for x, _ in c}) == 54
    # Independent point arithmetic: verify the stated source generator's order.
    multiples = [None]
    for _ in range(1, R):
        multiples.append(ec_add(multiples[-1], GENERATOR))
    assert len(set(multiples)) == R and set(multiples[1:]) == e
    assert ec_add(multiples[-1], GENERATOR) is None
    # Projector coefficient: 3*73=219 = 1 mod 109.
    inv_degree = pow(3, -1, R)
    assert inv_degree == 73 and (3*inv_degree) % R == 1
    # Under J_C ~_F101 E x A (dim A=3), Weil gives
    # #A(F101) >= (sqrt(101)-1)^6 > (10-1)^6 = 531441.
    # Hence integer #A >= 531442 and #J=#E*#A is composite.
    floor_root = isqrt(P)
    assert floor_root**2 < P
    lower_complement = (floor_root-1)**6 + 1
    assert lower_complement == 531442
    # Control: the rational-point bijection fails to follow when the cube
    # exponent is not a permutation of the field's multiplicative group.
    assert gcd(3, 7-1) == 3
    assert len({pow(x, 3, 7) for x in range(7)}) < 7
    return {
        'schema': 'ct220-cubic-split-prime-order-obstruction-v1',
        'field': P, 'source_group_order': R, 'source_generator': list(GENERATOR),
        'cover_genus': 4, 'cover_affine_points': len(c),
        'source_affine_points': len(e),
        'cover_to_source_rational_point_bijection': True,
        'cube_map_permutation': True, 'signed_cover_x_factor_symbols': 54,
        'projector_scalar_inverse_degree_mod_r': inv_degree,
        'complement_dimension_if_split': 3,
        'rigorous_complement_cardinality_at_least_if_split': lower_complement,
        'rigorous_jacobian_cardinality_at_least_if_split': R*lower_complement,
        'prime_order_jacobian_compatible_with_split': False,
        'negative_control_cube_map_F7_nonpermutation': True,
        'standing': 'exact toy point census plus conditional standard Weil/isogeny proof; no speedup or independent PCC',
        'pcc': 'unverified',
    }


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--self-test', action='store_true')
    args = ap.parse_args()
    result = audit()
    if args.self_test:
        print('CT220_SELFTEST_PASS', result['source_group_order'],
              result['signed_cover_x_factor_symbols'],
              result['rigorous_complement_cardinality_at_least_if_split'])
    else:
        print(json.dumps(result, sort_keys=True, indent=2))
