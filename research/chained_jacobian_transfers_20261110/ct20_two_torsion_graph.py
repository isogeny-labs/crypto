#!/usr/bin/env python3
"""CT20b: small symbolic 2-torsion graph census, not geometric verification.
Run: python3 -S ct20_two_torsion_graph.py
No cryptographic curves or extension-field/Cantor engine imported.
[pcc:unverified]
"""
from collections import Counter
from itertools import combinations, product

# Nonzero elements of (Z/2)^2; distinct roots of each elliptic cubic.
T = (1, 2, 3)
# The hyperelliptic branch points of C: y^2=h(t^2) are (i,+/-).
B = tuple(product(range(3), (1, -1)))


def norm_image(pair):
    # At a branch point, both elliptic quotient maps have y=0 and
    # corresponding nonzero 2-torsion labels i. Signs disappear.
    (i, _), (j, _) = pair
    return (T[i] ^ T[j], T[i] ^ T[j])


def in_target_glue(point, rotated=False):
    # Correct image graph is (z,z). The negative control uses a different
    # elliptic 2-torsion matching (three-cycle on the nonzero labels).
    a, b = point
    perm = {0: 0, 1: 2, 2: 3, 3: 1} if rotated else {t: t for t in (0, 1, 2, 3)}
    return b == perm[a]


def main():
    assert len(B) == 6
    pairs = list(combinations(B, 2))
    assert len(pairs) == 15
    norms = Counter(norm_image(pair) for pair in pairs)
    assert norms == {(0, 0): 3, (1, 1): 4, (2, 2): 4, (3, 3): 4}
    assert all(in_target_glue(norm_image(pair)) for pair in pairs)
    assert sum(in_target_glue(norm_image(pair), rotated=True) for pair in pairs) == 3
    assert sum(not in_target_glue(norm_image(pair), rotated=True) for pair in pairs) == 12

    # Each opposite pair is a distinct nonzero J[2] class in ker(N0).
    # Together with zero these form the four-element graph-kernel.
    opposite = [q for q in pairs if q[0][0] == q[1][0]]
    assert len(opposite) == 3 and all(norm_image(q) == (0, 0) for q in opposite)

    # Degree/dual bookkeeping assuming actual geometric 2-gluings P_i,N_i
    # with P_i N_i=N_i P_i=[2] and alphaHat alpha=[7].
    dim = 2
    deg_p = deg_n = 4
    deg_alpha = 7 ** dim
    deg_two = 2 ** (2 * dim)
    assert deg_p * deg_alpha * deg_n == 784
    assert (deg_p * deg_alpha * deg_n) // deg_two == 49
    assert (2 * 7 * 2) // 4 == 7
    # On rational source components of coprime orders (109, 113), [7]
    # is an automorphism. These checks are algebra, not a map test.
    assert all((7 * pow(7, -1, m)) % m == 1 for m in (109, 113))
    print('15 branch pairs: 3 zero, 4 per nonzero matched-graph element')
    print('correct graph 15/15; rotated-matching negative control 3/15')
    print('conditional degree of descended Phi = 49; dual multiplier = 7')
    print('[pcc:unverified] symbolic check only; geometric map unimplemented')


if __name__ == '__main__':
    main()
