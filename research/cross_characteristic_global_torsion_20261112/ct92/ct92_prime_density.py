#!/usr/bin/env python3
"""CT92: exact GL2 point-stabilizer/Chebotarev count (no external packages)."""
from fractions import Fraction
import json


def gl2_count(ell):
    group = 0
    has_fixed = 0
    total_fixed = 0
    histogram = {0: 0, ell - 1: 0, ell * ell - 1: 0}
    # Enumerate every matrix and independently every nonzero vector.
    vectors = [(x, y) for x in range(ell) for y in range(ell) if (x, y) != (0, 0)]
    for a in range(ell):
        for b in range(ell):
            for c in range(ell):
                for d in range(ell):
                    if (a * d - b * c) % ell == 0:
                        continue
                    group += 1
                    fixed = sum(1 for x, y in vectors
                                if (a * x + b * y - x) % ell == 0
                                and (c * x + d * y - y) % ell == 0)
                    if fixed:
                        has_fixed += 1
                    total_fixed += fixed
                    if fixed not in histogram:
                        raise AssertionError(('unexpected eigenspace dimension', ell, fixed))
                    histogram[fixed] += 1
    assert group == ell * (ell - 1)**2 * (ell + 1)
    assert has_fixed == ell * (ell * ell - 2)
    assert total_fixed == group  # Burnside: transitive action on nonzero vectors.
    assert histogram[ell * ell - 1] == 1
    assert histogram[ell - 1] == has_fixed - 1
    density = Fraction(has_fixed, group)
    assert density == Fraction(ell * ell - 2, (ell - 1)**2 * (ell + 1))
    return {'ell': ell, 'gl2_order': group, 'matrices_with_eigenvalue_one': has_fixed,
            'degree_one_prime_counts': [[k, histogram[k]] for k in sorted(histogram)],
            'chebotarev_density': f'{density.numerator}/{density.denominator}',
            'average_degree_one_places': 1,
            'oriented_field_degree_if_transitive': ell * ell - 1,
            'x_field_degree_if_transitive': (ell * ell - 1)//2,
            'dense_monic_x_polynomial_entries': (ell * ell + 1)//2,
            'source_subgroup_full_lookup_entries': ell}


def count_curve(p):
    return 1 + sum(1 for x in range(p) for y in range(p)
                   if (y*y + y - x*x*x + x*x + 10*x + 20) % p == 0)



def residue_orbits(matrix, projective_sign=False):
    """Frobenius cycle lengths on oriented or ±-unoriented F7² torsion."""
    ell = 7
    def normal(v):
        opposite = tuple((-z) % ell for z in v)
        return min(v, opposite) if projective_sign else v
    def apply(v):
        return ((matrix[0][0]*v[0] + matrix[0][1]*v[1]) % ell,
                (matrix[1][0]*v[0] + matrix[1][1]*v[1]) % ell)
    remaining = {normal((a, b)) for a in range(ell) for b in range(ell) if (a, b) != (0, 0)}
    degrees = {}
    while remaining:
        first = next(iter(remaining))
        v = first
        length = 0
        while True:
            if v not in remaining:
                raise ValueError('non-permutation')
            remaining.remove(v)
            length += 1
            v = normal(apply(v))
            if v == first:
                break
        degrees[length] = degrees.get(length, 0) + 1
    return [[k, degrees[k]] for k in sorted(degrees)]


def main():
    source = count_curve(37)
    target = count_curve(2)
    assert source == 35 and target == 5
    assert (37 + 1 - source) % 7 == 3
    assert (37 % 7) == 2
    assert {1, 2} == {z for z in range(7) if (z*z - 3*z + 2) % 7 == 0}
    source_oriented = residue_orbits(((1, 0), (0, 2)))
    source_x = residue_orbits(((1, 0), (0, 2)), True)
    binary_oriented = residue_orbits(((0, 5), (1, 5)))
    binary_x = residue_orbits(((0, 5), (1, 5)), True)
    assert source_oriented == [[1, 6], [3, 14]] and source_x == [[1, 3], [3, 7]]
    assert binary_oriented == [[24, 2]] and binary_x == [[12, 2]]
    result = {'source_oriented_residue_degrees': source_oriented,
              'source_x_residue_degrees': source_x,
              'binary_oriented_residue_degrees': binary_oriented,
              'binary_x_residue_degrees': binary_x,
              'experiment': 'ct92-degree-one-place-density',
              'galois_image_hypothesis': 'full_GL2_mod_ell',
              'source_prime': 37, 'source_curve_order': source,
              'source_trace': 3, 'source_eigenvalues_mod_7': [1, 2],
              'source_degree_one_primes': 6,
              'binary_curve_order': target,
              'ell_cases': [gl2_count(ell) for ell in (3, 5, 7, 11)]}
    print(json.dumps(result, sort_keys=True, separators=(',', ':')))


if __name__ == '__main__':
    main()
